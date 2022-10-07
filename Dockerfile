FROM rust:1.64.0 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM gcr.io/distroless/cc
COPY --from=builder /usr/src/myapp/target/x86_64-unknown-linux-gnu/release/liuliget_bot /usr/local/bin/liuliget_bot
CMD ["liuliget_bot"]
