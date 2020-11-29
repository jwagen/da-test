
#![no_std]
#![no_main]

use avr_device;
// Pull in the panic handler from panic-halt
extern crate panic_halt;

//#[no_mangle]
#[avr_device::entry]
fn main() -> ! {
    let dp = avr_device::avr128da48::Peripherals::take().unwrap();

    dp.PORTC.dirset.write(|w| w.pc6().set_bit().pc7().clear_bit());
    dp.PORTC.pin7ctrl.write(|w| w.pullupen().set_bit());

    // enable USART1
    // set baud rate USART1.BAUD
    let clk_per: u32 = 4_000_000;
    let baudrate = 115200;
    let baud = ((64*clk_per) / (16*baudrate)) as u16;
    dp.USART1.baud.write(|w| unsafe{ w.bits(baud)});
    // set format USART1.CTRLC
    dp.USART1.ctrlc.write(|w| unsafe{ w.bits(3)} ); // Set char size to 8
    // config TXD as output
    dp.PORTC.dirset.write_with_zero(|w| w.pc0().set_bit());
    // enable tx and rx USART1.CTRLB
    dp.USART1.ctrlb.write_with_zero(|w| w
                                    .rxen().set_bit()
                                    .txen().set_bit());



    let p = &dp.PORTC;
    let u = &dp.USART1;



    usart_send_slice(u, b"\n\rHello World");
    loop {
        short_delay();
        short_delay();

        if p.in_.read().pc7().bit_is_clear(){
            p.outtgl.write(|w| w.pc6().set_bit());
            usart_send_slice(u, b"Button is pressed\n\r");
        }

    }


}

fn usart_send_byte(usart: &avr_device::avr128da48::USART1, data: u8) {
    usart.txdatal.write(|w| w.data().bits(data));
    while usart.status.read().txcif().bit_is_clear() {}
    usart.status.write(|w| w.txcif().set_bit());
}
fn usart_send_slice(usart: &avr_device::avr128da48::USART1, data: &[u8]) {
    for c in data {
        usart_send_byte(usart, *c);
    }
}

fn short_delay() {
    for _ in 0..20000 {
        avr_device::asm::nop();
    }
}
