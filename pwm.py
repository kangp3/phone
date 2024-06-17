from threading import Event
from time import sleep
from signal import pause

from gpiozero import PWMOutputDevice
from gpiozero import DigitalInputDevice
from gpiozero import DigitalOutputDevice


shk_pin = DigitalInputDevice(pin=27, bounce_time=0.01)

rm_dev = DigitalOutputDevice(pin=17)   # RM (ringing mode) pin
fr_dev = PWMOutputDevice(pin=12, active_high=False, frequency=20, initial_value=0)  # FR (forward/reverse) pin


OFF_HOOK = Event()


def ring_me():
    while not OFF_HOOK.is_set():
        rm_dev.on()
        fr_dev.value = 0.5
        OFF_HOOK.wait(10)

        if OFF_HOOK.is_set():
            rm_dev.off()
            fr_dev.value = 0
            break

        rm_dev.off()
        fr_dev.value = 0
        OFF_HOOK.wait(3)


def main():
    print("Running")

    def stop_ringing():
        print("Stopping ringing?")
        OFF_HOOK.set()
        print("Canceled ringing?")

    shk_pin.when_activated = stop_ringing

    ring_me()


if __name__ == "__main__":
    main()
