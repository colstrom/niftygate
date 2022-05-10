ARG org=colstrom
ARG alpine=3.15

FROM alpine:${alpine} AS checkout
ARG git=2.34.2-r0
RUN apk add --no-cache git=${git}
ARG upstream=github.com/colstrom/niftygate
ARG version=0.8.0
WORKDIR /src
RUN git clone --branch=${version} --depth=1 --recurse-submodules https://${upstream} .

FROM rust:alpine${alpine} AS build
ENV CARGO_HOME="/mnt/cargo"
ENV CARGO_INSTALL_ROOT="/usr/local"
ENV CARGO_PROFILE_RELEASE_CODEGEN_UNITS="1"
ENV CARGO_PROFILE_RELEASE_DEBUG="false"
ENV CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS="false"
ENV CARGO_PROFILE_RELEASE_INCREMENTAL="false"
ENV CARGO_PROFILE_RELEASE_LTO="true"
ENV CARGO_PROFILE_RELEASE_OPT_LEVEL="s"
ENV CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS="false"
ENV CARGO_PROFILE_RELEASE_PANIC="unwind"
ENV CARGO_PROFILE_RELEASE_RPATH="false"
RUN cargo search --limit=0
ARG mksh=59c-r3
RUN apk add --no-cache mksh=${mksh}
RUN ln -sf /bin/mksh /bin/sh
ARG elfutils=0.185-r0
ARG musl=1.2.2-r7
ARG openssl=1.1.1n-r0
ARG upx=3.96-r1
RUN apk add --no-cache --update \
    elfutils=${elfutils} \
    musl-dev=${musl} \
    openssl{,-dev}=${openssl} \
    upx=${upx}
WORKDIR /src
COPY --from=checkout /src .
RUN cargo install --path niftygate
RUN eu-strip /usr/local/bin/*
RUN upx /usr/local/bin/*

FROM alpine:${alpine} AS runtime
COPY --from=checkout /src/LICENSE.txt /LICENSE
COPY --from=build /usr/local/bin/ /usr/local/bin/
LABEL org.opencontainers.image.licenses="MIT"
ENTRYPOINT [ "niftygate" ]
CMD [ "help" ]
