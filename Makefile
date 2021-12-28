APP_NAME:=zettai-ryouiki

all: build install

build:
	# > Building...
	cargo build --release
	# > Done

install:
	# > Installing...
	cp target/release/$(APP_NAME) /usr/bin/
	# > Done

uninstall:
	# > Uninstalling...
	rm /usr/bin/$(APP_NAME)
	# > Done