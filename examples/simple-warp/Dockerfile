FROM debian:bullseye-slim

ARG RUN_MODE=production
ENV TZ=Etc/UTC \
    RUN_MODE=${RUN_MODE} \
    RUST_LOG=info

EXPOSE 3000

# Install some Debian packages
RUN apt update && \
    apt-get -y install --no-install-recommends ca-certificates tzdata && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    # Set up the app user
    groupadd simple-warp && \
    useradd -g simple-warp simple-warp && \
    mkdir -p /usr/src/app && \
    chown simple-warp:simple-warp /usr/src/app

COPY --chown=simple-warp:simple-warp ../../target/release/nakago-examples-simple-warp /usr/src/app/simple-warp
COPY --chown=simple-warp:simple-warp ../../examples/simple-warp/config/*.toml /usr/src/app/config/

USER simple-warp
WORKDIR /usr/src/app

CMD ["./simple-warp"]
