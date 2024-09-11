FROM rust:1.81-bullseye as builder
WORKDIR /usr/src/timewave_zero
COPY . .
RUN cargo install --path .

FROM rust:1.81-bullseye
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/timewave_zero /usr/local/bin/timewave_zero
CMD ["timewave_zero"]