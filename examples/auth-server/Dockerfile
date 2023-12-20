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
    groupadd auth-server && \
    useradd -g auth-server auth-server && \
    mkdir -p /usr/src/app && \
    chown auth-server:auth-server /usr/src/app

COPY --chown=auth-server:auth-server ../../target/release/nakago-examples-auth-server /usr/src/app/auth-server
COPY --chown=auth-server:auth-server ../../examples/auth-server/config/*.toml /usr/src/app/config/

USER auth-server
WORKDIR /usr/src/app

CMD ["./auth-server"]
