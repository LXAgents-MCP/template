# syntax=docker/dockerfile:1

# The payload is a few kilobytes of JavaScript.
# There is no build step, so this is a single stage: a builder stage would copy
# the same files twice to produce a smaller context, not a smaller image.
FROM node:22-alpine

# Lifecycle scripts are disabled. The package declares none, and running them
# by default is supply-chain risk with nothing to offset it. `--omit=dev` keeps
# the test-only tree out of the image; `.dockerignore` already excludes the
# test files themselves, so this image cannot run its own suite.
#
# Dependencies are installed from the lockfile only. `npm ci` fails rather than
# silently resolving something the lockfile does not contain, which is the point
# of building an image from a versioned tree.
WORKDIR /srv

COPY package.json package-lock.json ./
RUN npm ci --ignore-scripts --omit=dev

# `src/` is the product. It is copied, not generated, and it is the main
# thing in this image that a change to the repository is expected to alter.
COPY src ./src

# Not root. The process reads files and answers JSON-RPC, and nothing else.
USER node

# 3000 is `src/http.js`'s default, and `PORT` overrides it at runtime.
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
#  docker run --rm -p 3000:3000 … node src/http.js
#
# The whole default lives in CMD, and this file sets no ENTRYPOINT of its own, which is
# what makes that command work. An ENTRYPOINT cannot be replaced by the command that
# follows the image name, only appended to, so `ENTRYPOINT ["node", "src/index.js"]` turned
# the documented form into `node src/index.js node src/http.js`: the stdio server started,
# read a closed stdin, and exited 0 with the published port closed — a silent failure.
# `ENTRYPOINT ["node"]` is no better, giving `node node src/http.js` and an exit 1. Both
# were built and run before this line was chosen.
#
# What this file inherits from `node:22-alpine` is `ENTRYPOINT ["docker-entrypoint.sh"]`,
# which ends in `exec "$@"`. That is what makes the override work: the documented command
# replaces CMD, the inherited entrypoint execs it, and `node src/http.js` runs as written.
# Setting an ENTRYPOINT here would replace that script with a fixed prefix, which is the
# failure above. Do not add one back.
#
# Set MCP_ALLOWED_HOSTS when the container is reachable from anywhere but this machine;
# the Host allow-list is off unless you set it. See wiki/environments/env.md.
CMD ["node", "src/index.js"]
