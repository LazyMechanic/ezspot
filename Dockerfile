FROM rust:1.51

WORKDIR /usr/src/ezspot
COPY . .

RUN cargo test
RUN cargo install --path .

CMD ["ezspot", "-c", "config_sample.yaml"]