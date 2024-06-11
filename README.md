Sequence is an intepreted language written in Rust. It was created by Brennan Cottrell as a 
hobbyist project and is not intended for profesional development.

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
This program prints 2 and then terminates. Expect block are checked before running the body of program on each loop (including the first). If the expresion is evaluates as true, the code block is executed and then the program terminates. Running a program without an expect block will generate warning.

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

Histories can be indexed using the accessor operator `::`. The history be indexed into (the source) must be preceeded by `$`. If the history is on the left side, it counts back from the current value. If on the right side, it counts forward from index 0. 
```
a <- 1
a <- 2
a <- 3
a <- 4
a <- 5

print($a::1) -- 4
print(2::$a) -- 4

print(prev a == $a::1) -- true 
print(a == $a::0) -- true
```

The len of a history is returned by the `#` operator. 
```
a <- 1
a <- 2
a <- 3
a <- 4
a <- 5

print(#a) -- 5
```