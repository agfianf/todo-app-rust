FROM rust:1.85.0-bookworm AS build
WORKDIR /app
COPY . .
RUN cargo build --release

# we can use a distroless image for the final image
FROM gcr.io/distroless/cc-debian12
# or we can use a slim debian image but it will be larger, 80MB
# FROM debian:12-slim
COPY --from=build /app/target/release/todo-app /app/server
CMD ["/app/server"]