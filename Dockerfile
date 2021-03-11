FROM rust:1.50-buster as cargo-build
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM debian:10.8
RUN apt update && apt install -y  ca-certificates
WORKDIR /workspace
COPY --from=cargo-build /usr/src/myapp/target/release/my-portfolio /workspace/my-portfolio
ENTRYPOINT ["/workspace/my-portfolio"]
