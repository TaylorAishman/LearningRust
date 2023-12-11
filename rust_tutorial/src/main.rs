#![allow(unused)]

use std::io;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Didn't Receive Input"); 

    println!("Hello {}! {}", name.trim_end(), greeting);
}
