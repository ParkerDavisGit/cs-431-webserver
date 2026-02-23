FROM rust:1.85

WORKDIR /
COPY . .

RUN cargo install --path .

CMD ["cs431-web-server"]
