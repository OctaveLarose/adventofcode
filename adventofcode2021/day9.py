#!/usr/bin/python


def part1(hm: [[int]]) -> int:
    return 42


def main():
    hm = [[int(i) for i in l.strip()] for l in open("inputs/tests/testinput9", 'r').readlines()]

    print("Part 1:", part1(hm))


if __name__ == "__main__":
    main()
