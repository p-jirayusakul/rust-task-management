FROM rust:latest as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp

ENV DB_HOST=localhost
ENV DB_PORT=5432
ENV DB_DATABASE=task_management
ENV DB_USERNAME=postgres
ENV DB_PASSWORD=1234
ENV DB_SCHEMA=public
ENV APP_ENV=localhost
ENV APP_PORT=4000
ENV ALLOW_ORIGINS=http://localhost:3000
ENV JWT_SECRET=NhfWytDIzKNteB5zChVsYBYL99Yed4Cx
ENV JWT_EXPIRE_MILLISECOND=28800000

EXPOSE 4000

CMD ["myapp"]
