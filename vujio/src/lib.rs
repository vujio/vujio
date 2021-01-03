/// vuj.io
///
/// An _experimental_ fast and pertinent web platform for modern devices.
/// Rust backend and TypeScript frontend.
///
/// See [https://github.com/vujio/vujio](https://github.com/vujio/vujio)
///
/// Example:
/// ```
/// use vujio::*;
///
/// #[server("127.0.0.1:8080")]
/// async fn main() {
///     #[get_html("/")]
///     async fn main(_req: Request<AppState>) -> String {
///         let directory_links = ["test_path"].map(|v| format!("<a href=\"{}\">{}</a>", v, v));
///         let directory_list = format!("<p>Directory:<ul>{}</ul></p>", directory_links.join(""));
///
///         format!(
///             "Pages:<br>{}", directory_list
///         )
///     }
///
///     #[get_html]
///     async fn test_path(_req: Request<AppState>) -> String {
///         "Page: /test_path".into()
///     }
///
///     #[message("/websocket")]
///     async fn message(stream: &WebSocketConnection, input: String) {
///         println!("Client says: {:?}", input);
///         stream.send_string("server response".into()).await;
///     }
///
///     #[binary_stream("/ws")]
///     async fn my_stream(stream: &WebSocketConnection, input: Vec<u8>) {
///         stream.send_bytes(input).await;
///     }
/// }
/// ```
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, NestedMeta, ReturnType};

#[proc_macro_attribute]
pub fn server(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let mut input = parse_macro_input!(input as ItemFn);

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &mut input.sig;
    let body = &input.block;

    let mut listen: String = "127.0.0.1:8080".into();
    if args.len() > 0 {
        listen = match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => unreachable!(),
        };
    }

    sig.output = ReturnType::Default;

    let output = quote!(
        use vujio_client::*;
        use vujio_server::*;

        #(#attrs)*
        #vis #sig -> vujio_server::Result<()> {
            let client_bundle = bundler::bundle(
                "src/main.ts",
                &bundler::BundlerConfig {
                    minify: true,
                    compat: false,
                    source_maps: cfg!(debug_assertions),
                },
            );

            let state = AppState { client_bundle };
            let mut app = vujio_server::with_state(state);

            app.at("/").get(async move |_req: vujio_server::Request<AppState>| -> vujio_server::Result {
                Ok(vujio_server::Response::builder(200)
                    .body("<!DOCTYPE html><html><head><title></title><script src=\"bundle\"></script></head><body></body></html>")
                    .content_type(mime::HTML)
                    .build())
            });

            app.at("/bundle").get(async move |req: vujio_server::Request<AppState>| -> vujio_server::Result<String> {
                Ok(req.state().client_bundle.clone())
            });

            /*
            app.at("/ws").get(WebSocket::new(|_request, mut stream| async move {
                while let Some(Ok(Message::Binary(input))) = stream.next().await {
                    stream.send_bytes(input).await?;
                }

                Ok(())
            }));
            */

            #body;

            app.at("/favicon.ico").get(async move |_req: vujio_server::Request<AppState>| -> vujio_server::Result {
                Ok(vujio_server::Response::builder(404).content_type(mime::ICO).build())
            });

            println!("Listen {}", #listen);
            app.listen(#listen).await?;
            Ok(())
        }
    );

    output.into()
}

#[proc_macro_attribute]
pub fn get_html(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let mut input = parse_macro_input!(input as ItemFn);

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &mut input.sig;
    let body = &input.block;

    let mut app_path = format!("/{}", sig.ident.to_string());
    if args.len() > 0 {
        app_path = match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => unreachable!(),
        };
    }

    let function_ident = sig.ident.clone();
    sig.output = ReturnType::Default;

    let output = quote!(
        app.at(#app_path).get(#function_ident);

        #(#attrs)*
        #vis #sig -> vujio_server::Result {
            let result: String = (async move || {
                #body
            })().await;

            Ok(vujio_server::Response::builder(200)
                .body(result)
                .content_type(mime::HTML)
                .build())
        }
    );

    output.into()
}

#[proc_macro_attribute]
pub fn binary_stream(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let mut input = parse_macro_input!(input as ItemFn);

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &mut input.sig;
    let body = &input.block;

    let mut app_path = format!("/{}", sig.ident.to_string());
    if args.len() > 0 {
        app_path = match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => unreachable!(),
        };
    }

    let function_ident = sig.ident.clone();
    sig.output = ReturnType::Default;

    let output = quote!(
        app.at(#app_path).get(WebSocket::new(|_request, mut stream| async move {
            while let Some(Ok(Message::Binary(input))) = stream.next().await {
                let result = #function_ident(&stream, input).await;
                match result {
                    (()) => (),
                    _ => println!("Failed: {}", #app_path),
                }
            }
            Ok(())
        }));

        #(#attrs)*
        #vis #sig {
            #body
        }
    );

    output.into()
}

#[proc_macro_attribute]
pub fn message(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let mut input = parse_macro_input!(input as ItemFn);

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &mut input.sig;
    let body = &input.block;

    let mut app_path = format!("/{}", sig.ident.to_string());
    if args.len() > 0 {
        app_path = match &args[0] {
            NestedMeta::Lit(Lit::Str(lit)) => lit.value(),
            _ => unreachable!(),
        };
    }

    let function_ident = sig.ident.clone();
    sig.output = ReturnType::Default;

    let output = quote!(
        app.at(#app_path).get(WebSocket::new(|_request, mut stream| async move {
            while let Some(Ok(Message::Text(input))) = stream.next().await {
                let result = #function_ident(&stream, input).await;
                match result {
                    (()) => (),
                    _ => println!("Failed: {}", #app_path),
                }
            }
            Ok(())
        }));

        #(#attrs)*
        #vis #sig {
            #body
        }
    );

    println!("{}", output);

    output.into()
}
