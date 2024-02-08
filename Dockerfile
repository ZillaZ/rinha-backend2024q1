FROM rust:1.75

WORKDIR /usr/src/rinha

RUN mkdir src; touch src/main.rs

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY src/ ./src/

RUN cargo build --release

EXPOSE 8000

CMD ./target/release/rinha-backend2024