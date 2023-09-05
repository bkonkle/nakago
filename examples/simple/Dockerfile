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
    groupadd simple && \
    useradd -g simple simple && \
    mkdir -p /usr/src/app && \
    chown simple:simple /usr/src/app

COPY --chown=simple:simple ../../target/release/nakago-examples-simple /usr/src/app/simple
COPY --chown=simple:simple ../../examples/simple/config/*.toml /usr/src/app/config/

USER simple
WORKDIR /usr/src/app

CMD ["./simple"]
