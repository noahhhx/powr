use iced::{
    Length, Renderer, Subscription, Theme, keyboard,
    widget::{Container, MouseArea, button, column, container, mouse_area, text},
};

mod colour;
mod config;
mod power;

use crate::{
    colour::ColourConfig,
    config::{AppConfig, ConfigError, load_or_init},
    power::{Action, PowerConfig},
};

#[derive(Default)]
struct Powr {
    btn_index: usize,
    colour: ColourConfig,
    power: PowerConfig,
}

#[derive(Debug, Clone)]
enum Message {
    Next,
    Prev,
    Activate,
    ButtonHovered(usize),
    Run(Action),
}

impl Powr {
    fn from_config(config: AppConfig) -> Self {
        Powr {
            btn_index: 0,
            colour: config.colour,
            power: config.power,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Next => {
                let size = Action::ALL.len();
                if self.btn_index == size - 1 {
                    self.btn_index = 0;
                } else {
                    self.btn_index += 1;
                }
            }
            Message::Prev => {
                let size = Action::ALL.len();
                if self.btn_index == 0 {
                    self.btn_index = size - 1;
                } else {
                    self.btn_index -= 1;
                }
            }
            Message::ButtonHovered(index) => {
                self.btn_index = index;
            }
            Message::Activate => {
                self.run(Action::ALL[self.btn_index]);
            }
            Message::Run(action) => self.run(action),
        }
    }

    fn view(&self) -> Container<'_, Message> {
        let interface = column(Action::ALL.iter().enumerate().map(|(index, &action)| {
            build_button(
                action.label(),
                self.btn_index == index,
                Message::Run(action),
                index,
                self.colour,
            )
            .into()
        }))
        .spacing(10)
        .padding(20);

        container(interface)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| iced::widget::container::Style {
                background: Some(self.colour.background.0.into()),
                text_color: Some(self.colour.button_active_text.0),
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
                    Some(Message::Prev)
                }
                keyboard::key::Named::ArrowDown | keyboard::key::Named::ArrowRight => {
                    Some(Message::Next)
                }
                keyboard::key::Named::Enter | keyboard::key::Named::Accept => {
                    Some(Message::Activate)
                }
                _ => None,
            }
        })
    }

    fn run(&self, action: Action) {
        power::run_cmd(self.power.cmd(action));
    }
}

fn build_button(
    label: &str,
    active: bool,
    message: Message,
    index: usize,
    colour: ColourConfig,
) -> MouseArea<'_, Message, Theme, Renderer> {
    mouse_area(
        button(text(label).height(iced::Fill).center())
            .style(move |_theme, status| {
                if active || matches!(status, button::Status::Hovered) {
                    button::Style {
                        background: Some(iced::Background::Color(
                            colour.button_active_background.0,
                        )),
                        text_color: colour.button_active_text.0,
                        ..Default::default()
                    }
                } else {
                    button::Style {
                        background: Some(iced::Background::Color(
                            colour.button_inactive_background.0,
                        )),
                        text_color: colour.button_inactive_text.0,
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
    let config = match load_or_init() {
        Ok(v) => v,
        Err(err) => {
            match err {
                ConfigError::IoError(err) => {
                    eprintln!("An error occurred while loading the config: {err}");
                }
                ConfigError::InvalidConfig(err) => {
                    eprintln!("An error occurred while parsing the config:");
                    eprintln!("{err}");
                }
            }
            AppConfig::default()
        }
    };

    iced::application(
        move || Powr::from_config(config.clone()),
        Powr::update,
        Powr::view,
    )
    .subscription(Powr::subscription)
    .window(iced::window::Settings {
        size: iced::Size::new(300.0, 400.0),
        resizable: false,
        platform_specific: iced::window::settings::PlatformSpecific {
            application_id: "powr".to_string(),
            ..Default::default()
        },
        ..Default::default()
    })
    .run()
}
