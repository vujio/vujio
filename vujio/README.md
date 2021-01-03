# vuj.io

[![github.com vujio](https://img.shields.io/badge/github-vujio-informational?style=flat-square&logo=github)](https://crates.io/crates/vujio)
[![crates.io vujio](https://img.shields.io/crates/v/vujio.svg?style=flat-square&logo=rust)](https://crates.io/crates/vujio)

_/vu-hē-oʊ/_

## Description

An _experimental_ fast and pertinent web platform for modern devices.  
Rust backend and TypeScript frontend.

See [https://github.com/vujio/vujio](https://github.com/vujio/vujio)

Example:

```
use vujio::*;

#[server("127.0.0.1:8080")]
async fn main() {
    #[get_html("/")]
    async fn main(_req: Request<AppState>) -> String {
        let directory_links = ["test_path"].map(|v| format!("<a href=\"{}\">{}</a>", v, v));
        let directory_list = format!("<p>Directory:<ul>{}</ul></p>", directory_links.join(""));

        format!(
            "Pages:<br>{}", directory_list
        )
    }

    #[get_html]
    async fn test_path(_req: Request<AppState>) -> String {
        "Page: /test_path".into()
    }

    #[message("/websocket")]
    async fn message(stream: &WebSocketConnection, input: String) {
        println!("Client says: {:?}", input);
        stream.send_string("server response".into()).await;
    }

    #[binary_stream("/ws")]
    async fn my_stream(stream: &WebSocketConnection, input: Vec<u8>) {
        stream.send_bytes(input).await;
    }
}
```
