
#![no_std]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate ssd1306;

extern crate rnarduino as rn;

use rn::rnGpio as rnGpio;
use rn::rnI2C::rnI2C as rnI2C;

use rn::rnGpio::rnPin as rnPin;
use rnarduino::rnOsHelper::rnDelayUs;
use rnGpio::rnFastIO as rnFastIO;


use rnPin::NoPin;

use ssd1306::access::SSD1306Access;
use ssd1306::ssd1306_cmd::*;
use rnarduino::rnOsHelper::rnDelay as rnDelay;


//---------------------------------------
pub fn fail()
{
    panic!("oop");
}
//---------------------------------------
pub struct i2c_ssd1306
{
    i2c     : rnI2C,
    address : u8,
}
//---------------------------------------
impl i2c_ssd1306
{   
    pub fn new(i2c_interface: rnI2C , address : u8) -> Self
    {
        let mut me : i2c_ssd1306 = i2c_ssd1306
        {
                i2c         : i2c_interface,
                address     : address,                
        };
        me
    }
   
}
//-----------------------------------
impl SSD1306Access for i2c_ssd1306
{
    fn  send_command(&mut self, command : u8)
    {
        let cmd2: [u8;2]=[SSD1306_COMMAND, command];
        self.i2c.writeTo(self.address, &cmd2);
    
    }
    fn  screen_update(&mut self, width : usize, height : usize, data : &[u8])
    {
        let cmd : [u8;8]=[   SSD1306_COMMAND , SSD1306_SET_COLUMN_ADDR, 0, 127, SSD1306_SET_PAGE_ADDR,0,7,SSD1306_DATA];
        self.i2c.writeTo(self.address, &cmd );
        let intermediary : [u8;1]= [ SSD1306_DATA_CONTINUE ];
        let lens: [u32;2]=[1,1024];
        let datas : &[&[u8]]= &[&intermediary,data];
        self.i2c. multi_write_to(self.address, &lens, datas);    
    }
    fn  reset(&mut self, )
    {

    }
}
//---------------------------------------