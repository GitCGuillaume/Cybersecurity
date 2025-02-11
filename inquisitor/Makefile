all:
	echo ""
clean:
	echo ""
fclean:
	echo ""
re:
	echo ""

build:
	docker compose -f docker/docker-compose.yml pull filezilla
	docker compose -f docker/docker-compose.yml build

up:
	docker compose -f docker/docker-compose.yml pull filezilla
	docker compose -f docker/docker-compose.yml up -d

down:
	docker compose -f docker/docker-compose.yml down

poison:
	docker compose -f docker/docker-compose.yml pull filezilla
	docker compose -f docker/docker-compose.yml up -d
	#docker compose -f docker/docker-compose.yml \
	#	exec -d poison arpspoof -i eth0 -t client ftp
	#docker compose -f docker/docker-compose.yml \
	#	exec -d poison arpspoof -i eth0 -t ftp client
	#GATEWAY=$$(docker inspect \
	 # 		--format='{{range .NetworkSettings.Networks}}{{.Gateway}}{{end}}' \
	#  		poison); \
	#docker compose -f docker/docker-compose.yml \
	#	exec -d poison arpspoof -i eth0 -t "$${GATEWAY}" poison
	GATEWAY=$$(docker inspect \
	 		--format='{{range .NetworkSettings.Networks}}{{.Gateway}}{{end}}' \
	  		poison); \
	docker compose -f docker/docker-compose.yml \
		exec -d poison arpspoof -i eth0 -t ftp "$${GATEWAY}"
	docker compose -f docker/docker-compose.yml \
		exec -d poison arpspoof -i eth0 -t "$${GATEWAY}" ftp
	docker compose -f docker/docker-compose.yml \
		exec -d poison arpspoof -i eth0 -t ftp fl
	docker compose -f docker/docker-compose.yml \
		exec -d poison arpspoof -i eth0 -t fl ftp

test:
	#install
	rm -rf tests/Catch2
	git -C tests/ clone https://github.com/catchorg/Catch2.git	
	cd ./tests/Catch2; \
	cmake -B build -S . -DBUILD_TESTING=OFF; \
	sudo cmake --build build/ --target install;
	rm -rf ./tests/files/build;
	cd ./tests/files; \
		mkdir build; \
		cd build; \
	        cmake -S .. -B .; \
		make;
	./tests/files/build/tests
	#cp 
	#docker compose -f tests/docker-compose.yml build
	#docker compose -f tests/docker-compose.yml up -d

.PHONY: all clean fclean re \
	build up down poison test
