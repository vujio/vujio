extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use vujio_client::*;

    #[test]
    fn it_works() {
        bundler::bundle(
            "src/main.ts",
            &bundler::BundlerConfig {
                minify: true,
                compat: true,
                source_maps: true,
            },
        );
    }

    #[bench]
    fn bench_compiler(b: &mut Bencher) {
        b.iter(|| {
            bundler::bundle(
                "src/main.ts",
                &bundler::BundlerConfig {
                    minify: true,
                    compat: false,
                    source_maps: true,
                },
            )
        });
    }
}
