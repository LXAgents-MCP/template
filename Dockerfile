# syntax=docker/dockerfile:1

# Two stages, because the image needs the binaries and not the toolchain.
#
# The build stage has the Rust toolchain and compiles a release build from the lockfile. The
# runtime stage is a slim Debian with the two binaries and nothing else, so cargo, rustc and the
# source never reach the image.

FROM rust:1-slim-bookworm AS build

WORKDIR /srv

# `--locked` fails rather than resolving anything Cargo.lock does not already pin, which is the
# point of building an image from a versioned tree.
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --locked --bins

FROM debian:bookworm-slim

# Not root. The process reads its environment and answers JSON-RPC, and nothing else.
RUN useradd --system --uid 10001 --no-create-home mcp
COPY --from=build /srv/target/release/template-server /srv/target/release/template /usr/local/bin/
USER mcp

# 3000 is the default in `src/config/settings.rs`, and `PORT` overrides it at runtime.
#
# EXPOSE documents; it does not publish. `docker run -p 3000:3000 …` is what makes the port
# reachable from outside the container.
EXPOSE 3000

# stdio stays the default, so `docker run -i` behaves as it always has and a client spawning the
# process needs no change. HTTP is an environment variable away rather than a second image:
#
#  docker run --rm -p 3000:3000 -e MCP_TRANSPORT=http … template
#
# The whole default lives in CMD and there is no ENTRYPOINT, so a command after the image name
# (`template tools`, for instance) replaces it instead of being appended to it.
#
# Set MCP_ALLOWED_HOSTS when the container is reachable from anywhere but this machine; the Host
# allow-list is off unless you set it. See wiki/environments/env.md.
CMD ["template-server"]
