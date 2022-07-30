FROM lukemathwalker/cargo-chef:latest-rust-1.62.0 as chef
WORKDIR app

FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

# Builder stage
# this means we still have a link to the std library
# does not contribute to to final size, is discarded after
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
# if you need to run in dev comment out --release
RUN cargo chef cook --release --recipe-path recipe.json
# Up to this point, if our dependency tree stays the same,
# all layers should be cached.
COPY . .

# Build our project
RUN cargo build --release --bin ptb_api

# Runtime stage
# no reference to std stuff and is just its own binary
# this is our final product
FROM debian:bullseye-slim AS runtime
EXPOSE 8000
WORKDIR app

# Copy the compiled binary from the builder environment
# to our runtime environment
COPY --from=builder /app/target/release/ptb_api /usr/local/bin

# We need the configuration file at runtime!
COPY Rocket.toml Rocket.toml


# this is what to execute
ENTRYPOINT ["/usr/local/bin/ptb_api"]