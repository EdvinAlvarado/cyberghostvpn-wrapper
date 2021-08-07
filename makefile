CC = g++
WFLAGS = -Wall -Wextra
SRC = vpn.cpp
FLAGS = $(WFLAGS)

vpn-rust: vpn.rs
	rustc vpn.rs

vpn: $(SRC)
	$(CC) $(FLAGS) $(SRC) -o vpn

clean:
	rm vpn

install: vpn
	cp vpn  ~/bin/vpn

uninstall: vpn
	rm ~/bin/vpn

pyinstall: vpn.py
	cp vpn.py ~/bin/vpn
