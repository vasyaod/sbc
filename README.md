# Simple bicycle computer

## Install toolchain


`rustup target add thumbv6m-none-eabi`

Description here

https://rust-embedded.github.io/book/intro/install.html

Controller: STM32F030F4P6
OLED1: SSD1327 Driver Chip
OLED2: SSD1327 too, http://www.raspberrypiwiki.com/index.php/1.5_inch_OLED_Shield

Pins:
B1 - inner led
A0 - magnetic button input
A9 (17 pin) - I2C1_SCL
A10 (18 pin) - I2C1_SDA


## Building and deploy

### Flushing

1. First way to flush is using openocd:

`openocd -f interface/stlink-v2.cfg -f target/stm32f0x.cfg -c "program blink verify reset exit"`


2. Second way to flush is using st-flash:

```
arm-none-eabi-objcopy -O binary blink blink.bin
st-flash write blink.bin 0x08000000
```

`cargo objcopy --bin blink --release -- -O binary blink.bin`


#### openocd

`openocd`

In case stlink-v2

`openocd -f interface/stlink-v2.cfg -f target/stm32f0x.cfg`

In case stlink-v2-1

`openocd -f interface/stlink-v2-1.cfg -f target/stm32f0x.cfg`

```

sudo openocd
cargo build --bin blink
cargo run --bin blink
```
## 

target/thumbv7m-none-eabi/release 

## Links

 * https://electronics.stackexchange.com/questions/116876/why-doesnt-my-program-execute-work
 * [Rust for embedding](https://rust-embedded.github.io/book/start/hardware.html)
 * [Blue Pill with Rust and Visual Studio Code](https://medium.com/coinmonks/coding-the-stm32-blue-pill-with-rust-and-visual-studio-code-b21615d8a20)
 * [List of crates for embedded systems](https://github.com/rust-embedded/awesome-embedded-rust)
 * [STM32F0 Tutorial, External Interrupts](https://letanphuc.net/2015/03/stm32f0-tutorial-3-external-interrupts/)
 * [STM32F030x4/x6/x8/xC Reference Manual](https://www.st.com/content/ccc/resource/technical/document/reference_manual/cf/10/a8/c4/29/fb/4c/42/DM00091010.pdf/files/DM00091010.pdf/jcr:content/translations/en.DM00091010.pdf)

