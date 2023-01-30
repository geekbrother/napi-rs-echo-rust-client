use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::runtime::{Builder, Runtime};
pub mod pb {
  tonic::include_proto!("grpc.examples.echo");
}
use anyhow::Result;
use pb::{echo_client::EchoClient, EchoRequest};
use tokio::sync::mpsc;
use tonic::transport::Channel;

lazy_static! {
  pub static ref RUNTIME: Arc<Runtime> = Arc::new(
    Builder::new_multi_thread()
      .worker_threads(1)
      .max_blocking_threads(1)
      .enable_all()
      .build()
      .unwrap()
  );
}

pub fn initialize_client(addr: String) -> EchoClient<Channel> {
  RUNTIME
    .block_on(EchoClient::connect(addr))
    .expect("Failed to create Tokio runtime for the Tunnelbroker client")
}

pub async fn send(tx: &mpsc::Sender<EchoRequest>, message: String) -> Result<()> {
  tx.send(EchoRequest { message: message }).await?;
  Ok(())
}
