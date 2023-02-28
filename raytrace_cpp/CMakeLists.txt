cmake_minimum_required(VERSION 3.23)
project(ray_trace)
set(CMAKE_CXX_STANDARD 11)


add_subdirectory(sample)

add_executable(weekend
    lib/stb/stb_image_write.h
    weekend.cpp
)

target_include_directories(weekend SYSTEM PUBLIC "lib/eigen-master" "lib/stb")