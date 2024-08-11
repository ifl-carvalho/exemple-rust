FROM rust:latest AS builder
WORKDIR /app

COPY Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY src src
RUN touch src/main.rs
RUN cargo build --release

RUN strip target/release/banana

FROM alpine:latest as release
WORKDIR /app
COPY --from=builder /app/target/release/banana .

EXPOSE 3333

CMD ["./banana"]