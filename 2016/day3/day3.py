"""AoC Day 3

Usage:
  day3.py <filename>
  day3.py (-h | --help)
  day3.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt

def is_triangle(t):
    a = t[0]
    b = t[1]
    c = t[2]
    if (a + b > c) and (a + c > b) and (b + c > a):
        return True
    return False


def count_col_triangles(filename):
    fn = open(filename, 'r')

    potentials = [ [], [], [] ]
    count = 0
    for l in fn:
        t = [int(x) for x in l.split()]
        potentials[0].append(t[0])
        potentials[1].append(t[1])
        potentials[2].append(t[2])

        if len(potentials[0]) == 3:
            for i in range(0, len(potentials)):
                if is_triangle(potentials[i]):
                    count += 1
                potentials[i] = []

    return count


def count_row_triangles(filename):
    fn = open(filename, 'r')

    count = 0
    for l in fn:
        t = [int(x) for x in l.split()]
        if is_triangle(t):
            count += 1

    fn.close()
    return count

if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    num_row_triangles = count_row_triangles(arguments["<filename>"])
    print "Number of row triangles = " + str(num_row_triangles)

    num_col_triangles = count_col_triangles(arguments["<filename>"])
    print "Number of col triangles = " + str(num_col_triangles)
    

