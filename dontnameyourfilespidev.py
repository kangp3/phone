import os
import socket
import time

import spidev


SAMPLE_FREQ = 2_000
SAMPLE_PERD = 1./SAMPLE_FREQ * 0.1


spi = spidev.SpiDev()
spi.open(0, 0)
spi.max_speed_hz = 2_800_000

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

try:
    start = time.time()
    while True:
        adc = spi.xfer2([0x00, 0x00])
        sock.sendto(bytes(adc), ("10.100.5.130", 5003))
        if time.time() - start > 1:
            break
except KeyboardInterrupt:
    pass
finally:
    spi.close()
