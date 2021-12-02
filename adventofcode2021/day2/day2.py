#!/usr/bin/python

def part1(instrs: [(str, int)]) -> (int, int):
    l = [(val if dir == "forward" else 0, val if dir == "down" else -val if dir == "up" else 0) for dir, val in instrs]
    return tuple(map(sum, zip(*l)))


def part2(instrs: [(str, int)]) -> (int, int):
    h, d, a = 0, 0, 0

    for dir, val in instrs:
        if dir == "forward":
            h = h + val
            d = d + a * val
        elif dir == "down":
            a = a + val
        elif dir == "up":
            a = a - val

    return h, d


def main():
    instrs = [(line[:-1].split()[0], int(line[:-1].split()[1])) for line in open("input2", 'r').readlines()]

    p1 = part1(instrs)
    p2 = part2(instrs)
    print(f"Part 1: {p1}, product: {p1[0] * p1[1]}")
    print(f"Part 2: {p2}, product: {p2[0] * p2[1]}")


if __name__ == "__main__":
    main()
