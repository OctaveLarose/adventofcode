#!/usr/bin/python

def reduce_stack(stack):
    pairs = ['()', '[]', '{}', '<>']
    while len(stack) != 0 and stack[-1] in ")]}>":
        if ''.join(stack[-2:]) in pairs:
            stack = stack[:-2]
        else:
            return None
    return stack


def part1(lines) -> int:
    points_map = {')': 3, ']': 57, '}': 1197, '>': 25137}
    invalid_chars = []

    for l in lines:
        stack = []
        for c in l:
            stack.append(c)
            stack = reduce_stack(stack)

            if stack is None:
                invalid_chars.append(c)
                break

            if len(stack) == 0:
                break

    return sum([points_map.get(c) for c in invalid_chars])


def main():
    lines = [list(l.strip()) for l in open("inputs/input10", 'r').readlines()]

    print("Part 1:", part1(lines))


if __name__ == "__main__":
    main()
