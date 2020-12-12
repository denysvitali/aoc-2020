import re
import math

example = open("example.txt").readlines()
puzzle = open("puzzle.txt").readlines()

problem_input = puzzle

row_re = re.compile("^[FB]+")
column_re = re.compile("[LR]+")


def find_row(x):
    row_code = row_re.match(x).group(0)
    row_range = [0, 127]

    for i in row_code:
        distance = row_range[1] - row_range[0]
        if i == "F":
            row_range = [row_range[0], row_range[0] + math.floor(distance / 2)]
        elif i == "B":
            row_range = [row_range[0] + math.ceil(distance / 2), row_range[1]]

    return row_range[0]


def find_column(x):
    column_code = column_re.findall(x)[0]
    column_range = [0, 7]

    for i in column_code:
        distance = column_range[1] - column_range[0]

        if i == "L":
            column_range = [column_range[0], column_range[0] + math.floor(distance / 2)]
        elif i == "R":
            column_range = [column_range[0] + math.ceil(distance / 2), column_range[1]]

    return column_range[0]


def pos(x):
    return find_row(x), find_column(x)


def get_seat_id(p):
    return p[0] * 8 + p[1]


def part_a():
    max_v = -1
    for i in problem_input:
        m_pos = pos(i)
        seat_id = get_seat_id(m_pos)
        if seat_id > max_v:
            max_v = seat_id

    return max_v


def part_b():
    seats = []
    for i in problem_input:
        m_pos = pos(i)
        seat_id = get_seat_id(m_pos)
        seats.append(seat_id)

    seats.sort()
    min_v = min(seats)
    max_v = max(seats)
    found = -1

    for i in range(min_v, max_v):
        if seats[i - min_v] != i:
            found = i
            break
    return found


print("part_a: %d" % part_a())
print("part_b: %d" % part_b())
