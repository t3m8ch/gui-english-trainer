use app::Application;

pub mod app;
pub mod domain;
pub mod messages;
pub mod widgets;

fn main() -> iced::Result {
    iced::run("English trainer", Application::update, Application::view)
}
