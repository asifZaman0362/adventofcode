mod day1;
mod day2;
mod day3;
mod day4;


fn main() -> std::io::Result<()> {
    let (score, copies) = day4::soln()?;
    println!("score: {score}\ncopies: {copies}");
    Ok(())
}
