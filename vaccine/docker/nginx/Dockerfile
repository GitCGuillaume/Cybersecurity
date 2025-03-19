FROM debian:bookworm-20250317-slim
USER root
RUN mkdir -p /var/www/html
RUN apt-get update -y && apt-get upgrade -y && apt-get install wget -y
RUN mkdir -p /run/php
RUN apt-get install -y php8.2 php8.2-common php8.2-fpm php8.2-cgi php8.2-mysql php8.2-pgsql nginx
COPY php-fpm.conf /etc/php/8.2/fpm/php-fpm.conf
COPY www.conf /etc/php/8.2/fpm/pool.d/www.conf
COPY entrypoint.sh /usr/bin
COPY conf/default /etc/nginx/sites-enabled/default
COPY --chown=www-data ./site var/www/site
RUN rm /var/www/html/index.nginx-debian.html;
RUN chmod +x /usr/bin/entrypoint.sh
ENTRYPOINT ["/usr/bin/entrypoint.sh"]
CMD ["/usr/sbin/php-fpm8.2", "-F"]
