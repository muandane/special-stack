ARG NODE_VERSION=22.6.0
FROM node:${NODE_VERSION}-alpine3.20 AS base
ARG PNPM_VERSION=9.4.0

# Set working directory
WORKDIR /usr/src/app

# Switch to root to install global packages

# Install pnpm globally
RUN --mount=type=cache,target="/root/.npm" \
    npm install -g pnpm@${PNPM_VERSION}

FROM base AS deps
# Install production dependencies
RUN --mount=type=bind,source=package.json,target=package.json \
    --mount=type=bind,source=pnpm-lock.yaml,target=pnpm-lock.yaml \
    --mount=type=cache,target=/home/node/.local/share/pnpm/store \
    pnpm install --prod --frozen-lockfile --ignore-scripts

FROM deps AS build
# Install all dependencies
RUN --mount=type=bind,source=package.json,target=package.json \
    --mount=type=bind,source=pnpm-lock.yaml,target=pnpm-lock.yaml \
    --mount=type=cache,target=/home/node/.local/share/pnpm/store \
    pnpm install --frozen-lockfile --ignore-scripts

COPY . .

# Run the build script
RUN pnpm run build \
    && pnpm prune --production

FROM base AS final
ENV NODE_ENV=production
USER node
# Copy package.json and the built application
COPY --chown=node:node package.json .
COPY --chown=node:node --from=deps /usr/src/app/node_modules ./node_modules
COPY --chown=node:node --from=build /usr/src/app/build ./build

EXPOSE 3000

CMD [ "node", "-r", "dotenv/config", "build" ]