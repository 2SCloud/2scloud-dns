FROM docker.io/library/rust:1.93-bookworm AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY config ./config
COPY zones ./zones

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app

COPY --from=builder /app/target/release/scloud-dns /app/scloud-dns
COPY --from=builder /app/config /app/config
COPY --from=builder /app/zones /app/zones

USER nonroot:nonroot

EXPOSE 53/udp 53/tcp 8053/tcp

ENTRYPOINT ["/app/scloud-dns"]
