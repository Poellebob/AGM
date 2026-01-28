use iced::{
    Application,
    Command,
    Element,
    Settings,
    executor,
    Border,
    Color,
    Theme,
    Length,
    widget::{
        container,
        button,
        Space,
        column,
        Column,
        row,
        Row
    }
};

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

fn create_column_elements(elements: &[String]) -> Column<Message> {
    let mut area: Column<Message> = column![];

    for element in elements {
        area = area.push(
            button(element.as_str()).on_press(Message::ButtonPressed(element.clone())));
    }
    area
}

fn create_row_elements(elements: &[String]) -> Row<Message> {
    let mut area: Row<Message> = row![];

    for element in elements {
        area = area.push(
            button(element.as_str()).on_press(Message::ButtonPressed(element.clone())));
    }
    area
}

struct Profiles { items: Vec<String> }
struct Presets  { items: Vec<String> }
struct Mods     { items: Vec<String> }
struct Toolbar  { items: Vec<String> }

impl Profiles {
    fn view(&self) -> Element<Message> {
        create_column_elements(&self.items).into()
    }
}
impl Presets {
    fn view(&self) -> Element<Message> {
        create_column_elements(&self.items).into()
    }
}
impl Mods {
    fn view(&self) -> Element<Message> {
        create_column_elements(&self.items).into()
    }
}
impl Toolbar {
    fn view(&self) -> Element<Message> {
        create_row_elements(&self.items).into()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ToolbarStyle {
    Default,
}

impl container::StyleSheet for ToolbarStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border: Border {
                width: 2.0,
                color: Color::from_rgb(0.6, 0.6, 0.6),
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
}

impl From<ToolbarStyle> for iced::theme::Container {
    fn from(style: ToolbarStyle) -> Self {
        iced::theme::Container::Custom(Box::new(style))
    }
}

struct MainWindow {
    profiles: Profiles,
    presets: Presets,
    mods: Mods,
    toolbar: Toolbar
}

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
                toolbar: Toolbar { items: vec!["Save".into(), "Load".into()] }
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "AGM".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        if let Message::ButtonPressed(label) = message {
            println!("Button pressed: {}", label);
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let main_section = row![
            self.profiles.view(),
            Space::with_width(Length::Fill),
            self.presets.view(),
            Space::with_width(Length::Fill),
            self.mods.view(),
        ]
        .spacing(20);

        let toolbar = container(self.toolbar.view())
            .width(Length::Fill)
            .style(ToolbarStyle::Default);

        let layout = column![
            main_section,
            Space::with_height(Length::Fill),
            toolbar,
        ]
        .padding(20)
        .spacing(20);

        layout.into()
    }
}
