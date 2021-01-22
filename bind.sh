CASTXML=/Volumes/src/CastXML/build/bin/castxml

mkdir -p build
rm -rf build/*

pushd build
${CASTXML} \
    -std=c++17 \
    -isysroot/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk \
    -isystem /Volumes/src/clang+llvm-11.0.0-x86_64-apple-darwin/include/c++/v1 \
    -I ../thirdparty/include           \
    --castxml-output=1 \
      ../bind/bind.hpp
popd

