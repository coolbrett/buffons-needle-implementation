use std::thread;
use std::sync::mpsc::channel;//idk if we need this 
use threadpool::ThreadPool; 
use std::io::{stdin, stdout, Write}; // had to import 'Write' to get flush() working
use rand::{thread_rng, Rng};



//remove below derive before submitting
#[derive(Debug, Clone,Copy)]
struct Experiment {
    length: f64,
    distance: f64,
    needles: f64,
    threads: f64
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

        let p_needles = n_buffer.trim().parse::<f64>().expect("Failed to parse input");

        // Gets the number of threads to use
        print!("Enter the number of threads to use > ");
        stdout().flush().expect("Failed to flush stdout");

        stdin().read_line(&mut t_buffer).expect("Failed to read user input");

        let p_threads = t_buffer.trim().parse::<f64>().expect("Failed to parse input");

        Experiment {
            length : p_length,
            distance : p_distance,
            needles : p_needles,
            threads : p_threads
        }

    }

    pub fn set_needles(mut self, value : f64) {
        self.needles = value; 
    }

    ///Toss needles and check hit count
    ///
    /// # Returns
    /// * integer representing the amount of hits
    pub fn toss_needles(&self) -> f64{
        // count the number of hits
        let mut count: f64 = 0.0;
        println!("inside toss_needles()");
        let mut rng = rand::thread_rng();

        for _ in 0..self.needles as i64{
            let angle: f64 = rng.gen::<f64>() * 180.0_f64.to_radians();
            let position: f64 = self.distance * rng.gen::<f64>();

            if ( 0.5 * (position + self.length * angle.sin()) >= self.distance)
                && (0.5 * (position - self.length * angle.sin()) <= self.distance)
                || (0.5 * (position + self.length * angle.sin()) >= 0.0)
                && ( 0.5 * (position - self.length * angle.sin()) <= 0.0) {
                count += 1.0;
            }
        }
        println!("toss_needles() is done");
        return count;
    }
}

fn divide_needles(exp : Experiment) -> (i64,i64) {
    let mut expected_needles = (0, 0);

    if (exp.needles % exp.threads) as i64 != 0 {
        expected_needles.1 = (exp.needles % exp.threads) as i64;
    }

    expected_needles.0 = (exp.needles / exp.threads) as i64;
    
    return expected_needles;
}




fn create_threadpool(exp : Experiment) {

    let split_needles = divide_needles(exp);

    let mut exp_vec : Vec<Experiment> = Vec::new(); 
    
    //iterate through dont worry about remainder 
    for _ in 0..exp.threads as i64 {
        let cloned_exp = exp.clone(); 
        cloned_exp.set_needles(split_needles.0 as f64);
        exp_vec.push(cloned_exp);
    }

    // add the raminder to the first element in the vector
    exp_vec[0].set_needles(exp_vec[0].needles + (split_needles.1 as f64));

    let pool = ThreadPool::new(exp.threads as usize);

    let (sender, receiver) = channel();

    let size_to_iterate = exp.threads as usize;
    for i in 0..size_to_iterate  {
        let cloned_sender = sender.clone();
        pool.execute(move|| {
        //println!("I ({:?}) am working on a task: ", thread::current().id());
        cloned_sender.send(exp_vec[i].toss_needles()).expect("");

    });
    }
    drop(sender);

    // This is where the main thread gets acces to the data 
    for element in receiver {
        println!("{}", element);
    }
}

///Main function for our program
fn main() {
    println!("Buffon's Needle\n");
    let main_exp = Experiment::new();
    create_threadpool(main_exp);


    // if main is too large create a calc_pi() function
    // let hits  = new_exp.toss_needles();

    // let misses = new_exp.needles as f64 - hits; 
    // let pi = (2.0 * new_exp.length * (hits + misses)) / (new_exp.distance * hits);

    // Create a new threadpool 

    // println!("Number of hits: {}", hits);
    // println!("Number of misses: {}", misses);
    // println!("Number total : {}", hits + misses);
    // println!("This is PI: {}", pi);
    // Print a usage message if the length is greater that distance apart

}