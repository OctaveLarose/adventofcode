#!/usr/bin/python
from typing import Tuple, Dict


def update_system(pos: Dict[str, int], velocity: Dict[str, int]):
    pos['x'] += velocity['x']
    pos['y'] += velocity['y']
    if velocity['x'] != 0:
        velocity['x'] += -1 if velocity['x'] > 0 else 1
    velocity['y'] -= 1
    return pos, velocity


def has_overshot(pos: Dict[str, int], target_area: Tuple[Tuple[int, ...]]):
    return pos['x'] > target_area[0][1] and pos['y'] < target_area[1][0]


def has_undershot(pos: Dict[str, int], target_area: Tuple[Tuple[int, ...]]):
    return pos['x'] < target_area[0][0] and pos['y'] < target_area[1][0]


def is_in_target_area(pos: Dict[str, int], target_area: Tuple[Tuple[int, ...]]):
    return target_area[0][0] <= pos['x'] <= target_area[0][1] and \
            target_area[1][0] <= pos['y'] <= target_area[1][1]


def shoot(velocity: Dict[str, int], target_area: Tuple[Tuple[int, ...]]):
    positions = [{'x': 0, 'y': 0}]
    while not is_in_target_area(positions[-1], target_area) and not positions[-1]['y'] < target_area[1][0]:
        pos, velocity = update_system(positions[-1].copy(), velocity.copy())
        positions.append(pos)

    if is_in_target_area(positions[-1], target_area):
        return max([pos['y'] for pos in positions])
    elif has_overshot(positions[-1], target_area):
        return -1
    elif has_undershot(positions[-1], target_area):
        return -2
    else:
        return -3


def adjust_repeatedly(target_area):
    velocity = {'x': 0, 'y': 100}
    prev_res = None
    while True:
        res = shoot(velocity, target_area)

        if res >= 0:
            return velocity['y']
        elif res == -1:
            velocity['x'] -= 1
        elif res == -2:
            velocity['x'] += 1
        elif res == -3:
            velocity['y'] -= 1

        if (prev_res == -1 and res == -2) or (prev_res == -2 and res == -1):
            velocity['y'] -= 1
        prev_res = res


def main():
    input_text = open("inputs/input17").read()
    # input_text = "target area: x=20..30, y=-10..-5"
    target_coords = tuple([tuple(int(v) for v in c[2:].split("..")) for c in input_text.replace("target area: ", "").split(", ")])

    # velocity = {'x': 6, 'y': 100}
    # print(velocity, ":", shoot(velocity, target_coords))

    print(adjust_repeatedly(target_coords))


if __name__ == "__main__":
    main()
