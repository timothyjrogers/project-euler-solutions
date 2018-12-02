#!/usr/bin/env python

"""Solution to Project Euler Problem 1: Multiples of 3 and 5"""
__author__ = "Timothy J. Rogers"

def main():
    total = 0
    for i in range(1000):
        if i % 3 == 0 or i % 5 == 0:
            total += i
    print(total)

if __name__ == "__main__":
    main()