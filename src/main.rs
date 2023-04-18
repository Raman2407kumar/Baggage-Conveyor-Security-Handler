use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
mod constants;

use crate::constants::{
    BAG_OK, BAG_OK_ON_BELT1_MSG, BAG_OK_ON_BELT2_MSG, BAG_OK_ON_BELT3_MSG, BELT_1, BELT_2, BELT_3,
    CHECK_BAG_ON_BELT1_MSG, CHECK_BAG_ON_BELT2_MSG, CHECK_BAG_ON_BELT3_MSG, INITIALISE_BELT_RUN,
    LOCK, SUSPICIOUS_BAG, UNLOCK,
};

fn main() {
    let bags_belt1 = vec![
        BAG_OK,
        SUSPICIOUS_BAG,
        BAG_OK,
        SUSPICIOUS_BAG,
        BAG_OK,
        BAG_OK,
        BAG_OK,
        BAG_OK,
        SUSPICIOUS_BAG,
    ];
    let bags_belt2 = vec![
        BAG_OK,
        SUSPICIOUS_BAG,
        BAG_OK,
        BAG_OK,
        BAG_OK,
        BAG_OK,
        SUSPICIOUS_BAG,
        BAG_OK,
        BAG_OK,
        BAG_OK,
    ];
    let bags_belt3 = vec![
        SUSPICIOUS_BAG,
        BAG_OK,
        BAG_OK,
        SUSPICIOUS_BAG,
        BAG_OK,
        SUSPICIOUS_BAG,
        BAG_OK,
        BAG_OK,
        BAG_OK,
    ];

    // The locks for the belts to stop the belt when suspicious bag is encountered.
    let belt1_lock = Arc::new(Mutex::new(UNLOCK));
    let belt2_lock = Arc::new(Mutex::new(UNLOCK));
    let belt3_lock = Arc::new(Mutex::new(UNLOCK));

    // The lock variable clones to unlock and resume the stopped belt on receiving user input
    // from the receiver thread.
    let receiver_key_lock1 = Arc::clone(&belt1_lock);
    let receiver_key_lock2 = Arc::clone(&belt2_lock);
    let receiver_key_lock3 = Arc::clone(&belt3_lock);

    println!("{INITIALISE_BELT_RUN}");

    // Belt 1 thread
    let belt1_handle = thread::spawn(move || {
        for bag_type in &bags_belt1 {
            thread::sleep(Duration::from_secs(2));
            if *bag_type == SUSPICIOUS_BAG {
                println!("{CHECK_BAG_ON_BELT1_MSG}");
                thread::sleep(Duration::from_secs(1));
                {
                    let mut belt1_lock_val = belt1_lock.lock().unwrap();
                    *belt1_lock_val = LOCK;
                }
                while *(belt1_lock.lock().unwrap()) == LOCK {}
            } else {
                println!("{BAG_OK_ON_BELT1_MSG}");
            }
        }
    });

    // Belt 2 thread
    let belt2_handle = thread::spawn(move || {
        for bag_type in &bags_belt2 {
            thread::sleep(Duration::from_secs(2));
            if *bag_type == SUSPICIOUS_BAG {
                println!("{CHECK_BAG_ON_BELT2_MSG}");
                thread::sleep(Duration::from_secs(1));
                {
                    let mut belt2_lock_val = belt2_lock.lock().unwrap();
                    *belt2_lock_val = LOCK;
                }
                while *(belt2_lock.lock().unwrap()) == LOCK {}
            } else {
                println!("{BAG_OK_ON_BELT2_MSG}");
            }
        }
    });

    // Belt 3 thread
    let belt3_handle = thread::spawn(move || {
        for bag_type in &bags_belt3 {
            thread::sleep(Duration::from_secs(2));
            if *bag_type == SUSPICIOUS_BAG {
                println!("{CHECK_BAG_ON_BELT3_MSG}");
                thread::sleep(Duration::from_secs(1));
                {
                    let mut belt3_lock_val = belt3_lock.lock().unwrap();
                    *belt3_lock_val = 1;
                }
                while *(belt3_lock.lock().unwrap()) == LOCK {}
            } else {
                println!("{BAG_OK_ON_BELT3_MSG}");
            }
        }
    });

    // Receiver thread to read the input and resume the stopped belt using belt number specified by the user.
    let receiver_handle = thread::spawn(move || loop {
        if belt1_handle.is_finished() && belt2_handle.is_finished() && belt3_handle.is_finished() {
            break;
        }
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");
        let belt_to_resume = input
            .trim()
            .parse()
            .expect("Error parsing input to the belt number");
        match belt_to_resume {
            BELT_1 => {
                let mut belt1_lock_val = receiver_key_lock1.lock().unwrap();
                *belt1_lock_val = UNLOCK;
            }
            BELT_2 => {
                let mut belt2_lock_val = receiver_key_lock2.lock().unwrap();
                *belt2_lock_val = UNLOCK;
            }
            BELT_3 => {
                let mut belt3_lock_val = receiver_key_lock3.lock().unwrap();
                *belt3_lock_val = UNLOCK;
            }
            _ => break,
        }
    });
    receiver_handle.join().unwrap();
}
