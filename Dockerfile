FROM rust:latest
COPY . /rsweb
WORKDIR /rsweb
RUN cargo build --release
RUN apt-get install pkg-config libssl-dev -y
CMD ["./target/release/rsweb-bin"]
