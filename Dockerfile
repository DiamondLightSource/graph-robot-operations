FROM docker.io/library/rust:1.76.0-bullseye AS build

ARG DATABASE_URL

WORKDIR /app

COPY Cargo.toml Cargo.lock .
COPY models/Cargo.toml models/Cargo.toml
COPY robot-operations/Cargo.toml robot-operations/Cargo.toml

RUN mkdir models/src \
    && touch models/src/lib.rs \
    && mkdir robot-operations/src \
    && echo "fn main() {}" > robot-operations/src/main.rs \
    && cargo build --release

COPY . /app

RUN touch models/src/lib.rs \
    && touch robot-operations/src/main.rs \
    && cargo build --release

FROM gcr.io/distroless/cc AS deploy

COPY --from=build /app/target/release/robot-operations /robot-operations

ENTRYPOINT ["/robot-operations"]
