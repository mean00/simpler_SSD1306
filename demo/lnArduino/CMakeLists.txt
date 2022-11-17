#-----------------------------------------------------------------------------
#
# CMakeLists for lnDSO
# An opensource firmware for the DSO SHELL 150
# With optional hardware mods :
#   - Alternative rotary encoder wiring : MUCH Better, almost mandatory
#
#-----------------------------------------------------------------------------

cmake_minimum_required(VERSION 3.0)
MESSAGE(STATUS "===============")
MESSAGE(STATUS "Starting lnDSO")
MESSAGE(STATUS "===============")
#____________________
# Options
#____________________

SET(LN_ENABLE_I2C  ON CACHE INTERNAL "")
SET(LN_ENABLE_SPI  OFF CACHE INTERNAL "")
SET(LN_USB_NB_CDC  0 CACHE INTERNAL "")
SET(LN_USB_NB_HID  0 CACHE INTERNAL "")
SET(LN_USB_DFU_RUNTIME 0 CACHE INTERNAL "")

SET(AF_FOLDER  ${CMAKE_SOURCE_DIR}/lnArduino/)
include(./mcuSelect.cmake)
SET(CMAKE_TOOLCHAIN_FILE ${AF_FOLDER}/lnArduino.cmake)

PROJECT(lnDSO NONE) # the NONE is very important !

enable_language(C CXX ASM) # this is important too!

#

include_directories(${CMAKE_SOURCE_DIR})
include_directories(${CMAKE_SOURCE_DIR}/include)
add_subdirectory(${AF_FOLDER})

include(${AF_FOLDER}/setup.cmake)
add_subdirectory(simpler_SSD1306/platforms/lnArduino)
include_directories(simpler_SSD1306/)
include_directories(simpler_SSD1306/platforms/lnArduino)


GENERATE_GD32_FIRMWARE(demo_ssd1306 src/hello.cpp)
target_link_libraries(demo_ssd1306 ssd1306)

