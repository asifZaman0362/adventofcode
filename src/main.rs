mod day1;
mod day2;


fn main() -> std::io::Result<()> {
    let (sum_possible, sum_power) = day2::soln()?;
    println!("possible: {sum_possible}; power: {sum_power}");
    Ok(())
}
