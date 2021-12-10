#!/usr/bin/python
from pprint import pprint


def get_corrupted_char_in_line(line) -> (bool, str):
    stack = []
    for c in line:
        stack.append(c)

        while len(stack) != 0 and stack[-1] in ")]}>":
            if ''.join(stack[-2:]) in ['()', '[]', '{}', '<>']:
                stack = stack[:-2]
            else:
                return c

        if len(stack) == 0:
            return None

    return None


def part1(lines) -> int:
    invalid_chars = [get_corrupted_char_in_line(l) for l in lines]
    return sum([{')': 3, ']': 57, '}': 1197, '>': 25137, None: 0}.get(c) for c in invalid_chars])


def part2(lines) -> int:
    missing_vals = []

    for l in lines:
        if get_corrupted_char_in_line(l) is not None:
            continue
        stack = []
        for c in l:
            stack.append(c)

            while len(stack) != 0 and stack[-1] in ")]}>":
                if ''.join(stack[-2:]) in ['()', '[]', '{}', '<>']:
                    stack = stack[:-2]
                else:
                    return c

        missing_vals.append(list(reversed([{'(': ')', '[': ']', '{': '}', '<': '>'}.get(c) for c in stack])))

    total_points = []
    for m in missing_vals:
        score = 0
        for c in m:
            score *= 5
            score += {')': 1, ']': 2, '}': 3, '>': 4}.get(c)
        total_points.append(score)

    return sorted(total_points)[len(total_points) // 2]


def main():
    lines = [list(l.strip()) for l in open("inputs/input10", 'r').readlines()]

    print("Part 1:", part1(lines))
    print("Part 2:", part2(lines))


if __name__ == "__main__":
    main()
