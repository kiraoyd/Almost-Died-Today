WORKDIR /app
RUN USER-root cargo new --bin backend
COPY Cargo.toml Cargo.lock ./backend/

WORKDIR /app/backend
RUN cargo build

COPY./ ./
RUN cargo build

FROM debian::buster