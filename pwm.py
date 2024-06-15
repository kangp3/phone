from time import sleep
from signal import pause

from gpiozero import PWMOutputDevice
from gpiozero import DigitalOutputDevice

rm_dev = DigitalOutputDevice(pin=17)   # RM (ringing mode) pin
fr_dev = PWMOutputDevice(pin=12, active_high=False, frequency=20, initial_value=0)  # FR (forward/reverse) pin

while True:
    rm_dev.on()
    fr_dev.value = 0.5
    sleep(1)

    rm_dev.off()
    fr_dev.value = 0
    sleep(3)
