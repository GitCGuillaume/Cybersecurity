#!/bin/sh
#chown -R www-data /var/www/html/adminer
#chgrp -R www-data /var/www/html/adminer
nginx &
exec "$@"
