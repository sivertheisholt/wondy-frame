# ---------- Build ----------
FROM rust:1.97.0-slim-bookworm AS builder

RUN apt update && apt install -y \
  musl-tools \
  perl \
  make \
  && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY ./wondyframe/ . 

ENV CC=musl-gcc
ENV RUSTFLAGS="-C target-feature=+crt-static"

RUN cargo build --release --target x86_64-unknown-linux-musl

# ---------- Runtime ----------
FROM gcr.io/distroless/static

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/wondyframe /wondyframe
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

CMD ["/wondyframe"] 