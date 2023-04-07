cmake_minimum_required(VERSION 3.23)
project(draw)
set(CMAKE_CXX_STANDARD 20)

set(GLFW_DIR lib/glfw-3.3.8)
option(GLFW_BUILD_EXAMPLES "Build the GLFW example programs" OFF)
option(GLFW_BUILD_TESTS "Build the GLFW test programs" OFF)
option(GLFW_BUILD_DOCS "Build the GLFW documentation" OFF)
option(GLFW_INSTALL "Generate installation target" OFF)
option(GLFW_DOCUMENT_INTERNALS "Include internals in documentation" OFF)
add_subdirectory(${GLFW_DIR} binary_dir EXCLUDE_FROM_ALL)
include_directories(${GLFW_DIR}/include)

#find_path(
#        Vulkan_INCLUDE_DIR NAMES vulkan/vulkan.h
#        HINTS "$ENV{VULKAN_SDK}/Include"
#)
#find_library(
#        Vulkan_LIBRARY NAMES vulkan-1
#        HINTS "$ENV{VULKAN_SDK}/Lib" "$ENV{VULKAN_SDK}/Bin"
#)
#set(Vulkan_LIBRARIES ${Vulkan_LIBRARY})
#set(Vulkan_INCLUDE_DIRS ${Vulkan_INCLUDE_DIR})
#include(FindPackageHandleStandardArgs)
#find_package_handle_standard_args(
#        Vulkan
#        DEFAULT_MSG
#        Vulkan_LIBRARY Vulkan_INCLUDE_DIR
#)
#mark_as_advanced(Vulkan_INCLUDE_DIR Vulkan_LIBRARY)
#if(Vulkan_FOUND AND NOT TARGET Vulkan::Vulkan)
#    add_library(Vulkan::Vulkan UNKNOWN IMPORTED)
#    set_target_properties(
#            Vulkan::Vulkan PROPERTIES
#            IMPORTED_LOCATION "${Vulkan_LIBRARIES}"
#            INTERFACE_INCLUDE_DIRECTORIES "${Vulkan_INCLUDE_DIRS}"
#    )
#endif()

find_package(Vulkan REQUIRED)

message("VULKAN_FOUND ? " ${VULKAN_FOUND})
message("VULKAN_LIBRARY at " ${VULKAN_LIBRARY})
message("VULKAN_INCLUDE_DIR at " ${VULKAN_INCLUDE_DIR})

set(LIBS
        glfw;
        Vulkan::Vulkan
)

add_executable (tri sample/tri/tri.cpp)
target_link_libraries (tri ${LIBS})

add_executable (texture
        sample/texture/texture.cpp;
        sample/texture/stb_image.h;
)
target_link_libraries (texture ${LIBS})

add_executable (vertex sample/vertex/vertex.cpp)
target_link_libraries (vertex ${LIBS})

add_executable (uniform sample/uniform/uniform.cpp)
target_link_libraries (uniform ${LIBS})