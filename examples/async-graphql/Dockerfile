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
    groupadd async-graphql && \
    useradd -g async-graphql async-graphql && \
    mkdir -p /usr/src/app && \
    chown async-graphql:async-graphql /usr/src/app

COPY --chown=async-graphql:async-graphql ../../target/release/nakago-examples-async-graphql /usr/src/app/async-graphql
COPY --chown=async-graphql:async-graphql ../../examples/async-graphql/config/*.toml /usr/src/app/config/

USER async-graphql
WORKDIR /usr/src/app

CMD ["./async-graphql"]
