FROM python:3.11-alpine

RUN mkdir -p /app
WORKDIR /app

RUN apk add make
RUN apk add sqlite

COPY requirements.txt /app

ADD . /app

RUN make setup

EXPOSE 4003

CMD ["gunicorn", "-w", "4", "-b", "0.0.0.0:4003", "app:server"]
