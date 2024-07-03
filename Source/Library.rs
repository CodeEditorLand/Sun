use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt, net::TcpListener};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[derive(Serialize, Deserialize)]
struct FileEdit {
	path: String,
	content: String,
}

#[tokio::main]
async fn main() {
	let Listen = TcpListener::bind("127.0.0.1:9999").await.unwrap();

	while let Ok((stream, _)) = Listen.accept().await {
		tokio::spawn(Handler(stream));
	}
}

async fn Handler(Stream: tokio::net::TcpStream) {
	let Stream = accept_async(Stream).await.unwrap();
	let (Write, Read) = Stream.split();

	Read.for_each(|message| async {
		if let Ok(msg) = message {
			if msg.is_text() {
				let file_edit: FileEdit = serde_json::from_str(msg.to_text().unwrap()).unwrap();
				let mut file = File::create(file_edit.path).await.unwrap();
				file.write_all(file_edit.content.as_bytes()).await.unwrap();
			}
		}
	})
	.await;
}
