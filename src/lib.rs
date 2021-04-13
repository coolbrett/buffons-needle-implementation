use std::thread;
use std::sync::mpsc::{channel, Receiver, Sender};

//remove below derive before submitting
#[derive(Debug)]
struct Experiment {
    length: f64,
    distance: f64,
    needles: i64,
    threads: i64
}

impl Experiment {

    ///Constructor for Experiment structs
    fn new(){
        //take input then assign that to fields
    }

    ///doc this later
    fn create_thread_pool(&self){

    }

}

///Main function for our program
fn main() {

}