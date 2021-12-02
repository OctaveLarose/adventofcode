#!/usr/bin/python

def part1(nbrs: [int]) -> int:
    counter = 0
    prev = None

    for nbr in nbrs:
        if prev is None:
            prev = nbr
        else:
            if nbr > prev:
                counter += 1
            prev = nbr

    return counter


def part2(nbrs: [int]) -> int:
    counter = 0
    WINDOW_SIZE = 3

    window = nbrs[0:WINDOW_SIZE]
    prev = sum(window)

    for nbr in nbrs[WINDOW_SIZE:]:
        window = window[1:] + [nbr]
        if sum(window) > prev:
            counter += 1
        prev = sum(window)

    return counter


def main():
    nbrs = [int(line[:-1]) for line in open("input1", 'r').readlines()]

    print("Part 1:", part1(nbrs))
    print("Part 2:", part2(nbrs))


if __name__ == "__main__":
    main()
