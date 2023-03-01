cmake_minimum_required(VERSION 3.23)
project(ray_trace)
set(CMAKE_CXX_STANDARD 17)

add_subdirectory(sample)

include_directories(
    "lib/eigen"
    "lib/stb"
)

add_executable(weekend
    lib/stb/stb_image_write.h
    weekend.cpp
    weekend.h
)

add_executable(weekend_sample
    lib/stb/stb_image_write.h
    weekend_sample.cpp
)