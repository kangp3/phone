import pigpio
import time

# MCP3001 SPI settings
SPI_CHANNEL = 0
SPI_SPEED = 1000000  # 1 MHz

def read_adc(pi, spi_handle):
    count, data = pi.spi_xfer(spi_handle, [0x00, 0x00])
    adc_value = (data[0] << 5) | (data[1] >> 3)
    return adc_value

def sample_callback(gpio, level, tick):
    print(gpio, level, tick)
#    global spi_handle
#    print("HI???")
#    adc_value = read_adc(pi, spi_handle)
#    print("ADC Value:", adc_value)

# Initialize pigpio
pi = pigpio.pi()
if not pi.connected:
    print("PI NOT CONNECTED")
    exit()

# Open SPI connection
spi_handle = pi.spi_open(SPI_CHANNEL, SPI_SPEED, 0)

# Set up a hardware timed callback for 10 kHz sampling
SAMPLE_RATE_HZ = 10
SAMPLE_INTERVAL_US = int(1e6 / SAMPLE_RATE_HZ)

# Create a wave to call the sample_callback function at the desired rate
pi.set_watchdog(0, 0)  # Clear any existing watchdogs
pi.wave_clear()

# Create a pulse train for triggering
pulses = []
for i in range(10):  # Create a pulse train for 1000 samples (100 ms duration)
    pulses.append(pigpio.pulse(1<<26, 1<<19, SAMPLE_INTERVAL_US))
    pulses.append(pigpio.pulse(1<<19, 1<<26, SAMPLE_INTERVAL_US))

pi.wave_add_generic(pulses)
wave_id = pi.wave_create()

# Set up a GPIO callback to sample the ADC
pi.callback(26, pigpio.EITHER_EDGE, sample_callback)

# Send the pulse train once
pi.wave_send_once(wave_id)

print("BLOCKING ON WAVE TX")
# Block until the wave completes
while pi.wave_tx_busy():
    time.sleep(0.01)  # Sleep for a short interval

print("Wave transmission completed")

# Cleanup
pi.wave_tx_stop()
pi.spi_close(spi_handle)
pi.stop()
