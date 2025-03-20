FROM debian:bookworm-20250317-slim
RUN mkdir -p /var/www/html
RUN apt-get update -y && apt-get upgrade -y && apt-get install wget -y
RUN apt install -y sqlite3 libsqlite3-0
RUN mkdir -p /run/php
RUN apt-get install -y php8.2 php8.2-common php8.2-fpm php8.2-cgi php8.2-mysql php8.2-pgsql nginx php8.2-sqlite3
COPY php-fpm.conf /etc/php/8.2/fpm/php-fpm.conf
COPY www.conf /etc/php/8.2/fpm/pool.d/www.conf
COPY entrypoint.sh /root/entrypoint.sh
COPY conf/default /etc/nginx/sites-enabled/default
COPY --chown=www-data ./site var/www/site
RUN rm /var/www/html/index.nginx-debian.html;
RUN chmod +x /root/entrypoint.sh
ENTRYPOINT ["/root/entrypoint.sh"]
CMD ["/usr/sbin/php-fpm8.2", "-F"]
