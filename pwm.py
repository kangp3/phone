import array
import math
from threading import Event
from time import sleep
from signal import pause
import sys

import pyaudio

from gpiozero import PWMOutputDevice
from gpiozero import DigitalInputDevice
from gpiozero import DigitalOutputDevice


VOLUME = 0.5
SAMPLING_F = 44100
AUDIO_CHUNK_SIZE = 40960
TONES = [350, 440]


def audio_samples():
    tick = 0
    while True:
        yield sum(
        )
        tick += 1


shk_pin = DigitalInputDevice(pin=27, bounce_time=0.01)

rm_dev = DigitalOutputDevice(pin=17)   # RM (ringing mode) pin
fr_dev = PWMOutputDevice(  # FR (forward/reverse) pin
    pin=12,
    active_high=False,
    frequency=20,
    initial_value=0,
)


ON_HOOK = Event()
OFF_HOOK = Event()


def ring_me():
    while not OFF_HOOK.is_set():
        rm_dev.on()
        fr_dev.value = 0.5
        OFF_HOOK.wait(10)

        rm_dev.off()
        fr_dev.value = 0
        OFF_HOOK.wait(3)


def stop_ringing():
    print("Stopping ringing?")
    OFF_HOOK.set()
    print("Canceled ringing?")


def stop_tone():
    print("Stopping dial tone?")
    ON_HOOK.set()
    print("Canceled dial tone?")


def main():
    aud = pyaudio.PyAudio()

    print("Generating dial tone")
    dial_tone_samples = [
        sum([
            VOLUME * math.sin(2 * math.pi * tick * tone / SAMPLING_F)
            for tone in TONES
        ])
        for tick in range(SAMPLING_F * 5)
    ]

    print("Running")

    shk_pin.when_activated = stop_ringing
    shk_pin.when_deactivated = stop_tone

    ring_me()

    print("Playing dial tone")
    stream = aud.open(
        format=pyaudio.paFloat32,
        channels=1,
        rate=SAMPLING_F,
        frames_per_buffer=AUDIO_CHUNK_SIZE,
        output=True,
    )
    bytes = array.array('f', dial_tone_samples).tobytes()
    for i in range(0, len(bytes), AUDIO_CHUNK_SIZE):
        stream.write(bytes[i:i+AUDIO_CHUNK_SIZE])


if __name__ == "__main__":
    main()
