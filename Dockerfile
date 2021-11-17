FROM rust:latest
COPY . /rsweb
WORKDIR /rsweb
RUN cargo build --release
CMD ["./target/release/rsweb-bin"]
