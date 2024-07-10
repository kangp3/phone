#!/usr/bin/env python3

import socketserver

import numpy as np
from scipy.io import wavfile
from matplotlib import pyplot as plt


VALUES = []


class MyUDPHandler(socketserver.BaseRequestHandler):
    """
    This class works similar to the TCP handler class, except that
    self.request consists of a pair of data and client socket, and since
    there is no connection the client address must be given explicitly
    when sending data back via sendto().
    """

    def handle(self):
        global VALUES

        data = self.request[0].strip()
        socket = self.request[1]
        VALUES.append(data)


if __name__ == "__main__":
    HOST, PORT = "0.0.0.0", 5003
    with socketserver.UDPServer((HOST, PORT), MyUDPHandler) as server:
        try:
            server.serve_forever()
        except KeyboardInterrupt:
            print(f"{VALUES=}")
            print(f"{len(VALUES)=}")
            int_vals = [int.from_bytes(v, 'big') >> 3 for v in VALUES]
            data = np.array(int_vals)
            scaled = np.int16(data / np.max(np.abs(data)) * 32767)
            wavfile.write('test.wav', 2000, scaled)
            plt.plot([i for i in range(len(int_vals))], int_vals)
            plt.show()
            plt.close()
