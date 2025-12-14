FROM ubuntu:noble

RUN echo "America/New_York" > /etc/timezone && \
    apt-get update && \
    apt-get install -y asterisk vim netcat-traditional

CMD ["asterisk", "-f"]
