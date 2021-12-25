#!/usr/bin/python
from typing import Tuple

SUCCESS = 0
FAILURE = 1


def part1(target_area):
    return (-target_area[1][0] - 1) * (-target_area[1][0]) // 2


def shoot(velocity: Tuple[int, int], target_area: Tuple[Tuple[int, ...]]):
    pos, velocity = (0, 0), velocity
    while not pos[1] < target_area[1][0]:
        pos = (pos[0] + velocity[0], pos[1] + velocity[1])
        velocity = (velocity[0] if velocity[0] == 0 else velocity[0] + (-1 if velocity[0] > 0 else 1), velocity[1] - 1)

        if target_area[0][0] <= pos[0] <= target_area[0][1] and target_area[1][0] <= pos[1] <= target_area[1][1]:
            return SUCCESS
        elif pos[0] > target_area[0][1] and pos[1] < target_area[1][0]:
            return FAILURE
    return FAILURE


def part2(target_area):
    correct_velocities = []
    for y in range(target_area[1][0], -target_area[1][0]):
        for x in range(target_area[0][1] + 1):
            if shoot((x, y), target_area) == SUCCESS:
                correct_velocities.append((x, y))
    return len(correct_velocities)


def main():
    input_text = open("inputs/input17").read()
    target_coords = tuple([tuple(int(v) for v in c[2:].split("..")) for c in input_text.replace("target area: ", "").split(", ")])

    print("Part 1:", part1(target_coords))
    print("Part 2:", part2(target_coords))


if __name__ == "__main__":
    main()
