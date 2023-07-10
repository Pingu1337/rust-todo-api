FROM rust:1-slim-buster as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/todo-api /usr/local/bin/todo-api
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["todo-api"]
