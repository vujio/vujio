#![feature(async_closure)]
#![feature(array_map)]
#![feature(test)]

use vujio::*;

#[cfg(test)]
mod tests;

#[derive(Clone)]
struct AppState {
    client_bundle: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientState {
    some_string: String,
}

#[server("127.0.0.1:8080", AppState, ClientState)]
#[async_std::main]
async fn main() {
    #[get_html("/")]
    async fn homepage(_req: Request<AppState>) -> String {
        let body = "<script src=\"bundle\"></script>Hello World!".to_string();

        let directory_links = ["test_path"].map(|v| format!("<a href=\"{}\">{}</a>", v, v));

        let directory_list = format!("<p>Directory:<ul>{}</ul></p>", directory_links.join(""));

        format!(
            "{}{}<p><a href=\"https://github.com/vujio/vujio\">Fork vuj.io on GitHub!</a></p>",
            body, directory_list
        )
    }

    #[get_html]
    async fn test_path(_req: Request<AppState>) -> String {
        "Hello World! - /test_path<br><a href=\"https://github.com/vujio/vujio\">Fork vuj.io on GitHub!</a>".into()
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
