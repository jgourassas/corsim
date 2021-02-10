
use fltk::*;
use fltk::{app,  button::*, prelude::*};
use std::ops::{Deref, DerefMut};

#[derive(Debug,Clone)]
pub struct MyButton{
    btn: button::Button,

}//struct

//The set_callback method has a second variant, unimaginatively called set_callback2, 
//where the closure takes a mutable reference to the widget itself.

impl MyButton{
    pub fn new(x: i32, y: i32, w: i32, h: i32, title: &str) -> MyButton {
     let mut btn = button::Button::new(x, y, w, h, title);
         btn.set_callback2(move |b| b.parent().unwrap().hide() );
         //btn.set_color(Color::from_u32(32));
        // btn.set_color(Color::from_u32(0x304FFE));
     //    btn.set_color(Color::from_rgb(255,255,179));
         btn.set_color(Color::from_rgb(238,	232,205));
         btn.set_label_size(18);

     MyButton{
            btn
        }

    }//new

}//impl

/*

So given a var_pointer which was populated by assigning 
the result of &var to it, then the original value in var 
can be accessed via *var_pointer.  This is called dereferencing.

Deref coercion is a convenience that Rust performs 
on arguments to functions and methods. 
It allows recursive dereferencing to be applied until 
the value that matches arguments to a function/method is found.
*/
impl Deref for MyButton {
    type Target = Button;

    fn deref(&self ) -> &Self::Target{
        &self.btn
    }
}

impl DerefMut for MyButton {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.btn
    }
}

