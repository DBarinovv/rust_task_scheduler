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
- Find out the optimal amount of workers in our threadpool for the current input data. ~~It should depend on the number of cores count and the size of the data~~ Let's use [crate num_cpus](https://docs.rs/num_cpus/latest/num_cpus/)
- Use threadpool with chunks as input
- Each worker receives a chunk and compute ```f(chunk)```
- Output data is ```vec<R>``` with a length equal to the input ```vec<T>```
- Each worker after computation puts the calculated data in the correct place in ```vec<T>```
3. Use [Rayon](https://github.com/rayon-rs/rayon)

## Implementations  
1. First of all I implemented [1.](https://github.com/DBarinovv/rust_task_scheduler#solution-ideas). There were some main thoughts and problems:
- if ```threshold >= input_size ``` we don't create threadpool
- we create `threadpool` with `num_cpus` workers. Look at [crate num_cpus](https://docs.rs/num_cpus/latest/num_cpus/)
- we need to send vector to different threads. The solution is to use [Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) and clone the reference for each thread
- split the input data into `N` slices, where `N` is some step
- for each thread run `process` function 
- `process` is a template function that returns ```Vec<Out>``` with the same size as the input slice
- we also need to send back some data (```Vec<Out>```). The solution is to use [channel](https://doc.rust-lang.org/rust-by-example/std_misc/channels.html)
- than we wait until all threads have finished their jobs (using ```pool.join()```)
- we output some additional information as well
