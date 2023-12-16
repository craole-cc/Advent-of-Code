/// Macro for creating an `Package` with optional parameters and automatically converting to a string.
/// Example usage:
/// ```
/// use crate::generate_pkg_name;
/// let name = generate_pkg_name!("day");
/// let name_with_params = generate_pkg_name!("day", 1, 3);
/// ```
#[macro_export]
macro_rules! generate_pkg_name {
    ($base_name:expr) => {
        Package::from($base_name)
    };
    ($base_name:expr, $number:expr, $digits:expr) => {
        Package::new()
            .with_name($base_name)
            .with_sequence_number($number)
            .with_digits($digits)
            .to_string()
    };
}
