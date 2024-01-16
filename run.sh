cargo build
if [ $? -eq 0 ]; then
    ./target/debug/n-app-installer "app.AppImage"
else
    echo "Build failed"
fi