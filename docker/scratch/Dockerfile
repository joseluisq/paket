FROM scratch

ARG PAKET_VERSION=0.0.0
ENV PAKET_VERSION=${PAKET_VERSION}

LABEL version="${PAKET_VERSION}" \
    description="A simple package manager for the Fish shell written in Rust." \
    maintainer="Jose Quintana <joseluisq.net>"

COPY ./bin/paket /

ENTRYPOINT ["/paket"]

# Metadata
LABEL org.opencontainers.image.vendor="Jose Quintana" \
    org.opencontainers.image.url="https://github.com/joseluisq/paket" \
    org.opencontainers.image.title="Paket" \
    org.opencontainers.image.description="A simple package manager for the Fish shell written in Rust." \
    org.opencontainers.image.version="${PAKET_VERSION}" \
    org.opencontainers.image.documentation="https://github.com/joseluisq/paket"
