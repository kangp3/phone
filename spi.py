#!/usr/bin/env python
import socket
from time import time

from gpiozero import MCP3001


sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

dev = MCP3001()
while True:
    a = dev.raw_value
    print(a)
    sock.sendto(dev.raw_value.to_bytes(2, "big"), ("10.100.5.130", 5003))
