use pinkyoshi::run;
use std::env;
fn main() {
    env::set_var("PROJECT_NAME", "pinkYoshi");
    pollster::block_on(run());
}