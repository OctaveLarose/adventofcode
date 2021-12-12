#!/usr/bin/python


def part1(vals):
    return 42


def main():
    conns = [tuple(v.strip().split("-")) for v in open("inputs/tests/testinput12", 'r').readlines()]
    rooms = set([room for pair in conns for room in pair])

    print(conns)
    print(rooms)

    print("Part 1:", part1(conns))


if __name__ == "__main__":
    main()
