// mod getters;

use crate::{generate_pkg_name, Package};

pub fn hello_from_jobs() {
    println!("Hello from Jobs!");
    let name = Package::new();
    let name_shortcut = Package::new()
        .with_name("day")
        .with_sequence_number(1)
        .with_digits(2);
    let name_from = generate_pkg_name!("job", 1, 2);

    println!("{}", name_from);
    println!("{}", name.to_string());
    println!("{:?}", name_shortcut);
}
