cmake_minimum_required(VERSION 3.29)
project(day-01-1)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(SOURCES src/main.cpp)

add_executable(${PROJECT_NAME} ${SOURCES} ${HEADERS})

target_include_directories(${PROJECT_NAME}
        PUBLIC
        ${PROJECT_SOURCE_DIR}/include
)

set(RESOURCES_DIR "${CMAKE_SOURCE_DIR}/../resources")

add_custom_command(
        TARGET ${PROJECT_NAME} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E make_directory $<TARGET_FILE_DIR:${PROJECT_NAME}>/resources
        COMMAND ${CMAKE_COMMAND} -E copy_directory ${RESOURCES_DIR} $<TARGET_FILE_DIR:${PROJECT_NAME}>/resources
)

target_compile_options(${PROJECT_NAME} PRIVATE
        -Wall
        -Wextra
        -pedantic
)
