FROM golang:latest

RUN mkdir /app
WORKDIR /app
ADD . /app


RUN go get -u github.com/gorilla/mux

CMD ["go", "run", "cmd/todo/main.go"]
