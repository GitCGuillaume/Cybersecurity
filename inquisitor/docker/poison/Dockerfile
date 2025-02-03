FROM debian:bookworm-slim
LABEL user="gchopin"
LABEL email="gchopin@student.42.fr"
RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y net-tools iputils-ping
STOPSIGNAL SIGKILL
CMD ["tail", "-f", "/dev/null"]
