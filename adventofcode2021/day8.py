#!/usr/bin/python


def part1() -> int:
    return 42


def main():
    parsed_vals = [int(v) for v in open("inputs/input8", 'r').readline().split(",")]

    print(parsed_vals)

    print("Part 1:", part1())


if __name__ == "__main__":
    main()
