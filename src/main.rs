use iced::{
    Length, Renderer, Subscription, Theme, color, keyboard,
    widget::{Column, Container, MouseArea, button, column, container, mouse_area},
};

#[derive(Default)]
struct Powr {
    btn_index: usize,
}

const BUTTONS: &[(&str, Message)] = &[
    ("Lock", Message::Lock),
    ("Sleep", Message::Sleep),
    ("Suspend", Message::Suspend),
    ("Hibernate", Message::Hibernate),
    ("Reboot", Message::Reboot),
    ("Shutdown", Message::Shutdown),
];

#[derive(Debug, Clone)]
enum Message {
    DownPressed,
    UpPressed,
    EnterPresed,
    ButtonHovered(usize),
    Lock,
    Sleep,
    Suspend,
    Hibernate,
    Reboot,
    Shutdown,
}

impl Powr {
    fn update(&mut self, message: Message) {
        match &message {
            Message::DownPressed => {
                self.btn_index += 1;
            }
            Message::UpPressed => {
                self.btn_index -= 1;
            }
            Message::ButtonHovered(index) => {
                self.btn_index = index.to_owned();
            }
            Message::EnterPresed => todo!(),
            Message::Lock => todo!(),
            Message::Sleep => todo!(),
            Message::Suspend => todo!(),
            Message::Hibernate => todo!(),
            Message::Reboot => todo!(),
            Message::Shutdown => todo!(),
        }
    }

    fn view(&self) -> Container<Message> {
        let interface = column(BUTTONS.iter().enumerate().map(|(index, (label, message))| {
            build_button(label, self.btn_index == index, message.clone(), index).into()
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
            .into()
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

fn build_button<'a>(
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

fn sleep() {
    println!("test")
}

fn shutdown() {
    println!("computer down");
}
