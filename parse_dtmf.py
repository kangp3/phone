#!/usr/bin/env python
import math
import time

import matplotlib.pyplot as plt
import numpy as np
from scipy.constants import pi
from scipy.io import wavfile


DTMF_FREQS = [697, 770, 852, 941, 1209, 1336, 1477, 1633, 900]
DTMF_ENCODINGS = {
    (DTMF_FREQS[0], DTMF_FREQS[4]): 1,
    (DTMF_FREQS[0], DTMF_FREQS[5]): 2,
    (DTMF_FREQS[0], DTMF_FREQS[6]): 3,
    (DTMF_FREQS[1], DTMF_FREQS[4]): 4,
    (DTMF_FREQS[1], DTMF_FREQS[5]): 5,
    (DTMF_FREQS[1], DTMF_FREQS[6]): 6,
    (DTMF_FREQS[2], DTMF_FREQS[4]): 7,
    (DTMF_FREQS[2], DTMF_FREQS[5]): 8,
    (DTMF_FREQS[2], DTMF_FREQS[6]): 9,
    (DTMF_FREQS[3], DTMF_FREQS[4]): 10,
    (DTMF_FREQS[3], DTMF_FREQS[5]): 11,
    (DTMF_FREQS[3], DTMF_FREQS[6]): 12,
}
WINDOW_INTERVAL = 1000
CHUNK_SIZE = 1000
GMAG_THRESHOLD = np.exp(28.5)


def compute_goertzel_coeff(target_freq: int, sample_freq: int) -> float:
    w = 2 * pi / CHUNK_SIZE * int(0.5 + CHUNK_SIZE * target_freq / sample_freq)
    return 2 * math.cos(w)


def goertzel(samples, coeff) -> float:
    q0 = 0
    q1 = 0
    q2 = 0
    for sample in samples:
        q0 = coeff * q1 - q2 + sample
        q2 = q1
        q1 = q0
    return q1*q1 + q2*q2 - q1*q2*coeff


if __name__ == "__main__":
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("fname")
    args = ap.parse_args()

    f_sample, data = wavfile.read(args.fname)
    l_chan = [d[0] for d in data]

    goertzel_coeffs = {freq: compute_goertzel_coeff(freq, f_sample) for freq in DTMF_FREQS}
    print(goertzel_coeffs)

    g_mags = np.zeros(len(l_chan))
    decoded_vals = np.zeros(len(l_chan))
    for sample_idx in range(0, len(l_chan), WINDOW_INTERVAL):
        start = time.time()
        goertzels = [
            [goertzel(l_chan[sample_idx:sample_idx+CHUNK_SIZE], goertzel_coeffs[freq]), 1]
            for freq in DTMF_FREQS
        ]
        thresh = max([g[0] for g in goertzels]) / 8.0
        active_freqs = tuple(DTMF_FREQS[idx] for idx, g in enumerate(goertzels) if g[0] > thresh and g[0] > GMAG_THRESHOLD)
        decoded_val = DTMF_ENCODINGS.get(active_freqs, 0)
        g_mags[sample_idx:sample_idx+len(goertzels)*2] = [i for j in goertzels for i in j]
        decoded_vals[sample_idx+30] = decoded_val * 3
    plt.plot(range(len(l_chan)), [i for i in zip(np.log(g_mags), np.log(l_chan), decoded_vals)])
    plt.show()
