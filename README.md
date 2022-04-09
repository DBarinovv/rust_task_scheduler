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
2. Like [1.](https://github.com/DBarinovv/rust_task_scheduler#solution-ideas), but implement threadpool with own hand
- Implement own threadpool (if threshold is bigger than input data length, otherwise we don't create any other thread)
- We assume that the input data is a ```vec<T>```
- Split the input data into small chunks. Size of each chunk remains to be determined but we definitely don't need very small and very large pieces
- Find out the optimal amount of workers in our threadpool for the current input data. It should depend on the number of cores count and the size of the data
- Use threadpool with chunks as input
- Each worker receives a chunk and compute ```f(chunk)```
- Output data is ```vec<R>``` with a length equal to the input ```vec<T>```
- Each worker after computation puts the calculated data in the correct place in ```vec<T>```
3. Use [Rayon](https://github.com/rayon-rs/rayon)
