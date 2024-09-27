FROM ubuntu:22.04
COPY ./target/release/postgres-monitoring /target/release/postgres-monitoring
COPY ./dist /target/release/dist
RUN chmod +x /target/release/postgres-monitoring
WORKDIR /target/release/
ENTRYPOINT ["./postgres-monitoring" ]