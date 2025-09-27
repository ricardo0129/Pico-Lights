cmake_minimum_required(VERSION 3.14...3.22)

set(PICO_SDK_FETCH_FROM_GIT on)

#include(cmake/CPM.cmake)

include(pico_sdk_import.cmake)

project(pico_leds C CXX ASM)

# initialize the Raspberry Pi Pico SDK
pico_sdk_init()

# ---- Collect source files ----
file(GLOB_RECURSE headers CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/include/*.h")
file(GLOB_RECURSE main_sources CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")

# ---- Parent executable ----
add_executable(main ${main_sources} ${headers})

pico_generate_pio_header(main ${CMAKE_CURRENT_LIST_DIR}/ws2812.pio OUTPUT_DIR ${CMAKE_CURRENT_LIST_DIR}/generated)

target_compile_definitions(main PRIVATE
    PICO_ENTER_USB_BOOT_ON_EXIT=1
)

target_link_libraries(main
    pico_stdlib
    hardware_pio
)

target_include_directories(main PUBLIC
    $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include/${PROJECT_NAME}-${PROJECT_VERSION}>
)

target_include_directories(main PUBLIC
    ${PROJECT_SOURCE_DIR}/include
)


pico_enable_stdio_usb(main 1)
pico_add_extra_outputs(main)
