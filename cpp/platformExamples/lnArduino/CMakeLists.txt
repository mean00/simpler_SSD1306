SET(TOP ${CMAKE_CURRENT_SOURCE_DIR}/../..)
SET(COMMON_SRCS 
                ${TOP}/ssd1306_base.cpp
                ${TOP}/ssd1306_ex.cpp
                ${TOP}/ssd1306_ex_ll.cpp
        )
SET(DRIVER_SRCS 
                ${TOP}/ssd1306_i2c_f1.cpp
         )


include_directories(${CMAKE_CURRENT_SOURCE_DIR} ${TOP} )
add_library(${libPrefix}ssd1306 
                            SRCS ${COMMON_SRCS} ${DRIVER_SRCS}
        )
