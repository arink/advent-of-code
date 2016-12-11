"""AoC Day 6

Usage:
  day6.py <filename>
  day6.py (-h | --help)
  day6.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt

def update_char_count_dict(line, count_dict):
    line = line.rstrip()
    for i in range(len(line)):
        if line[i] in count_dict[i].keys():
            count_dict[i][line[i]] += 1
        else:
            count_dict[i][line[i]] = 1
    return count_dict


def common_chars(filename, most):
    fd = open(filename, 'r')

    # initialize storage
    l = fd.readline().rstrip()
    count_dict = [{} for _ in range(len(l))]
    count_dict = update_char_count_dict(l, count_dict)

    for l in fd:
        count_dict = update_char_count_dict(l.rstrip(), count_dict)
   
    ecc = ""
    for e in count_dict:
        res = sorted(e.items(), key=lambda x: x[1], reverse=most)
        ecc += res[0][0]
    return ecc


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    print common_chars(arguments["<filename>"], True)
    print common_chars(arguments["<filename>"], False)


    

