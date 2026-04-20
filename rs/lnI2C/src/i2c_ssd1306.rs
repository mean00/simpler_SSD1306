extern crate ssd1306;
use rust_esprit::i2c;
use ssd1306::access::SSD1306Access;
use ssd1306::ssd1306_cmd::*;

pub const SSD1306_DEFAULT_I2C_ADDRESS: u8 = 0x3c;

//---------------------------------------
pub fn fail() {
    panic!("oop");
}
//---------------------------------------
pub struct I2cSsd106 {
    i2c: i2c,
    address: u8,
}
//---------------------------------------
impl I2cSsd106 {
    pub fn new(i2c_interface: i2c, address: u8) -> Self {
        let me: I2cSsd106 = I2cSsd106 {
            i2c: i2c_interface,
            address,
        };
        me
    }
}
//-----------------------------------
impl SSD1306Access for I2cSsd106 {
    fn send_command(&mut self, command: u8) {
        let cmd2: [u8; 2] = [SSD1306_COMMAND, command];
        self.i2c.write_to(self.address, &cmd2);
    }
    fn screen_update(
        &mut self,
        width: usize,
        _height: usize,
        first_page: usize,
        nb_page: usize,
        data: &[u8],
    ) {
        let cmd: [u8; 8] = [
            SSD1306_COMMAND,
            SSD1306_SET_COLUMN_ADDR,
            0,
            (width - 1) as u8,
            SSD1306_SET_PAGE_ADDR,
            first_page as u8,
            (first_page + nb_page - 1) as u8,
            SSD1306_DATA,
        ];
        self.i2c.write_to(self.address, &cmd);
        let intermediary: [u8; 1] = [SSD1306_DATA_CONTINUE];
        let lens: [u32; 2] = [1, (width * nb_page) as u32];
        let sub_data = &data[(first_page * width)..];
        let datas: &[&[u8]] = &[&intermediary, sub_data];
        self.i2c.multi_write_to(self.address, &lens, datas);
    }
    fn reset(&mut self) {}
}
//---------------------------------------

