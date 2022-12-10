This is a modified version of the SSD1306 driver from http://www.RinkyDinkElectronics.com/, mostly rewritten

## What are the changes ?

- Simple & fast
- C++ and Rust version
- Adafruit "Truetype" font support

## And the details ?

* Rust : Only the modified page is refreshed on screen, that version is optimized a lot.
* Rust : There is a simulator to test your layout on a PC in the sim folder ( using macroquad ).
* C++  : Basic optimization, draw_line is slow.

* There is not a lot of sanity check in the code, it slows things down

## Porting
* C++: The access to the device is done through derivation. So just write your derived class providing the sendCommand and update functions for YOUR platform, for YOUR connectivity (i2c/spi). No #ifdef all around the place + you keep the code optimized for a specific platform.
* Rust : There is an access traits to talk to the SSD1306. some sample are provided.

## Platforms supported
Two examples provided : 

* lnArduino : stm32duino i2c
* Arduino : untested, probably not working
