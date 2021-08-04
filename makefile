CC = g++
WFLAGS = -Wall -Wextra
SRC = vpn.cpp
FLAGS = $(WFLAGS)

vpn: $(SRC)
	$(CC) $(FLAGS) $(SRC) -o vpn

clean:
	rm vpn

install: vpn
	cp vpn  /usr/local/bin/vpn

uninstall: vpn
	rm /usr/local/vin/vpn

