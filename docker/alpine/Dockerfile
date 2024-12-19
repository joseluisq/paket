FROM --platform=$BUILDPLATFORM alpine:3.21.0 as build

ARG TARGETPLATFORM
ARG PAKET_VERSION=0.0.0
ENV PAKET_VERSION=${PAKET_VERSION}

RUN apk --no-cache add ca-certificates tzdata
RUN set -ex; \
    case "$TARGETPLATFORM" in \
        "linux/amd64") target='x86_64-unknown-linux-musl' ;; \
        "linux/arm64") target='aarch64-unknown-linux-musl' ;; \
        "linux/386") target='i686-unknown-linux-musl' ;; \
        *) echo >&2 "error: unsupported $TARGETPLATFORM architecture"; exit 1 ;; \
    esac; \
    wget --quiet -O /tmp/paket.tar.gz "https://github.com/joseluisq/paket/releases/download/v${PAKET_VERSION}/paket-v${PAKET_VERSION}-${target}.tar.gz"; \
    tar xzvf /tmp/paket.tar.gz; \
    cp paket-v${PAKET_VERSION}-${target}/paket /usr/local/bin/; \
    rm -rf /tmp/paket.tar.gz paket-v${PAKET_VERSION}-${target}; \
    chmod +x /usr/local/bin/paket

FROM alpine:3.21.0

ARG PAKET_VERSION=0.0.0
ENV PAKET_VERSION=${PAKET_VERSION}

LABEL version="${PAKET_VERSION}" \
    description="A simple and fast package manager for the Fish shell written in Rust." \
    maintainer="Jose Quintana <joseluisq.net>"

RUN set -eux \
    && apk --no-cache add fish git less openssh \
    && rm -rf /var/lib/apt/lists/* \
    && true

COPY --from=build /usr/local/bin/paket /usr/local/bin/
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
