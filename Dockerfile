FROM rust:slim-bullseye as build
  # create a new empty shell project
  RUN USER=root cargo new --bin forum
  WORKDIR /forum
  # copy manifests
  COPY ./Cargo.lock ./Cargo.lock
  COPY ./Cargo.toml ./Cargo.toml
  # this build step will cache your dependencies
  RUN cargo build --release
  RUN rm src/*.rs
  # copy source tree
  COPY ./src ./src
  # copy sqlx offline data
  COPY ./sqlx-data.json ./sqlx-data.json
  # copy db migrations
  COPY ./migrations ./migrations
  # build for release
  RUN rm ./target/release/deps/forum*
  RUN cargo build --release

# our final base
FROM debian:bullseye-slim
  # copy the build artifact from the build stage
  COPY --from=build /forum/target/release/forum .
  # copy static files and templates
  COPY ./templates ./templates
  COPY ./static ./static
  # copy rocket configuration file
  COPY ./Rocket.toml ./Rocket.toml
  # set the startup command to run the binary
  CMD ["./forum"]
