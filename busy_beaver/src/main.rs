use std::{collections::HashMap, env};

const HALTED: char = 'H';
const MAX_ITERATIONS: usize = 8;

type Symbol = usize;
type State = char;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

struct Transition {
    symbol: Symbol,
    state: State,
    direction: Direction,
} 

impl Transition {
    fn new(symbol: Symbol, state: State, direction: Direction) -> Self {
        Transition { symbol, state, direction }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ProgramKey {
    symbol: Symbol,
    state: State,
}

impl ProgramKey {
    fn new(symbol: Symbol, state: State) -> Self {
        ProgramKey { symbol, state }
    }
}

fn  main() {
    let args: Vec<String> = env::args().collect();
    let states = args.get(1).map_or(1, |arg| arg.parse::<usize>().unwrap_or(1));
    let symbols = args.get(2).map_or(2, |arg| arg.parse::<usize>().unwrap_or(2));
    ZanyZoo::new(states, symbols).run();

}

struct ZanyZoo {
    states: usize,
    symbols: usize,
}

impl ZanyZoo {
    fn new(states: usize, symbols: usize) -> Self {
        ZanyZoo { states, symbols }
    }

    fn run(&self) {
        
    }
}

struct BusyBeaver {
    program: HashMap<ProgramKey, Transition>,
    halted: bool,
    tape: Vec<usize>,
}

impl BusyBeaver {
    fn new(program: HashMap<ProgramKey, Transition>) -> Self {
        BusyBeaver { 
            program,
            halted: false,
            tape: vec![0; MAX_ITERATIONS],
         }
    }

    fn run(&mut self) {
        let mut current_pos = 0 as usize;
        let mut current_state: State = 'A';
        for iteration in 1..(MAX_ITERATIONS + 1) {
            let current_symbol = self.tape[current_pos];
            let transition = self.program.get(&ProgramKey::new(current_symbol, current_state));
            match transition {
                Some(transition) => {
                    println!("Iteration {}: Current State: {}, Current Symbol: {}, New Symbol: {}, New State: {}, Direction: {:?}", 
                             iteration, current_state, current_symbol, transition.symbol, transition.state, transition.direction);
                    if transition.state == HALTED {
                        println!("Halting after {} iterations", iteration);
                        break;
                    }
                    self.tape[current_pos] = transition.symbol;
                    match transition.direction {
                        Direction::Left => {
                            if current_pos == 0 {
                                self.tape.insert(0, 0); // Extend tape to the left
                            } else {
                                current_pos -= 1;
                            }                            
                        },
                        Direction::Right => {
                            if current_pos >= self.tape.len() - 1 {
                                println!("Cannot move right from the end of tape");
                                break;
                            }
                            current_pos += 1;
                        },
                    }
                    
                    current_state = transition.state;                   
                },
                None => {
                    println!("No transition found for symbol {} in state {}", current_symbol, current_state);
                    break;
                }
            }
            println!("{:?}", self.tape)
        }
    }


}

mod test {
    use super::*;

    #[test]
    fn test_busy_beaver() {
        let mut program = HashMap::new();
        program.insert(ProgramKey::new(0, 'A'), Transition::new(1, 'B', Direction::Right));
        program.insert(ProgramKey::new(0, 'B'), Transition::new(1, 'A', Direction::Left));
        program.insert(ProgramKey::new(1, 'A'), Transition::new(1, 'B', Direction::Left));
        program.insert(ProgramKey::new(1, 'B'), Transition::new(1, HALTED, Direction::Right));
        let mut bb = BusyBeaver::new(program);
        bb.run();
    }
}