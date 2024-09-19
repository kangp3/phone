import argparse

import matplotlib.pyplot as plt
import numpy as np
from scipy import signal
from scipy.io import wavfile


FREQS = [697, 770, 852, 941, 1209, 1336, 1477]


ap = argparse.ArgumentParser()
ap.add_argument("fname")
ap.add_argument("-s", "--samples", type=int, default=2000)
args = ap.parse_args()

sample_rate, samples = wavfile.read(args.fname)
#samples = samples[:, 0]

nfft = args.samples
noverlap = nfft // 2
window = 'hann'

frequencies, times, Sxx = signal.spectrogram(samples, fs=sample_rate, nperseg=nfft, noverlap=noverlap, window=window, nfft=nfft)

min_freq = 400
max_freq = 2000
freq_mask = (frequencies >= min_freq) & (frequencies <= max_freq)
frequencies = frequencies[freq_mask]
Sxx = Sxx[freq_mask, :]

Sxx_dB = 10 * np.log10(Sxx)

plt.pcolormesh(times, frequencies, Sxx_dB, shading='gouraud')
plt.yscale('log')
plt.colorbar(label='Intensity [dB]')
plt.ylabel('Frequency [Hz]')
plt.xlabel('Time [sec]')
plt.ylim(min_freq, max_freq)
for f in FREQS:
    plt.axhline(y=f, color='r', linestyle='-')
plt.show()

