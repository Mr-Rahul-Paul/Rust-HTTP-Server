# base image  , use the same verison for OS
FROM rust:1.93-bookworm AS builder

# for dependencies / caching 
WORKDIR /usr/src/app
RUN cargo init --bin

COPY Cargo.toml Cargo.lock ./
# to cache 

RUN cargo build --release

COPY src ./src 

RUN cargo build --release

#use same version as rust one 
FROM debian:bookworm-slim AS runtime

# copy from stage one to final image
COPY --from=builder  /usr/src/app/target/release/Rust-HTTP-Server /usr/local/bin/Rust-HTTP-Server

EXPOSE 3000

CMD ["/usr/local/bin/Rust-HTTP-Server"]
