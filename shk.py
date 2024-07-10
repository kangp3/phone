from signal import pause
from threading import Event

from gpiozero import DigitalInputDevice


shk_pin = DigitalInputDevice(pin=27, bounce_time=0.01)


ON_HOOK = Event()
OFF_HOOK = Event()


def shk_on():
    print("SHK ON")


def shk_off():
    print("SHK OFF")


def main():
    shk_pin.when_activated = shk_on
    shk_pin.when_deactivated = shk_off

    pause()


if __name__ == "__main__":
    main()

