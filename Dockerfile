FROM rust:1.56.1 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /usr/src/myapp/target/release/liuliget_bot /usr/local/bin/liuliget_bot
CMD ["liuliget_bot"]