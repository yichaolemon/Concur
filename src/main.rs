use std::thread;

fn main() {
  println!("Hello, world!");
  let t = create_async();
  println!(t.value)
}

#[derive(Default, Debug)]
struct Task<T> {
  value: Option<T>,
}

fn create_async() -> Task<i32> {
  let mut task = Task{
    value: None
  };
  thread::spawn(move || {
    task.value = Some(5)
  });
  task
}
