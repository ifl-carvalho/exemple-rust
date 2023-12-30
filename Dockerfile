## TODO: Fix Dockerfile (Images are too big)
FROM rust:1.74.1

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["banana"]