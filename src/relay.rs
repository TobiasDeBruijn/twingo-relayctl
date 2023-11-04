use color_eyre::Result;
use rppal::gpio::Gpio;
use tracing::debug;

#[derive(Debug)]
pub enum Relay {
    PowerAmplifierRelay,
    SecondPowerRelay,
}

impl Relay {
    /// Get the BCM pin number to which the relay is connected.
    fn to_bcm_pin(&self) -> u8 {
        match self {
            Self::PowerAmplifierRelay => 4,
            Self::SecondPowerRelay => 15,
        }
    }
}

/// Set a relay to be on or off (set or not).
///
/// # Errors
///
/// If changing the GPIO value fails
pub fn set_relay(relay: Relay, set: bool) -> Result<()> {
    let mut output = Gpio::new()?
        .get(relay.to_bcm_pin())?
        .into_output();

    if set {
        debug!("Turning relay {relay:?} on.");
        output.set_high();
    } else {
        debug!("Turning relay {relay:?} off.");
        output.set_low();
    }

    Ok(())
}