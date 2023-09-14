mod todo_reducer;
mod fib;
use todo_reducer::{ TodoState, TodoAction };

fn main() {
    let mut state = TodoState { todos: vec![] };
    println!("State: {:?}", state);
    state = todo_reducer::reducer(state, TodoAction::AddTodo(String::from("Learn Rust")));
    println!("State: {:?}", state);
    state = todo_reducer::reducer(state, TodoAction::RemoveTodo(0));
    println!("State: {:?}", state);
}
