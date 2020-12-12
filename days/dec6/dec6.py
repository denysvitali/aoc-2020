import re

example = "".join(open("example.txt").readlines())
puzzle = "".join(open("puzzle.txt").readlines())

problem_input = puzzle


def parse_group(input: str):
    votes = input.split("\n")
    votes_map = dict()
    for vote in votes:
        for el in vote:
            if el not in votes_map:
                votes_map[el] = 0
            votes_map[el] += 1

    return votes_map, len(votes)


def part_a():
    groups = problem_input.split("\n\n")
    sum = 0
    for g in groups:
        parsed, votes = parse_group(g)
        sum += len(parsed)
    return sum


def part_b():
    groups = problem_input.split("\n\n")
    sum = 0
    for g in groups:
        parsed, votes = parse_group(g)
        for i in parsed:
            if parsed[i] == votes:
                sum += 1
    return sum


print("part_a: %d" % part_a())
print("part_b: %d" % part_b())
