use fltk::valuator::*;
use fltk::*;


pub fn make_slider(
   x: i32,
   y: i32, 
   w: i32,
   h: i32,
   title: &str,
   bounds: [i32; 2]
) -> NiceSlider{
let mut sl =NiceSlider::new( 
    x,
    y,
    w,
    h, 
    title);
    sl.set_color(Color::from_rgb(158, 188, 218)); //
    sl.set_frame(FrameType::RoundUpBox);
    //sl.set_bounds(-90.0, 90.0);
    sl.set_bounds(bounds[0] as f64 , bounds[1] as f64);
    sl.set_step(1.0, 1);
    sl.set_label_size(18);
    sl.set_trigger(CallbackTrigger::Changed);
    sl

}   //make slider


