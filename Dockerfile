FROM rust
RUN mkdir /code
WORKDIR /code
COPY Cargo.toml .
COPY Cargo.lock .
COPY src/ src/
RUN cargo build --release && cargo install
ENTRYPOINT [ "/usr/local/cargo/bin/csv-query" ]
