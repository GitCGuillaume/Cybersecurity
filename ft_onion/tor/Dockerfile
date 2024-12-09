FROM debian:bookworm-20241202-slim
RUN apt-get update -y
RUN apt install -y tor
RUN rm -f /etc/tor/torrc
ADD ./torrc /etc/tor/.
USER debian-tor
ENTRYPOINT ["tor"]