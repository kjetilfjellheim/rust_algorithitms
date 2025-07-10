use std::collections::HashMap;
/**
 * State to halt program.
 */
const HALTED: char = 'Z';

/**
 * Symbol and state types.
 * These are used to represent the symbols on the tape and the states of the Busy Beaver program.
 * 
 * Symbols are by convention represented as usize, and states are represented as char.
 */
type Symbol = usize;
type State = char;

/**
 * Array of states used in the Busy Beaver program.
 * This array contains the states from 'A' to 'Y', and the HALTED state 'Z'.
 * 
 * The states are used to represent the different states of the Busy Beaver program.
 * This can be calculated, but this is just easier.
 */
const STATES : [State; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J',
    'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T',
    'U', 'V', 'W', 'X', 'Y', HALTED
];

/**
 * Direction enum to represent the direction of movement on the tape. This is 
 * used in the transitions.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

/**
 * BusyBeaverTransitionResult enum to represent the result of a transition in the Busy Beaver program.
 * It indicates whether to continue processing, break the loop, or if the transition failed.
 * 
 * Continue: Indicates that the program should continue processing the next transition.
 * Break: Indicates that the program should stop processing further transitions.
 * Failed: Indicates that the transition failed, which could happen if the program encounters an unexpected state or symbol.
 */
enum BusyBeaverTransitionResult {
    Continue,
    Break,
    Failed
}

fn  main() {

}

/**
 * Transition struct to represent a transition in the Busy Beaver program.
 */
#[derive(Debug)]
struct Transition {
    // The symbol to write on the tape
    symbol: Symbol,
    // The state to transition to
    state: State,
    // The direction to move the tape head
    direction: Direction,
} 

impl Transition {
    /**
     * Creates a new Transition instance.
     * 
     * # Arguments
     * `symbol` - The symbol to write on the tape.
     * `state` - The state to transition to.
     * `direction` - The direction to move the tape head (Left or Right).
     * 
     * # Returns
     * A new instance of Transition.
     */
    fn new(symbol: Symbol, state: State, direction: Direction) -> Self {
        Transition { symbol, state, direction }
    }
}

/**
 * ProgramKey struct to represent a unique key for the Busy Beaver program.
 * It combines a symbol and a state to uniquely identify a transition
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ProgramKey {
    // The symbol associated with key.
    symbol: Symbol,
    // The state associated with key.
    state: State,
}

impl ProgramKey {
    /**
     * Creates a new ProgramKey instance.
     * 
     * # Arguments
     * `symbol` - The symbol associated with the key.
     * `state` - The state associated with the key.
     * 
     * # Returns
     * A new instance of ProgramKey.
     */
    fn new(symbol: Symbol, state: State) -> Self {        
        ProgramKey { symbol, state }
    }
}

/**
 * ZanyZoo struct to represent the Zany Zoo program.
 * It contains the number of states and symbols, and it runs the Busy Beaver program.
 */
struct ZanyZoo {
    // A vector of strings representing the Busy Beaver programs.
    // Each string is a Busy Beaver program in the format "1RB1LB_1LA1RZ",
    // where the first part represents the transitions for state A, and the
    // second part represents the transitions for state B, and so on.
    programs: Vec<String>,
}

impl ZanyZoo {
    /**
     * Creates a new ZanyZoo instance.
     * 
     * # Arguments
     * `programs` - A vector of strings representing the Busy Beaver programs.
     * 
     * # Returns
     * A new instance of ZanyZoo.
     */
    fn new(programs: Vec<String>) -> Self {
        ZanyZoo { programs }
    }

    /**
     * Runs the Busy Beaver program for each program in the Zany Zoo.
     * It initializes the program transitions and runs the Busy Beaver program.
     * 
     * # Arguments
     * `max_iterations` - The maximum number of iterations to run the Busy Beaver program.
     * 
     * # Returns
     * A vector of BusyBeaverResult containing the results of running each program.
     */
    fn run(&self, max_iterations: usize) -> Vec<BusyBeaverResult> {
        let mut results: Vec<BusyBeaverResult> = Vec::new();
        for program_str in &self.programs {
            let parts: Vec<&str> = program_str.split("_").collect();
            let symbols = parts.first().unwrap_or(&"0").chars().count() / 3;
            let states = parts.len();
            let mut program: HashMap<ProgramKey, Transition> = HashMap::new();
            for symbol_index in 0..symbols {
                for state_index in 0..states {
                    let state_key = STATES[state_index];
                    let transition_state = Self::get_transition_state(&parts, symbol_index, state_index);
                    let transition_symbol = Self::get_transition_symbol(&parts, symbol_index, state_index);
                    let transition_direction = Self::get_transition_direction(&parts, symbol_index, state_index);
                    program.insert(ProgramKey::new(symbol_index, state_key), Self::get_transition(transition_state, transition_symbol, transition_direction));                    
                }
            }
            let mut busy_beaver = BusyBeaver::new(program);
            results.push(busy_beaver.run(max_iterations));    
        }
        results
    }

