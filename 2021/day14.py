#!/usr/bin/python
from collections import Counter

NBR_STEPS = 10


def grow_polymer(template, pairs):
    for idx in range(NBR_STEPS):
        new_template = ""
        for subt in [template[i:i + 2] for i in range(len(template) - 1)]:
            new_template += subt[0]
            if ''.join(subt) in pairs:
                new_template += pairs.get(''.join(subt))
        new_template += template[-1]
        template = new_template

    c = Counter(template)
    return max(c.values()) - min(c.values())


def main():
    template, pairs_str = open("inputs/input14", "r").read().split("\n\n")
    pairs = {p[0]: p[1] for p in map(lambda l: l.split(' -> '), pairs_str.split("\n"))}

    print("Part 1:", grow_polymer(template, pairs))


if __name__ == "__main__":
    main()
