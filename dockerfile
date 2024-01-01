FROM rust:1.74-slim as builder

WORKDIR /usr/src/

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /usr/app

COPY --from=builder /usr/src/frontend/dist /usr/app/frontend/dist
COPY --from=builder /usr/src/frontend/dist/index.html /usr/app/frontend/dist/index.html
COPY --from=builder /usr/src/config /usr/app/config
COPY --from=builder /usr/src/target/release/liftcalc-cli /usr/app/liftcalc-cli

ENTRYPOINT ["/usr/app/liftcalc-cli"]

CMD ["start", "-e", "production"]