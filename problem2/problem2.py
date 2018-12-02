#!/usr/bin/env python

"""Solution to Project Euler Problem 1: Multiples of 3 and 5"""
__author__ = "Timothy J. Rogers"

def main():
    lim = 4000000
    f1, f2 = 1, 2
    total = 2
    while f1 < lim:
        f3 = f1 + f2
        if f3 % 2 == 0:
            total += f3
        f1 = f2
        f2 = f3
    print(total)

if __name__ == "__main__":
    main()