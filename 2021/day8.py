#!/usr/bin/python
from collections import Counter


def part1(vals) -> int:
    return len([x for x in [y for x in [v[1] for v in vals] for y in x] if len(x) in [2, 3, 4, 7]])


def part2(vals) -> int:
    letters = {}
    sums = []
    for patterns, output in vals:
        mapped_vals = {i: None for i in range(10)}
        for a, b in [(2, 1), (3, 7), (4, 4), (7, 8)]:
            mapped_vals[b] = next(filter(lambda val: len(val) == a, patterns))

        letters['f'] = next((letter for letter, nbr in Counter(''.join([p for p in patterns])).items() if nbr == 9), None)
        letters['c'] = mapped_vals[1].replace(letters['f'], '')
        letters['e'] = next((letter for letter, nbr in Counter(''.join([p for p in patterns])).items() if nbr == 4), None)

        mapped_vals[2] = next(filter(lambda val: letters['f'] not in val, patterns))
        mapped_vals[6] = next(filter(lambda val: sorted(mapped_vals[8].replace(letters['c'], '')) == sorted(val), patterns))
        mapped_vals[5] = next(filter(lambda val: sorted(mapped_vals[6].replace(letters['e'], '')) == sorted(val), patterns))
        mapped_vals[3] = next(filter(lambda val: len(val) == 5 and val != mapped_vals[2] and val != mapped_vals[5], patterns))
        mapped_vals[9] = next(filter(lambda val: sorted(mapped_vals[8].replace(letters['e'], '')) == sorted(val), patterns))
        mapped_vals[0] = next(filter(lambda val: val not in mapped_vals.values(), patterns))

        sums.append(sum([pow(10, idx) * [k for k, v in mapped_vals.items() if sorted(o) == sorted(v)][0] for idx, o in enumerate(output[::-1])]))
    return sum(sums)


def main():
    vals = [tuple([c.strip().split(" ") for c in l.split("|")]) for l in open("inputs/input8", 'r').readlines()]

    print("Part 1:", part1(vals))
    print("Part 2:", part2(vals))


if __name__ == "__main__":
    main()
