use codeeditorland_echo::{Action, ActionResult, Job, Work, Worker, Yell};

use futures::future::join_all;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::mpsc};
use tokio_tungstenite::accept_async;

struct Worker;

#[async_trait::async_trait]
impl Worker for Worker {
	async fn Receive(&self, Action: Action) -> ActionResult {
		Box::pin(async move {
			match Action {
				Action::Write { Path, Content } => match tokio::fs::write(&Path, &Content).await {
					Ok(_) => {
						ActionResult { Action, ActionResult: Ok(format!("File written: {}", Path)) }
					}
					Err(Error) => ActionResult {
						Action,
						ActionResult: Err(format!("Failed to write file: {}", Error)),
					},
				},
				_ => ActionResult {
					Action,
					ActionResult: Err("Invalid Action for Worker".to_string()),
				},
			}
		})
	}
}

#[tokio::main]
async fn main() {
	let Work = Arc::new(Work::new());
	let (tx, rx) = mpsc::channel(100);

	let num_workers = 4;
	let workers: Vec<_> = (0..num_workers)
		.map(|_| {
			let worker = Arc::new(Worker) as Arc<dyn Worker>;
			tokio::spawn(Job(worker, Work.clone(), tx.clone()))
		})
		.collect();

	while let Ok((stream, _)) =
		TcpListener::bind("127.0.0.1:8080").await.expect("Cannot TcpListener.").accept().await
	{
		let queue = Work.clone();
		let rx = rx.clone();
		let ws_stream = accept_async(stream).await.expect("Failed to accept");
		tokio::spawn(Yell(ws_stream, queue, rx));
	}

	join_all(workers).await;
}
