#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt, net::TcpListener};

#[derive(Serialize, Deserialize)]
struct FileEdit {
	Path: String,
	Content: String,
}

#[tokio::main]
async fn main() {
	while let Ok((Stream, _)) = TcpListener::bind("127.0.0.1:9999").await.unwrap().accept().await {
		tokio::spawn(Handler(Stream));
	}
}

async fn Handler(Stream: tokio::net::TcpStream) {
	let (Read, Write) = tokio_tungstenite::accept_async(Stream).await.unwrap().get_ref().split();

	Read.for_each(|Message| async {
		if let Ok(Message) = Message {
			if Message.is_text() {
				let Edit: FileEdit = serde_json::from_str(Message.to_text().unwrap()).unwrap();

				File::create(Edit.Path)
					.await
					.unwrap()
					.write_all(Edit.Content.as_bytes())
					.await
					.unwrap();
			}
		}
	})
	.await;
}
