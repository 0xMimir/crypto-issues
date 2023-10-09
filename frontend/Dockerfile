FROM node:latest as builder

ARG branch=main

WORKDIR /root/
COPY . .

RUN npm install
RUN npm run build

FROM nginx

COPY --from=builder /root/dist /usr/share/nginx/html

EXPOSE 80