use druid::widget::{Align, Button, Flex, Label, Padding, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

mod socket_client;
mod socket_server;

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Hello World!");

#[derive(Clone, Data, Lens)]
struct HelloState {
	name: String,
}

fn main() {
	// socket_client::connect();
	socket_server::listen();

	let main_window = WindowDesc::new(build_root_widget)
		.title(WINDOW_TITLE)
		.window_size((400.0, 400.0));

	let initial_state = HelloState {
		name: "World".into(),
	};

	AppLauncher::with_window(main_window)
		.launch(initial_state)
		.expect("Failed to launch application");
}

fn build_root_widget() -> impl Widget<HelloState> {
	let label = Label::new(|data: &HelloState, _env: &Env| format!("Hello {}!", data.name));
	let textbox = TextBox::new()
		.with_placeholder("Who are we greeting?")
		.fix_width(TEXT_BOX_WIDTH)
		.lens(HelloState::name);

	let host_button = Button::from_label(Label::new("Host로 시작"));
	let guest_button = Button::from_label(Label::new("Guest로 시작"));

	let layout = Flex::column()
		// .with_child(label)
		// .with_spacer(VERTICAL_WIDGET_SPACING)
		// .with_child(textbox)
		// .with_spacer(VERTICAL_WIDGET_SPACING)
		.with_child(
			Flex::row()
				.with_child(host_button)
				.with_spacer(VERTICAL_WIDGET_SPACING)
				.with_child(guest_button),
		);

	Padding::new(10.0, Align::centered(layout))
}
