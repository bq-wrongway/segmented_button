use iced::widget::container;
use iced::{Element, Length, Sandbox, Settings};
use iced_widget::{column, row, svg, text};
use segmented_button::SegmentedButton;

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

#[derive(Default)]
struct Example {
    selected_radio: Option<Choice>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadioSelected(Choice),
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Self {
            selected_radio: Some(Choice::A),
        }
    }

    fn title(&self) -> String {
        String::from("Radio - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RadioSelected(value) => {
                self.selected_radio = Some(value);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        // let selected_radio = Some(Choice::A);

        let handle =
            svg::Handle::from_path(format!("{}/resources/ico.svg", env!("CARGO_MANIFEST_DIR")));
        let svg1 = svg(handle).width(30).height(20);
        let a = SegmentedButton::new(
            row!(text("HEAVY"), svg1),
            Choice::A,
            self.selected_radio,
            Message::RadioSelected,
        );
        let handle1 = svg::Handle::from_path(format!(
            "{}/resources/heart.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg2 = svg(handle1).width(30).height(20);
        let b = SegmentedButton::new(
            row!(text("MEDIUM"), svg2),
            Choice::B,
            self.selected_radio,
            Message::RadioSelected,
        );
        let handle2 = svg::Handle::from_path(format!(
            "{}/resources/light.svg",
            env!("CARGO_MANIFEST_DIR")
        ));
        let svg3 = svg(handle2).width(30).height(20);
        let c = SegmentedButton::new(
            row!(text("LIGHT"), svg3),
            Choice::C,
            self.selected_radio,
            Message::RadioSelected,
        );
        let content = column![
            row![a, b, c],
            text(self.selected_radio.unwrap().to_string())
        ]
        .align_items(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Choice {
    #[default]
    A,
    B,
    C,
}

impl std::fmt::Display for Choice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Choice::A => "A",
                Choice::B => "B",
                Choice::C => "C",
            }
        )
    }
}
