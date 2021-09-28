FROM rust:slim-buster

COPY ./ ./

RUN cargo build --release

CMD ["./target/release/main"]