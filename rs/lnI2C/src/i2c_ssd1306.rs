#![no_std]
/*#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
*/

extern crate ssd1306;
extern crate rnarduino as rn;
use rn::rn_i2c::rnI2C as rnI2C;
use ssd1306::access::SSD1306Access;
use ssd1306::ssd1306_cmd::*;


pub const SSD1306_DEFAULT_I2C_ADDRESS : u8 = 0x3c;

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
        let me : i2c_ssd1306 = i2c_ssd1306
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
        self.i2c.write_to(self.address, &cmd2);
    
    }
    fn screen_update(&mut self, width : usize, _height : usize, first_page : usize, nb_page : usize, data : &[u8])     
    {
        let cmd : [u8;8]=[   SSD1306_COMMAND , 
                    SSD1306_SET_COLUMN_ADDR, 0, (width-1) as u8, 
                    SSD1306_SET_PAGE_ADDR, first_page as u8 ,( first_page+nb_page-1  ) as u8,
                    SSD1306_DATA];
        self.i2c.write_to(self.address, &cmd );
        let intermediary : [u8;1]= [ SSD1306_DATA_CONTINUE ];
        let lens: [u32;2]=[1,((width*nb_page)) as u32];        
        let sub_data = &data[ (first_page*width)..];
        let datas : &[&[u8]]= &[&intermediary, sub_data];
        self.i2c. multi_write_to(self.address, &lens, datas);    
    }
    fn  reset(&mut self, )
    {

    }
}
//---------------------------------------