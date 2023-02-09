FROM rust:1.67.0 as builder

ARG FLAPPER_VERSION
ENV FLAPPER_VERSION = $FLAPPER_VERSION

WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/flapper /
EXPOSE 8080
CMD ["./flapper"]