    /**
     * Gets the transition symbol from the parts vector.
     * This is a number that represents the symbol to write on the tape.
     * 
     * # Arguments
     * `parts` - A vector of strings representing the Busy Beaver program.
     * `symbol_index` - The index of the symbol in the transition.
     * `state_index` - The index of the state in the transition.
     * 
     * # Returns
     * The transition symbol as a char.
     */
    fn get_transition_symbol(parts: &Vec<&str>, symbol_index: usize, state_index: usize) -> usize {
        let part = match parts.get(state_index) {
            Some(state) => state,
            None => panic!("Invalid part"),
        };
        let symbol_as_str = match part.chars().nth(symbol_index * 3) {
            Some(value) => value,
            None => panic!("Invalid transition symbol"),
        };
        let symbol_value = match symbol_as_str.to_digit(10) {
            Some(value) => value as usize,
            None => panic!("Invalid transition symbol value"),
        };
        symbol_value
    }

    /**
     * Gets the transition state from the parts vector.
     * This is a character that represents the state to transition to.
     * 
     * # Arguments
     * `parts` - A vector of strings representing the Busy Beaver program.
     * `symbol_index` - The index of the symbol in the transition.
     * `state_index` - The index of the state in the transition.
     * 
     * # Returns
     * The transition state as a char.
     */
    fn get_transition_state(parts: &Vec<&str>, symbol_index: usize, state_index: usize) -> State {
        let part = match parts.get(state_index) {
            Some(state) => state,
            None => panic!("Invalid part"),
        };
        let state_value = match part.chars().nth(symbol_index * 3 + 2) {
            Some(value) => value,
            None => panic!("Invalid transition state"),
        };
        state_value
    }

    /**
     * Gets the transition direction from the parts vector.
     * This is a character that represents the direction to move the tape head (L for Left, R for Right).
     * 
     * # Arguments
     * `parts` - A vector of strings representing the Busy Beaver program. Each part corresponds to a state.
     * `symbol_index` - The index of the symbol in the transition.
     * `state_index` - The index of the state in the transition.
     * 
     * # Returns
     * The transition direction as a char.
     */
    fn get_transition_direction(parts: &Vec<&str>, symbol_index: usize, state_index: usize) -> char {
        let part = match parts.get(state_index) {
            Some(state) => state,
            None => panic!("Invalid part"),
        };
        let state_value = match part.chars().nth(symbol_index * 3 + 1) {
            Some(value) => value,
            None => panic!("Invalid transition state"),
        };
        state_value
    }    

    /**
     * Creates a Transition from the transition state, symbol, and direction.
     * 
     * # Arguments
     * `transition_state` - The state to transition to.
     * `transition_symbol` - The symbol to write on the tape.
     * `transition_direction` - The direction to move the tape head (L for Left, R for Right).
     * 
     * # Returns
     * A new instance of Transition.
     */
    fn get_transition(transition_state: char, transition_symbol: usize, transition_direction: char) -> Transition {
        let transition = Transition::new(
            transition_symbol,
            transition_state,
            match transition_direction {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction: {}", transition_direction),
            },
        );
        transition
    }    
}

/**
 * BusyBeaver struct to represent the Busy Beaver program.
 * It contains the program transitions, a flag to indicate if it has halted, and a tape to store the symbols.
 */
struct BusyBeaver {
    // The program transitions, mapping ProgramKey to Transition
    program: HashMap<ProgramKey, Transition>,
}

impl BusyBeaver {
    /**
     * Creates a new BusyBeaver instance.
     * 
     * # Arguments
     * `program` - A HashMap containing the program transitions.
     * 
     * # Returns
     * A new instance of BusyBeaver.
     */
    fn new(program: HashMap<ProgramKey, Transition>) -> Self {
        BusyBeaver { 
            program,
         }
    }

