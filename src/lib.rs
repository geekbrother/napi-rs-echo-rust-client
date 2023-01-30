pub mod cclient;
use crate::cclient::pb::EchoRequest;
use napi::{
  bindgen_prelude::*,
  threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
};
use napi_derive::napi;
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, StreamExt};
use tonic::Request;

#[napi]
pub struct EchoRustClient {
  tx: mpsc::Sender<EchoRequest>,
}

#[napi]
impl EchoRustClient {
  #[napi(constructor)]
  pub fn new(address: String, on_receive_callback: ThreadsafeFunction<String>) -> Self {
    let mut client = cclient::initialize_client(address);
    let (tx, rx) = mpsc::channel(1);
    let stream = ReceiverStream::new(rx);

    // Spawning asynchronous Tokio task for handling incoming messages from the client
    cclient::RUNTIME.spawn({
      async move {
        let response = client
          .bidirectional_streaming_echo(Request::new(stream))
          .await
          .unwrap();
        let mut resp_stream = response.into_inner();
        while let Some(received) = resp_stream.next().await {
          let received = received.unwrap();
          on_receive_callback.call(
            Ok(received.message),
            ThreadsafeFunctionCallMode::NonBlocking,
          );
        }
      }
    });
    EchoRustClient { tx }
  }

  // Sending message to the stream
  #[napi]
  pub async unsafe fn send(&mut self, message: String) -> napi::Result<()> {
    if let Err(_) = cclient::send(&self.tx, message).await {
      return Err(Error::from_status(Status::GenericFailure));
    }
    Ok(())
  }
}
