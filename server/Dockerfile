FROM ubuntu:24.10

RUN apt-get update && apt-get install -y \
    python3.12 \
    python3-pip \
    libmagic1 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY requirements.txt requirements.txt
RUN pip3 install -r requirements.txt --break-system-packages

COPY . .

CMD ["python3.12", "run.py"]
