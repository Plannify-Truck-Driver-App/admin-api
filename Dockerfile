FROM rust:1.89-alpine AS builder

RUN apk add --no-cache pkgconfig openssl-dev musl-dev gcc

WORKDIR /app
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:latest

COPY --from=builder /app/target/release/plannify-admin-api /app/plannify-admin-api

EXPOSE 3000

CMD ["/app/plannify-admin-api"]
