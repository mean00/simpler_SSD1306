//

use core::any::Any;
//
pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
//
impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
//
pub trait SSD1306Access: AsAny {
    fn send_command(&mut self, command: u8);
    fn screen_update(
        &mut self,
        width: usize,
        height: usize,
        first_page: usize,
        nb_pages: usize,
        data: &[u8],
    );
    fn reset(&mut self);
}

impl dyn SSD1306Access {
    fn reset(&mut self) // do nothing as far as reset is concerned by default (I2C)
    {
    }
}
