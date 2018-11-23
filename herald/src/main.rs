extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;
use std::process::Command;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;

fn ring () {

}

fn main() {
	let mut core = Core::new().unwrap();
	let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
	let api = Api::configure(token).build(core.handle()).unwrap();

	let future = api.stream().for_each(|update| {
		if let UpdateKind::Message(message) = update.kind {
			if let MessageKind::Text {ref data, ..} = message.kind {
				match data.as_str() {
					"ring" => {
						println!("dingdong! {} rings!", &message.from.first_name);
						api.spawn(message.text_reply(
							format!("{} just {}s", &message.from.first_name, data)
						));
						ring();
					}
					_ => println!("<{}>: {}", &message.from.first_name, data)
				}
			}
		}
	
		Ok(())
	});

	core.run(future).unwrap();
}

