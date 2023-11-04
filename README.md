# Twingo Relayctl
Program to automatically control the relays in my Renault Twingo.
The relays are not present from the factory, instead they integrate with my custom infotainment system.

This program is intended to run on a Raspberry Pi.

## Relays

### SecondPowerRelay
As most cars, my car runs on 12V. Furthermore, it has a 12V 'Remote' wire. This wire is high when:
- The engine is running.
- The contact key is in the 'Ready' position.

However, when you start the engine by turning the key to the 'Ignition' position, this wire goes low.

This would mean that the Pi boots, but then when you start your engine it goes off again.
For an AC unit this makes sense, however for my use case it does not.

The Pi's power supply is supplied via two parallel relays. One is triggered by the Remote wire, to turn
the Pi on initially. The second relay is triggered by the Pi itself, to keep it on.

When the car is turned off, the pi detects this via a third relay. It will then turn itself off by turning the second
power relay off.

### PowerAmplifierRelay
I have a dedicated power amplifier. I could hook the amplifier's remote input directly to the car's remote wire.
However, when the Pi then boots, you get a 'pop' noise on the speakers. To avoid this, the Pi
determines when the power amplifier should be turned off.

## License
This program is licensed under the Apache-2.0 or MIT license, at your option.