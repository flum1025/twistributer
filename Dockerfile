FROM rustlang/rust:nightly as build

COPY ./ /build
WORKDIR /build

RUN rustup update && \
  cargo build --release

#------------------------------------

FROM ubuntu:18.04

RUN apt update && \
  apt install -y openssl

COPY --from=build /build/target/release/twistributer /app/twistributer
WORKDIR /app

EXPOSE 3000

CMD ["/app/twistributer"]
