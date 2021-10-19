FROM rust:1.54

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build --tests

CMD ["cargo", "test"]
