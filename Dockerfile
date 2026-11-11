# syntax=docker/dockerfile:1

# Two stages, because the source is TypeScript and the image runs JavaScript.
#
# The build stage installs every dependency, compilers included, and runs `tsc`. The
# runtime stage installs production dependencies only and takes `dist/` from the build
# stage, so TypeScript, tsx and the type packages never reach the image.

FROM node:22-alpine AS build

WORKDIR /srv

# Lifecycle scripts are disabled here as in the runtime stage: the `prepare` script would
# run `tsc` before `src/` is copied in. The build is run explicitly below instead.
COPY package.json package-lock.json tsconfig.json ./
RUN npm ci --ignore-scripts

COPY src ./src
RUN npm run build

FROM node:22-alpine

# Lifecycle scripts are disabled. Running them by default is supply-chain risk with
# nothing to offset it, and the one the package declares, `prepare`, builds from source
# this stage does not have. `--omit=dev` keeps the compiler and the test-only tree out of
# the image; `.dockerignore` already excludes the test files themselves, so this image
# cannot run its own suite.
#
# Dependencies are installed from the lockfile only. `npm ci` fails rather than
# silently resolving something the lockfile does not contain, which is the point
# of building an image from a versioned tree.
WORKDIR /srv

COPY package.json package-lock.json ./
RUN npm ci --ignore-scripts --omit=dev

# `dist/` is the product: compiled in the build stage from `src/`, and the main thing in
# this image that a change to the repository is expected to alter.
COPY --from=build /srv/dist ./dist

# Not root. The process reads files and answers JSON-RPC, and nothing else.
USER node

# 3000 is the default in `src/config/settings.ts`, and `PORT` overrides it at runtime.
#
# This is declared because the server now has a listener. An earlier revision of this
# file omitted EXPOSE deliberately, on the reasoning that it would be a lie while stdio
# was the only transport — which was true then, and is the reason it is here now.
#
# EXPOSE documents; it does not publish. `docker run -p 3000:3000 …` is what makes the
# port reachable from outside the container. See wiki/environments/docker.md.
EXPOSE 3000

# stdio stays the default, so `docker run -i` behaves as it always has and a client
# spawning the process needs no change.
#
# The HTTP transport is a command away rather than a second image, because the two share
# every byte of payload and differ only in the entry point:
#
#  docker run --rm -p 3000:3000 … npm run start:http
#
# The whole default lives in CMD, and this file sets no ENTRYPOINT of its own, which is
# what makes that command work. An ENTRYPOINT cannot be replaced by the command that
# follows the image name, only appended to, so `ENTRYPOINT ["node", "dist/index.js"]` turned
# the documented form into `node dist/index.js npm run start:http`: the stdio server started,
# read a closed stdin, and exited 0 with the published port closed — a silent failure.
# `ENTRYPOINT ["node"]` is no better, giving `node npm run start:http` and an exit 1. Both
# were built and run before this line was chosen.
#
# What this file inherits from `node:22-alpine` is `ENTRYPOINT ["docker-entrypoint.sh"]`,
# which ends in `exec "$@"`. That is what makes the override work: the documented command
# replaces CMD, the inherited entrypoint execs it, and `npm run start:http` runs as written.
# Setting an ENTRYPOINT here would replace that script with a fixed prefix, which is the
# failure above. Do not add one back.
#
# Set MCP_ALLOWED_HOSTS when the container is reachable from anywhere but this machine;
# the Host allow-list is off unless you set it. See wiki/environments/env.md.
CMD ["node", "dist/index.js"]
