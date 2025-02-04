# Looping 

Every program in Sequence is treated as if was implicity wrapped in a loop. For example the following program 
``` 
print(1 + 1)
```
will print `2` forever. In order to control the end of a loop, an `expect` block can be added to the program to specify the termination condition.
```
expect true { 
    print(1 + 1)
}
``` 
This program prints 2 and then terminates. Expect block are checked on each loop (including the first) after running the body of the program. If the expresion is evaluates as true, the code block is executed and then the program terminates. Running a program without an expect block will generate warning.

The `begin` executes only the first time the program executes. 
```
begin {
    print(1 + 1)
}
print(1 + 2)
```
This will print 2 followed by an infinite number of 3 since there is no expect block set.

# Histories

Variables in Sequence are called Histories are can store all values they have ever had. Assignments are done with the `<-` operator. Using a history in an expression will return its most recent value. The previous value can be fetched by the `prev` operator. 
```
a <- 2
print(a) -- 2
a <- 3
print(a, prev a) -- 3 2
```

Histories can be indexed using the accessor operator `::`. The history be indexed into (the source) must be preceeded by `$`. If the history is on the left side, it counts back from the end of the history. If the history is on the right side, it counts forward from index 0. 
```
a <- 1
a <- 2
a <- 3
a <- 4
a <- 5

print($a::1) -- 4
print(1::$a) -- 2

print(prev a == $a::1) -- true 
print(a == $a::0) -- true
```

Previous values are read-only, so `$a::1 <- 5` is not allowed.

The len of a history is returned by the `#` operator. 
```
a <- 1
a <- 2
a <- 3
a <- 4
a <- 5

print(#a) -- 5
```

The deep copy operator `=:` allows one history to be replaced by another. 

```
a <- 1
a <- 2
a <- 3

b <- 1
b <- 2

print(#b) -- 2

b =: a -- b is now an indentical copy of a

print(#b) -- 3
print(a == b) -- true
```

The `reveal` statement can be used to print the entire history. 

```
a <- 1
a <- 2
a <- 3
reveal a -- 1 2 3 
```

# Expressions 
Sequence suppots standard arithmetic, multiplication, and comparative operators. Logical operators are `and`, `or`, and `not`. 

Sequence also supports absolute value as `|` as in `|(x - b)`, factorial with `!4` and exponent with `^`.  
