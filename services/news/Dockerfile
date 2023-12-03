FROM python:3.11-alpine

RUN mkdir -p /app
WORKDIR /app

COPY requirements.txt /app
RUN make setup

RUN apk add sqlite

ADD . /app

EXPOSE 4003

CMD ["gunicorn", "-w", "4", "-b", "0.0.0.0:4003", "app:server"]
