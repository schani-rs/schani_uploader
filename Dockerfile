from debian:unstable

RUN apt-get update && \
       apt-get install -y \
       libssl1.1 \
       ca-certificates \
       --no-install-recommends

COPY target/release/schani_uploader /usr/local/bin

EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/schani_uploader"]
