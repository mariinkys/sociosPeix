# Build stage
FROM docker.io/library/node:24-alpine AS build
WORKDIR /app

ARG VITE_PRIMEUI_LICENSE_KEY
ENV VITE_PRIMEUI_LICENSE_KEY=$VITE_PRIMEUI_LICENSE_KEY

RUN corepack enable && corepack prepare pnpm@latest --activate

COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY . .
RUN pnpm run build

# Runtime Stage
FROM docker.io/library/nginx:alpine
COPY --from=build /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80