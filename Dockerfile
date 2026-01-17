
# imagem oficial do rust -> https://hub.docker.com/_/rust
FROM rust:1.92.0-alpine3.22

RUN apk update

RUN apk add curl wget bash git build-base

# "$(id -u)":"$(id -g)"

WORKDIR /app

# docker build . -t ubunturust
# docker exec -it rust bash
# docker system prune -af --volumes
