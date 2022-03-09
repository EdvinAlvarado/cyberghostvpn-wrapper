CC = g++
WFLAGS = -Wall -Wextra
SRC = vpn.cpp
FLAGS = $(WFLAGS)

RUST_SRC = ./rusty_wrapper/src/main.rs
RUST_RLS = ./rusty_wrapper/target/release/rusty_wrapper
CARGO = ./rusty_wrapper/Cargo.toml

PYTHON = vpn.py

rust: $(RUST_SRC)
	cargo build --manifest-path $(CARGO)
	cp $(RUST_RLS) vpn

cpp: $(SRC)
	$(CC) $(FLAGS) $(SRC) -o vpn

python: $(PYTHON)
	cp $(PYTHON) ~/bin/vpn

clean:
	rm vpn

install: vpn
	cp vpn  ~/bin/vpn

uninstall: vpn
	rm ~/bin/vpn

