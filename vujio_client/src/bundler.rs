use anyhow::Error;
use relative_path::RelativePathBuf;
use std::collections::HashMap;
use std::{
    fmt::{self, Display, Formatter},
    io::{self, Write},
    path::Path,
    sync::{Arc, RwLock},
};
use swc::{
    config::{JscConfig, SourceMapsConfig},
    Compiler,
};
use swc_bundler::{Bundler, Config, Hook, Load, ModuleData, ModuleRecord, Resolve};
use swc_common::{
    errors::{self, DiagnosticBuilder, Handler},
    sync::Lrc,
    FileName, FilePathMapping, Globals, SourceFile, SourceMap, Span,
};
use swc_ecma_ast::KeyValueProp;
use swc_ecma_codegen::{text_writer::JsWriter, Emitter};
use swc_ecma_parser::{lexer::Lexer, JscTarget, Parser, StringInput, Syntax, TsConfig};

//#[path = "mangle.rs"]
//mod mangle;

pub struct BundlerConfig {
    pub minify: bool,
    pub compat: bool,
    pub source_maps: bool,
}

fn compiler_from_entry(entry_path: &str, cm: &Arc<SourceMap>) -> (Compiler, Arc<SourceFile>) {
    let mut bundle_vector: Vec<u8> = Vec::new();

    let globals = Globals::new();
    let external_modules = vec![];
    let bundler = Bundler::new(
        &globals,
        cm.clone(),
        PathLoader { cm: cm.clone() },
        PathResolver,
        Config {
            require: false,
            external_modules,
            ..Default::default()
        },
        Box::new(Noop),
    );
    let mut entries = HashMap::default();
    entries.insert("main".to_string(), FileName::Real(entry_path.into()));

    {
        let bundle = bundler
            .bundle(entries)
            .expect("failed to bundle entries")
            .pop()
            .unwrap();

        let mut emitter = Emitter {
            cfg: swc_ecma_codegen::Config {
                minify: false, // Don't minify, removes whitespace from sourcemaps.
            },
            cm: cm.clone(),
            comments: None,
            wr: Box::new(JsWriter::new(cm.clone(), "\n", &mut bundle_vector, None)),
        };

        emitter.emit_module(&bundle.module).unwrap();
    }

    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let handler = Arc::new(Handler::with_emitter(
        true,
        false,
        Box::new(MyEmiter::default()),
    ));

    let c = swc::Compiler::new(cm.clone(), handler);
    let fm = c.cm.new_source_file(
        FileName::Real(entry_path.into()),
        String::from_utf8(bundle_vector).unwrap(),
    );

    return (c, fm);
}

pub fn bundle(entry_path: &str, bundler_config: &BundlerConfig) -> String {
    let cm = Lrc::new(SourceMap::new(FilePathMapping::empty()));

    let (c, fm) = compiler_from_entry(entry_path, &cm);

    let jsc_target: JscTarget = match bundler_config.compat {
        true => JscTarget::Es2015,
        false => JscTarget::Es2020,
    };

    let source_maps_config = match bundler_config.source_maps {
        true => SourceMapsConfig::Str(String::from("inline")),
        false => SourceMapsConfig::Bool(false),
    };

    let swc_options = swc::config::Options {
        config: Some(swc::config::Config {
            jsc: JscConfig {
                syntax: Some(Syntax::Typescript(TsConfig {
                    tsx: true,
                    decorators: true,
                    ..Default::default()
                })),
                transform: Some(swc::config::TransformConfig {
                    optimizer: Some(swc::config::OptimizerConfig {
                        globals: None,
                        jsonify: None,
                    }),
                    ..Default::default()
                }),
                external_helpers: true,
                target: jsc_target,
                loose: false,
            },
            minify: Some(bundler_config.minify),
            ..Default::default()
        }),
        swcrc: false,
        is_module: false,
        source_maps: Some(source_maps_config),
        ..Default::default()
    };

    let program = c
        .parse_js(
            fm.clone(),
            jsc_target,
            Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: true,
                ..Default::default()
            }),
            true,
            true,
        )
        .unwrap();

    //program = c.transform(program, true, &mut mangle::mangle());

    let output = c
        .process_js(program, &swc_options)
        .expect("failed to process js file");

    return output.code;
}

#[derive(Clone, Default)]
struct MyEmiter(BufferedError);

impl errors::Emitter for MyEmiter {
    fn emit(&mut self, db: &DiagnosticBuilder<'_>) {
        let z = &(self.0).0;
        for msg in &db.message {
            z.write().unwrap().push_str(&msg.0);
        }
    }
}

#[derive(Clone, Default)]
pub(crate) struct BufferedError(Arc<RwLock<String>>);

impl Write for BufferedError {
    fn write(&mut self, d: &[u8]) -> io::Result<usize> {
        self.0
            .write()
            .unwrap()
            .push_str(&String::from_utf8_lossy(d));

        Ok(d.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Display for BufferedError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.0.read().unwrap(), f)
    }
}

struct PathLoader {
    cm: Lrc<SourceMap>,
}

impl Load for PathLoader {
    fn load(&self, file: &FileName) -> Result<ModuleData, Error> {
        let file = match file {
            FileName::Real(v) => v,
            _ => unreachable!(),
        };

        let fm = self.cm.load_file(file)?;
        let lexer = Lexer::new(
            Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: true,
                ..Default::default()
            }),
            Default::default(),
            StringInput::from(&*fm),
            None,
        );

        let mut parser = Parser::new_from(lexer);
        let module = parser.parse_typescript_module().unwrap();

        Ok(ModuleData {
            fm,
            module,
            helpers: Default::default(),
        })
    }
}

struct PathResolver;

impl Resolve for PathResolver {
    fn resolve(&self, base: &FileName, module_specifier: &str) -> Result<FileName, Error> {
        let base = match base {
            FileName::Real(v) => v,
            _ => unreachable!(),
        };

        let base_path = base
            .parent()
            .unwrap()
            .join(format!("{}{}", module_specifier, ".ts"));

        let base_path = RelativePathBuf::from_path(base_path)
            .unwrap()
            .normalize()
            .to_path(Path::new("."));

        Ok(FileName::Real(base_path))
    }
}

struct Noop;

impl Hook for Noop {
    fn get_import_meta_props(&self, _: Span, _: &ModuleRecord) -> Result<Vec<KeyValueProp>, Error> {
        unimplemented!()
    }
}
