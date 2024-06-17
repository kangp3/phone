import array
import math
from threading import Event
from time import sleep
from signal import pause
import sys

import pyaudio


VOLUME = 0.3
SAMPLING_F = 44100
TONES = [350, 440]
CHUNK_SIZE = 40960


def main():
    aud = pyaudio.PyAudio()

    print("Running")

    stream = aud.open(
        format=pyaudio.paFloat32,
        channels=1,
        rate=SAMPLING_F,
        frames_per_buffer=CHUNK_SIZE,
        output=True,
    )
    samples = [
        sum([
            VOLUME * math.sin(2 * math.pi * tick * tone / SAMPLING_F)
            for tone in TONES
        ])
        for tick in range(SAMPLING_F * 2)
    ]
    print("Outputting sound")
    bytes = array.array('f', samples).tobytes()
    for i in range(0, len(bytes), CHUNK_SIZE):
        stream.write(bytes[i:i+CHUNK_SIZE])


if __name__ == "__main__":
    main()

