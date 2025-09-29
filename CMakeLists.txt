cmake_minimum_required(VERSION 3.14...3.22)

set(PICO_SDK_FETCH_FROM_GIT on)
set(WIFI_SSID $ENV{WIFI_SSID})
set(WIFI_PASSWORD $ENV{WIFI_PASSWORD})
set(TEST_TCP_SERVER_IP $ENV{TEST_TCP_SERVER_IP})

#include(cmake/CPM.cmake)

include(pico_sdk_import.cmake)

project(pico_lights C CXX ASM)

# initialize the Raspberry Pi Pico SDK
pico_sdk_init()

# ---- Collect source files ----
file(GLOB_RECURSE headers CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/include/*.h")
file(GLOB_RECURSE sources CONFIGURE_DEPENDS "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")


# ---- Parent executable ----
add_executable(${PROJECT_NAME} ${sources} ${headers})

pico_generate_pio_header(${PROJECT_NAME} ${CMAKE_CURRENT_LIST_DIR}/ws2812.pio OUTPUT_DIR ${CMAKE_CURRENT_LIST_DIR}/generated)

target_compile_definitions(${PROJECT_NAME} PRIVATE
    WIFI_SSID=\"${WIFI_SSID}\"
    WIFI_PASSWORD=\"${WIFI_PASSWORD}\"
    TEST_TCP_SERVER_IP=\"${TEST_TCP_SERVER_IP}\"
    PICO_ENTER_USB_BOOT_ON_EXIT=1
)


target_link_libraries(${PROJECT_NAME}
    pico_stdlib            
    pico_cyw43_arch_lwip_poll
    hardware_pio
)

target_include_directories(${PROJECT_NAME} PUBLIC 
    $<BUILD_INTERFACE:${PROJECT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include/${PROJECT_NAME}-${PROJECT_VERSION}>
)

pico_enable_stdio_usb(${PROJECT_NAME} 1)
pico_add_extra_outputs(${PROJECT_NAME})
