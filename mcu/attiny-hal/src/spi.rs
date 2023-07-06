#[allow(unused_imports)]
use crate::port;
use avr_hal_generic::hal::spi::{Mode, Polarity, Phase};
pub use avr_hal_generic::spi::*;

#[cfg(feature = "attiny88")]
pub type Spi = avr_hal_generic::spi::Spi<
    crate::Atmega,
    crate::pac::SPI,
    port::PB5,
    port::PB3,
    port::PB4,
    port::PB2,
>;
#[cfg(feature = "attiny88")]
avr_hal_generic::impl_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::SPI,
    sclk: port::PB5,
    mosi: port::PB3,
    miso: port::PB4,
    cs: port::PB2,
}

#[cfg(feature = "attiny167")]
pub type Spi = avr_hal_generic::spi::Spi<
        crate::Atmega,
    crate::pac::SPI,
    port::PA5,
    port::PA4,
    port::PA2,
    port::PA6,
    >;
#[cfg(feature = "attiny167")]
avr_hal_generic::impl_spi! {
    hal: crate::Atmega,
    peripheral: crate::pac::SPI,
    sclk: port::PA5,
    mosi: port::PA4,
    miso: port::PA2,
    cs: port::PA6,
}


#[cfg(feature = "attiny202")]
pub type Spi = avr_hal_generic::spi::Spi<
        crate::Attiny,
    crate::pac::SPI0,
    port::PA3,
    port::PA1,
    port::PA2,
    port::PA0,
    >;
#[cfg(feature = "attiny202")]
impl crate::spi::SpiOps<crate::Attiny, port::PA3, port::PA1, port::PA2, port::PA0> for crate::pac::SPI0 {
    fn raw_setup(&mut self, _settings: &Settings) {

        self.ctrlb.write(|w| {
            if let Polarity::IdleHigh = _settings.mode.polarity {
               if let Phase::CaptureOnFirstTransition = _settings.mode.phase {
                    w.mode()._2()
                } else {
                    w.mode()._3()
               }
            } else {
               if let Phase::CaptureOnFirstTransition = _settings.mode.phase {
                    w.mode()._0()
                } else {
                    w.mode()._1()
               }
            }
        });
        self.ctrla.write(|w| {
            if _settings.master == true {
                w.master().set_bit();

                match _settings.clock {
                   SerialClockRate::OscfOver4 => w.presc().div4(),
                   SerialClockRate::OscfOver16 => w.presc().div16(),
                   SerialClockRate::OscfOver64 => w.presc().div64(),
                   SerialClockRate::OscfOver128 => w.presc().div128(),
                   _ => unreachable!(),
                };
            } else {
                w.master().clear_bit();
            }

            if let DataOrder::LeastSignificantFirst = _settings.data_order {
                w.dord().set_bit();
            }

            w.enable().set_bit()
        });
    }

    fn raw_release(&mut self) {
        self.ctrla.write(|w| w.enable().clear_bit())
    }

    fn raw_check_iflag(&self) -> bool {
        // TODO fix the svd file to include the IF flag
        self.intflags.read().bits() & (1<<7) != 0
    }

    fn raw_read(&self) -> u8 {
        self.data.read().bits()
    }

    fn raw_write(&mut self, byte: u8) {
        self.data.write(|w| w.bits(byte))
    }
}
