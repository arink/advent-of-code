"""AoC Day 4

Usage:
  day4.py <filename>
  day4.py (-h | --help)
  day4.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
import re
from docopt import docopt
from collections import defaultdict

def is_real_room(name, checksum):
    name = name.translate(None, "-")
    freq = defaultdict(int)
    for c in name:
        freq[c] += 1

    success = True
    count = 0
    # Sort by alphabet and then frequency (python sort is stable so alphabetical
    # order will hold in case of ties
    freq = sorted(freq.items(), key=lambda x: x[0])
    freq = sorted(freq, key=lambda x: x[1], reverse=True)
    for (k,v) in freq: 
        if checksum[count] == k:
            count += 1
            if count == 5:
                break
        else:
            success = False
    return success


def decrypt_room(name, sector_id):
    shift = sector_id % 26
    true_name = ""
    for c in name:
        if c == '-':
            true_name += ' '
        else:
            #print "BEFORE: " + str(c) + " at " + str(ord(c)) + " with shift " + str(shift)
            new_char = chr(ord(c) + shift)
            if new_char > 'z':
                new_char = chr(ord('a') + (ord(new_char) - ord('z')) - 1)
            #print "AFTER:  " + str(new_char) + " at " + str(ord(new_char))
            true_name += new_char
    return true_name


def sum_sector_id_of_real_rooms(filename):
    fn = open(filename, 'r')
    north_pole_sector_id = 0
    sum_ids = 0
    for l in fn:
        m = re.match('^([\w+-]+)-(\d+?)\[(\w+?)\]$', l)
        name = m.group(1)
        sector_id = int(m.group(2))
        checksum = m.group(3)

        if is_real_room(name, checksum):
            sum_ids += sector_id
            if "north" in decrypt_room(name, sector_id):
                north_pole_sector_id = sector_id
    return (sum_ids, north_pole_sector_id)


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    print sum_sector_id_of_real_rooms(arguments["<filename>"])


    

