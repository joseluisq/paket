FROM alpine:3.15

ARG PAKET_VERSION=0.0.0
ENV PAKET_VERSION=${PAKET_VERSION}

LABEL version="${PAKET_VERSION}" \
    description="A simple and fast package manager for the Fish shell written in Rust." \
    maintainer="Jose Quintana <joseluisq.net>"

RUN set -eux \
    && apk --no-cache add fish git less openssh \
    && wget --quiet -O /tmp/paket.tar.gz "https://github.com/joseluisq/paket/releases/download/v$PAKET_VERSION/paket-v$PAKET_VERSION-x86_64-unknown-linux-musl.tar.gz"; \
        tar xzvf /tmp/paket.tar.gz; \
        cp paket-v${PAKET_VERSION}-x86_64-unknown-linux-musl/paket /usr/local/bin/; \
        rm -rf /tmp/paket.tar.gz paket-v${PAKET_VERSION}-x86_64-unknown-linux-musl; \
        chmod +x /usr/local/bin/paket \
    && rm -rf /var/lib/apt/lists/* \
    && true

COPY ./docker/alpine/entrypoint.sh /

ENTRYPOINT ["/entrypoint.sh"]

CMD ["paket"]

# Metadata
LABEL org.opencontainers.image.vendor="Jose Quintana" \
    org.opencontainers.image.url="https://github.com/joseluisq/paket" \
    org.opencontainers.image.title="Paket" \
    org.opencontainers.image.description="A simple and fast package manager for the Fish shell written in Rust." \
    org.opencontainers.image.version="${PAKET_VERSION}" \
    org.opencontainers.image.documentation="https://github.com/joseluisq/paket"
