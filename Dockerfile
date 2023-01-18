FROM rust:1.61.0 as builder

WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/flapper /
EXPOSE 8080
CMD ["./flapper"]

#WORKDIR /flapper
#COPY . .
#RUN cargo install --path .
# RUN cargo build --release
 
# FROM scratch
# COPY --from=builder /flapper/target/release/flapper /flapper
# CMD ["flapper"]

#FROM alpine:3.16.0 AS runtime

#COPY --from=builder /flapper/target/release/flapper /usr/local/bin/flapper

#FROM debian:buster-slim
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
#COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/flapper
#EXPOSE 8080

#CMD ["flapper"]
#CMD ["/usr/local/bin/flapper"]
