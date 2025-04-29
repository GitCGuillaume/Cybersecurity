SRC = ./src/Pcap.cpp ./src/main.cpp

NAME = inquisitor

FLAGS = -Wall -Wextra -Iinclude

OBJS = $(SRC:.cpp=.o)

%.o : %.cpp
	clang++ $(FLAGS) -c $< -o $(<:.cpp=.o)

$(NAME):	$(OBJS)
	clang++ $(FLAGS) -o $(NAME) $(OBJS) -lpcap

all: $(NAME)

clean:
	rm -rf $(OBJS)

fclean:
	make clean
	rm -f $(NAME)

re:
	make fclean
	make all

build:
	docker compose -f docker/docker-compose.yml pull filezilla
	docker compose -f docker/docker-compose.yml build

up:
	docker compose -f docker/docker-compose.yml pull filezilla
	docker compose -f docker/docker-compose.yml up -d

down:
	docker compose -f docker/docker-compose.yml down

.PHONY: all clean fclean re \
	build up down
