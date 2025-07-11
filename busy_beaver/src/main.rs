/**
 * Busy Beaver program in Rust.
 * Description: https://en.wikipedia.org/wiki/Busy_beaver
 */
mod config;

use std::{collections::HashMap, io::Read};

use config::Config;
/**
 * State to halt program.
 */
const HALTED: char = 'Z';

/**
 * Unspecified state, used to indicate that a transition does not specify a symbol or state.
 * This is used in the transitions to indicate that there is no specific action for that transition.
 */
const UNSPECIFICED: char = '-';

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
const STATES: [State; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', HALTED];

/**
 * Direction enum to represent the direction of movement on the tape. This is
 * used in the transitions.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    // Represents moving the tape head to the left
    Left,
    // Represents moving the tape head to the right
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
    // Indicates that the program should continue processing the next transition
    Continue,
    // Indicates that the program should stop processing further transitions
    Break,
}

/**
 * Main function to run the Busy Beaver program.
 * It reads the configuration from stdin, initializes the Zany Zoo with the programs,
 * and runs the Busy Beaver program for each program in the Zany Zoo.
 */
fn main() {
    let config = get_read_config();
    println!("Running Zany Zoo with {} programs", config.programs.len());
    let zany_zoo = ZanyZoo::new(config.programs);
    let results = zany_zoo.run(config.max_iterations);
    for (index, result) in results.iter().enumerate() {
        println!("Program {}: Iterations: {}, Values: {}, Halted: {}, Tape: {:?}", index + 1, result.iterations, result.values, result.halted, result.tape);
    }
    println!("Total halted programs: {}", results.len());
}

/**
 * Reads the configuration from stdin and parses it as TOML.
 * This is used to read the configuration for the Busy Beaver program.
 *
 * The format must be:
 * ```
 * max_iterations = 100
 * programs = [
 *     "1RB1RZ_1LB0RC_1LC1LA",
 *     "1RB1RZ_0LC0RC_1LC1LA"
 * ]
 *
 * # Returns
 * A Config struct containing the maximum number of iterations and the programs to run.
 */
fn get_read_config() -> Config {
    let mut config_str = String::new();
    std::io::stdin().read_to_string(&mut config_str).expect("Failed to read from stdin");
    toml::from_str(config_str.as_str()).expect("Failed to parse configuration")
}

/**
 * BusyBeaverError enum to represent errors that can occur in the Busy Beaver program.
 */
#[derive(Debug)]
enum BusyBeaverError {
    InvalidTransition { message: String, transition: Transition },
    TransitionNotFound { key: ProgramKey, transition: Transition },
    ProgramReadError { message: String },
}

/**
 * Transition struct to represent a transition in the Busy Beaver program.
 */
#[derive(Debug, Clone)]
struct Transition {
    // The symbol to write on the tape
    symbol: Option<Symbol>,
    // The state to transition to
    state: Option<State>,
    // The direction to move the tape head
    direction: Option<Direction>,
}

impl Transition {
    /**get_trans
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
    fn new(symbol: Option<Symbol>, state: Option<State>, direction: Option<Direction>) -> Self {
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
            match Self::run_program(program_str, max_iterations) {
                Ok(result) => results.push(result),
                Err(err) => {
                    Self::print_error(err);
                }
            }
        }
        results
    }

    /**
     * Prints the error message for a BusyBeaverError.
     * This is used to print the error messages in a user-friendly way.
     *
     * # Arguments
     * `err` - The BusyBeaverError to print.
     */
    fn print_error(err: BusyBeaverError) {
        match err {
            BusyBeaverError::InvalidTransition { message, transition } => {
                eprintln!("Invalid Transition: {message} - {transition:?}");
            }
            BusyBeaverError::TransitionNotFound { key, transition } => {
                eprintln!("Transition Not Found: {key:?} - {transition:?}");
            }
            BusyBeaverError::ProgramReadError { message } => {
                eprintln!("Program Read Error: {message}");
            }
        }
    }

    /**
     * Runs a single Busy Beaver program.
     * It prepares the program transitions and runs the Busy Beaver program.
     *
     * # Arguments
     * `program_str` - A string representing the Busy Beaver program in the format "1RB1LB_1LA1RZ".
     * `max_iterations` - The maximum number of iterations to run the Busy Beaver program.
     *
     * # Returns
     * A BusyBeaverResult containing the result of running the program.
     */
    fn run_program(program_str: &str, max_iterations: usize) -> Result<BusyBeaverResult, BusyBeaverError> {
        let parts: Vec<&str> = program_str.split("_").collect();
        let num_symbols = Self::get_number_of_states(&parts)?;
        let num_states = parts.len();
        let program = Self::prepare_program(parts, num_symbols, num_states)?;
        let busy_beaver = BusyBeaver::new(program);
        busy_beaver.run(max_iterations)
    }

    /**
     * Gets the number of states from the parts vector.
     * This is calculated by taking the length of the first part and dividing it by 3,
     * since each transition is represented by 3 characters (symbol, direction, state).
     *
     * # Arguments
     * `parts` - A vector of strings representing the Busy Beaver program.
     *
     * # Returns
     * The number of states in the program.
     */
    fn get_number_of_states(parts: &Vec<&str>) -> Result<usize, BusyBeaverError> {
        match parts.first() {
            // Each transition is 3 characters long (symbol, direction, state)
            Some(first_part) => Ok(first_part.len() / 3),
            None => Err(BusyBeaverError::ProgramReadError { message: "No parts found in the program".to_string() }),
        }
    }

    /**
     * Prepares the Busy Beaver program from the parts vector.
     * It creates a HashMap of ProgramKeys to Transition.
     *
     * # Arguments
     * `parts` - A vector of strings representing the Busy Beaver program.
     * `symbols` - The number of symbols in the program.
     * `states` - The number of states in the program.
     *
     * # Returns
     * A HashMap mapping ProgramKey to Transition.
     * Each ProgramKey is a combination of a symbol and a state, and each Transition contains
     * the symbol to write, the state to transition to, and the direction to move the
     */
    fn prepare_program(parts: Vec<&str>, num_symbols: usize, num_states: usize) -> Result<HashMap<ProgramKey, Transition>, BusyBeaverError> {
        let mut program: HashMap<ProgramKey, Transition> = HashMap::new();
        for symbol_index in 0..num_symbols {
            for (state_index, state_key) in STATES.iter().enumerate().take(num_states) {
                let transition_state = Self::get_transition_state(&parts, symbol_index, state_index)?;
                let transition_symbol = Self::get_transition_symbol(&parts, symbol_index, state_index)?;
                let transition_direction = Self::get_transition_direction(&parts, symbol_index, state_index)?;
                let transition = Self::get_transition(transition_state, transition_symbol, transition_direction)?;
                program.insert(ProgramKey::new(symbol_index, *state_key), transition);
            }
        }
        Ok(program)
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
    fn get_transition_symbol(parts: &Vec<&str>, symbol_index: usize, state_index: usize) -> Result<Option<usize>, BusyBeaverError> {
        let part = match parts.get(state_index) {
            Some(state) => state,
            None => return Err(BusyBeaverError::ProgramReadError { message: format!("Invalid part for symbol index state index {state_index} and parts {parts:?}") }),
        };
        let symbol_as_str = match part.chars().nth(symbol_index * 3) {
            Some(value) => value,
            None => return Err(BusyBeaverError::ProgramReadError { message: format!("Invalid symbol in part part {part:?}") }),
        };
        if symbol_as_str == UNSPECIFICED {
            return Ok(None);
        }
        let symbol_value = match symbol_as_str.to_digit(10) {
            Some(value) => value as usize,
            None => return Err(BusyBeaverError::ProgramReadError { message: format!("Symbol must be a number in part part {part:?} value {symbol_as_str:?}") }),
        };
        Ok(Some(symbol_value))
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
    fn get_transition_state(parts: &Vec<&str>, symbol_index: usize, state_index: usize) -> Result<Option<State>, BusyBeaverError> {
        let part = match parts.get(state_index) {
            Some(state) => state,
            None => return Err(BusyBeaverError::ProgramReadError { message: format!("Invalid part for state index {state_index} and parts {parts:?}") }),
        };
        match part.chars().nth(symbol_index * 3 + 2) {
            Some(value) => {
                if value == UNSPECIFICED {
                    return Ok(None);
                }
                Ok(Some(value))
            }
            None => Err(BusyBeaverError::ProgramReadError { message: format!("Invalid transition state for symbol index {symbol_index} and state index {state_index} in part: {part}") }),
        }
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
    fn get_transition_direction(parts: &Vec<&str>, symbol_index: usize, state_index: usize) -> Result<Option<char>, BusyBeaverError> {
        let part = match parts.get(state_index) {
            Some(state) => state,
            None => return Err(BusyBeaverError::ProgramReadError { message: format!("Invalid part for state index {state_index} and parts {parts:?}") }),
        };
        match part.chars().nth(symbol_index * 3 + 1) {
            Some(value) => {
                if value == UNSPECIFICED {
                    return Ok(None);
                }
                Ok(Some(value))
            }
            None => Err(BusyBeaverError::ProgramReadError { message: format!("Invalid transition direction for symbol index {symbol_index} and state index {state_index} in part: {part}") }),
        }
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
    fn get_transition(transition_state: Option<char>, transition_symbol: Option<usize>, transition_direction: Option<char>) -> Result<Transition, BusyBeaverError> {
        let direction = match transition_direction {
            Some(dir) => {
                if dir == 'L' {
                    Some(Direction::Left)
                } else if dir == 'R' {
                    Some(Direction::Right)
                } else if dir == UNSPECIFICED {
                    None
                } else {
                    return Err(BusyBeaverError::ProgramReadError { message: format!("Invalid transition direction: {dir}") });
                }
            }
            None => None,
        };
        Ok(Transition::new(transition_symbol, transition_state, direction))
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
        BusyBeaver { program }
    }

    /**
     * Runs the Busy Beaver program.
     * It initializes the tape, current position, current state, and iteration count.
     * It processes transitions until it either halts or reaches the maximum number of iterations.
     *
     * # Returns
     * A BusyBeaverResult containing the number of iterations, final tape state, number of values written, and a flag indicating if it halted.
     */
    fn run(self, max_iterations: usize) -> Result<BusyBeaverResult, BusyBeaverError> {
        let mut tape = vec![0; 2];
        let mut halted: bool = false;
        let mut current_pos = 0_usize;
        let mut current_state: State = 'A';
        let mut iteration = 0_usize;
        for _iteration in 1..(max_iterations + 1) {
            iteration += 1;
            let current_symbol = *tape.get(current_pos).unwrap_or(&0);
            let program_key: ProgramKey = ProgramKey::new(current_symbol, current_state);
            let transition = self.program.get(&program_key);
            let transition = match transition {
                Some(transition) => transition,
                None => {
                    return Err(BusyBeaverError::TransitionNotFound { key: program_key, transition: Transition::new(None, None, None) });
                }
            };
            match Self::handle_transition(&mut tape, &mut halted, &mut current_pos, &mut current_state, transition)? {
                BusyBeaverTransitionResult::Continue => {}
                BusyBeaverTransitionResult::Break => break,
            }
        }
        Ok(BusyBeaverResult::new(iteration, tape.clone(), tape.iter().filter(|&&x| x >= 1).count(), halted))
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
     * `transition` - An reference to the Transition to be applied.    
     *
     * # Returns
     * A BusyBeaverTransitionResult indicating whether to continue processing, break the loop, or if the transition failed.
     */
    fn handle_transition(tape: &mut Vec<usize>, halted: &mut bool, current_pos: &mut usize, current_state: &mut State, transition: &Transition) -> Result<BusyBeaverTransitionResult, BusyBeaverError> {
        // Ensure the tape has enough space. This is a special tape that can grow dynamically.
        if *current_pos >= tape.len() {
            tape.push(0);
        }
        // Write the transition symbol to the tape
        if let Some(symbol) = &transition.symbol {
            tape[*current_pos] = *symbol;
        } else {
            return Err(BusyBeaverError::InvalidTransition { message: "Transition symbol was None".to_string(), transition: transition.clone() });
        }
        // Set the current state to the transition state
        if let Some(state) = &transition.state {
            *current_state = *state;
            // Check if the program has halted
            if Self::check_for_halt(halted, state) {
                return Ok(BusyBeaverTransitionResult::Break);
            }
        } else {
            return Err(BusyBeaverError::InvalidTransition { message: "Transition state was None".to_string(), transition: transition.clone() });
        }
        // Move the tape head left or right based on the transition direction
        if let Some(direction) = &transition.direction {
            match direction {
                Direction::Left => Self::move_tape_left(tape, current_pos),
                Direction::Right => Self::move_tape_right(tape, current_pos),
            }
        } else {
            return Err(BusyBeaverError::InvalidTransition { message: "Transition direction was None".to_string(), transition: transition.clone() });
        }
        Ok(BusyBeaverTransitionResult::Continue)
    }

    /**
     * Checks if the program has halted.
     * If the transition state is HALTED, it sets the halted flag to true and returns a ControlFlow::Break.
     * Otherwise, it returns None.
     */
    fn check_for_halt(halted: &mut bool, state: &State) -> bool {
        if *state == HALTED {
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
        if *current_pos == 0 {
            tape.insert(0, 0);
        } else {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_busy_beaver_1x1_halted() {
        let mut program = HashMap::new();
        program.insert(ProgramKey::new(0, 'A'), Transition::new(Some(1), Some(HALTED), Some(Direction::Right)));
        let bb = BusyBeaver::new(program);
        let result = bb.run(30).unwrap();
        assert_eq!(result.iterations, 1);
        assert_eq!(result.values, 1);
        assert!(result.halted);
    }

    #[test]
    fn test_busy_beaver_1x1_failed() {
        let mut program = HashMap::new();
        program.insert(ProgramKey::new(0, 'A'), Transition::new(Some(1), Some('B'), Some(Direction::Right)));
        let bb = BusyBeaver::new(program);
        match bb.run(30) {
            Ok(_) => panic!("Expected an error, but got a result"),
            Err(BusyBeaverError::TransitionNotFound { key, transition }) => {
                assert_eq!(key.symbol, 0);
                assert_eq!(key.state, 'B');
                assert_eq!(transition.symbol, None);
                assert_eq!(transition.state, None);
                assert_eq!(transition.direction, None);
            }
            _ => panic!("Expected a TransitionNotFound error"),
        }
    }

    #[test]
    fn test_busy_beaver_1x1_continue_forever() {
        let mut program = HashMap::new();
        program.insert(ProgramKey::new(0, 'A'), Transition::new(Some(0), Some('A'), Some(Direction::Right)));
        let bb = BusyBeaver::new(program);
        let result = bb.run(30).unwrap();
        assert_eq!(result.iterations, 30);
        assert_eq!(result.values, 0);
        assert!(!result.halted);
    }

    #[test]
    fn test_busy_beaver_2x2_halted() {
        let mut program = HashMap::new();
        program.insert(ProgramKey::new(0, 'A'), Transition::new(Some(1), Some('B'), Some(Direction::Right)));
        program.insert(ProgramKey::new(0, 'B'), Transition::new(Some(1), Some('A'), Some(Direction::Left)));
        program.insert(ProgramKey::new(1, 'A'), Transition::new(Some(1), Some('B'), Some(Direction::Left)));
        program.insert(ProgramKey::new(1, 'B'), Transition::new(Some(1), Some(HALTED), Some(Direction::Right)));
        let bb = BusyBeaver::new(program);
        let result = bb.run(30).unwrap();
        assert_eq!(result.iterations, 6);
        assert_eq!(result.values, 4);
        assert!(result.halted);
    }

    #[test]
    fn test_busy_beaver_3x2_halted() {
        let mut program = HashMap::new();
        program.insert(ProgramKey::new(0, 'A'), Transition::new(Some(1), Some('B'), Some(Direction::Right)));
        program.insert(ProgramKey::new(0, 'B'), Transition::new(Some(1), Some('B'), Some(Direction::Left)));
        program.insert(ProgramKey::new(0, 'C'), Transition::new(Some(1), Some('C'), Some(Direction::Left)));
        program.insert(ProgramKey::new(1, 'A'), Transition::new(Some(1), Some(HALTED), Some(Direction::Right)));
        program.insert(ProgramKey::new(1, 'B'), Transition::new(Some(0), Some('C'), Some(Direction::Right)));
        program.insert(ProgramKey::new(1, 'C'), Transition::new(Some(1), Some('A'), Some(Direction::Left)));
        let bb = BusyBeaver::new(program);
        let result = bb.run(30).unwrap();
        assert_eq!(result.iterations, 21);
        assert_eq!(result.values, 5);
        assert!(result.halted);
    }

    #[test]
    fn test_zany_zoo_2x2() {
        let programs = vec!["1RB1LB_1LA1RZ".to_string()];
        let zany_zoo = ZanyZoo::new(programs);
        let results = zany_zoo.run(30);
        let result = &results[0];
        assert_eq!(result.iterations, 6);
        assert_eq!(result.values, 4);
        assert!(result.halted);
    }

    #[test]
    fn test_zany_zoo_2x3() {
        let programs = vec!["1RB2LB1RZ_2LA2RB1LB".to_string()];
        let zany_zoo = ZanyZoo::new(programs);
        let results = zany_zoo.run(80);
        let result = &results[0];
        assert_eq!(result.iterations, 38);
        assert_eq!(result.values, 9);
        assert!(result.halted);
    }

    #[test]
    fn test_zany_zoo_3x2() {
        let programs = vec!["1RB1RZ_1LB0RC_1LC1LA".to_string()];
        let zany_zoo = ZanyZoo::new(programs);
        let results = zany_zoo.run(40);
        let result = &results[0];
        assert_eq!(result.iterations, 21);
        assert_eq!(result.values, 5);
        assert!(result.halted);
    }

    #[test]
    fn test_zany_zoo_5x2() {
        let programs = vec!["1RB1LC_0LA1RE_0LD0LB_1RA1RZ_1LA0RE".to_string()];
        let zany_zoo = ZanyZoo::new(programs);
        let results = zany_zoo.run(2000);
        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.iterations, 1035);
        assert_eq!(result.values, 35);
        assert!(result.halted);
    }

    #[test]
    fn test_zany_zoo_with_unused_states() {
        let programs = vec!["1RB---_1RC1RZ_0LB---".to_string()];
        let zany_zoo = ZanyZoo::new(programs);
        let results = zany_zoo.run(20);
        assert_eq!(results.len(), 1);
        let result = &results[0];
        assert_eq!(result.iterations, 4);
        assert_eq!(result.values, 2);
        assert!(result.halted);
    }

    #[test]
    fn test_zany_zoo_with_invalid_program() {
        let programs = vec!["1RB---_------_------".to_string()];
        let zany_zoo = ZanyZoo::new(programs);
        let results = zany_zoo.run(20);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_zany_zoo_with_multiple_programs() {
        let programs = vec!["1RB---_1RC1RZ_0LB---".to_string(), "1RB1LC_0LA1RE_0LD0LB_1RA1RZ_1LA0RE".to_string()];
        let zany_zoo = ZanyZoo::new(programs);
        let results = zany_zoo.run(20);
        assert_eq!(results.len(), 2);
    }
}
