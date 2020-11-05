FROM alpine:3.11

ARG PAKET_VERSION=0.0.0
ENV PAKET_VERSION=${PAKET_VERSION}

LABEL version="${PAKET_VERSION}" \
    description="A simple and fast package manager for the Fish shell written in Rust." \
    maintainer="Jose Quintana <joseluisq.net>"

RUN set -eux \
    && apk --no-cache add fish git less openssh \
    && rm -rf /var/lib/apt/lists/* \
    && true

COPY ./docker/alpine/entrypoint.sh /
COPY ./bin/paket /usr/local/bin/

ENTRYPOINT ["/entrypoint.sh"]

CMD ["paket"]

# Metadata
LABEL org.opencontainers.image.vendor="Jose Quintana" \
    org.opencontainers.image.url="https://github.com/joseluisq/paket" \
    org.opencontainers.image.title="Paket" \
    org.opencontainers.image.description="A simple and fast package manager for the Fish shell written in Rust." \
    org.opencontainers.image.version="${PAKET_VERSION}" \
    org.opencontainers.image.documentation="https://github.com/joseluisq/paket"
