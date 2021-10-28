/*
  For Rust bruker jeg kattis-craten: https://crates.io/crates/kattis
  Når den er satt opp riktig kan man gjøre `kattis new [PROBLEMID]` i terminalen og automagisk 
  få en mal og testinput satt opp.
  NB: Dette vil ikke funke for Bergen Open fordi oppgavene ikke ligger på open.kattis.com(men er praktisk ellers)

  Under er malen jeg bruker. Den vil funke helt fint både til vanlige Kattis-oppgaver og til Bergen Open.
*/


#![allow(unused_imports)]
#![allow(unused_macros)]
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    io::{self, BufRead, Read, Write},
};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let nums: Vec<i64> = line.split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let a = nums[0];
        let b = nums[1];
    }
}

// Her er et annet eksempel på en måte å gjøre det. Denne er for en oppgave med
// `t` linjer med 4 tall på hver linje.
/*
fn main() {
    let stdin = io::stdin();
    let mut s = String::new();
    stdin.read_line(&mut s).expect(":(");
    let mut parts = s.split_whitespace().map(|s| s.parse::<i32>());
    let t = parts.next().unwrap().unwrap();
    for _ in 0..t {
        let mut s = String::new();
        stdin.read_line(&mut s).expect(":(");
        let mut parts = s.split_whitespace().map(|s| s.parse::<i32>());
        let first = parts.next().unwrap().unwrap();
        let second = parts.next().unwrap().unwrap();
        let third = parts.next().unwrap().unwrap();
        let fourth = parts.next().unwrap().unwrap();
        let res = someFunction(first, second, third, fourth);
        println!("{}", coins);
    }
}
*/
