#[path = "two/a.rs"]
mod task_a;

#[path = "two/b.rs"]
mod task_b;

fn main() {
    task_a::main();
    task_b::main();
}
