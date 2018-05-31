RUST_TARGET_PATH=$(pwd) xargo build --target=x86_64-DiX_os
echo "Compiling done"
vagrant reload
echo "Reloading done"
vagrant ssh -- -Y 'cd /vagrant;make run'
echo "SSH done"
