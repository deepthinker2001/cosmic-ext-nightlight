use cosmic::app::{Core, Task};
use cosmic::Element;

const APP_ID: &str = "io.github.cosmic_ext.Nightlight";
const ICON_ON: &str = "io.github.cosmic_ext.Nightlight-on-symbolic";
const ICON_OFF: &str = "io.github.cosmic_ext.Nightlight-off-symbolic";

fn main() -> cosmic::iced::Result {
    cosmic::applet::run::<Nightlight>(())
}

struct Nightlight {
    core: Core,
    enabled: bool,
}

#[derive(Clone, Debug)]
enum Message {
    Toggle,
}

impl cosmic::Application for Nightlight {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = APP_ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Message>) {
        (Self { core, enabled: false }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // For now the toggle only flips state in the UI.
            Message::Toggle => self.enabled = !self.enabled,
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let icon = if self.enabled { ICON_ON } else { ICON_OFF };
        self.core
            .applet
            .icon_button(icon)
            .on_press(Message::Toggle)
            .into()
    }
}
