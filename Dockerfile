FROM rust:1.39 as build

RUN apt-get update
RUN apt-get install musl-tools libssl-dev -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /project

# Compile the dependencies
COPY Cargo.toml Cargo.toml
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN RUSTFLAGS=-Clinker=musl-gcc PKG_CONFIG_ALLOW_CROSS=1 cargo build --release --target=x86_64-unknown-linux-musl
RUN rm -f target/x86_64-unknown-linux-musl/release/deps/rendertron_cache_server*

# Compile the main program
COPY . .
RUN RUSTFLAGS=-Clinker=musl-gcc PKG_CONFIG_ALLOW_CROSS=1 cargo build --release --target=x86_64-unknown-linux-musl

# Final stage (Run application)
FROM alpine:latest

## Create user
RUN addgroup -g 1000 rce
RUN adduser -D -s /bin/sh -u 1000 -G rce rce
WORKDIR /home/rendertron_cache_server

# Setup the environment
COPY --from=build /project/target/x86_64-unknown-linux-musl/release/rendertron_cache_server .
RUN chown rce:rce rendertron_cache_server

RUN mkdir ./cache
VOLUME ./cache
RUN chown -R rce:rce ./cache

USER rce
CMD RENDERTRON_CACHE_ROOT=./cache RENDERTRON_CACHE_SOCKET=0.0.0.0:5000 ./rendertron_cache_server

EXPOSE 5000