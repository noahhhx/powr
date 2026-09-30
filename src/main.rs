use iced::{
    Length, Renderer, Subscription, Theme, color, keyboard,
    widget::{Container, MouseArea, button, column, container, mouse_area},
};

mod power;

#[derive(Default)]
struct Powr {
    btn_index: usize,
}

struct PowrButton {
    name: &'static str,
    message: Message,
    run: ButtonFn,
}

type ButtonFn = fn();

const BUTTONS: &[PowrButton] = &[
    PowrButton {
        name: "Lock",
        message: Message::Lock,
        run: power::lock,
    },
    PowrButton {
        name: "Sleep",
        message: Message::Sleep,
        run: power::sleep,
    },
    PowrButton {
        name: "Hibernate",
        message: Message::Hibernate,
        run: power::hibernate,
    },
    PowrButton {
        name: "Reboot",
        message: Message::Reboot,
        run: power::reboot,
    },
    PowrButton {
        name: "Shutdown",
        message: Message::Shutdown,
        run: power::shutdown,
    },
];

#[derive(Debug, Clone)]
enum Message {
    DownPressed,
    UpPressed,
    EnterPresed,
    ButtonHovered(usize),
    Lock,
    Sleep,
    Hibernate,
    Reboot,
    Shutdown,
}

impl Powr {
    fn update(&mut self, message: Message) {
        match &message {
            Message::DownPressed => {
                let size = BUTTONS.len();
                if self.btn_index == size - 1 {
                    self.btn_index = 0;
                } else {
                    self.btn_index += 1;
                }
            }
            Message::UpPressed => {
                let size = BUTTONS.len();
                if self.btn_index == 0 {
                    self.btn_index = size - 1;
                } else {
                    self.btn_index -= 1;
                }
            }
            Message::ButtonHovered(index) => {
                self.btn_index = index.to_owned();
            }
            Message::EnterPresed => {
                let btn = BUTTONS.get(self.btn_index).unwrap();
                (btn.run)();
            }
            Message::Lock => {
                power::lock();
            }
            Message::Sleep => power::sleep(),
            Message::Hibernate => power::hibernate(),
            Message::Reboot => power::reboot(),
            Message::Shutdown => power::shutdown(),
        }
    }

    fn view(&self) -> Container<'_, Message> {
        let interface = column(BUTTONS.iter().enumerate().map(|(index, btn)| {
            build_button(
                btn.name,
                self.btn_index == index,
                btn.message.clone(),
                index,
            )
            .into()
        }))
        .spacing(10)
        .padding(20);

        container(interface)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| iced::widget::container::Style {
                background: Some(color!(0x1e1e2e).into()),
                text_color: Some(color!(0xcdd6f4)),
                ..Default::default()
            })
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::listen().filter_map(|event| {
            let keyboard::Event::KeyPressed {
                modified_key: keyboard::Key::Named(modified_key),
                repeat: false,
                ..
            } = event
            else {
                return None;
            };

            match modified_key {
                keyboard::key::Named::ArrowUp | keyboard::key::Named::ArrowLeft => {
                    Some(Message::UpPressed)
                }
                keyboard::key::Named::ArrowDown | keyboard::key::Named::ArrowRight => {
                    Some(Message::DownPressed)
                }
                keyboard::key::Named::Enter | keyboard::key::Named::Accept => {
                    Some(Message::EnterPresed)
                }
                _ => None,
            }
        })
    }
}

fn build_button(
    text: &str,
    active: bool,
    message: Message,
    index: usize,
) -> MouseArea<'_, Message, Theme, Renderer> {
    mouse_area(
        button(text)
            .style(move |_theme, status| {
                if active || matches!(status, button::Status::Hovered) {
                    button::Style {
                        background: Some(color!(0x313244).into()),
                        text_color: color!(0xcdd6f4),
                        ..Default::default()
                    }
                } else {
                    button::Style {
                        background: Some(color!(0x1e1e2e).into()),
                        text_color: color!(0xcdd6f4),
                        ..Default::default()
                    }
                }
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .on_press(message),
    )
    .on_enter(Message::ButtonHovered(index))
}

fn main() -> iced::Result {
    iced::application(Powr::default, Powr::update, Powr::view)
        .subscription(Powr::subscription)
        .window(iced::window::Settings {
            size: iced::Size::new(300.0, 400.0),
            resizable: false,
            platform_specific: iced::window::settings::PlatformSpecific {
                application_id: "powerdeck".to_string(),
                ..Default::default()
            },
            ..Default::default()
        })
        .run()
}
