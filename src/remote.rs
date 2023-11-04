use rppal::gpio::Gpio;

/// The BCM pin to which the voltage detector for the remote wire is connected.s
const REMOTE_BCM_PIN: u8 = 14;

/// Check if the car's remote wire is set high.
///
/// # Errors
///
/// If reading GPIO fails.
pub fn is_remote_on() -> color_eyre::Result<bool> {
    Ok(Gpio::new()?
        .get(REMOTE_BCM_PIN)?
        .into_input()
        .is_high())
}