#![feature(duration_float)]
use std::sync::mpsc::{Sender, Receiver, channel};
use std::thread;
use std::time::SystemTime;

fn run_printer_app(receiver: Receiver<String>) {
    loop {
        match receiver.recv() {
            Ok(message) => println!("{}", message),
            Err(_) => {}
        }
    }
}

fn run_counter_app(printer: Sender<String>) {
    let mut counter = 0;
    loop {
        let message = format!("COUNTER APP: {}", counter);
        //printer.send(message).unwrap();
        counter += 1;
    }
}

fn run_benchmark_app(printer: Sender<String>) {
    let mut counter = 0;
    let mut benchmark_counter = 1;
    let mut last_time = SystemTime::now();
    loop {
        counter += 1;

        let elapsed = last_time.elapsed().unwrap().as_float_secs();
        if elapsed >= 5.0 {
            let rate = counter as f64 / (elapsed * 5.0);
            let message = format!("BENCHMARK {}: {} messages per second", benchmark_counter, rate);
            printer.send(message).unwrap();

            last_time = SystemTime::now();
            counter = 0;
            benchmark_counter += 1;
        }
    }
}

fn main() {
    let channels = channel();
    let sender_node = channels.0;
    let receiver = channels.1;

    let printer_app = thread::spawn(|| {
        run_printer_app(receiver);
    });

    let sender = sender_node.clone();
    let counter_a_app = thread::spawn(|| {
        run_counter_app(sender);
    });

    let sender = sender_node.clone();
    let counter_b_app = thread::spawn(|| {
        run_counter_app(sender);
    });

    let sender = sender_node.clone();
    let benchmarking_app = thread::spawn(|| {
        run_benchmark_app(sender);
    });

    printer_app.join().unwrap();
    counter_b_app.join().unwrap();
    benchmarking_app.join().unwrap();
    counter_a_app.join().unwrap();
}

fn factorial(n: u64) -> u64 {
    (1..n+1).fold(1, |p, n| p*n)
}