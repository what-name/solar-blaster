FROM rust:1.58 as build

# create a new empty shell project
RUN USER=root cargo new --bin solar-blaster
WORKDIR /solar-blaster

## Solana crates must have some special dependency here
RUN apt-get update && apt-get install libudev-dev

# copy over manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Copy source
RUN rm src/*.rs
COPY ./src ./src

# Cache dependencies
RUN cargo build --release

# Build for release
RUN rm -rf target/release/deps/solar-blaster*
RUN cargo build --release

# Final base
FROM rust:1.58

# Copy the build artifact from the build stage
COPY --from=build /solar-blaster/target/release/solar-blaster .
COPY ./keys ./keys

# Set the startup command
CMD ["./solar-blaster"]