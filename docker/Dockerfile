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
ENV OPENSSL_INCLUDE_DIR=/usr/include/openssl
ENV OPENSSL_LIB_DIR=/usr/lib
ENV OPENSSL_STATIC=1
ENV PKG_CONFIG_ALL_STATIC=1


# (optimization - caching) pre-fetch the cargo index.
RUN cargo search --limit=0

# (dependency) add musl development files for /usr/lib/crti.o
ARG musl=1.2.2-r7
RUN apk add musl-dev=${musl}

# (workaround) use mksh to avoid parameter expansion issues with busybox's shell
ARG mksh=59c-r3
RUN apk add mksh=${mksh}
RUN ln -sf /bin/mksh /bin/sh

# (dependency) add openssl development files.
ARG openssl=1.1.1o-r0
RUN apk add openssl{,-dev,-libs-static}=${openssl}

# (build) import source code into the build environment.
WORKDIR /src
COPY --from=checkout /src .

# (optimization) update Cargo manifests.
# RUN cargo update --workspace

# (optimization - caching) fetch dependencies ahead of time (cacheable).
RUN cargo fetch --locked

# (build) build and install the package.
RUN cargo install --frozen --offline --path niftygate

# (optimization) add elfutils for eu-strip, to strip the binaries.
ARG elfutils=0.185-r0
RUN apk add elfutils=${elfutils}
RUN eu-strip /usr/local/bin/*

# (optimization) add upx to pack the binaries.
ARG upx=3.96-r1
WORKDIR /opt
RUN case $(uname -m) in \
    aarch64) true \
    && wget https://github.com/upx/upx/releases/download/v${upx%-r*}/upx-${upx%-r*}-arm64_linux.tar.xz \
    && tar x --strip-components=1 -C /usr/bin -f upx-${upx%-r*}-arm64_linux.tar.xz upx-${upx%-r*}-arm64_linux/upx \
    ;; \
    *) apk add upx=${upx} ;; \
    esac
RUN upx /usr/local/bin/*


FROM scratch as runtime
COPY --from=checkout /src/LICENSE.txt /LICENSE
COPY --from=build /usr/local/bin/ /usr/local/bin/
LABEL org.opencontainers.image.licenses="MIT"
ENTRYPOINT [ "/usr/local/bin/niftygate" ]
CMD [ "help" ]