    /**
     * Runs the Busy Beaver program.
     * It initializes the tape, current position, current state, and iteration count.
     * It processes transitions until it either halts or reaches the maximum number of iterations.
     * 
     * # Returns
     * A BusyBeaverResult containing the number of iterations, final tape state, number of values written, and a flag indicating if it halted.
     */
    fn run(&mut self, max_iterations: usize) -> BusyBeaverResult {
        let mut tape = vec![0; max_iterations];
        let mut halted: bool = false;
        let mut current_pos = 0 as usize;
        let mut current_state: State = 'A';
        let mut iteration = 0 as usize;
        for _iteration in 1..(max_iterations + 1) {
            iteration += 1;
            let current_symbol = tape[current_pos];
            let transition = self.program.get(&ProgramKey::new(current_symbol, current_state));
            match Self::handle_transition(&mut tape, &mut halted, &mut current_pos, &mut current_state, transition) {
                BusyBeaverTransitionResult::Continue => { },
                BusyBeaverTransitionResult::Break => break,
                BusyBeaverTransitionResult::Failed => {
                    eprintln!("Transition failed at iteration {}", iteration);
                    break;
                }
            }
        }
        BusyBeaverResult::new(
            iteration,
            tape.clone(),
            tape.iter().filter(|&&x| x >= 1).count(),
            halted,
        )
    }

    /**
     * Handles the transition for the Busy Beaver program.
     * 
     * 1. It writes the transition symbol to the tape at the current position.
     * 2. It updates the current state to the transition state.
     * 3. It checks if the transition state is HALTED, and if so, it sets the halted flag to true and returns ControlFlow::Break.
     * 4. It moves the tape head left or right based on the transition direction.
     * 
     * # Arguments
     * `tape` - A mutable reference to the tape where symbols are written.
     * `halted` - A mutable reference to a boolean indicating if the program has halted.
     * `current_pos` - A mutable reference to the current position of the tape head.
     * `current_state` - A mutable reference to the current state of the program.
     * `current_symbol` - The current symbol being processed.
     * `transition` - An optional reference to the Transition to be applied.    
     * 
     * # Returns
     * A BusyBeaverTransitionResult indicating whether to continue processing, break the loop, or if the transition failed.
     */
    fn handle_transition(tape: &mut Vec<usize>, halted: &mut bool, current_pos: &mut usize, current_state: &mut State, transition: Option<&Transition>) -> BusyBeaverTransitionResult {         
        match transition {
            Some(transition) => {
                tape[*current_pos] = transition.symbol.clone(); // Write the transition symbol to the tape
                *current_state = transition.state.clone();                 
                if Self::check_for_halt(halted, transition) {
                    return BusyBeaverTransitionResult::Break;
                }   
                match transition.direction {
                    Direction::Left => Self::move_tape_left(tape, current_pos),                  
                    Direction::Right => Self::move_tape_right(tape, current_pos)
                }
                              
            },
            None => {
                return BusyBeaverTransitionResult::Failed;
            }
        }
        BusyBeaverTransitionResult::Continue
    }

    /**
     * Checks if the program has halted.
     * If the transition state is HALTED, it sets the halted flag to true and returns a ControlFlow::Break.
     * Otherwise, it returns None.
     */
    fn check_for_halt(halted: &mut bool, transition: &Transition) -> bool {
        if transition.state == HALTED {
            *halted = true;
            return true;
        }
        false
    }    

    /**
     * Moves the tape head to the left.
     * If the current position is at the start of the tape, it inserts a new symbol (0) at the beginning of the tape.
     * Otherwise, it decrements the current position.
     */
    fn move_tape_left(tape: &mut Vec<usize>, current_pos: &mut usize) {
        if *current_pos <= 0 {
            tape.insert(0, 0);                             
        }  else {
            *current_pos -= 1;
        }
    }

    /**
     * Moves the tape head to the right.
     * If the current position is at the end of the tape, it appends a new symbol (0) to the tape.
     * Otherwise, it increments the current position.
     */
    fn move_tape_right(tape: &mut Vec<usize>, current_pos: &mut usize) {
        if *current_pos >= tape.len() {
            tape.push(0);
        }
        *current_pos += 1;
    }

}

/**
 * BusyBeaverResult struct to represent the result of running a Busy Beaver program.
 * It contains the number of iterations, the final state of the tape, the number of symbols with value more than one written, and a flag indicating if it completed.
 */
#[derive(Debug, Clone)]
struct BusyBeaverResult {
    // The number of iterations taken by the Busy Beaver program.
    iterations: usize,
    // The final state of the tape.
    tape: Vec<usize>,
    // The number of symbols with value more than one written on the tape.
    values: usize,
    // Flag to indicate if the program has halted.
    halted: bool,
}

