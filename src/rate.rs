use rate_my_sudoku::Sudoku;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Please provide a serialized Sudoku board");
        return;
    }
    if args[1].len() != 81 {
        println!("Please provide a string of length 81");
        return;
    }
    let mut s0 = Sudoku::new();
    s0.set_board_string(&args[1]);
    let start = std::time::Instant::now();
    s0.solve_puzzle();
    let duration = start.elapsed();
    println!(
        "Time to solve: {:.3} ms",
        1e-3 * duration.as_micros() as f64
    );

    let start = std::time::Instant::now();
    let mut s1 = Sudoku::new();
    s1.set_board_string(&args[1]);
    s1.solve_by_backtracking();
    let duration = start.elapsed();
    println!(
        "For comparison: time to solve with backtracker: {:.3} ms",
        1e-3 * duration.as_micros() as f64
    );

    if s0.serialized() != s1.serialized() {
        println!("\nSOLUTIONS DIFFER\n");
        println!("Human-like solver:");
        s0.print();
        println!("Backtracking solver:");
        s1.print();
    }
}
