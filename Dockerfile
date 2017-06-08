from debian:latest

COPY target/release/schani_uploader /usr/local/bin

EXPOSE 8000

ENTRYPOINT ["/usr/local/bin/schani_uploader"]
