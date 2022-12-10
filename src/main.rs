#[path = "10/a.rs"]
mod task_a;

#[path = "10/b.rs"]
mod task_b;

fn main() {
    task_a::main();
    task_b::main();
}
