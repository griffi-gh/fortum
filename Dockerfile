FROM rust:slim-buster as build
  # import env vars
  ARG DATABASE_URL
  ENV DATABASE_URL=$DATABASE_URL

  # create a new empty shell project
  RUN USER=root cargo new --bin forum
  WORKDIR /forum

  # copy over manifests
  COPY ./Cargo.lock ./Cargo.lock
  COPY ./Cargo.toml ./Cargo.toml

  # this build step will cache your dependencies
  RUN cargo build --release
  RUN rm src/*.rs

  # copy your source tree
  COPY ./src ./src

  # build for release
  RUN rm ./target/release/deps/forum*
  RUN cargo build --release

# preprocess css
FROM node:buster-slim as preprocess
  # init npm project
  RUN mkdir pre
  WORKDIR /pre
  RUN npm init -y

  # copy assets
  COPY ./postcss.config.js ./postcss.config.js
  COPY ./static ./static
  COPY ./static/css ./input
  RUN rm -r ./static/css

  # instal postcss
  RUN npm i -g postcss-cli
  RUN npm i -D postcss-import postcss-preset-env cssnano

  # run preprocessors
  RUN postcss ./input/**/*.css --dir ./css --no-map --base ./input

  # move minified css
  RUN mv ./css ./static

# our final base
FROM debian:buster-slim
  # copy the build artifact from the build stage
  COPY --from=build /forum/target/release/forum .

  # copy stuff
  COPY --from=preprocess /pre/static .
  COPY ./templates ./templates

  # set the startup command to run your binary
  CMD ["./forum"]
