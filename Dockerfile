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

# preprocess css
FROM node:bullseye-slim as preprocess
  # init npm project
  RUN mkdir pre
  WORKDIR /pre
  RUN npm init -y
  # copy assets
  COPY ./postcss.config.js ./postcss.config.js
  COPY ./static ./static
  COPY ./static/css ./input
  RUN rm -r ./static/css
  # instal postcss and plugins
  RUN npm i -g postcss-cli
  RUN npm i -D postcss-import postcss-preset-env cssnano
  # run preprocessors
  RUN postcss ./input/**/*.css --dir ./css --no-map --base ./input
  # move minified css
  RUN mv ./css ./static

# our final base
FROM debian:bullseye-slim
  # copy the build artifact from the build stage
  COPY --from=build /forum/target/release/forum .
  # copy preprocessed static files
  COPY --from=preprocess /pre/static .
  # copy tera templates
  COPY ./templates ./templates
  # set the startup command to run the binary
  CMD ["./forum"]
