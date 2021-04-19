.PHONY: all build clean copy deep-clean

all: build copy

build: $(wildcard */)
	make clean
	for i in $?; do cd $$i; make all; cd ..; done;

clean:
	rm -rf ./*/build
	rm -rf ./build

deep-clean: $(wildcard */)
	make clean
	for i in $?; do cd $$i; make clean; cd ..; done;

copy:
	mkdir -p ./build
	cp ./*/build/*.wasm ./build