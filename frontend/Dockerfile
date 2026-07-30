# Build stage
FROM docker.io/library/node:24-alpine AS build
WORKDIR /app

RUN corepack enable && corepack prepare pnpm@latest --activate

COPY package.json pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile
COPY . .

RUN --mount=type=secret,id=vite_primeui_license_key,env=VITE_PRIMEUI_LICENSE_KEY \
    pnpm run build

# Runtime Stage
FROM docker.io/library/nginx:alpine
COPY --from=build /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 80