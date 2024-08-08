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
            #offset = 9 if i % 8 == 0 else 1
            offset = 0
            shifted = ((frames[i+3] << 24) + (frames[i+2] << 16) + (frames[i+1] << 8) + (frames[i])) >> offset
            #if shifted > 65536 and i % 8 == 0:
            #    other_shifted = ((frames[i+3] << 24) + (frames[i+2] << 16) + (frames[i+1] << 8) + (frames[i])) >> offset
            #    for j in range(-5,  0):
            #        idx = i+4*j
            #        print(i // 4 + j, f"{frames[idx]:02x} {frames[idx+1]:02x} {frames[idx+2]:02x} {frames[idx+3]:02x}")
            #    print(i // 4, f"{frames[i]:02x} {frames[i+1]:02x} {frames[i+2]:02x} {frames[i+3]:02x}", twos_comp(shifted, 24))
            #    for j in range(1, 6):
            #        idx = i+4*j
            #        print(i // 4 + j, f"{frames[idx]:02x} {frames[idx+1]:02x} {frames[idx+2]:02x} {frames[idx+3]:02x}")
            #    print()
            if 58260 <= i // 8 <= 58320:
                val = shifted & 0xffffff
                print(i // 8, f"{frames[i+3]:08b} {frames[i+2]:08b} {frames[i+1]:08b} {frames[i]:08b}", twos_comp(val, 32))
                print(i // 8, f"{(val >> 24) & 0xff:08b} {(val >> 16) & 0xff:08b} {(val >> 8) & 0xff:08b} {val & 0xff:08b}")
                print()
            shifted_frames.append(twos_comp(shifted, 32))
            #shifted_frames.append(twos_comp(shifted, 16))

        # Convert frames to numpy array
        if sample_width >= 3:
            dtype = np.int32
        elif sample_width == 2:
            dtype = np.int16
        else:
            raise ValueError("Unsupported sample width")

        #samples = np.frombuffer(frames, dtype=dtype)
        samples = np.array(shifted_frames, dtype=np.int32)
        print(samples.shape)

        # Reshape array based on number of channels
        reshaped = samples.reshape(-1, n_channels)

        # Extract left channel
        left_channel = reshaped[:, 0]

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

