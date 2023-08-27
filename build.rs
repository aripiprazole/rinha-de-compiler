use lalrpop::Configuration;

fn main() {
    Configuration::new().always_use_colors().process_current_dir().unwrap();
}
