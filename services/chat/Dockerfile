FROM rust:1.67-alpine

RUN apk add alpine-sdk

RUN mkdir -p /app
WORKDIR /app

COPY mock.rs .
COPY Cargo.toml .

# A trick to cache dependencies
RUN sed -i 's#src/main.rs#mock.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#mock.rs#src/main.rs#' Cargo.toml
# After this section no cache

COPY . .
RUN cargo build --release


CMD ["/app/target/release/chat-app"]
