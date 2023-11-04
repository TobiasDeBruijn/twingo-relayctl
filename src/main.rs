use std::thread::sleep;
use std::time::Duration;
use color_eyre::Result;
use tracing::{debug, info};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::relay::{Relay, set_relay};
use crate::remote::is_remote_on;

mod relay;
mod remote;

fn main() -> Result<()> {
    color_eyre::install()?;
    install_tracing();
    info!("Starting. v{}", env!("CARGO_PKG_VERSION"));

    // Turn on the second relay for the Pi's power supply.
    set_relay(Relay::SecondPowerRelay, true)?;

    // Wait a bit before turning on the power amplifier.
    // If the we're starting the car, we do not want it on.
    sleep(Duration::from_secs(10));

    // If the car isn't on yet, wait until it is.
    while !is_remote_on()? {
        sleep(Duration::from_secs(1));
    }

    // Finally, turn on the speaker power amplifier
    set_relay(Relay::PowerAmplifierRelay, true)?;

    // Poll the car's remote to watch for when it is turned off.
    // Once the car is turned off, we first turn off the
    // power amplifier, then the Pi itself.

    loop {
        if !is_remote_on()? {
            debug!("Detected car's remote off. Checking again soon.");

            // Check again in a few seconds, to verify that the car is actually off.
            sleep(Duration::from_secs(5));
            if !is_remote_on()? {
                debug!("Car's remote is on again. Proceeding as normal.");

                // Car is still running, proceed as normal.
                continue;
            }

            debug!("Detected that car is turned off. Shutting down systems.");

            // Car is off now. Turn off various systems.
            set_relay(Relay::PowerAmplifierRelay, false)?;
            // Wait for the power amplifier to be really off.
            // This avoids a popping sound from the speakers.
            set_relay(Relay::SecondPowerRelay, false)?;

            // The pi is now off.

            // But in case it is not, exit the program.
            return Ok(());
        } else {
            // Poll again later.
            sleep(Duration::from_secs(2));
        }
    }
}

fn install_tracing() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", &format!("WARN,{}=INFO", env!("CARGO_PKG_NAME")));
    }

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(layer().compact())
        .init();
}