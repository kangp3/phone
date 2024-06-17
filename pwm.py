import array
import math
from threading import Event
from time import sleep
from signal import pause
import sys

import pyaudio

#from gpiozero import PWMOutputDevice
#from gpiozero import DigitalInputDevice
#from gpiozero import DigitalOutputDevice


VOLUME = 0.5
SAMPLING_F = 44100
TONES = [350, 440]


def audio_samples():
    tick = 0
    while True:
        yield sum(
        )
        tick += 1


#shk_pin = DigitalInputDevice(pin=27, bounce_time=0.01)

#rm_dev = DigitalOutputDevice(pin=17)   # RM (ringing mode) pin
#fr_dev = PWMOutputDevice(  # FR (forward/reverse) pin
#    pin=12,
#    active_high=False,
#    frequency=20,
#    initial_value=0,
#)


OFF_HOOK = Event()


def ring_me():
    while not OFF_HOOK.is_set():
        rm_dev.on()
        fr_dev.value = 0.5
        OFF_HOOK.wait(10)

        rm_dev.off()
        fr_dev.value = 0
        OFF_HOOK.wait(3)


def main():
    aud = pyaudio.PyAudio()

    print("Running")

    #def stop_ringing():
    #    print("Stopping ringing?")
    #    OFF_HOOK.set()
    #    print("Canceled ringing?")

    #shk_pin.when_activated = stop_ringing

    #ring_me()

    stream = aud.open(
        format=pyaudio.paFloat32,
        channels=1 if sys.platform == 'darwin' else 2,
        rate=SAMPLING_F,
        output=True,
    )
    samples = [
        sum([
            VOLUME * math.sin(2 * math.pi * tick * tone / SAMPLING_F)
            for tone in TONES
        ])
        for tick in range(SAMPLING_F * 10)
    ]
    stream.write(array.array('f', samples).tobytes())


if __name__ == "__main__":
    main()
