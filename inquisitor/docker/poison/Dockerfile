FROM debian:bookworm-slim
LABEL user="gchopin"
LABEL email="gchopin@student.42.fr"
RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y net-tools iputils-ping tcpdump dsniff
RUN apt-get install -y clang && apt-get install -y libpcap-dev make
COPY ./sysctl.conf /etc/sysctl.conf
COPY ./inquisitor /root/inquisitor
RUN chmod 644 /etc/sysctl.conf
WORKDIR /root/inquisitor
#RUN clang++ -Iinclude src/Pcap.cpp src/main.cpp -lpcap
RUN make all
WORKDIR /
STOPSIGNAL SIGKILL
#CMD ["tcpdump"]
CMD ["tail", "-f", "/dev/null"]
