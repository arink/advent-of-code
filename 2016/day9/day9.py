"""AoC Day 9

Usage:
  day9.py <filename>
  day9.py (-h | --help)
  day9.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt


def decompress_length_ver2(string):
    length = 0

    i = 0
    while i < len(string):
        if string[i] == '(':
            end = string.find(")", i)
            substr = string[i+1:end]
            counts = substr.split('x')


            length += int(counts[1]) * decompress_length_ver2(string[end+1:end+1+int(counts[0])])

            i = end + int(counts[0])
        else:
            length += 1
        i += 1
    return length



def decompress_string(string):
    chars = list(string.rstrip())
    result = ""

    i = 0
    i = 0
    while i < len(string):
        if string[i] == '(':
            end = string.find(")", i)
            substr = string[i+1:end]
            counts = substr.split('x')
            repeat_substr = string[end+1:end+1+int(counts[0])]
            result += repeat_substr * int(counts[1])
            i = end + int(counts[0])
        else:
            result += str(string[i])
        i += 1
    #print result
    return len(result)


def decompress(filename):
    fn = open(filename, 'r')

    for l in fn:
        print l.rstrip()
        print "\tVersion 1 = " + str(decompress_string(l.rstrip()))
        print "\tVersion 2 = " + str(decompress_length_ver2(l.rstrip()))


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    decompress(arguments["<filename>"])
