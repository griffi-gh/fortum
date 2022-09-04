FROM rust:slim-bullseye as build
  # create a new empty shell project
  RUN USER=root cargo new --bin forum
  WORKDIR /forum
  # copy manifests
  COPY ./Cargo.lock ./Cargo.lock
  COPY ./Cargo.toml ./Cargo.toml
  # copy sqlx offline data
  COPY ./sqlx-data.json ./sqlx-data.json
  # this build step will cache your dependencies
  RUN cargo build --release
  RUN rm src/*.rs
  # copy source tree
  COPY ./src ./src
  # build for release
  RUN rm ./target/release/deps/forum*
  RUN cargo build --release

# our final base
FROM debian:bullseye-slim
  # copy the build artifact from the build stage
  COPY --from=build /forum/target/release/forum .
  # copy files
  COPY ./templates ./templates
  COPY ./static ./static
  # set the startup command to run the binary
  CMD ["./forum"]
