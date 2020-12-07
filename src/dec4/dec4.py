import re

example = "".join(open("./example.txt").readlines())
puzzle = "".join(open("./puzzle.txt").readlines())

input = puzzle

required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
optional_fields = ["cid"]
height_re = re.compile("^(\d+)(cm|in)$")
hair_re = re.compile("^#[0-9a-f]{6}$")


def has_required_fields(attrs):
    for rf in required_fields:
        if rf not in attrs:
            return False
    return True


def is_valid(passport):
    m = passport.replace("\n", " ").split(" ")
    attributes = dict()
    for match in m:
        split = match.split(":")
        attributes[split[0]] = split[1]

    return has_required_fields(attributes)


def validate_num(x, min, max):
    return min <= int(x) <= max


def validate_height(x):
    m = height_re.match(x)
    if not m:
        return False

    if m.group(2) == "cm":
        return validate_num(m.group(1), 150, 193)
    return validate_num(m.group(1), 59, 76)


def validate_hair(color):
    return hair_re.match(color) is not None


def validate_eye(color):
    return color in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]


pid_re = re.compile("^\d{9}$")


def validate_pid(num):
    return pid_re.match(num) is not None


def validate_attributes(attrs):
    if not validate_num(attrs["byr"], 1920, 2002):
        return False

    if not validate_num(attrs["iyr"], 2010, 2020):
        return False

    if not validate_num(attrs["eyr"], 2020, 2030):
        return False

    if not validate_height(attrs["hgt"]):
        return False

    if not validate_hair(attrs["hcl"]):
        return False

    if not validate_eye(attrs["ecl"]):
        return False

    if not validate_pid(attrs["pid"]):
        return False

    return True


def is_valid_b(passport):
    m = passport.replace("\n", " ").split(" ")
    attributes = dict()
    for match in m:
        split = match.split(":")
        attributes[split[0]] = split[1]

    if not has_required_fields(attributes):
        return False

    return validate_attributes(attributes)


def part_a():
    valid_passports = 0
    for passport in input.split("\n\n"):
        if is_valid(passport):
            valid_passports += 1
    return valid_passports


def part_b():
    valid_passports = 0
    for passport in input.split("\n\n"):
        if is_valid_b(passport):
            valid_passports += 1
    return valid_passports


print("part_a: %d" % part_a())
print("part_b: %d" % part_b())
