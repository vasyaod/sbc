pub const LCD_WIDTH: u16 = 128;
pub const LCD_HEIGHT: u16 = 128;

//mod super::interface;
use super::interface::DisplayInterface;

extern crate font8x8;
use self::font8x8::legacy::BASIC_LEGACY;

enum Command {
    InverseDisplay = 0xA7,
    NormalDisplay = 0xA6,
    Off = 0xAE,
    On = 0xAF,
    SetContrastLevel = 0x81,
    ResumeToGDDRAM = 0xA4,

    SetDisplayOffset = 0xD3,
    SetDisplayClockDiv = 0xB3,
    SetMultiplex = 0xA8,
    SetStartLine = 0x40,
    SegRemap = 0xA0,
    
    SetColumnAddress = 0x15,
    SetRowAddress = 0x75,
    SetPhaseLength = 0xB1,
}

pub struct SSD13xx<DI> {
    lcd_width: u16,
    lcd_height: u16,
    i2c: DI,
}

impl<DI> SSD13xx<DI>
where
    DI: DisplayInterface
{
    pub fn new(i2c: DI) -> SSD13xx<DI> {
        let w = LCD_WIDTH;
        let h = LCD_HEIGHT;
        SSD13xx {
            lcd_width: 128,
            lcd_height: 128,
            i2c: i2c
        }
    }

    pub fn init(&mut self) -> Result<(), DI::Error> {
        self.i2c.send_commands(&[Command::Off as u8]).ok();

        self.i2c.send_commands(&[0x2E]).ok(); // Disable scroll
        
        self.i2c.send_commands(&[Command::On as u8]).ok();
        
        //i.send_commands(&[Command::SetContrastLevel as u8]).ok();
        //i.send_commands(&[0x80]).ok();
        
        // // i.send_commands(&[Command::SegRemap as u8]).ok();
        // // i.send_commands(&[0x51]).ok();
        
        // i.send_commands(&[Command::SetStartLine as u8]).ok();
        // i.send_commands(&[0x00]).ok();
        
        // i.send_commands(&[Command::SetDisplayOffset as u8]).ok();
        // i.send_commands(&[0x00]).ok();
        
        // i.send_commands(&[Command::ResumeToGDDRAM as u8]).ok();
        // i.send_commands(&[Command::SetMultiplex as u8]).ok();
        // i.send_commands(&[0x7F]).ok();
        
        // i.send_commands(&[Command::SetPhaseLength as u8]).ok();
        // i.send_commands(&[0xF1]).ok();
        
   //     i.send_commands(&[Command::SetDisplayClockDiv as u8]).ok();
   //     i.send_commands(&[0x91]).ok();  //80Hz:0xc1 90Hz:0xe1   100Hz:0x00   110Hz:0x30 120Hz:0x50   130Hz:0x70 

        // i.send_commands(&[0xAB]).ok();
        // i.send_commands(&[0x01]).ok();
        
        // i.send_commands(&[0xB6]).ok();
        // i.send_commands(&[0x0F]).ok();
        
        // i.send_commands(&[0xBE]).ok();
        // i.send_commands(&[0x0F]).ok();
        
        // i.send_commands(&[0xBC]).ok();
        // i.send_commands(&[0x08]).ok();

        // i.send_commands(&[0xD5]).ok();
        // i.send_commands(&[0x62]).ok();
        
        // i.send_commands(&[0xFD]).ok();
        // i.send_commands(&[0x12]).ok();

//        self.i2c.send_commands(&data);
        Ok(())
    }

    fn copy_to_buf(&mut self, bitmap: &[u8; 8], buf: &mut[u8; 16], offset: usize) -> Result<(), ()> {
        let mut x1 = 0;
        for i in (offset..8).step_by(2) {
            let x = bitmap[i];
            for bit in (0..8).step_by(2) {
                let b = match (((x >> (bit + 1)) & 1), ((x >> (bit)) & 1)) {
                    (0, 0) => 0x00,
                    (1, 0) => 0xF0,
                    (0, 1) => 0x0F,
                    (1, 1) => 0xFF,
                    _ => 0x00
                };
                x1 = x1 + 1;
            }
        }
        Ok(())
    }
    
    pub fn draw(&mut self, x: &u8, y: &u8, bitmap: &[u8; 8]) -> Result<(), ()> {
        
       // let x = 16;
       // let y = 16;

        let mut buf1: [u8; 16] = [0x0; 16];
        let mut buf2: [u8; 16] = [0x0; 16];
        self.copy_to_buf(bitmap, &mut buf1, 0).ok();
        self.copy_to_buf(bitmap, &mut buf2, 1).ok();

        self.i2c.send_commands(&[Command::SetColumnAddress as u8]).ok();
        self.i2c.send_commands(&[0 + x/2]).ok();
        self.i2c.send_commands(&[3 + x/2]).ok();
        
        self.i2c.send_commands(&[Command::SetRowAddress as u8]).ok();
        self.i2c.send_commands(&[0 + y/2]).ok();
        self.i2c.send_commands(&[4 + y/2]).ok();

        self.i2c.send_data(&buf1).ok();

        self.i2c.send_commands(&[Command::SetColumnAddress as u8]).ok();
        self.i2c.send_commands(&[0 + x/2 + 64]).ok();
        self.i2c.send_commands(&[3 + x/2 + 64]).ok();
        
        self.i2c.send_commands(&[Command::SetRowAddress as u8]).ok();
        self.i2c.send_commands(&[0 + y/2 + 64]).ok();
        self.i2c.send_commands(&[4 + y/2 + 64]).ok();

        self.i2c.send_data(&buf2).ok();

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), ()> {
        for x in (0..128).step_by(8) {
            for y in (0..128).step_by(8) {
                self.draw(&x, &y, &BASIC_LEGACY[1]).ok();
            }
        }

        Ok(())
    }

    pub fn draw_char(&mut self, x: &u8, y: &u8, ch: &char) -> Result<(), ()> {
        self.draw(&x, &y, &BASIC_LEGACY[*ch as usize]).ok();

        Ok(())
    }

    pub fn draw_digit(&mut self, x: &u8, y: &u8, dig: &u8) -> Result<(), ()> {
        self.draw(&x, &y, &BASIC_LEGACY[(dig + 48) as usize]).ok();

        Ok(())
    }

    pub fn draw_text(&mut self, x: &u8, y: &u8, text: &str) -> Result<(), ()> {
        let mut x1 = *x;
        for chr in text.chars() {
            self.draw_char(&x1, y, &chr).ok();
            x1 = x1 + 8;
        }

        Ok(())
    }

    pub fn draw_int(&mut self, x: &u8, y: &u8, value: &usize) -> Result<(), ()> {
        let mut x1 = *x + 6 * 8;
        let mut v = *value;

        for i in 0..6 {
            let f = v % 10;
            v = v / 10;
            self.draw_digit(&x1, y, &(f as u8)).ok();
            x1 = x1 - 8;
        }

        Ok(())
    }
 /*
    fn draw_char(&mut self, x1: i16, y1: i16, chr: char, colour: u16) -> Result<(), ()> {
        
        if x1 > (self.lcd_width as i16) - 1 || y1 > (self.lcd_height as i16) - 1 || x1 < 0 || y1 < 0 {
            return Ok(());
        }
        
        let mut x_point = x1;
        let mut y_point = y1;
        
        //println!("{}",chr);
        for x in &BASIC_LEGACY[chr as usize] {
            for bit in 0..8 {
                //match *x & 1 << bit {
                    //0 => print!(" "),
                    //_ => print!("█"),
                //}
                match *x & 1 << bit {
                    0 => self.draw_colour( x_point, y_point , BLACK).unwrap(),
                    _ => self.draw_colour( x_point, y_point , colour).unwrap(),
                }
                x_point = x_point + 1;
            }   
            x_point = x1;
            y_point = y_point + 1;
            //println!(" ");
        }
        return Ok(());
    }
    
    fn draw_text(&mut self, x1: i16, y1: i16, text: &str, colour: u16) -> Result<(), ()> {

        if x1 > (self.lcd_width as i16) - 1 || y1 > (self.lcd_height as i16) - 1 || x1 < 0 || y1 < 0 {
            return Ok(());
        }
        
        let mut x_point = x1;
        let mut y_point = y1;

        for chr in text.chars() {
            if(x_point + 8 ) > (self.lcd_width as i16)  {
                x_point = x1;
                y_point += 8;
            }
            
            if(y_point  + 8 ) > (self.lcd_height as i16) {
                x_point = x1;
                y_point = y1;
            }
            self.draw_char(x_point, y_point, chr, colour).unwrap();
            x_point = x_point + 8;
        }   
        return Ok(());
    }
    // fn send_command(&mut self, c: u8) {
    //     match self.i2c.smbus_write_byte_data(COMMAND_MODE, c) {
    //         Ok(_) => (),
    //         Err(x) => panic!(format!("{:?}", x)),
    //     };
    // }
*/
}