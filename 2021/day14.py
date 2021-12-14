#!/usr/bin/python

NBR_STEPS = 4


def grow_polymer(template, pairs):
    for i in range(NBR_STEPS):
        new_template = ""
        new_template = template
    return template


def main():
    template, pairs_str = open("inputs/tests/testinput14", "r").read().split("\n\n")
    pairs = [(p[0], p[1]) for p in map(lambda l: l.split(' -> '), pairs_str.split("\n"))]

    print("Part 1:", grow_polymer(template, pairs))


if __name__ == "__main__":
    main()
