FROM debian:bookworm-slim
LABEL user="gchopin"
LABEL email="gchopin@student.42.fr"
RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y net-tools iputils-ping tcpdump dsniff
COPY ./sysctl.conf /etc/sysctl.conf
RUN chmod 644 /etc/sysctl.conf
STOPSIGNAL SIGKILL
#CMD ["tcpdump"]
CMD ["tail", "-f", "/dev/null"]
