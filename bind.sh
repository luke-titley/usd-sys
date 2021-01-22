CASTXML=$PWD/castxml/bin/castxml

mkdir -p build
rm -rf build/*

pushd build
#${CPPMM} ../bind -u         \
#    -o .                    \
#    -i $PWD/../bind/        \
#    -i $PWD/../thirdparty/include  \
#    --                      \
#    -I ../thirdparty/include/           \
#    -isysroot/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk \
#    -isystem /Volumes/src/clang+llvm-11.0.0-x86_64-apple-darwin/include/c++/v1 \
#    -isystem /Volumes/src/clang+llvm-11.0.0-x86_64-apple-darwin/lib/clang/11.0.0/include
#cmake .
${CASTXML}
popd
