# GRPC Experiments
## To generate Java stubs
```bash
protoc --plugin=protoc-gen-grpc-java=/usr/bin/grpc_java_plugin --grpc-java_out=. --
java_out=. src/grpc-client/proto/voting.proto
```
Fix the generated output by replacing `javax` with `jakarta` and removing `makeExtensionsImmutable();` lines.
## Rust Client
```bash
cargo run --bin grpc-client
```
## Note
tonic 0.12 will break the code ... check the latest [uds](https://github.com/hyperium/tonic/blob/master/examples/src/uds/client.rs) example