use day_17::*;

fn main() {
    /*  Part One Example
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
    */

    let mut m = Machine::new(729, 0, 0);
    let program = vec![0, 1, 5, 4, 3, 0];
    let output = m.run(&program);
    println!("Output: {:?}", output);

    /*  Part two Example
       Register A: 2024
       Register B: 0
       Register C: 0

       Program: 0,3,5,4,3,0

       The machine init values don't actually matter
       here since we are looking for the correct
       value of Register A
    */
    let program = vec![0, 3, 5, 4, 3, 0];
    if let Some(output) = Machine::search(&program) {
        println!("Output: {:?}", output);

        let mut m = Machine::new(output, 0, 0);
        let program_output = m.run(&program);
        println!("input: {:?}, output: {:?}", program, program_output);
    }
}
