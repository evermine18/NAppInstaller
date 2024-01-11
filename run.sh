cargo build
if [ $? -eq 0 ]; then
    ./target/debug/n-app-installer
else
    echo "Build failed"
fi