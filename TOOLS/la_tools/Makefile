INSTALL_PATH := /opt/tools/bin
PROGRAM_NAME := lart

all:
	cargo build --release

install:
	sudo install ./target/release/${PROGRAM_NAME} ${INSTALL_PATH}

clean:
	cargo clean