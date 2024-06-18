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

    beat_width = 490
    sample_length = beat_width * 9
    samples = [
        sum([
            VOLUME * math.sin(2 * math.pi * tick * tone / SAMPLING_F)
            for tone in TONES
        ])
        for tick in range(sample_length)
    ]
    print("Outputting sound")
    bytes = array.array('f', samples).tobytes()

    stream = aud.open(
        format=pyaudio.paFloat32,
        channels=1,
        rate=SAMPLING_F,
        frames_per_buffer=CHUNK_SIZE,
        output=True,
    )
    while True:
        stream.write(bytes)


if __name__ == "__main__":
    main()

