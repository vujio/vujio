# vuj.io

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
}