all:
	echo ""
clean:
	echo ""
fclean:
	echo ""
re:
	echo ""

build:
	docker compose -f docker/docker-compose.yml build

up:
	docker compose -f docker/docker-compose.yml up -d

down:
	docker compose -f docker/docker-compose.yml down

poison_ap:
	docker compose -f docker/docker-compose.yml up -d
	docker compose -f docker/docker-compose.yml \
		exec -d poison arpspoof -i eth0 -t client ftp
	#docker compose -f docker/docker-compose.yml \
	#	exec -d poison arpspoof -i eth0 -t ftp client
	#GATEWAY=$$(docker inspect \
	#  		--format='{{range .NetworkSettings.Networks}}{{.Gateway}}{{end}}' \
	#  		poison); \
	#docker compose -f docker/docker-compose.yml \
	#	exec -d poison arpspoof -i eth0 -t "$${GATEWAY}" poison

