#[path = "11/a.rs"]
mod task_a;

#[path = "11/b.rs"]
mod task_b;

fn main() {
    task_a::main();
    task_b::main();
}
