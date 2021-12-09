#!/usr/bin/python


def part1(coords) -> int:
    lps = []
    for (x, y), v in coords.items():
        sc = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        if all(map(lambda cv: cv is None or cv > v, [coords.get(s) for s in sc])):
            lps.append(v)
    return sum([lp + 1 for lp in lps])


def main():
    hm = [[int(i) for i in l.strip()] for l in open("inputs/input9", 'r').readlines()]
    coords = {(x, y): v for y, l in enumerate(hm) for x, v in enumerate(l)}

    print("Part 1:", part1(coords))


if __name__ == "__main__":
    main()
