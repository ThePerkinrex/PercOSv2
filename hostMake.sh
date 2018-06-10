#rustup component add --toolchain nightly rust-src
#RUST_TARGET_PATH=$(pwd) xargo build --target=x86_64-PercOS
#echo "Compiling done"
#vagrant reload
#echo "Reloading done"
#vagrant ssh -- -Y 'cd /vagrant;make run'
#echo "SSH done"
echo "Starting make"
make -f host.makefile $@
echo "Done"
