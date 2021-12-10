#!/usr/bin/python

def part1(lines) -> int:
    return 42


def main():
    lines = [list(l.strip()) for l in open("inputs/tests/testinput10", 'r').readlines()]

    print(lines)
    print("Part 1:", part1(lines))


if __name__ == "__main__":
    main()
