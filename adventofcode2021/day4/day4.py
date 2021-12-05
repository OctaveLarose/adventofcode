#!/usr/bin/python

# def part1(values: [[str]]):
#     gamma = int(''.join(map(lambda lst: max(set(lst), key=lst.count), zip(*values))), 2)
#     return gamma * (gamma ^ int(f"0b{'1' * gamma.bit_length()}", 2))


def parse(filename: str) -> ([int], [[int]]):
    with open(filename, 'r') as f:
        nbrs, grids = f.readline().strip().split(","), []
        while f.readline():  # Blank line skip
            grids.append([f.readline().strip().split() for _ in range(5)])
        return nbrs, grids


def main():
    print(parse("testinput4.txt"))

    # values = [list(line[:-1]) for line in open("input3", 'r').readlines()]
    # print(f"Part 1: {part1(values)}")
    # print(f"Part 2: {part2(values)}")


if __name__ == "__main__":
    main()
