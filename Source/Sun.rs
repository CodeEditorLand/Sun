#![allow(non_snake_case)]

use Echo::Fn::Job::{Action, ActionResult, Work, Worker};

use futures::future::join_all;
use std::sync::Arc;

struct Site;

#[async_trait::async_trait]
impl Worker for Site {
	async fn Receive(&self, Action: Action) -> ActionResult {
		match Action {
			Action::Write { ref Path, ref Content } => {
				match tokio::fs::write(&Path, &Content).await {
					Ok(_) => ActionResult {
						Action: Action.clone(),
						Result: Ok(format!("Success: {}", Path)),
					},
					Err(Error) => {
						ActionResult { Action, Result: Err(format!("Cannot Action: {}", Error)) }
					}
				}
			}
			_ => ActionResult { Action, Result: Err("Cannot Action.".to_string()) },
		}
	}
}

#[tokio::main]
async fn main() {
	let Work = Arc::new(Work::Begin());
	let (Approval, Receipt) = tokio::sync::mpsc::unbounded_channel();
	let Receipt = Arc::new(tokio::sync::Mutex::new(Receipt));

	// @TODO: Auto-calc number of workers on the force
	let Force: Vec<_> = (0..4)
		.map(|_| tokio::spawn(Echo::Fn::Job::Fn(Arc::new(Site), Work.clone(), Approval.clone())))
		.collect();

	while let Ok((stream, _)) = tokio::net::TcpListener::bind("127.0.0.1:9999")
		.await
		.expect("Cannot TcpListener.")
		.accept()
		.await
	{
		tokio::spawn(Echo::Fn::Job::Yell::Fn(
			tokio_tungstenite::accept_async(stream).await.expect("Cannot accept_async."),
			Work.clone(),
			Arc::clone(&Receipt),
		));
	}

	join_all(Force).await;
}