impl BusyBeaverResult {
    /**
     * Creates a new BusyBeaverResult instance.
     * 
     * # Arguments
     * `iterations` - The number of iterations taken by the Busy Beaver program.
     * `tape` - The final state of the tape.
     * `values` - The number of tape with a value higher than one.
     * `halted` - Flag to indicate if the program has halted.
     * 
     * # Returns
     * A new instance of BusyBeaverResult.
     */
    fn new(iterations: usize, tape: Vec<usize>, values: usize, halted: bool) -> Self {
        BusyBeaverResult { iterations, tape, values, halted }
    }
}

mod test {
    use super::*;

    #[test]
    fn test_busy_beaver_1x1_halted() {
        let mut program = HashMap::new();
        program.insert(ProgramKey::new(0, 'A'), Transition::new(1, HALTED, Direction::Right));
        let mut bb = BusyBeaver::new(program);
        let result = bb.run(30);
        assert_eq!(result.iterations, 1);
        assert_eq!(result.values, 1);
        assert!(result.halted);
    }

    #[test]
    fn test_busy_beaver_1x1_failed() {
        let mut program = HashMap::new();
        program.insert(ProgramKey::new(0, 'A'), Transition::new(1, 'B', Direction::Right));
        let mut bb = BusyBeaver::new(program);
        let result = bb.run(30);
        assert_eq!(result.iterations, 2);
        assert_eq!(result.values, 1);
        assert!(!result.halted);
    }    

    #[test]
    fn test_busy_beaver_1x1_continue_forever() {
        let mut program = HashMap::new();
        program.insert(ProgramKey::new(0, 'A'), Transition::new(0, 'A', Direction::Right));
        let mut bb = BusyBeaver::new(program);
        let result = bb.run(30);
        assert_eq!(result.iterations, 30);
        assert_eq!(result.values, 0);
        assert!(!result.halted);
    }        

    #[test]
    fn test_busy_beaver_2x2_halted() {
        let mut program = HashMap::new();
        program.insert(ProgramKey::new(0, 'A'), Transition::new(1, 'B', Direction::Right));
        program.insert(ProgramKey::new(0, 'B'), Transition::new(1, 'A', Direction::Left));
        program.insert(ProgramKey::new(1, 'A'), Transition::new(1, 'B', Direction::Left));
        program.insert(ProgramKey::new(1, 'B'), Transition::new(1, HALTED, Direction::Right));
        let mut bb = BusyBeaver::new(program);
        let result = bb.run(30);
        assert_eq!(result.iterations, 6);
        assert_eq!(result.values, 4);
        assert!(result.halted);
    }

    #[test]
    fn test_busy_beaver_3x2_halted() {
        let mut program = HashMap::new();
        program.insert(ProgramKey::new(0, 'A'), Transition::new(1, 'B', Direction::Right));
        program.insert(ProgramKey::new(0, 'B'), Transition::new(0, 'C', Direction::Right));
        program.insert(ProgramKey::new(0, 'C'), Transition::new(1, 'C', Direction::Left));
        program.insert(ProgramKey::new(1, 'A'), Transition::new(1, 'H', Direction::Right));
        program.insert(ProgramKey::new(1, 'B'), Transition::new(1, 'B', Direction::Right));
        program.insert(ProgramKey::new(1, 'C'), Transition::new(1, 'A', Direction::Left));
        let mut bb = BusyBeaver::new(program);
        let result = bb.run(30);
        assert_eq!(result.iterations, 14);
        assert_eq!(result.values, 6);
        assert!(result.halted);
    }    

    #[test]
    fn test_zany_zoo_2x2() {
        let programs = vec![
            "1RB1LB_1LA1RZ".to_string()
        ];
        let zany_zoo = ZanyZoo::new(programs);
        let results = zany_zoo.run(30);
        println!("{:?}", results);
    }

    #[test]
    fn test_zany_zoo_2x3() {
        let programs = vec![
            "1RB2LB1RZ_2LA2RB1LB".to_string()
        ];
        let zany_zoo = ZanyZoo::new(programs);
        let results = zany_zoo.run(80);
        println!("{:?}", results);
    }    

    #[test]
    fn test_zany_zoo_3x2() {
        let programs = vec![
            "1RB1RZ_1LB0RC_1LC1LA".to_string()
        ];
        let zany_zoo = ZanyZoo::new(programs);
        let results = zany_zoo.run(40);
        println!("{:?}", results);
    }        

}

