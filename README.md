This is a modified version of the SSD1306 driver from http://www.RinkyDinkElectronics.com/, mostly rewritten

## What are the changes ?
- Simple & fast
- C++ and Rust version

## And the details ?
* C++: The access to the device is done through derivation. So just write your derived class providing the sendCommand and update functions for YOUR platform, for YOUR connectivity (i2c/spi). No #ifdef all around the place + you keep the code optimized for a specific platform.
* Rust : There is an access traits to talk to the SSD1306. some sample are provided.
* C++/Rust : Truetype font support from Adafruit. Not perfect but good enough
* There is not a lot of sanity check in the code, it slows things down

## Platforms supported
Two examples provided : 

* lnArduino : stm32duino i2c
* Arduino : untested, probably not working
