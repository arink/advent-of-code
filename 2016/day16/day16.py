"""AoC Day 16

Usage:
  day16.py <input> <length>
  day16.py (-h | --help)
  day16.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt


def checksum(string):
    string = list(string)

    while True:
        i = 0
        checksum = []
        while i < len(string):
            if string[i] == string[i+1]:
                checksum.append("1")
            else:
                checksum.append("0")

            i += 2

        if len(checksum) % 2 == 1:
            break
        else:
            string = checksum
    return ''.join(checksum)


def generate_string(string, length):
    length = int(length)
    while len(string) < length:
        a = string
        b = list(string[::-1])
        
        i = 0
        while i < len(b):
            if b[i] == '0':
                b[i] = '1'
            else:
                b[i] = '0'
            i += 1

        string = a + '0' + ''.join(b)
    return string[0:length]


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    string = generate_string(arguments["<input>"], arguments["<length>"])
    print checksum(string)
