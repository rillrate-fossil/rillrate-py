cargo build

pushd target/debug
mv librillpy.so rillpy.so
ls
popd
