# Solution on Homework 4

## For `S = {0, 1, 2, 3, 4, 5, 6}`

- 4 + 4 = 8 mod 7 = 1
- 3 x 5 = 15 mod 7 = 1
- inverse of 3
	* inverse in addition $a + (-a) = 0 mod m$: for `3 + -3 = 0`
	* under multiplication inverse of 3 exists and it is 5, 
		- inverse exists when $gcd(a,m) = 1$: $gcd(3,7) = 1$
		- reason $a . b mod c = 1$: for $3 x 5 mod 7 = 1$

## For `S = {0, 1, 2, 3, 4, 5, 6}` is S and `+` a group?

Yes, because the operation satisfies all condition.
[x] we can add and multiply any two numbers and the result is always in the set (ring).
[x] addition and multiplication are associative: $a + (b + c) = (a + b) + c$ and $a . (b . c) = (a . b) . c$ 
[x] their is a neutral element 0 with respect to addition: $a + 0 = a mod m$
[ ] for every element `a` their is a negative element $-a$ such that $a + (-a) = 0$.
[x] their is the neutral element 1 with respect to multiplication: $a . 1 = a mod m$
[x] multiplicative inverse exists for some elements: $a . a^-1 = 1 mod m$

## What is -13 mod 5?

- it is 2, because $5 | (-13) = (-3 x 5) + 2$

## What is the degree and positive root of the polynomial $x^3 - x^2 + 4x - 12$

- 3 degree
- $x^3 - x^2 + 4x - 12$ = $(x - 2)(x^2 + x + 6)$ = 2
