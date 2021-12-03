#!/usr/bin/python

def part1(values: [[str]]):
    gamma = int(''.join(map(lambda lst: max(set(lst), key=lst.count), zip(*values))), 2)
    return gamma * (gamma ^ int(f"0b{'1' * gamma.bit_length()}", 2))


def main():
    values = [list(line[:-1]) for line in open("input3", 'r').readlines()]
    print(f"Part 1: {part1(values)}")


if __name__ == "__main__":
    main()
