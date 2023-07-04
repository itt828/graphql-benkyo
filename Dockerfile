FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /srv/blog

FROM chef AS planner
COPY . .
WORKDIR /srv/blog
COPY ./Cargo.toml /srv/blog/Cargo.toml
COPY ./Cargo.lock /srv/blog/Cargo.lock
COPY ./src /srv/blog/src
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /srv/blog/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc AS runtime
COPY --from=builder /srv/blog/target/release/blog /