cmake_minimum_required(VERSION 3.20)

add_executable(parakeet-stream-worker main.cpp)
target_include_directories(parakeet-stream-worker PRIVATE ${CMAKE_SOURCE_DIR}/include)
target_link_libraries(parakeet-stream-worker PRIVATE parakeet)
target_compile_features(parakeet-stream-worker PRIVATE cxx_std_17)
