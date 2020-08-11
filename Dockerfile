FROM rust:latest as BUILDER

WORKDIR /app
COPY . .
RUN cargo install --path . --features="bot" --locked

FROM debian:10-slim

RUN apt-get update
RUN apt-get install -y libssl-dev ca-certificates


COPY --from=builder /usr/local/cargo/bin/cli /usr/local/cargo/bin/bot /bin/
CMD ["bot"]
