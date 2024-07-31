FROM ubuntu:24.10

RUN apt-get update && apt-get install -y \
    python3.12 \
    python3-pip \
    libmagic1 \
    postgresql-client \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY requirements.txt requirements.txt
RUN pip3 install -r requirements.txt --break-system-packages

COPY . .

# Copy the wait-for script
COPY wait-for.sh /usr/local/bin/wait-for.sh
RUN chmod +x /usr/local/bin/wait-for.sh

# Command to run the application
ENTRYPOINT ["wait-for.sh", "python3.12", "run.py"]
