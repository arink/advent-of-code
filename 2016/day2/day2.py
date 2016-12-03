"""AoC Day 2

Usage:
  day2.py <filename>
  day2.py (-h | --help)
  day2.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt

if __name__ == '__main__':
    arguments = docopt(__doc__, version='AoC Day 2')
    print arguments["<filename>"]

