FROM rust:1.81.0

WORKDIR /app/

# Copy the project files into the container
COPY . .

# Install Diesel CLI with Postgres support
RUN cargo install diesel_cli --no-default-features --features postgres

# Install cargo-watch for live code watching and building
RUN cargo install cargo-watch

# Set up cargo watch to rebuild in release mode when changes occur
CMD ["cargo", "watch", "-w", "/app", "-x", "run --release --bin server"]
