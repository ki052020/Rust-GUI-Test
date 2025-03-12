use gtk::prelude::*;
use gtk::glib;

use crate::Payment;

#[derive(Debug)]
pub struct Ledger {
    model: gtk::gio::ListStore,
    balance: gtk::Label,
}

impl Default for Ledger {
	fn default() -> Self {
		let label = gtk::Label::builder()
			.use_markup(true)
			.label(format!("<big>{}円</big>", 0))
			.build();
		
		let model = gtk::gio::ListStore::builder()
			.item_type(Payment::static_type())
			.build();

		model.connect_items_changed(
			gtk::glib::clone!(
				#[weak] label,
				move |m, _, _, _| {
					let balance = m
						.into_iter()
						.map(|item| item.unwrap().downcast::<Payment>().unwrap().amount())
						.sum::<i64>();
					label.set_markup(&format!("<big>{}</big>円", balance));
				}
			));

		Self {
			model,
			balance: label,
		}
	}
}

impl Ledger {
	pub fn record_payment(&mut self, payment: &Payment) {
		self.model.append(payment);
	}

	pub fn model(&self) -> &gtk::gio::ListStore {
		&self.model
	}
	
	pub fn balance(&self) -> &gtk::Label {
		&self.balance
	}
}