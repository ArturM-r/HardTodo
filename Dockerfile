FROM rust:latest
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .sqlx ./.sqlx
COPY migrations ./migrations
RUN cargo build --release
CMD ["./target/release/todo"]