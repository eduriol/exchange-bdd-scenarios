FROM rust:1.54

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

CMD ["exchange-bdd-scenarios"]

