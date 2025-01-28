```
a <- 1
a <- 2
a <- 3

b <- 0
b <- [a]

reveal b
-- [0, [1, 2, 3]]

[b] <- 4
b <- 5

reveal b 
-- [0, [1, 2, 4], 5]

```

distinction between scoping into a sub history and getting all the values stored in the history, not just the most recent

```
a <- 1 
a <- 2 
a <- 3 

c <- 5
c <- 6

b <- all a
b <- all c

reaval b
-- [[1, 2, 3], [5, 6]]

[b] <- 4
[prev b] <- 7

reaval b
-- [[1, 2, 3, 4], [5, 6, 7]]


-- how would b + 1 be evalulated? 
-- runtime error?

```


so the left hand of the assignment need to be an expression too

should make a special history expression 