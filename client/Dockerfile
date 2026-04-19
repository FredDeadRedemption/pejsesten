FROM node:22-slim AS builder
WORKDIR /app
COPY package*.json .
RUN npm ci
COPY . .
ARG PUBLIC_SERVER_URL
ENV PUBLIC_SERVER_URL=$PUBLIC_SERVER_URL
RUN npm run build

FROM nginx:alpine
COPY --from=builder /app/build /usr/share/nginx/html
EXPOSE 80
