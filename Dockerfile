# Stage de construction
FROM rust:1.89-alpine AS builder

# Installer les dépendances de compilation
RUN apk add --no-cache pkgconfig openssl-dev musl-dev gcc

WORKDIR /app
COPY . .

# Construire l'application
RUN cargo build --release

# Stage de production avec image minimale
FROM gcr.io/distroless/cc-debian12:latest

# Copier le binaire depuis le stage de construction
COPY --from=builder /app/target/release/plannify-admin-api /app/plannify-admin-api

# Exposer le port
EXPOSE 3000

# Commande de démarrage
CMD ["/app/plannify-admin-api"]
