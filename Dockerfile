FROM rust:1.76.0-buster as builder

# install protobuf
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev
COPY Cargo.toml build.rs /usr/src/app/
COPY src /usr/src/app/src/
COPY proto /usr/src/app/proto/
WORKDIR /usr/src/app

# DB
RUN apt-get install sqlite3
RUN sqlite3

# RUST setup
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN yes | apt install gcc-x86-64-linux-gnu
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
RUN cargo build --target x86_64-unknown-linux-musl --release --bin chessbicos-server
FROM gcr.io/distroless/static-debian11 as runner
# get binary
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/chessbicos-server /

# set run env
# grpc
EXPOSE 50051
# sqlite
EXPOSE 1433

# run it
CMD ["/chessbicos-server"]