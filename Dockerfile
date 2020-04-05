FROM rust:latest as cargo-build
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release
RUN cargo install --path .

FROM debian:buster
WORKDIR /workspace
COPY --from=cargo-build /usr/local/cargo/bin/my-portfolio /workspace/my-portfolio
ENTRYPOINT ["/workspace/my-portfolio"]
