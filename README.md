# napi-rs-echo-rust-client
Example of using the Rust [Tonic](https://github.com/hyperium/tonic) gRPC bidirectional streaming client in the Node app using the NAPI-RS.

This example uses [echo proto file from the Tonic examples](https://github.com/hyperium/tonic/tree/master/examples/proto/echo). You can use the [Tonic echo server](https://github.com/hyperium/tonic/blob/master/examples/src/streaming/server.rs) for this client as an example.

- Rust library is in `src/lib.rs`
- NodeJS calling in `test.mjs`

## How to Run
- `yarn`
- `yarn build`
- Start the [echo server](https://github.com/hyperium/tonic/blob/master/examples/src/streaming/server.rs)
- Run `node test.mjs`
