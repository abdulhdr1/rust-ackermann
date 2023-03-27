[The Ackermann function](https://en.wikipedia.org/wiki/Ackermann_function) is a recursive mathematical function that takes two non-negative integers as input and returns a single non-negative integer as output. It is defined recursively and has three cases:

```
Given A(m,n)
If m = 0, the function returns n + 1.
If n = 0, the function returns itself with A(m-1, 1).
If both m and n are greater than 0, the function calls itself recursively with m-1 and A(m, n-1) as its arguments.
```

Learned about it on [this video](https://www.youtube.com/watch?v=i7sm9dzFtEI)
