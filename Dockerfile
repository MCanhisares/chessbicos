FROM rust:1.64.0-buster as builder
# install protobuf
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev
COPY Cargo.toml build.rs /usr/src/app/
COPY src /usr/src/app/src/
COPY proto /usr/src/app/proto/
WORKDIR /usr/src/app
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release --bin rpc-server
FROM gcr.io/distroless/static-debian11 as runner
# get binary
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/rpc-server /
# set run env
EXPOSE 50051
# run it
CMD ["/rpc-server"]