cargo build

pushd target/debug
mv librillrate.so rillrate.so
mv librillrate.dylib rillrate.so
ls
popd
