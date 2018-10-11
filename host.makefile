arch ?= x86_64
target ?= $(arch)-PercOS
pwd := /Users/juan/Desktop/Rust\ Projects/PercOSv2

.PHONY: build

setup:
	@echo Setting up vagrant
	@vagrant up
	@echo Vagrant setup

run: build solorun

solorun:
	@echo Starting run
	@vagrant ssh -- -Y 'cd /vagrant;ls &> /dev/null;make run'
	@echo Run done

build: rustbuild asmbuild

rustbuild:
	@rustup component add rust-src
	@RUST_TARGET_PATH=$(pwd) xargo build --target=$(target)
	@echo Rust compile done

asmbuild:
	@echo Starting ASM build and ISO build
	@vagrant ssh -- -Y 'cd /vagrant;ls &> /dev/null;make iso'
	@echo ASM and ISO build done

clean:
	@echo Starting clean
	@xargo clean
	@vagrant ssh -- -Y 'cd /vagrant;ls &> /dev/null;make clean'
	@echo Clean done

halt:
	@echo Halting
	@vagrant halt
	@echo Halted
