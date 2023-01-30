extern crate napi_build;

fn main() {
  tonic_build::compile_protos("protos/echo.proto").unwrap();
  napi_build::setup();
}
