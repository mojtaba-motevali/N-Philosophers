use rand::Rng;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, SystemTime};

struct Philospher {
    id: usize,
    number_of_think: usize,
    number_of_eat: usize,
    number_of_wait: usize,
    waiting_time: Vec<u64>,
}
impl Philospher {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            number_of_eat: 0,
            number_of_think: 0,
            number_of_wait: 0,
            waiting_time: Vec::new(),
        }
    }
}
struct Fork {}
impl Fork {
    pub fn new() -> Self {
        Self {}
    }
}
fn calculate_time_to_wait(remaining: &u64, waiting_time: &u64) -> u64 {
    if *remaining < *waiting_time {
        *remaining
    } else {
        *waiting_time
    }
}
fn generate(between: f64) -> u64 {
    let mut rng = rand::thread_rng();
    ((rng.gen::<f64>() * (between - 1.0)) + 1.0) as u64
}
fn calculate_index(i: usize, len: usize) -> usize {
    if i == 0 {
        len - 1
    } else {
        i - 1
    }
}
fn start_philospher(
    i: usize,
    phil: Arc<RwLock<Philospher>>,
    forks: Arc<Vec<Mutex<Fork>>>,
    time_to_run: u64,
) {
    let start_time = SystemTime::now();
    while start_time.elapsed().unwrap().as_secs() < time_to_run {
        let mut philosopher = phil.as_ref().write().unwrap();
        {
            let waiting_time = SystemTime::now();
            let _lock2 = forks[calculate_index(i, forks.len())].lock();
            let _lock1 = forks[i].lock();
            (*philosopher).number_of_wait += 1;
            let waited_in_sec = waiting_time.elapsed().unwrap().as_secs();
            (*philosopher).waiting_time.push(waited_in_sec);
            println!(
                "Philosopher {} waited for {} seconds.",
                philosopher.id, waited_in_sec
            );
            let time_to_eat = calculate_time_to_wait(
                &(time_to_run - start_time.elapsed().unwrap().as_secs()),
                &generate(10.0),
            );
            if time_to_eat > 0 {
                thread::sleep(Duration::from_secs(time_to_eat));
                (*philosopher).number_of_eat += 1;
                println!(
                    "Philosopher {} ate for {} seconds.",
                    philosopher.id, time_to_eat
                );
            }
        }
        let time_to_think = calculate_time_to_wait(
            &(time_to_run - start_time.elapsed().unwrap().as_secs()),
            &generate(5.0),
        );
        if time_to_think > 0 {
            thread::sleep(Duration::from_secs(time_to_think));
            (*philosopher).number_of_think += 1;
            println!(
                "Philosopher {}, thought for {} seconds.",
                philosopher.id, time_to_think
            );
        }
    }
}
fn main() {
    // how long program runs
    let time_to_run = 20;
    // number of philosyphers
    let number_of_philosophers = 5;
    let mut threads = vec![];
    let mut forks = Vec::new();
    let mut philosophers: Vec<Arc<RwLock<Philospher>>> = vec![];
    let start_time = SystemTime::now();
    for i in 0..number_of_philosophers {
        let philospher = Arc::new(RwLock::new(Philospher::new(i)));
        philosophers.push(philospher.clone());
        forks.push(Mutex::new(Fork::new()));
    }
    let arc_forks: Arc<Vec<Mutex<Fork>>> = Arc::from(forks);
    for i in 0..number_of_philosophers {
        let _forks = arc_forks.clone();
        let _phil = philosophers[i].clone();
        threads.push(thread::spawn(move || {
            start_philospher(i, _phil, _forks, time_to_run)
        }));
    }
    for in_thread in threads {
        in_thread.join().unwrap();
    }
    println!(
        "Program finished in {}",
        start_time.elapsed().unwrap().as_secs()
    );
    for philospher in philosophers {
        let ph = philospher.as_ref().read().unwrap();
        let sum = (&ph.waiting_time)
            .into_iter()
            .fold(0, |acc, x: &u64| acc + x);
        println!("philosopher:{}\n number of think:{}\n number of eat: {}\n number of wait: {}\n average timeline: {} seconds"
        ,ph.id,ph.number_of_think,ph.number_of_eat,ph.number_of_wait,sum/ (ph.waiting_time.len() as u64) )
    }
}
