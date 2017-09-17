# [ Build container ]

FROM rust:1.19.0 AS build-env

WORKDIR /work

ENV TINI_VERSION v0.16.1

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    gpg \
 && rm -rf /var/lib/apt/lists/*

ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini tini
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini.asc tini.asc
RUN gpg \
      --keyserver hkp://p80.pool.sks-keyservers.net:80 \
      --recv-keys 595E85A6B1B4779EA4DAAEC70B588DFF0527A9B7 \
 && gpg --verify tini.asc
RUN chmod +x tini

COPY . .
RUN cargo build --release

# [ Run container ]

FROM debian:stretch-slim

LABEL maintainer="mail@alexjeffries.me"

COPY --from=build-env /work/tini /sbin/tini
COPY --from=build-env /work/target/release/yams /usr/local/bin/

USER nobody
EXPOSE 3333
ENTRYPOINT ["/sbin/tini",  "--"]
CMD ["yams", "/etc/yams.yml"]
