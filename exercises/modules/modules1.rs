// modules1.rs
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a hint.


mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}

// In Rust, you can define functions as either private or public depending on their intended visibility and access. 
// Here's an explanation of private and public functions in Rust:
// Private Functions: Private functions are only accessible within the same module where they are defined.
// They are denoted by the pub keyword, which is omitted. 
// Private functions are typically used for internal implementation details and helper functions that should not be exposed to other modules or external code. 
// Private functions help encapsulate and hide implementation details, promoting modular and organized code.
