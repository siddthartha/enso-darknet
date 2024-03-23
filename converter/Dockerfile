FROM python:3

WORKDIR /converter

RUN pip3 install --no-cache-dir numpy torch

COPY ./convert.py convert.py

CMD [ "python3", "convert.py" ]

