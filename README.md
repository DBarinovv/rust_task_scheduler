# Task
Implement basic function to split some generic computational work between threads. Split should occur only on some threshold - if computational work (input length) is shorter than this threshold, no splitting should occur and no threads should be created.

You get as input:

```rust
Vec
Function f(t: T) -> R
Threshold can be just constant.
```

You should return:

Up to you, but probably some Vec of the same length as input(1)

## Solution ideas
1. With threadpool
- Split the input data into small chunks (if chunk size is smaller than threshold skip next step)
- Use a [threadpool](https://docs.rs/threadpool/latest/threadpool/)
- Compute the answer
2. 
