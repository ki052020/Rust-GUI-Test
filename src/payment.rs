mod imp;
use gtk::glib::{self, DateTime};
use gtk::prelude::*;

use super::format_date;

// ---------------------------------------------------------------
glib::wrapper! {
	pub struct Payment(ObjectSubclass<imp::Payment>);
}

impl Payment {
	pub fn new(name: String, amount: i64, date: DateTime) -> Self {
		let obj = glib::Object::new::<Payment>();
		obj.set_name(name);
		obj.set_amount(amount);
		obj.set_date(date);
		obj
	}
}

// ---------------------------------------------------------------
pub fn display_ui(payment: &Payment) -> impl IsA<gtk::Widget> {
	let hbox = gtk::Box::builder()
		.orientation(gtk::Orientation::Horizontal)
		.spacing(10)
		.homogeneous(true)
		.build();
	
	let date = gtk::Label::builder().halign(gtk::Align::Start).build();
	payment
		.bind_property("date", &date, "label")
		.transform_to(|_, d: DateTime| Some(format_date(d)))
		.sync_create()
		.build();
	
	let name = gtk::Label::new(None);
	payment
		.bind_property("name", &name, "label")
		.sync_create()
		.build();
		
	let amount = gtk::Label::builder().halign(gtk::Align::End).build();
	payment
		.bind_property("amount", &amount, "label")
		.transform_to(|_, a: i64| Some(format!("{}円", a)))
		.sync_create()
		.build();
	
	hbox.append(&date);
	hbox.append(&name);
	hbox.append(&amount);
	hbox
}
