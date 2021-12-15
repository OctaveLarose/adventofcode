#!/usr/bin/python
from math import sqrt


def part1(cavern):
    len_cavern = int(sqrt(len(cavern)))
    print("Cavern width/length:", len_cavern)
    return None


def main():
    cavern = list(open("inputs/tests/testinput15", "r").read().replace('\n', ''))

    print(cavern)

    print("Part 1:", part1(cavern))


if __name__ == "__main__":
    main()
