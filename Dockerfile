FROM debian:buster-slim

ENV TZ Asia/Shanghai

WORKDIR /app

COPY ./src/config/log4rs.yaml /app/src/config/log4rs.yaml
COPY ./target/release/salvo-admin /app/

CMD ["./salvo-admin"]