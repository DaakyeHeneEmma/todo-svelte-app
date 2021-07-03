use juniper::{EmptyMutation, RootNode};

//the todo object
pub struct AllTodo {
title: String,
description: String,
}

//making the work
#[juniper::object(description = "A Todo App for Afrodew")]
impl AllTodo {
pub fn title(&self) -> String {
self.title.to_string()
}
pub fn description(&self) -> String {
    self.description.to_string()
        } 
    }

//creating an array of a todo
pub struct TodoApp;
#[juniper::object]
impl TodoApp {
fn Todos() -> Vec<AllTodo> {
vec![
    AllTodo {
        title: "Work".to_owned(),
        description: "Go to work at 8:00 am".to_owned(),
        }
     ]
  }  
 }
        
//exporting the schema
 pub type Schema = RootNode<'static, TodoApp, EmptyMutation<()>>;  
 pub fn create_schema() -> Schema {
   Schema::new(TodoApp {}, EmptyMutation::new())
 }