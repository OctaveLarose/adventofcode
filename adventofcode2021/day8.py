#!/usr/bin/python


def part1(vals) -> int:
    print(vals)
    return 42


def main():
    vals = [tuple([c.strip().split(" ") for c in l.split("|")]) for l in open("inputs/tests/testinput8", 'r').readlines()]

    print("Part 1:", part1(vals))


if __name__ == "__main__":
    main()
