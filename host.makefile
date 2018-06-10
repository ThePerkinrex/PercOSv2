arch ?= x86_64
target ?= $(arch)-PercOS
pwd := /Users/juan/Desktop/Rust\ Projects/PercOSv2

.PHONY: build

setup:
	Setting up vagrant
	@vagrant up
	Vagrant setup

run: build
	Starting run
	@vagrant ssh -- -Y 'cd /vagrant;ls;make run'
	Run done

build:
	@rustup component add rust-src
	@RUST_TARGET_PATH=$(pwd) xargo build --target=$(target)
	Rust compile done
	Starting asm build and iso build
	@vagrant ssh -- -Y 'cd /vagrant;ls;make iso'
	@echo Build done

clean:
	Starting clean
	@xargo clean
	@vagrant ssh -- -Y 'cd /vagrant;ls;make clean'
	Clean done

halt:
	Halting
	@vagrant halt
	Halted
