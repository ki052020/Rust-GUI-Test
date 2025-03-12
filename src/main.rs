use gtk::prelude::*;
use gtk::{ glib, Application, ApplicationWindow };

use std::cell::RefCell;
use std::rc::Rc;

mod payment;
mod ledger;
mod datepicker;

use payment::*;
use ledger::Ledger;
use datepicker::*;

const APP_ID: &str = "gtk_rs.Test-2";

// ---------------------------------------------------------------
fn main() {
	let app = Application::builder().application_id(APP_ID).build();
	app.connect_activate(build_ui);
	app.run();
}

// ---------------------------------------------------------------
fn format_date(d: gtk::glib::DateTime) -> String {
	format!("{:04}-{:02}-{:02}", d.year(), d.month(), d.day_of_month())
}

fn input_box(ledger: Rc<RefCell<Ledger>>) -> gtk::Box {
	let hbox = gtk::Box::builder()
		.orientation(gtk::Orientation::Horizontal)
		.build();
		
	let name = gtk::Entry::builder().placeholder_text("name").build();
	let amount = gtk::Entry::builder()
		.placeholder_text("amount")
		.input_purpose(gtk::InputPurpose::Digits)
		.build();
	
	let (picker, cal) = datepicker();
	let new_button = gtk::Button::builder().label("new").build();
	
	new_button.connect_clicked(glib::clone!(
		#[weak] name,
		#[weak] amount,
		#[weak] cal,
		#[strong] ledger,
		move |_| {
			let n = name.buffer().text().to_string();
			let amount = match amount.buffer().text().parse() {
				Ok(a) => a,
				Err(_) => {
					// 後で `dialog` を作る
					panic!("エラー: 金額は整数値で入力して下さい");
				},
			};
			let date = cal.date();
			let payment = Payment::new(n, amount, date);
			ledger.borrow_mut().record_payment(&payment);
		}
	));

	hbox.append(&picker);
	hbox.append(&name);
	hbox.append(&amount);
	hbox.append(&new_button);
	hbox
}

// ---------------------------------------------------------------
fn build_ui(app: &Application) {
	let mut ledger = Ledger::default();


	// ------------------------------------------
	ledger.record_payment(&Payment::new(
		"お小遣い".into(),
		1000,
		gtk::glib::DateTime::now_local().unwrap(),
	));
	
	ledger.record_payment(&Payment::new(
		"きゅうり".into(),
		-150,
		gtk::glib::DateTime::now_local().unwrap(),
	));


	// ------------------------------------------
	let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
	{
		let frame = gtk::Frame::builder()
			.label("残高")
			.child(ledger.balance())
			.build();
		vbox.append(&frame);
	}
	{
		let list_box = gtk::ListBox::new();
		list_box.bind_model(Some(ledger.model()), |item| {
			let payment = item.downcast_ref::<Payment>().unwrap();
			display_ui(payment).upcast::<gtk::Widget>()
		});

		let scrolled_window = gtk::ScrolledWindow::builder()
			.hscrollbar_policy(gtk::PolicyType::Never) // Disable horizontal scrolling
			.min_content_height(400)
			.child(&list_box)
			.build();
		vbox.append(&scrolled_window);
	}
	vbox.append(&input_box(Rc::new(RefCell::new(ledger))));

	// ------------------------------------------
	let window = ApplicationWindow::builder()
		.application(app)
		.title("GTK Test-2 App")
		.build();
	window.set_default_size(600, 600);

	window.set_child(Some(&vbox));
	window.present();
}

