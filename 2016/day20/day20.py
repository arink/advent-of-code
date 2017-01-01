"""AoC Day 20

Usage:
  day20.py <filename>
  day20.py (-h | --help)
  day20.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt

def read_input_file(filename):
    f = open(filename, 'r')

    numbers = []
    for i in f:
        numbers.append(list(map(int, i.rstrip().split('-'))))

    numbers.sort(key=lambda x: x[0])
    return numbers

def reduce_numbers(numbers):
    prev = 0
    i = 1
    while i < len(numbers):
        # if beginning of this entry is less than end of previous
        if numbers[i][0] <= numbers[prev][1] + 1:
            if numbers[i][1] <= numbers[prev][1]: # Completely inside
                numbers[i] = None
            elif numbers[i][1] > numbers[prev][1]: # partial
                numbers[prev][1] = numbers[i][1]
                numbers[i] = None
            else:
                prev = i
        else:
            prev = i

        i += 1

    numbers = list(filter(None, numbers))
    return numbers


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    numbers = read_input_file(arguments["<filename>"])

    numbers = reduce_numbers(numbers)

    print("First number: " + str(numbers[0][1] + 1))

    summation = 0
    prev = 0
    i = 1
    while i < len(numbers):
        summation += (numbers[i][0] - numbers[prev][1]) - 1
        prev = i
        i += 1
    if numbers[-1][1] != 4294967295:
        summation += 4294967295 - numbers[-1][1]
    print("Total: " + str(summation))

