"""AoC Day 5

Usage:
  day5.py <input>
  day5.py (-h | --help)
  day5.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt
import hashlib

def find_8ch_ordered_pw(input):
    pw = ""
    count = 0
    while len(pw) < 8:
        m = hashlib.md5()
        m.update(input + str(count))
        if m.hexdigest().startswith("00000"):
            pw += str(m.hexdigest()[5])
        count += 1
    print pw


def find_8ch_position_pw(input):
    pw = list("        ")
    added_chars = 0
    count = 0
    while added_chars < 8:
        m = hashlib.md5()
        m.update(input + str(count))
        if m.hexdigest().startswith("00000"):
            pos = int(m.hexdigest()[5], 16) 
            c = str(m.hexdigest()[6])
            if pos < 8 and pw[pos] == " ":
                pw[pos] = c
                added_chars += 1
        count += 1
    print ''.join(pw)


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    find_8ch_ordered_pw(arguments["<input>"])
    find_8ch_position_pw(arguments["<input>"])


    

