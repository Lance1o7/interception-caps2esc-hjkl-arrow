cmake_minimum_required(VERSION 3.0)

project(caps2eschjkl)

add_executable(caps2eschjkl caps2esc_hjkl.c)
target_compile_options(caps2eschjkl PRIVATE -Wall -Wextra)
install(TARGETS caps2eschjkl RUNTIME DESTINATION bin)
