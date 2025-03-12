use gtk::prelude::*;
use gtk::glib;

use super::format_date;

// ---------------------------------------------------------------
pub fn datepicker() -> (gtk::Box, gtk::Calendar) {
	let hbox = gtk::Box::builder()
		.homogeneous(false)
		.orientation(gtk::Orientation::Horizontal)
		.build();
	let button = gtk::Button::new();
	let cal = gtk::Calendar::new();
	
	// デフォルトで今日の日付を選択しておく
	cal.select_day(&glib::DateTime::now_local().unwrap());
	let pop = gtk::Popover::builder().child(&cal).autohide(true).build();

	button.connect_clicked(glib::clone!(
		#[weak] pop,
		move |_| pop.popup()
	)); 
   button.set_label(&format_date(cal.date()));
	
	cal.connect_day_selected(glib::clone!(
		#[weak] pop,
		#[weak] button,
		move |cal| {
			pop.popdown();
			button.set_label(&format_date(cal.date()));
		}
	));

	hbox.append(&button);
	hbox.append(&pop);

	(hbox, cal)
}
