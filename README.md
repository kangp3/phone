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

### SSH setup
```
sudo sed --in-place=.bak --expression='s/^#PermitEmptyPasswords no/PermitEmptyPasswords yes/g' /etc/ssh/sshd_config
sudo systemctl restart sshd
sudo passwd -d recurse
```

### Dial tone generator
These commands are used to send sine wave signals to the speakers of the Pi. By correctly routing the
output audio device of the Pi to "headphones" routed to pins, these signals can be sent to the phone.
The following two sine waves are the frequencies used in a standard dial tone.
```
speaker-test -t sine -f 350 -l0
speaker-test -t sine -f 440 -l0
```

### Cross-compile to Rasppi (old stinky cross-rs way)
```
cross build --target=arm-unknown-linux-gnueabihf
```

### Device Tree compile
```
scp phoneodeo.dts recurse@peterpi.local:
scp bootconfig.txt recurse@peterpi.local:
```
```
dtc -@ -H epapr -O dtb -o phoneodeo.dtbo -Wno-unit_address_vs_reg phoneodeo.dts
sudo cp phoneodeo.dtbo /boot/overlays
sudo chown root:root bootconfig.txt
sudo chmod 755 bootconfig.txt
sudo cp bootconfig.txt /boot/firmware/config.txt
```

### Install phreak.service
```
scp goertzel/phreak.service recurse@peterpi.local:
scp goertzel/target/arm-unknown-linux-gnueabihf/release/goertzel recurse@peterpi.local:
```
```
sudo chown root:root ~/phreak.service
sudo chmod 777 ~/phreak.service
sudo mv ~/phreak.service /etc/systemd/system
sudo systemctl disable phreak.service
sudo systemctl enable phreak.service
```

### Install .asoundrc
```
scp asoundrc recurse@peterpi.local:.asoundrc
```
```
sudo cp .asoundrc /root
```

### Set volume
```
amixer sset PCM -M '40%'
sudo alsactl store
```

### Take down Wi-Fi
```
sudo nmcli connection delete 'Recurse Center'; sudo reboot
```

### Cross-compile
Install some cross-compilation dependencies
```
brew tap messense/macos-cross-toolchains
brew install arm-unknown-linux-gnueabihf
rustup target add arm-unknown-linux-gnueabihf
brew install cmake  # dependency of aws-lc-sys
cargo install --force --locked bindgen-cli  # dependency of aws-lc-sys
```
Need to pass the sysroot in order to get around this error:
```
  --- stderr
  /Users/peter/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/aws-lc-sys-0.30.0/aws-lc/include/openssl/base.h:61:10: fatal error: 'stdlib.h' file not found

  thread 'main' panicked at /Users/peter/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/aws-lc-sys-0.30.0/builder/sys_bindgen.rs:110:10:
  Unable to generate bindings.: ClangDiagnostic("/Users/peter/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/aws-lc-sys-0.30.0/aws-lc/include/openssl/base.h:61:10: fatal error: 'stdlib.h' file not found\n")
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
Got some help from these sources:
- https://stackoverflow.com/questions/65392271/macos-big-sur-11-1-cant-find-stdlib-h-w
- https://github.com/rust-lang/rust-bindgen?tab=readme-ov-file#environment-variables
- https://aws.github.io/aws-lc-rs/requirements/linux.html#linux-requirements
- https://github.com/apache/opendal/pull/5004/files
```
BINDGEN_EXTRA_CLANG_ARGS=--sysroot="$(xcrun --sdk macosx --show-sdk-path)" cargo build --release --target=arm-unknown-linux-gnueabihf
```

### PBX
```
wget https://downloads.asterisk.org/pub/telephony/asterisk/asterisk-20-current.tar.gz
tar -xzvf asterisk-20-current.tar.gz
cd asterisk-20.9.3
./contrib/scripts/install_prereq install &&
./configure &&
make &&
make install
```

### Update users
Edit pjsip.conf on the remote
```
asterisk -x 'module reload'
```

### Certbot
```
sudo apt-get update
sudo apt-get install -y snapd
sudo snap install --classic certbot
sudo ln -s /snap/bin/certbot /usr/bin/certbot
sudo certbot -d pbx.frandline.com --manual --preferred-challenges dns certonly --manual-auth-hook /home/ubuntu/renew.sh
sudo su
```
```
curl -LsSf https://astral.sh/uv/install.sh | sh
```

Test:
```
sudo certbot renew --dry-run
```

### socat wiring on Windows
Cygwin:
```
socat -t30 -T30 UDP4-RECVFROM:5060,fork UDP4:"$(wsl hostname -I | tr -d ' ')":5062 &
socat -t30 -T30 UDP4-RECVFROM:5061,fork UDP4:"$(wsl hostname -I | tr -d ' ')":5063 &
```

### ssh wiring on Windows:
PowerShell (administrator):
```
netsh interface portproxy add v4tov4 listenaddress=x.x.x.x listenport=22 connectaddress=$($(wsl hostname -I).Trim()) connectport=22
netsh advfirewall firewall add rule name=”Open Port 22 for WSL2” dir=in action=allow protocol=TCP localport=22
```

## Helpful links
- [Security Group](https://us-east-2.console.aws.amazon.com/ec2/home?region=us-east-2#SecurityGroup:group-id=sg-04dbcc9ef474c2027)
- [Docker Asterisk](https://github.com/mlan/docker-asterisk)
- [Phil's SSL Guide](https://www.phildev.net/ssl/)
- [Certbot Asterisk](https://feeding.cloud.geek.nz/posts/using-letsencrypt-cert-with-asterisk/)

## Relevant Material
- [SLIC datasheet](https://silvertel.com/images/datasheets/Ag1171-datasheet-Low-cost-ringing-SLIC-with-single-supply.pdf)
- [gpiozero docs](https://gpiozero.readthedocs.io/en/latest/)
- [Cringely-named DT overlay](https://github.com/AkiyukiOkayasu/RaspberryPi_I2S_Slave)
    - Comment out playback and codec_out
    - dai-tdm-slot-width to 24
    - Add in capture_link block mclk-fs = <256>
    - Add in r_codec_dai system-clock-frequency = <2560000>
