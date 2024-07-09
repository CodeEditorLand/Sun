#![allow(non_snake_case)]

use Echo::Fn::Job::{Action, ActionResult, Work, Worker};

use futures::future::join_all;
use std::sync::Arc;
/// Represents a working site.
struct Site;

#[async_trait::async_trait]
impl Worker for Site {
	/// Asynchronously receives an action and returns the result.
	///
	/// This function handles different types of actions. Currently, it supports
	/// the `Write` action, which writes content to a specified path. If the write
	/// operation is successful, it returns an `ActionResult` with a success message.
	/// If the write operation fails, it returns an `ActionResult` with an error message.
	/// For unsupported actions, it returns an `ActionResult` with a generic error message.
	///
	/// # Arguments
	///
	/// * `Action` - The action to be executed.
	///
	/// # Returns
	///
	/// An `ActionResult` containing the result of the action.
	///
	/// # Example
	///
	/// ```rust
	/// let site = Site {};
	/// let action = Action::Write { Path: "path/to/file".to_string(), Content: "Hello, world!".to_string() };
	/// let result = site.Receive(action).await;
	/// match result.Result {
	///     Ok(message) => println!("{}", message),
	///     Err(error) => eprintln!("{}", error),
	/// }
	/// ```
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

/// Asynchronously executes work and sends the result to the approval channel.
///
/// This function continuously executes work from the provided `Work` object,
/// and sends the result to the `Approval` channel. If sending to the channel
/// fails, the loop breaks. If no work is available, it sleeps for 100 milliseconds
/// before trying again.
///
/// # Arguments
///
/// * `Site` - An `Arc` containing a type that implements the `Worker` trait. This is used to receive the action result.
/// * `Work` - An `Arc` containing the `Work` object that provides the actions to be executed.
/// * `Approval` - An unbounded sender channel to send the action results.
///
/// # Example
///
/// ```rust
/// // Example usage of the Fn function
/// let site = Arc::new(MyWorker {});
/// let work = Arc::new(MyWork {});
/// let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
/// tokio::spawn(async move {
///     Fn(site, work, tx).await;
/// });
/// ```
#[tokio::main]
async fn main() {
	let Work = Arc::new(Work::Begin());
	let (Approval, Receipt) = tokio::sync::mpsc::unbounded_channel();
	let Receipt = Arc::new(tokio::sync::Mutex::new(Receipt));

	// TODO: Auto-calc number of workers on the force
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
