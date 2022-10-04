#! /usr/bin/env python3

from itertools import product
from random import shuffle
from subprocess import check_output


VERSIONS = [
    line.split(" ")[1] for line in
    check_output(["pacman", "-Q"], text=True).split("\n")
    if line
]


def main():
    """Compare results of vercmp and cmpver to test the latter."""

    shuffle(VERSIONS)

    for index, (lhs, rhs) in enumerate(product(VERSIONS, VERSIONS), start=1):
        if index % 1000 == 0:
            print(index, "combinations processed")

        if (r1 := vercmp(lhs, rhs)) != (r2 := cmpver(lhs, rhs)):
            print(lhs, rhs, r1, r2)


def vercmp(lhs: str, rhs: str) -> str:
    """Return output from pacman's vercmp."""

    return check_output(["vercmp", lhs, rhs], text=True).strip()


def cmpver(lhs: str, rhs: str) -> str:
    """Returns output from reverse-engineered cmpver."""

    return check_output(["cmpver", lhs, rhs], text=True).strip()


if __name__ == '__main__':
    main()
