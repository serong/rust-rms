//! RMS Calculater
//! 
//! A simple CLI application that calculates RMS and Voltage average_square
//! of given signals.

extern crate clap;

use std::f32::consts::PI;
use clap::{Arg, App};



fn main () {
    // TODO: Find a way to get parameters from arguments.
    // let matches = App --- 
    let matches = App::new("RMS Calculator")
                        .version("0.01")
                        .author("github/serong")
                        .about("Calculates RMS and other things.")
                        .arg(Arg::with_name("parse")
                                .help("Parse text 'p,10,25 s,20,25' seperated by space")
                                .long("parse")
                                .short("p")
                                .takes_value(true)
                        )
                        .get_matches();
    
    let mut averages: Vec<f32> = Vec::new();        // List of individual average values.
    let mut rmss: Vec<f32> = Vec::new();            // List of individual rms values.
    let mut total: f32 = 0.0;                       // Running total ms of the signal.


    // Signal::from_input("p,12,15");


    println!("RMS and Averages calculation. \n");

    if let Some(parse) = matches.value_of("parse") {
        // Breaking up the parse text.
        let trimmed = parse.trim();
        let parsed: Vec<&str> = trimmed.split(" ").map(|x| x.trim()).collect();

        for p in parsed {
            println!("Parsing: {}", p);
            let signal: Signal = Signal::from_input(p.to_string());
            
            total = total + signal.time();
            averages.push(signal.average());
            rmss.push(signal.rms());

            println!("--> Current average: \t {}", get_average(&averages, total));
            println!("--> Current RMS: \t {}", get_rms(&rmss, total));
        }
        return;
    }

    loop {   
        let mut command = String::new();            // Command choice.

        println!("\n Options: ");
        println!("   [q] : Finish / Quit");
        println!("   [a] : Add new signal.");

        // Trimmed input command.
        std::io::stdin()
            .read_line(&mut command)
            .expect("Failed to read from stdin.");
        let command = command.trim();

        // Loop break.
        if command.to_string() == "q" {
            break;
        } else if command.to_string() == "a" {
            println!("Enter the signal: ");

            let mut input = String::new();
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read from stdin.");

            let signal: Signal = Signal::from_input(input);
            
            total = total + signal.time();
            averages.push(signal.average());
            rmss.push(signal.rms());

            println!("--> Current average: \t {}", get_average(&averages, total));
            println!("--> Current RMS: \t {}", get_rms(&rmss, total));
        } else {
            // Loop again.
            println!("Wrong command. {}", command);
        }     
        
    }


    println!("Final Average: {}", get_average(&averages, total));
    println!("Final RMS: {}", get_rms(&rmss, total));
}


struct Signal {
    signal: SignalType,
    args: Vec<f32>
}

enum SignalType {
    Periodical,
    Trapezoid,
    Square,
    None
}

impl Signal {
    // s, 12, 25
    fn from_input(s: String) -> Signal{
        // Trimming and splitting arguments into 2 seperate vectors.
        let trimmed = s.trim();
        let splits: Vec<&str> = trimmed.split(",").map(|x| x.trim()).collect();
        let (args, params) = splits.split_at(1);        // args -> ["s"]
                                                        // params -> ["12", "25"]

        // Defining the signal type.
        let mut t: SignalType = SignalType::Periodical;
        if args[0] == "p" { t = SignalType::Periodical;}
        else if args[0] == "s" { t = SignalType::Square;}
        else if args[0] == "t" { t = SignalType::Trapezoid;}
        else { 
            println!("[ERR] Wrong type of signal.");
            t = SignalType::None;
        }


        // Creating the inputs.
        let mut inp: Vec<f32> = Vec::new();
        for a in params {
            match a.parse::<f32>() {
                Ok(n) => inp.push(n),
                Err(_) => println!("Error casting as float.")
            }
        }
        
        Signal {signal: t, args: inp }
    }

    fn average(&self) -> f32 {
        match self.signal {
            SignalType::Square => average_square(self.args[0], self.args[1]),
            SignalType::Periodical => average_per(self.args[0], self.args[1]),
            SignalType::Trapezoid => average_trap(self.args[0], self.args[1], self.args[2]),
            SignalType::None => 0.0
        }
    }

    fn rms(&self) -> f32 {
        match self.signal {
            SignalType::Square => rms_square(self.args[0], self.args[1]),
            SignalType::Periodical => rms_per(self.args[0], self.args[1]),
            SignalType::Trapezoid => rms_trap(self.args[0], self.args[1], self.args[2]),
            SignalType::None => 0.0
        }
    }

    fn time(&self) -> f32{
        match self.signal {
            SignalType::Square => self.args[1],
            SignalType::Periodical => self.args[1],
            SignalType::Trapezoid => self.args[2],
            SignalType::None => 0.0
        }
    }
}


fn average_square(a: f32, t:f32) -> f32 {
    a * t
}

fn average_trap(a:f32, b: f32, t:f32) -> f32 {
    (a + b) * t / 2.0
}

fn average_per(a: f32, t: f32) -> f32 {
    2.0 * a * t / PI
}

fn rms_square(a: f32, t: f32) -> f32 {
    a * a * t
}

fn rms_trap(a: f32, b: f32, t: f32) -> f32 {
    ((a*a) + (a*b) + (b*b)) * t / 3.0
}

fn rms_per(a: f32, t: f32) -> f32 {
    a * a * t / 2.0
}

fn get_average(averages: &Vec<f32>, total: f32) -> f32 {
    let mut sum: f32 = 0.0;

    for i in averages {
        sum = sum + i;
    }

    sum / total
}

fn get_rms(rmss: &Vec<f32>, total: f32) -> f32 {
    let mut sum: f32 = 0.0;

    for i in rmss {
        sum = sum + i;
    }

    let root = sum / total;
    root.sqrt()
}