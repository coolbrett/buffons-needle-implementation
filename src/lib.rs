//! lib.rs - Holds the shareable code for creating Experiments
//! 
//! # Attributes 
//! 
//! ** Authors ** Fernando Rodriguez and Brett Dale 
//! ** Version ** April 21st 2021 
use std::io::{stdin, stdout, Write};
use rand::{thread_rng, Rng};
use std::process::exit;

// A struct that represents an Experiment
#[derive(Debug, Clone, Copy)]
pub struct Experiment {
    // holds the length of the needle 
    length: f64,
    // the distance between the lines
    distance: f64,
    // the number of needles to toss for the experiment
    needles: i64,
    // the number of threads to use 
    threads: i64
}

// Implement associated methods for Experiment
impl Experiment {
    ///Constructor for Experiment structs
    pub fn new() -> Self {
        //take input then assign that to fields
        let mut length_buffer = String::new();
        let mut distance_buffer = String::new();
        let mut needles_buffer = String::new();
        let mut threads_buffer = String::new();   

        // Gets the length of the needle 
        print!("Enter the needle length > ");
        stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(&mut length_buffer).
            expect("Failed to read user input");

        let parsed_length = length_buffer.trim().parse::<f64>().
            expect("Cannot parse input that is not numerical");

        if parsed_length < 0.0 {
            println!("Length can not be negative!");
            exit(2);
        }

        // Gets the distance between needles 
        print!("Enter the distance between the lines > ");
        stdout().flush().expect("Failed to flush stdout");

        stdin().read_line(&mut distance_buffer).expect("Failed to read user input");

        let parsed_distance = distance_buffer.trim().parse::<f64>().
            expect("Cannot parse input that is not numerical");

        if parsed_distance < 0.0 {
            println!("Distance can not be negative!");
            exit(2);
        }

        if parsed_length >= parsed_distance {
            println!("\nNeedle length must be less than distance! Re-run the program to try again");
            exit(1);
        }

        // Gets the number of needles 
        print!("Enter the number of needles to drop overall > ");
        stdout().flush().expect("Failed to flush stdout");

        stdin().read_line(&mut needles_buffer).expect("Failed to read user input");

        let parsed_needles = needles_buffer.trim().parse::<i64>().
            expect("Cannot parse input that is not numerical");

        if parsed_needles < 0 {
            println!("Needle count can not be negative!");
            exit(2);
        }

        // Gets the number of threads to use
        print!("Enter the number of threads to use > ");
        stdout().flush().expect("Failed to flush stdout");

        stdin().read_line(&mut threads_buffer).expect("Failed to read user input");

        let parsed_threads = threads_buffer.trim().parse::<i64>().
            expect("Cannot parse input that is not numerical");

        if parsed_threads < 0 {
            println!("Threads can not be negative!");
            exit(2);
        }

        // set the parsed values to an Experiment 
        Experiment {
            length : parsed_length,
            distance : parsed_distance,
            needles : parsed_needles,
            threads : parsed_threads
        }
    }

    /// Gets a reference to the number of needles 
    /// 
    /// # Returns 
    /// &self.needles - a reference to the number of needles
    pub fn get_needles(&self) -> &i64 {
        return &self.needles;
    }

    /// Gets a reference to the length of each needle 
    /// 
    /// # Returns 
    /// &self.length - a reference to the length of each needle 
    pub fn get_length(&self) -> &f64 {
        return &self.length;
    }

    /// Gets a reference to the distance between needles 
    /// 
    /// # Returns 
    /// &self.distance - a reference to the distance between needles
    pub fn get_distance(&self) -> &f64 {
        return &self.distance;
    }

    /// Gets the number of threads needed 
    /// 
    /// # Returns 
    /// self.threads - the number of threads
    pub fn get_threads(&self) -> i64 {
        return self.threads;
    }

    /// Gets a reference to the number of needles 
    /// 
    /// # Arguments
    /// 'value' - the value to set for the number of needles 
    pub fn set_needles(&mut self, value : i64) {
        self.needles = value; 
    }

    /// Toss needles and check hit count
    ///
    /// # Returns
    /// f64 representing the amount of hits
    pub fn toss_needles(&self) -> i64{
        // count the number of hits
        let mut count: i64 = 0;
        let mut rng = thread_rng();

        for _ in 0..self.needles as i64{
            let angle: f64 = rng.gen::<f64>() * 180.0_f64.to_radians();
            let position: f64 = self.distance * rng.gen::<f64>();

            if ( 0.5 * (position + self.length * angle.sin()) >= self.distance)
                && (0.5 * (position - self.length * angle.sin()) <= self.distance)
                || (0.5 * (position + self.length * angle.sin()) >= 0.0)
                && ( 0.5 * (position - self.length * angle.sin()) <= 0.0) {
                count += 1;
            }
        }
        return count;
    }
}