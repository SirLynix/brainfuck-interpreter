use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::num::Wrapping;

struct BrainfuckState {
    memory: Vec<Wrapping<u8>>,
    pointer: usize,
    stack: Vec<usize>,
}

impl BrainfuckState {
    fn new() -> BrainfuckState {
        BrainfuckState {
            memory: vec![Wrapping(0u8); 30000],
            pointer: 0,
            stack: Vec::new(),
        }
    }

    fn execute(&mut self, code: &str) {
        let mut i = 0;
        while i < code.len() {
            //println!("{}: {}", i, code.as_bytes()[i] as char);
            match code.as_bytes()[i] as char {
                '>' => {
                    self.pointer += 1;
                    if self.pointer >= self.memory.len() {
                        self.pointer = 0;
                    }
                }
                '<' => {
                    if self.pointer == 0 {
                        self.pointer = self.memory.len() - 1;
                    } else {
                        self.pointer -= 1;
                    }
                }
                '+' => {
                    assert!(
                        self.pointer < self.memory.len(),
                        "Out of bound access at {} (pointer is {})",
                        i,
                        self.pointer
                    );

                    self.memory[self.pointer] += Wrapping(1u8);
                }
                '-' => {
                    assert!(
                        self.pointer < self.memory.len(),
                        "Out of bound access at {} (pointer is {})",
                        i,
                        self.pointer
                    );

                    self.memory[self.pointer] -= Wrapping(1u8);
                }
                '.' => {
                    print!("{}", self.memory[self.pointer].0 as char);
                    io::stdout().flush().unwrap();
                }
                ',' => {
                    let mut buffer = [0u8; 1];
                    io::stdin().read_exact(&mut buffer).expect("Failed to read");
                    self.memory[self.pointer] = Wrapping(buffer[0]);
                }
                '[' => {
                    if self.memory[self.pointer].0 == 0 {
                        // Skip to ending bracket
                        let mut counter = 0;
                        while i < code.len() {
                            i += 1;
                            assert!(i < code.as_bytes().len());

                            match code.as_bytes()[i] as char {
                                '[' => {
                                    counter += 1;
                                }
                                ']' => {
                                    if counter == 0 {
                                        break;
                                    } else {
                                        counter -= 1;
                                    }
                                }
                                _ => {}
                            }
                        }
                    } else {
                        self.stack.push(i);
                    }
                }
                ']' => {
                    if self.memory[self.pointer].0 != 0 {
                        // Skip to opening bracket
                        let mut counter = 0;
                        loop {
                            i -= 1;
                            assert!(i < code.as_bytes().len());

                            match code.as_bytes()[i] as char {
                                ']' => {
                                    counter += 1;
                                }
                                '[' => {
                                    if counter == 0 {
                                        break;
                                    } else {
                                        counter -= 1;
                                    }
                                }
                                _ => {}
                            }
                        }
                    }

                    /*let matching_bracket =
                        self.stack.pop().expect(&format!("No matching [ at {}", i));
                    if self.memory[self.pointer].0 != 0 {
                        i = matching_bracket;
                        continue; //< Skip += 1
                    }*/
                }
                _ => {}
            }

            i += 1;

            /*let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read line");*/
        }
    }
}

fn main() {
    let content = fs::read_to_string("LostKng.b").expect("Something went wrong reading the file");

    let mut bf = BrainfuckState::new();
    bf.execute(&content);
}
