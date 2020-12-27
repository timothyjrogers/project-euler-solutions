# [Problem 3](https://projecteuler.net/problem=3)

This problem utilizes the [Babylonian Method](https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Babylonian_method) to calculate the square root of a given integer in the ```floored_sqrt``` function; these roots are truncated to the integer. The ```is_prime``` method iterates all numbers between 2 and the square root of the integer being queried and checks for any valid factors -- this square root is calculated using ```floored_sqrt```.

The first loop iterates from 2 to the floored square root of the given puzzle number checking for any factors. Then the factors that are found are checked for primeness and the max is stored in an ever increasing variable ```max```.