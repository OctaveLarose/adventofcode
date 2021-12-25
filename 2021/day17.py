#!/usr/bin/python
from typing import Tuple

OVERSHOT = -1
UNDERSHOT = -2
FELLTHROUGH = -3


def update_system(pos: Tuple[int, int], velocity: Tuple[int, int]):
    pos = (pos[0] + velocity[0], pos[1] + velocity[1])
    velocity = (velocity[0] + (-1 if velocity[0] > 0 else 1), velocity[1] - 1)
    return pos, velocity


def has_overshot(pos: Tuple[int, int], target_area: Tuple[Tuple[int, ...]]):
    return pos[0] > target_area[0][1] and pos[1] < target_area[1][0]


def has_undershot(pos: Tuple[int, int], target_area: Tuple[Tuple[int, ...]]):
    return pos[0] < target_area[0][0] and pos[1] < target_area[1][0]


def is_in_target_area(pos: Tuple[int, int], target_area: Tuple[Tuple[int, ...]]):
    return target_area[0][0] <= pos[0] <= target_area[0][1] and \
            target_area[1][0] <= pos[1] <= target_area[1][1]


def shoot(velocity: Tuple[int, int], target_area: Tuple[Tuple[int, ...]]):
    positions = [(0, 0)]
    while not is_in_target_area(positions[-1], target_area) and not positions[-1][1] < target_area[1][0]:
        pos, velocity = update_system(positions[-1], velocity)
        positions.append(pos)

    if is_in_target_area(positions[-1], target_area):
        return max([pos[1] for pos in positions])
    elif has_overshot(positions[-1], target_area):
        return OVERSHOT
    elif has_undershot(positions[-1], target_area):
        return UNDERSHOT
    else:
        return FELLTHROUGH


def adjust_repeatedly(target_area):
    velocity = (0, 15000)
    prev_res = None
    while True:
        res = shoot(velocity, target_area)
        print(velocity)
        if res >= 0:
            return velocity[1]
        elif res == OVERSHOT:
            velocity = (velocity[0] - 1, velocity[1])
        elif res == UNDERSHOT:
            velocity = (velocity[0] + 1, velocity[1])
        elif res == FELLTHROUGH:
            sub_res = None
            while sub_res != UNDERSHOT:
                # print("Undershot hunt:", velocity)
                velocity = (velocity[0] - 1, velocity[1])
                sub_res = shoot(velocity, target_area)
                if sub_res >= 0:
                    return velocity[1]
            while sub_res != OVERSHOT:
                # print("Overshot hunt:", velocity)
                velocity = (velocity[0] + 1, velocity[1])
                sub_res = shoot(velocity, target_area)
                if sub_res >= 0:
                    return velocity[1]
            velocity = (velocity[0], velocity[1] - 1)

        if (prev_res == OVERSHOT and res == UNDERSHOT) or (prev_res == UNDERSHOT and res == OVERSHOT):
            velocity = (velocity[0], velocity[1] - 1)
        prev_res = res


def get_y_t(ini_y, t):
    if t <= ini_y:
        return t * (t + 1) // 2
    else:
        return (ini_y * (ini_y + 1) - (t - ini_y) * ((t - ini_y) + 1)) // 2


def part1(target_area):
    return (-target_area[1][0] - 1) * (-target_area[1][0]) // 2


def main():
    input_text = open("inputs/input17").read()
    # input_text = "target area: x=20..30, y=-10..-5"
    target_coords = tuple([tuple(int(v) for v in c[2:].split("..")) for c in input_text.replace("target area: ", "").split(", ")])

    print(part1(target_coords))


if __name__ == "__main__":
    main()
