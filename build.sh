# reference: https://renenyffenegger.ch/notes/development/languages/C-C-plus-plus/GCC/create-libraries/index

# # create object files
# g++ -std=c++17 -c ray_tracer/scene.cpp -o bin/static/scene.o
# g++ -std=c++17 -c ray_tracer/utils/json_parser.cpp -o bin/static/json_parser.o

# # create static library
# ar rcs bin/static/libjson_parser.a bin/static/json_parser.o
# ar rcs bin/static/libscene.a bin/static/scene.o

# # link statically
# g++ --std=c++17 -Lbin/static -ljson_parser -lscene main.cpp -o bin/app

g++ --std=c++17 ray_tracer/scene.cpp ray_tracer/utils/json_parser.cpp main.cpp -o main

# ./main > image.ppm

# open image.ppm
