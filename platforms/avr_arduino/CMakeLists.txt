
ADD_DEFINITIONS(-DLogger=printf)
MESSAGE(STATUS "Generating SSD1306 lib")
SET(SRCS 
                ../../ssd1306_base.cpp
                ../../ssd1306_ex.cpp
                ../../ssd1306_ex_ll.cpp
                ssd1306_i2c_arduino.cpp
        )
include_directories(${CMAKE_CURRENT_SOURCE_DIR})
include_directories(${CMAKE_CURRENT_SOURCE_DIR}/../..)

add_library( ssd1306 ${SRCS} ) 
target_link_arduino_libraries(ssd1306 PRIVATE core Wire)
