use std::{sync::{Arc, mpsc::channel}, cmp::min};
use threadpool::ThreadPool;

extern crate num_cpus;

// type of output
type OUT = i32; // can be changed

fn main() {
    let input_size = 102; // can be changed
    let threshold = 20; // can be changed

    let input = Arc::new(get_vec::<i32>(input_size)); // Type can be changed
    
    if threshold >= input_size {
        let res = process(&input[..]);
        println!{"{:#?}", res};

        return
    }
    
    let pool = ThreadPool::new(num_cpus::get());
    let step = threshold;
    let mut res = get_vec::<i32>(input_size);

    for i in 0..input_size / step + 1 {
            
        let helper = input.clone();
        let (sender, receiver) = channel();
        let lower_bound: usize = (i * step).try_into().unwrap();
        let upper_bound: usize = min((i + 1) * step, helper.len() as i32).try_into().unwrap();
        
        pool.execute(move|| 
            { 
                let sub_res = process(&helper[lower_bound..upper_bound]);
                sender.send(sub_res).expect("Unable to send data");
                println!("Thread {i} passed")
            });

        let recv_value = receiver.recv().expect("Unable to receive data");
        res[lower_bound..upper_bound].copy_from_slice(&recv_value);
    }

    pool.join();

    println!("{:#?}", res);
}

pub fn process<T>(vec: &[T]) -> Vec<OUT>
where 
    OUT: Default + Clone,
{
    let mut res = Vec::with_capacity(vec.len());

    for (_ind, elem) in vec.iter().enumerate() {
        res.push(f(elem))
    } 
    
    res
}

// given function
fn f<T>(_t: T) -> OUT {
    5
    // OUT::default()
}

pub fn get_vec<T>(n: i32) -> Vec<T> 
where 
    T: Default + Clone,
{
    vec![T::default(); n as usize]
}