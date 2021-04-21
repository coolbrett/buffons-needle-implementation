//! main.rs - The driver that runs a multithreaded implementation of Buffon's needle 
//! 
//! # Atrributes 
//!
//! ** Authors ** Fernando Rodriguez and Brett Dale
//! ** Version ** April 21st 2021
pub mod lib;
use std::sync::mpsc::channel;
//use std::thread; //uncomment to be able to print the ID in the threadpool
use threadpool::ThreadPool; 
use crate::lib::Experiment; 


/// Takes the original Experminent object and divides the number of needles by the 
/// number of threads
/// 
/// # Arguments
/// 'exp' - the Experiment object to be used
/// 
/// # Returns 
/// 'expected_needles' - a tuple that contains the number of needles each new Experiment 
/// drops and the remainder if it has one
fn divide_needles(exp : Experiment) -> (i64, i64) {
    let mut expected_needles = (0, 0);

    // if the number cant be divided evenly we retrieve a remainder
    if (exp.get_needles() % exp.get_threads()) as i64 != 0 {
        expected_needles.1 = (exp.get_needles() % exp.get_threads()) as i64;
    }

    // The number of needles each new object will contain
    expected_needles.0 = (exp.get_needles() / exp.get_threads()) as i64;
    
    return expected_needles;
}

/// Creates a threadpool consisting of however many threads are entered by the user 
/// 
/// # Arguments
/// 'exp' - the original Experiment 
/// 
/// # Returns 
/// 'total' - The total number of hits calculated by all the threads
fn create_threadpool(exp: Experiment) -> f64 {

    // we divide the needles up for the new objects
    let split_needles = divide_needles(exp);
    println!("needles: {}\nremainder: {}", split_needles.0, split_needles.1);

    // a vector that will hold all the new Experiments with a new needle amount
    let mut exp_vec : Vec<Experiment> = Vec::new(); 

    // Iterate through the number of threads and add to exp_vec
    for i in 0..exp.get_threads() as i64 {
        let mut cloned_exp = exp.clone();
        if i == 0 {
            cloned_exp.set_needles((split_needles.0 as f64) + (split_needles.1 as f64))
        }else {
            cloned_exp.set_needles(split_needles.0 as f64);
        }
        exp_vec.push(cloned_exp);
    }

    //sets max to 200, doesn't actually do 200 threads - can only do 200 on Agora
    let pool = ThreadPool::new(200 as usize);

    let (sender, receiver) = channel();

    let size_to_iterate = exp.get_threads() as usize;
    for i in 0..size_to_iterate  {
        let cloned_sender = sender.clone();
        let temp = exp_vec[i];
        pool.execute(move|| {
            // This line allows us to print the IDs to see that they are in random order 
            //println!("I ({:?}) am working on a task: ", thread::current().id());
            cloned_sender.send(temp.toss_needles()).expect("");
        });
    }
    // drop the sender so the program does not keep waiting for more messages
    drop(sender);

    // This is where the main thread gets acces to the data
    let mut total = 0.0;
    for element in receiver {
        //println!("Hits: {}", element);
        total += element;
    }
    println!("total hits: {}", total);
    return total;
}

/// Main function that creates a new Experiment and calls the associated functions
fn main() {
    println!("Buffon's Needle\n");
    let main_exp = Experiment::new();

    let hits  = create_threadpool(main_exp);
    let misses = main_exp.get_needles() - hits;

    let pi = (2.0 * main_exp.get_length() * (hits + misses)) / (main_exp.get_distance() * hits);

    println!("This is PI: {}", pi);
}