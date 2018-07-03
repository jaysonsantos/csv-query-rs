FROM rust:slim
RUN mkdir /code
WORKDIR /code
COPY Cargo.toml .
COPY Cargo.lock .
COPY src/ src/
RUN cargo build --features=sqlite_bundled --release && cargo install --features=sqlite_bundled && rm -rf /code/target ~/.cargo

FROM debian:stretch-slim
COPY --from=0 /usr/local/cargo/bin/csv-query /usr/bin/csv-query
ENTRYPOINT [ "/usr/bin/csv-query" ]
