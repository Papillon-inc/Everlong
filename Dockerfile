FROM rust:1.31-slim AS build

RUN apt-get update && \
        apt-get install -y --no-install-recommends \
        libssl-dev \
        pkg-config \
        build-essential

RUN rustup install nightly
RUN rustup nightly run cargo build --release

EXPOSE 1935 8000

COPY ./entrypoint.sh .
RUN chmod +x entrypoint.sh

ENTRYPOINT ["./entrypoint.sh"]
