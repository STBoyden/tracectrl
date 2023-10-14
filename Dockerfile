FROM rust:slim-buster

RUN curl -fsSL https://bun.sh/install | bash
COPY . .
RUN cargo run --release