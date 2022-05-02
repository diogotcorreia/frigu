FROM rust:slim as frontend-compiler

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /app/frontend
COPY ./frontend .
RUN trunk build --release


FROM ekidd/rust-musl-builder as backend-compiler

WORKDIR /home/rust/src
COPY ./server .
RUN cargo build --release
RUN pwd && ls target/x86_64-unknown-linux-musl/release


FROM rust:alpine as server

COPY --from=frontend-compiler /app/dist /app/dist

WORKDIR /app/server
COPY --from=backend-compiler /home/rust/src/target/x86_64-unknown-linux-musl/release/server .

ENTRYPOINT [ "./server", "--addr", "0.0.0.0" ]
