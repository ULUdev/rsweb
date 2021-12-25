FROM rust:latest
COPY . /rsweb
WORKDIR /rsweb
RUN apt-get update
RUN apt-get install pkg-config libssl-dev -y
RUN cargo build --release
CMD ["./target/release/rsweb-bin"]
