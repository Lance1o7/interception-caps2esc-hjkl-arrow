cmake_minimum_required(VERSION 3.0)
project(caps2esc-hjkl-arrow)

add_executable(caps2esc-hjkl-arrow caps2esc_hjkl.c)
target_compile_options(caps2esc-hjkl-arrow PRIVATE -Wall -Wextra)
install(TARGETS caps2esc-hjkl-arrow RUNTIME DESTINATION bin)
