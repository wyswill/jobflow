FROM alpine:3.14
WORKDIR /usr/jobFlow

COPY ./target/release/jobFlow ./jobFlow
COPY ./config.yml ./config.yml
ENTRYPOINT [ "jobFlow /usr/jobFlow/config.yml" ]

