FROM rustlang/rust:nightly AS builder

RUN cargo install cargo-fuzz

ADD . /fend
WORKDIR /fend/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /fend/fuzz/target/x86_64-unknown-linux-gnu/release/fend-fuzz /