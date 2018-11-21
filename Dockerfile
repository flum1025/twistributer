FROM rustlang/rust:nightly as build

COPY ./ /build
WORKDIR /build

RUN rustup update
RUN cargo build --release

#------------------------------------

FROM ubuntu:18.04

RUN apt update && \
  apt install -y openssl ca-certificates

COPY --from=build /build/target/release/twistributer /app/twistributer
WORKDIR /app

EXPOSE 3000

CMD ["/app/twistributer"]
