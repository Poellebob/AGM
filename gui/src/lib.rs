use iced::widget::{button, column, row, Column};
use iced::{Application, Command, Element, Settings, executor};

pub fn run() {
    MainWindow::run(Settings::default()).unwrap();
}

// placeholder getters for now
fn GetProfiles() -> Vec<String> {
    //replace with smt that returns profiles
    vec!["Profile A".into(), "Profile B".into()]
}

fn GetPresets() -> Vec<String> {
    //replace with smt that returns presets
    vec!["Preset 1".into(), "Preset 2".into()]
}

fn GetMods() -> Vec<String> {
    //replace with smt that returns mods
    vec!["Mod X".into(), "Mod Y".into()]
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed(String),
}


fn create_elements(elements: &[String]) -> Column<Message> {
    let mut area: Column<Message> = column![];

    for element in elements {
        area = area.push(
            button(element.as_str())
                .on_press(Message::ButtonPressed(element.clone()))
        );
    }

    area
}

// -----------------------------
// UI Sections
// -----------------------------
struct Profiles { items: Vec<String> }
struct Presets { items: Vec<String> }
struct Mods { items: Vec<String> }
struct Toolbar { items: Vec<String> }

impl Profiles {
    fn view(&self) -> Element<Message> {
        create_elements(&self.items).into()
    }
}
impl Presets {
    fn view(&self) -> Element<Message> {
        create_elements(&self.items).into()
    }
}
impl Mods {
    fn view(&self) -> Element<Message> {
        create_elements(&self.items).into()
    }
}
impl Toolbar {
    fn view(&self) -> Element<Message> {
        create_elements(&self.items).into()
    }
}

struct MainWindow {
    profiles: Profiles,
    presets: Presets,
    mods: Mods,
    toolbar: Toolbar,
}

// renders the thingy i think
impl Application for MainWindow {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            MainWindow {
                profiles: Profiles { items: GetProfiles() },
                presets: Presets { items: GetPresets() },
                mods: Mods { items: GetMods() },
                toolbar: Toolbar { items: vec!["Save".into(), "Load".into()] },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "negro".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ButtonPressed(label) => {
                println!("Button pressed: {}", label);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let main_section = row![
            self.profiles.view(),
            self.presets.view(),
            self.mods.view(),
        ]
        .spacing(20);

        let main_layout = column![
            main_section,
            self.toolbar.view(),
        ]
        .spacing(20)
        .padding(20);

        main_layout.into()
    }
}