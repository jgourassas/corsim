
use fltk::*;
use fltk::{table::*, prelude::*};
use std::ops::{Deref, DerefMut};

use crate::graphics::data_gets::get_optimal_views;

#[derive(Debug,Clone)]

pub struct MyTable{
    table: table::Table,
}//struct

impl MyTable{
pub fn new(x: i32, y: i32, w: i32, h: i32, title: &str)-> MyTable{
/*******************************/
let mut table = table::Table::new(x,y,w,h,title);
    table.set_rows(14);
    table.set_row_header(true);
    table.set_row_resize(true);
    table.set_cols(3);
    table.set_col_header(true);
    table.set_col_width_all(240);
    table.set_row_height_all(25);
    table.set_col_resize(true);
    // table.selection_color(FL_YELLOW);
    table.end();
    let table_c = table.clone();

    table.draw_cell(Box::new(move |ctx, row, col, x, y, w, h| match ctx {
        table::TableContext::StartPage => draw::set_font(Font::Helvetica, 16),
        table::TableContext::ColHeader => {
            let header = vec!["SITE", "Option A", "Option B"];
            let idx = col as usize;
            draw_header(&format!("{}", header[idx]), x, y, w, h);
        } // Column titles
        table::TableContext::RowHeader => {
            draw_header(&format!("{}", row + 1), x, y, w, h)
        } // Row titles
        table::TableContext::Cell => {
            let max_col = 3;

            let sarray = get_optimal_views();
            let idx = (row * max_col + col) as usize;

            draw_data(
                &format!("{}", sarray[idx]),
                x,
                y,
                w,
                h,
                table_c.is_selected(row, col),
            ); // Data in cells
        }

        _ => (),
    }));
/************************/

    MyTable{
    table

}
}//new


}//myTabble

fn draw_header(txt: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::FrameDefault);
    draw::set_draw_color(Color::Black);
    draw::draw_text2(txt, x, y, w, h, Align::Center);
    draw::pop_clip();
}

fn draw_data(txt: &str, x: i32, y: i32, w: i32, h: i32, selected: bool) {
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(Color::from_u32(0xD3D3D3));
    } else {
        draw::set_draw_color(Color::White);
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(Color::Gray0);
    draw::draw_text2(txt, x, y, w, h, Align::Center);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}

impl Deref for MyTable {
    type Target = Table;

    fn deref(&self ) -> &Self::Target{
        &self.table
    }
}

impl DerefMut for MyTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.table
    }
}
