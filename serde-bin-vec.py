"""This utility makes it easy to go from python data to binary data stored in persistent memory. Note that it accommodates geo-data too.
It is on pypi as https://pypi.org/project/serde-bin-vec/
You will probablby have to change the vec_length to meet your use case.
"""

import struct
import binascii


num_bytes = 32
vec_length = 1024


def save(uuid, vector: list, geo = [0,0], filename='data.bin'):
    f = open(filename, 'ab')
    f.write(uuid.encode())
    f = open(filename, 'ab')
    data = [struct.pack('f', f) for f in vector]
    [f.write(d) for d in data]
    [f.write(d) for d in [struct.pack('f', f) for f in geo]]
    f.close()


def read_all(filename='data.bin'):
    f = open(filename, 'rb')
    output = []
    Is = 'B'*num_bytes
    Fs = 'f'*vec_length
    vec_len = vec_length*4
    while True:
        try:
            tmp = [binascii.hexlify(bytearray(struct.unpack(Is, f.read(num_bytes)))),
                   struct.unpack(Fs, f.read(vec_len)),
                   struct.unpack(Fs, f.read(num_bytes))]
            output.append(tmp)
        except:
            break
    f.close()
    return output


def save_all(list_of_lists, filename='data.bin'):
    for item in list_of_lists:
        save(item[0], item[1])
