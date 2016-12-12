"""AoC Day 7

Usage:
  day7.py <filename>
  day7.py (-h | --help)
  day7.py --version

Options:
  -h --help     Show this screen.
  --version     Show version.

"""
from docopt import docopt
import re


def find_all_bab(ip):
    bab = []

    if len(ip) >= 3:
        address = [ip[0], ip[1]]
        for i in range(2, len(ip)):
            if address[0] != address[1] and ip[i] == address[0]:
                bab.append(address[0] + address[1] + ip[i])
            address = [address[1], ip[i]]
    return bab


def is_abba(ip):
    abba = False
    
    if len(ip) >= 4:
        address = [ip[0], ip[1]]
        for i in range(2, len(ip) - 1):
            if address[0] != address[1] and ip[i] == address[1] and ip[i + 1] == address[0]:
                abba = True
                break
            else:
                address = [address[1], ip[i]]
    return abba


def split_network(line):
    supernet = []
    hypernet = []
    
    matches = re.split('(\[\w*?\])', line.rstrip())
    for m in matches:
        if m[0] == '[':
            hypernet.append(m[1:-1])
        else:
            supernet.append(m)
    return (supernet, hypernet)


def check_network(filename):
    fd = open(filename, 'r')
    sup_hyp = []
    for l in fd:
        res = split_network(l)
        sup_hyp.append(res)

    tls = 0
    for (supernet, hypernet) in sup_hyp:
        found_in_hypernet = False
        for h in hypernet:
            if is_abba(''.join(h)):
                found_in_hypernet = True
            continue

        if not found_in_hypernet:
            for s in supernet:
                if is_abba(''.join(s)):
                    tls += 1
                    break

    ssl = 0
    for (supernet, hypernet) in sup_hyp:
        bab = []
        for h in hypernet:
            bab.append(find_all_bab(''.join(h)))
        bab = [item for sublist in bab for item in sublist]

        if len(bab) > 0:
            found_in_supernet = False
            for b in bab:
                aba = b[1] + b[0] + b[1]            
                for s in supernet:
                    if aba in s:
                        found_in_supernet = True
                        ssl += 1
                        break
                if found_in_supernet:
                    break
    return (tls, ssl)


if __name__ == '__main__':
    arguments = docopt(__doc__, version='1')
    (tls, ssl) = check_network(arguments["<filename>"])
    print "TLS: " + str(tls)
    print "SSL: " + str(ssl)


    

