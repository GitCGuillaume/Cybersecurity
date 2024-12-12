FROM debian:bookworm-20241202-slim
USER root
ARG NGINX_PSW
#start This is for correction only
RUN useradd test
RUN echo test:$NGINX_PSW | cat | chpasswd
#end
RUN apt-get update -y
RUN apt-get install -y nginx
#RUN rm /var/www/html
RUN apt-get install -y openssh-server
ADD ./index.html /var/www/.
ADD ./nginx.conf /etc/nginx/sites-available/default
ADD ./sshd_config /etc/ssh/.
COPY run.sh /root/.
RUN chmod +x /root/run.sh
ENTRYPOINT ["/root/run.sh"]