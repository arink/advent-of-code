"""AoC Day 14

Usage:
  day14.py 
  day14.py (-h | --help)
  day14.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt
import hashlib
import re
from collections import OrderedDict


def find_keys(stretch, salt, start, end, three_dict, five_dict):
    three_regex = re.compile(r"(.)\1{2}")
    five_regex = re.compile(r"(.)\1{4}")

    count = start

    while count <= end:
        check = salt + str(count)

        if stretch:
            for i in range(2017):
                m = hashlib.md5()
                m.update(check)
                check = m.hexdigest()
        else:
            m = hashlib.md5()
            m.update(check)
        dig = m.hexdigest()
        #print str(count) + ": " + check + " = " + dig

        three_m = re.search(three_regex, dig)
        if three_m:
            three_dict[three_m.group(0)[0]].append(count)
        
        five_m = re.findall(five_regex, dig)
        if len(five_m):
            for v in five_m:
                five_dict[v].append(count)

        count += 1

    keys = []
    for (k,v) in five_dict.iteritems():
        for loc in v:
            for entry in three_dict[k]:
                if loc > entry and loc - entry <= 1000:
                    keys.append( (k, entry) )

    keys.sort(key=lambda tup: tup[1])
    if len(keys) >= 64:
        word  = "" if stretch else "out"
        print salt + " with" + word + " stretching" + str(keys[63])
    else:
        print "Not enough keys"


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')

    #salt = "abc" # Test
    salt = "yjdafjpo"
    three_dict = OrderedDict()
    five_dict = OrderedDict()

    for i in range(16):
        hexchar = format(i, 'x')
        three_dict[hexchar] = []
        five_dict[hexchar] = []

    find_keys(False, salt, 0, 30000, three_dict, five_dict)



    three_dict = OrderedDict()
    five_dict = OrderedDict()

    for i in range(16):
        hexchar = format(i, 'x')
        three_dict[hexchar] = []
        five_dict[hexchar] = []

    find_keys(True, salt, 0, 30000, three_dict, five_dict)

