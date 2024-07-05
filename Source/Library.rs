use codeeditorland_echo::{Action, ActionResult, Job, Work, Worker, Yell};

use futures::future::join_all;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::mpsc};
use tokio_tungstenite::accept_async;

struct Workers;

#[async_trait::async_trait]
impl Worker for Workers {
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
	let (Acceptance, Recept) = mpsc::channel(100);

	// @TODO: Auto-calc number of workers in the force
	let Force: Vec<_> = (0..4)
		.map(|_| {
			tokio::spawn(Job(Arc::new(Worker) as Arc<dyn Worker>, Work.clone(), Acceptance.clone()))
		})
		.collect();

	while let Ok((stream, _)) =
		TcpListener::bind("127.0.0.1:9999").await.expect("Cannot TcpListener.").accept().await
	{
		tokio::spawn(Yell(
			accept_async(stream).await.expect("Cannot accept_async."),
			Work.clone(),
			Recept.clone(),
		));
	}

	join_all(Force).await;
}
