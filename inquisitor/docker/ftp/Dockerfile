FROM debian:bookworm-slim
LABEL user="gchopin"
LABEL email="gchopin@student.42.fr"
EXPOSE 20 21
RUN apt-get update -y && apt-get upgrade -y
#RUN apt-get install dialog -y && apt-get install libterm-readkey-perl -y
RUN apt-get install -y --no-install-recommends vsftpd
RUN apt-get install -y net-tools iputils-ping
RUN adduser gchopin -gecos GECOS --disabled-password
COPY run.sh /usr/bin
RUN chmod 550 /usr/bin/run.sh && mkdir -p /var/run/vsftpd/empty && touch /var/log/vsftpd.log && mkdir -p /var/log/vsftpd
COPY ./vsftpd.conf /etc/vsftpd.conf
COPY ./sysctl.conf /etc/sysctl.conf
RUN chmod 600 /etc/vsftpd.conf
RUN chmod 644 /etc/sysctl.conf
#volume	/var/www/html
STOPSIGNAL SIGKILL
ENTRYPOINT ["bash", "/usr/bin/run.sh"]
CMD ["/usr/sbin/vsftpd"]
