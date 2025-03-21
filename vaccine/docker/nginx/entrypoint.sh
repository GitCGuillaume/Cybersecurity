#!/bin/sh
#chown -R www-data /var/www/html/adminer
#chgrp -R www-data /var/www/html/adminer
cd /var/www/site && sqlite3 test < test.sql && cd /;
chmod 644 /var/www/site/test;
chown www-data:www-data /var/www/site/test;
nginx &
exec "$@"
