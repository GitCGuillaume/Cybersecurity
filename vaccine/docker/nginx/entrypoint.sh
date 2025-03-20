#!/bin/sh
#chown -R www-data /var/www/html/adminer
#chgrp -R www-data /var/www/html/adminer
cd /var/www/site && sqlite3 main.db < main.sql && cd /
nginx &
exec "$@"
