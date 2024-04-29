FROM docker.io/library/rust:1.77.2-bullseye AS build

ARG DATABASE_URL

WORKDIR /app

COPY Cargo.toml Cargo.lock .
COPY models/Cargo.toml models/Cargo.toml
COPY robot_operations/Cargo.toml robot_operations/Cargo.toml

RUN mkdir models/src \
    && touch models/src/lib.rs \
    && mkdir robot_operations/src \
    && echo "fn main() {}" > robot_operations/src/main.rs \
    && cargo build --release

COPY . /app

RUN touch models/src/lib.rs \
    && touch robot_operations/src/main.rs \
    && cargo build --release

FROM gcr.io/distroless/cc AS deploy

COPY --from=build /app/target/release/robot_operations /robot_operations

ENTRYPOINT ["/robot_operations"]
