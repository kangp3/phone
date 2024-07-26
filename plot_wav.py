import wave
import numpy as np
import matplotlib.pyplot as plt


def twos_comp(val, bits):
    if (val & 1 << (bits-1)):
        return -((~val + 1) & (1 << bits) - 1)
    return val


def plot_wav_samples(wav_file, outfile):
    # Open the WAV file
    with wave.open(wav_file, 'r') as wf:
        # Read parameters
        n_channels = wf.getnchannels()
        n_frames = wf.getnframes()
        sample_width = wf.getsampwidth()
        sample_rate = wf.getframerate()

        # Read all frames
        frames = wf.readframes(n_frames)
        print(n_frames)
        shifted_frames = []
        for i in range(0, len(frames), 4):
            offset = 9 if i % 8 == 0 else 1
            shifted = ((frames[i+3] << 24) + (frames[i+2] << 16) + (frames[i+1] << 8) + (frames[i])) >> offset
            shifted_frames.append(twos_comp(shifted, 16))

        # Convert frames to numpy array
        if sample_width == 3:
            dtype = np.int32
        elif sample_width == 2:
            dtype = np.int16
        else:
            raise ValueError("Unsupported sample width")

        #samples = np.frombuffer(frames, dtype=dtype)
        samples = np.array(shifted_frames, dtype=np.int16)
        print(samples.shape)

        # Reshape array based on number of channels
        reshaped = samples.reshape(-1, n_channels)

        # Extract left channel
        left_channel = reshaped[:, 1]

        # Plot samples
        time = np.linspace(0, len(left_channel), num=len(left_channel))
        plt.figure(figsize=(12, 6))
        plt.plot(time, left_channel, label='Left Channel')
        plt.xlabel('Time [s]')
        plt.ylabel('Amplitude')
        plt.title('WAV File Samples (Left Channel)')
        plt.legend()
        plt.show()

    with wave.open(outfile, 'wb') as wf:
        wf.setnchannels(2)
        wf.setsampwidth(2)
        wf.setframerate(48000)
        wf.writeframes(samples)

# Example usage
import argparse
ap = argparse.ArgumentParser()
ap.add_argument("fname")
ap.add_argument("outfile")
args = ap.parse_args()
wav_file = args.fname
plot_wav_samples(wav_file, args.outfile)

