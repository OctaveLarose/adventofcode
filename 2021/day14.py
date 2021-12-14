#!/usr/bin/python
import math
from collections import Counter


def grow_polymer(template_p, pairs, nbr_steps):
    for idx in range(nbr_steps):
        new_template_p = {}
        for name, val in template_p.items():
            for v in [name[0] + pairs.get(name), pairs.get(name) + name[1]]:
                new_template_p[v] = new_template_p.get(v, 0) + template_p.get(name)
        template_p = new_template_p

    char_count = {}
    for pair_val, pair_nbr in template_p.items():
        for c in pair_val:
            char_count[c] = char_count.get(c, 0) + pair_nbr

    return math.ceil(max(char_count.values()) / 2) - math.ceil(min(char_count.values()) / 2)


def main():
    template_str, pairs_str = open("inputs/input14", "r").read().split("\n\n")
    template_pairs = dict(Counter([template_str[i:i + 2] for i in range(len(template_str) - 1)]))
    pairs = {p[0]: p[1] for p in map(lambda l: l.split(' -> '), pairs_str.split("\n"))}

    print("Part 1:", grow_polymer(template_pairs, pairs, 10))
    print("Part 2:", grow_polymer(template_pairs, pairs, 40))


if __name__ == "__main__":
    main()
