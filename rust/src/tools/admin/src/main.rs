use admin::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // match Package::new()
    //     .with_name("day")
    //     .with_digits(2)
    //     .with_aoc_day(60)
    //     .deploy()
    // {
    //     Ok(_) => {}
    //     Err(err) => eprintln!("{}", err),
    // };

    println!(
        "Hello, world! {}",
        std::env::current_dir()
            .expect("Failed to get current directory")
            .display()
    );

    Ok(())
}
