FROM python:3.9-alpine

RUN mkdir -p /application
WORKDIR /application

COPY requirements.txt /application
RUN pip install -r requirements.txt

ADD . /application

EXPOSE 4003

CMD ["gunicorn", "-w", "4", "-b", "0.0.0.0:4003", "app:server"]
