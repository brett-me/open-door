// ============================================================================================
// CH 09-01 Unrecoverable Errors with Panic!
// ============================================================================================

// general
// ============================================================================================
// recoverable (try again) and unrecoverable (terminate programme) errors
// rust doest not use exceptions
// it uses Result<T, E> for recoverable errors and panic! for unrecoverable

// panic!
// ============================================================================================
// by default panic!:
//      - prints failure message
//      - unwinds
//      - cleans up the stack
//      - quits

// unwinding
// ============================================================================================
// when panic occurs, Rust walks back up the stack and cleans up the data from each function
// this can be a lot of work and their is an option to abort immediately, without cleaning up
// [profile.'mode']
// panic = 'abort'

fn main() {
    // panic!("crash and burn!");
    let panic_example = vec![1, 2, 3];
    panic_example[99];
}
