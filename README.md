# phone

This is a repository containing some code artifacts but mostly testing fragments for a project to develop a device to
connect landline phones to other landline phones via a private VoIP network. At time of writing of this README
(July 2024) efforts are mostly focused on the design of the hardware side of the project, but this repository may
come to house more complete companion software as the project grows.

1. ~~Ring phone via software~~
    - Completed in [ring.py](ring.py)
2. ~~Play audio on phone speaker~~
    - Completed in [wav.py](wav.py)
3. ~~Receive audio from phone and convert to digital signal~~
4. Parse DTMF signals for dialing
5. Create system for initial phone setup
6. Create web app for call routing and social networking

## Development Utilities

### SSH
The Pi is set up with the hostname `peterpi.local` and can be SSH'ed to using the username `recurse`.
```
ssh recurse@peterpi.local
```

### Dial tone generator
These commands are used to send sine wave signals to the speakers of the Pi. By correctly routing the
output audio device of the Pi to "headphones" routed to pins, these signals can be sent to the phone.
The following two sine waves are the frequencies used in a standard dial tone.
```
speaker-test -t sine -f 350 -l0
speaker-test -t sine -f 440 -l0
```

### Cross-compile to Rasppi
```
cross build --target=arm-unknown-linux-gnueabihf
```

### Device Tree compile
```
dtc -@ -H epapr -O dtb -o phoneodeo.dtbo -Wno-unit_address_vs_reg phoneodeo.dts
sudo cp phoneodeo.dtbo /boot/overlays
```

## Relevant Material
- [SLIC datasheet](https://silvertel.com/images/datasheets/Ag1171-datasheet-Low-cost-ringing-SLIC-with-single-supply.pdf)
- [gpiozero docs](https://gpiozero.readthedocs.io/en/latest/)
- [Cringely-named DT overlay](https://github.com/AkiyukiOkayasu/RaspberryPi_I2S_Slave)
    - Comment out playback and codec_out
    - dai-tdm-slot-width to 24
    - Add in capture_link block mclk-fs = <256>
    - Add in r_codec_dai system-clock-frequency = <2560000>
