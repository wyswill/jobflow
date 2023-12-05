FROM alpine:latest

WORKDIR /root

COPY target/x86_64-unknown-linux-musl/release/job_flow ./job_flow
COPY ./config.yml ./config.yml
EXPOSE 8080

ENTRYPOINT [ "/root/job_flow","/root/config.yml" ]

