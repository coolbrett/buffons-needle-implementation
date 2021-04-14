use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender}; //idk if we need this 
use threadpool::ThreadPool; 
use std::io::{stdin, stdout, Write}; // had to import 'Write' to get flush() working 


//remove below derive before submitting
#[derive(Debug, Clone)]
struct Experiment {
    length: f64,
    distance: f64,
    needles: i64,
    threads: i64
}

impl Experiment {

    ///Constructor for Experiment structs
    fn new() -> Self {
        //take input then assign that to fields
        let mut l_buffer = String::new();
        let mut d_buffer = String::new();
        let mut n_buffer = String::new();
        let mut t_buffer = String::new();   

        // Gets the length of the needle 
        print!("Enter the needle length > ");
        stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(&mut l_buffer).expect("Failed to read user input");

        let p_length = l_buffer.trim().parse::<f64>().expect("Failed to parse input");

        // Gets the distance between needles 
        print!("Enter the distance between the lines > ");
        stdout().flush().expect("Failed to flush stdout");

        stdin().read_line(&mut d_buffer).expect("Failed to read user input");

        let p_distance = d_buffer.trim().parse::<f64>().expect("Failed to parse input");

        // Gets the number of needles 
        print!("Enter the number of needles to drop overall > ");
        stdout().flush().expect("Failed to flush stdout");

        stdin().read_line(&mut n_buffer).expect("Failed to read user input");

        let p_needles = d_buffer.trim().parse::<i64>().expect("Failed to parse input");

        // Gets the number of threads to use
        print!("Enter the number of threads to use > ");
        stdout().flush().expect("Failed to flush stdout");

        stdin().read_line(&mut t_buffer).expect("Failed to read user input");

        let p_threads = t_buffer.trim().parse::<i64>().expect("Failed to parse input");

        Experiment {
            length : p_length,
            distance : p_distance,
            needles : p_needles,
            threads : p_threads
        }

    }
}

// fn create_threadpool(Experiment exp) {

// }

///Main function for our program
fn main() {
    println!("Buffon's Needle\n");

    let new_exp = Experiment::new();
    let exp_clone = new_exp.clone();
    println!("This is the cloned needle len: {} ", exp_clone.distance)
}
