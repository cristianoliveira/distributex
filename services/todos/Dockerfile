FROM golang:latest
# FROM alpine:3.14

RUN mkdir -p /app
WORKDIR /app
ADD . /app

# Install database 
RUN apt update && apt install -y sqlite3

CMD ["make", "run"]
