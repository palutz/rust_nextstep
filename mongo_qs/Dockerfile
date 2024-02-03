# Builder image
FROM rust:1.72-bullseye as builder
WORKDIR /app
ADD . /app
RUN make clean && make release


# Working image
FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/mongo_qs /
CMD []
