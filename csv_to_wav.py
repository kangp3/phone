import csv
import numpy as np
import wave

def csv_to_wav(csv_file, wav_file, sample_rate=48000):
    # Read the CSV file
    with open(csv_file, 'r') as f:
        reader = csv.reader(f)
        next(reader)
        samples = list(int(x[5])//(2**8) for x in reader)

    # Convert the samples to a NumPy array
    samples = np.array(samples, dtype=np.int16)

    # Write the WAV file
    with wave.open(wav_file, 'w') as wf:
        wf.setnchannels(2)
        wf.setsampwidth(2)
        wf.setframerate(sample_rate)
        wf.writeframes(samples.tobytes())

# Example usage
csv_file = 'snow.csv'
wav_file = 'poopoodoodoo.wav'
csv_to_wav(csv_file, wav_file)

