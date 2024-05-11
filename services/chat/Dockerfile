FROM rust:1.67-alpine

RUN apk add alpine-sdk

RUN mkdir -p /app
RUN mkdir -p /var/dist
WORKDIR /app

COPY mock.rs .
COPY Cargo.toml .

# A trick to cache dependencies
RUN sed -i 's#src/main.rs#mock.rs#' Cargo.toml

ENV CARGO_TARGET_DIR=/var/dist
RUN cargo build --release

RUN sed -i 's#mock.rs#src/main.rs#' Cargo.toml
# After this section no cache

COPY src ./src

CMD ["cargo", "run"]
