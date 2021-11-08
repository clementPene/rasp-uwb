use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use dw3000::{hl, Config};
use embedded_hal::{blocking::spi, digital::v2::OutputPin};


fn main() -> Result<(), Box<dyn Error>> {
    println!("Coucou copain !");
    
    // SPI + RESET CONFIGURATION
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 8_000_000, Mode::Mode0)?;
    let cs = Gpio::new()?.get(24)?.into_output();
    let mut reset = Gpio::new()?.get(4)?.into_output();

    // reset DW3000 module 
    thread::sleep(Duration::from_millis(500));
    reset.set_low();
    reset.set_high();

    // DW3000 config
    let mut dw3000 = hl::DW3000::new(spi, cs);
    check_states(&mut dw3000).unwrap();
    thread::sleep(Duration::from_millis(1000));
    check_states(&mut dw3000).unwrap();

    let mut dw3000 = dw3000.init().expect("Failed init.");
	check_states(&mut dw3000).unwrap();
    println!("la pll est elle lock ? = {:#x?}", dw3000.idle_pll_passed());

    let mut dw3000 = dw3000.config(Config::default()).expect("Failed init.");
	check_states(&mut dw3000).unwrap();
    println!("la pll est elle lock ? = {:#x?}", dw3000.idle_pll_passed());


    loop {
        //pin.toggle();
        thread::sleep(Duration::from_millis(500));
    }
}


fn check_states<SPI, CS, State>(
	dw3000: &mut hl::DW3000<SPI, CS, State>,
) -> Result<(), hl::Error<SPI, CS>>
where
	SPI: spi::Transfer<u8> + spi::Write<u8>,
	CS: OutputPin,
	State: hl::Awake,
{
	if dw3000.init_rc_passed()? {
		println!("INIT_RC state (rcinit = 1)");
	}
	if dw3000.idle_rc_passed()? {
		println!("IDLE_RC state (spirdy = 1)");
	}
	if dw3000.idle_pll_passed()? {
		println!("IDLE_PLL state (cpclock = 1)");
	}
	println!(
		"the state is {:#x?}\n\n",
		dw3000.state()?
	);
	Ok(())
}
