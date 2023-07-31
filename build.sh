CPPFLAGS=-I/usr/local/opt/libomp/include
LDFLAGS=-L/usr/local/opt/libomp/lib

g++ --std=c++17 main.cpp -o main $CPPFLAGS $LDFLAGS

./main > image.ppm

open image.ppm
