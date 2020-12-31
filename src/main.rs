use clap::{App, Arg, SubCommand};
use std::{
    char, fs,
    io::{self, Write},
    process::exit,
};

mod solve_door;
use solve_door::solve_door;

fn get_value(kind: u16, registers: &[u16]) -> u16 {
    match kind {
        a @ 32768..=32775 => registers[a as usize - 32768],
        a => a,
    }
}

fn get_mem(addr: u16, memory: &[u16]) -> u16 {
    if addr < 32768 {
        memory[addr as usize]
    } else {
        eprintln!("Error: invalid address");
        exit(7);
    }
}

fn set_value(kind: u16, registers: &mut [u16], value: u16) {
    match kind {
        a @ 32768..=32775 => registers[a as usize - 32768] = value,
        _ => {
            eprintln!("uwu");
            exit(6);
        }
    }
}

fn set_mem(addr: u16, memory: &mut [u16], value: u16) {
    if addr < 32768 {
        memory[addr as usize] = value
    } else {
        eprintln!("Error: invalid address");
        exit(8);
    }
}

fn main() {
    let matches = App::new("synacor")
        .version("1.0")
        .author("Ella <computer.backup.15@gmail.com>")
        .arg(
            Arg::with_name("file")
                .takes_value(true)
                .value_name("input file")
                .help("Binary file to be run"),
        )
        .subcommand(SubCommand::with_name("solve").about("Solve the door puzzle"))
        .get_matches();

    if let ("solve", Some(_)) = matches.subcommand() {
        solve_door();
        return;
    }

    let file = match fs::read(
        matches
            .value_of("file")
            .unwrap_or("instructions/challenge.bin"),
    ) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Error: in opening input file: {}", e);
            exit(1);
        }
    };

    if file.len() % 2 != 0 {
        eprintln!("Error: malformed input file, invalid length, was the file truncated?");
        exit(2);
    }

    let mut memory: Vec<_> = file
        .chunks(2)
        .map(|num| ((num[1] as u16) << 8u16) | num[0] as u16)
        .collect();

    if memory.iter().any(|&x| x >= 32776) {
        eprintln!("Error: malformed input data, was the file corrupted?");
        exit(3);
    }

    memory.resize(2usize.pow(15), 0);

    let mut registers = [0u16; 8];
    let mut stack = vec![];

    let mut input_buffer = vec![];

    let mut ip = 0;
    loop {
        let opcode = memory[ip];

        match opcode {
            0 => {
                // halt
                exit(0);
            }
            1 => {
                // set
                let value = get_value(memory[ip + 2], &registers);
                set_value(memory[ip + 1], &mut registers, value);
                ip += 2;
            }
            2 => {
                // push
                let value = get_value(memory[ip + 1], &registers);
                stack.push(value);
                ip += 1;
            }
            3 => {
                // pop
                if let Some(num) = stack.pop() {
                    set_value(memory[ip + 1], &mut registers, num);
                    ip += 1;
                } else {
                    eprintln!("Error: cannot pop empty stack, at ip: {}", ip);
                    exit(5);
                }
            }
            4 => {
                // equal
                let b = get_value(memory[ip + 2], &registers);
                let c = get_value(memory[ip + 3], &registers);
                let value = if b == c { 1 } else { 0 };
                set_value(memory[ip + 1], &mut registers, value);
                ip += 3;
            }
            5 => {
                // greater
                let b = get_value(memory[ip + 2], &registers);
                let c = get_value(memory[ip + 3], &registers);
                let value = if b > c { 1 } else { 0 };
                set_value(memory[ip + 1], &mut registers, value);
                ip += 3;
            }
            6 => {
                // jump
                let value = get_value(memory[ip + 1], &registers) as usize;
                ip = value - 1;
            }
            7 => {
                // jump true
                let value = get_value(memory[ip + 1], &registers);
                if value != 0 {
                    let value = get_value(memory[ip + 2], &registers) as usize;
                    ip = value - 1;
                } else {
                    ip += 2;
                }
            }
            8 => {
                // jump false
                let value = get_value(memory[ip + 1], &registers);
                if value == 0 {
                    let value = get_value(memory[ip + 2], &registers) as usize;
                    ip = value - 1;
                } else {
                    ip += 2;
                }
            }
            9 => {
                // add
                let b = get_value(memory[ip + 2], &registers);
                let c = get_value(memory[ip + 3], &registers);
                set_value(memory[ip + 1], &mut registers, (b + c) % 32768);
                ip += 3;
            }
            10 => {
                // multiply
                let b = get_value(memory[ip + 2], &registers) as u32;
                let c = get_value(memory[ip + 3], &registers) as u32;
                set_value(memory[ip + 1], &mut registers, ((b * c) % 32768) as u16);
                ip += 3;
            }
            11 => {
                // modulo
                let b = get_value(memory[ip + 2], &registers);
                let c = get_value(memory[ip + 3], &registers);
                set_value(memory[ip + 1], &mut registers, b % c);
                ip += 3;
            }
            12 => {
                // and
                let b = get_value(memory[ip + 2], &registers);
                let c = get_value(memory[ip + 3], &registers);
                set_value(memory[ip + 1], &mut registers, b & c);
                ip += 3;
            }
            13 => {
                // or
                let b = get_value(memory[ip + 2], &registers);
                let c = get_value(memory[ip + 3], &registers);
                set_value(memory[ip + 1], &mut registers, b | c);
                ip += 3;
            }
            14 => {
                // not
                let b = get_value(memory[ip + 2], &registers);
                set_value(memory[ip + 1], &mut registers, (!b) % 32768);
                ip += 2;
            }
            15 => {
                // read memory
                let addr = get_value(memory[ip + 2], &registers);
                let value = get_mem(addr, &memory);
                set_value(memory[ip + 1], &mut registers, value);
                ip += 2;
            }
            16 => {
                // write memory
                let addr = get_value(memory[ip + 1], &registers);
                let value = get_value(memory[ip + 2], &registers);
                set_mem(addr, memory.as_mut_slice(), value);
                ip += 2;
            }
            17 => {
                // call
                stack.push(ip as u16 + 2);
                let value = get_value(memory[ip + 1], &registers) as usize;
                ip = value - 1;
            }
            18 => {
                // return
                if let Some(addr) = stack.pop() {
                    ip = addr as usize - 1;
                } else {
                    exit(0);
                }
            }
            19 => {
                // output
                ip += 1;
                let value = get_value(memory[ip], &registers) as u32;
                if let Some(c) = char::from_u32(value) {
                    print!("{}", c);
                } else {
                    print!("{{{}}}", value);
                }
            }
            20 => {
                // input
                if input_buffer.is_empty() {
                    let mut string = String::new();
                    loop {
                        print!("> ");
                        io::stdout()
                            .flush()
                            .expect("Error: while writing to stdout");
                        match io::stdin().read_line(&mut string) {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!("Error: stdio error: {}", e);
                                exit(10);
                            }
                        }

                        if !string.trim().is_empty() {
                            break;
                        }
                    }
                    input_buffer = string.chars().filter(|&x| x != '\r').rev().collect();
                }

                let c = input_buffer.pop().unwrap() as u32;
                if c >= 32768 {
                    eprintln!("Input character too large");
                    exit(9);
                }
                set_value(memory[ip + 1], &mut registers, c as u16);
                ip += 1;
            }
            21 => {
                // noop
            }
            _ => {
                eprintln!("Error: invalid opcode: {}", opcode);
                exit(4);
            }
        }

        ip += 1;
    }
}
