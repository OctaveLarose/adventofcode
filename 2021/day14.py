#!/usr/bin/python
import math
from collections import Counter


def grow_polymer(template_str, pairs, nbr_steps):
    template = dict(Counter([template_str[i:i + 2] for i in range(len(template_str) - 1)]))
    for idx in range(nbr_steps):
        new_template = {}
        for name, val in template.items():
            if name not in pairs:
                continue
            for v in [name[0] + pairs.get(name), pairs.get(name) + name[1]]:
                new_template[v] = new_template[v] + template.get(name) if v in new_template else template.get(name)

        template = new_template

    fd = {}
    for pair_val, pair_nbr in template.items():
        fd[pair_val[0]] = pair_nbr if pair_val[0] not in fd else pair_nbr + fd[pair_val[0]]
        fd[pair_val[1]] = pair_nbr if pair_val[1] not in fd else pair_nbr + fd[pair_val[1]]
    return math.ceil(max(fd.values()) / 2) - math.ceil(min(fd.values()) / 2)


def main():
    template_str, pairs_str = open("inputs/input14", "r").read().split("\n\n")
    pairs = {p[0]: p[1] for p in map(lambda l: l.split(' -> '), pairs_str.split("\n"))}

    print("Part 1:", grow_polymer(template_str, pairs, 10))
    print("Part 2:", grow_polymer(template_str, pairs, 40))


if __name__ == "__main__":
    main()
