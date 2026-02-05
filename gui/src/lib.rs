use iced::{
    Application, Command, Element, Settings, executor, Border, Color, Background, Theme, Length,
    widget::{container, button, Space, column, Column, row, Row, scrollable, Scrollable, text},
    Theme::{Dark}
};

pub fn run() {
    MainWindow::run(Settings::default()).unwrap();
}

// placeholder getters
fn GetProfiles() -> Vec<String> {
    vec!["Profile A".into(), "Profile B".into()]
}
fn GetPresets() -> Vec<String> {
    vec!["Preset 1".into(), "Preset 2".into()]
}
fn GetMods() -> Vec<String> {
    vec!["Mod X".into(), "Mod Y".into()]
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed { list: ListType, id: String },
}

#[derive(Debug, Clone, Copy)]
pub enum ListType {
    Profiles,
    Presets,
    Mods,
    Toolbar,
}

fn create_column_elements(list_type: ListType, elements: &Vec<String>) -> Column<Message> {
    let mut area = column![]
        .spacing(5)
        .width(Length::Fill);

    for i in elements {
        area = area.push(
            button(text(i))
                .width(Length::Fill)
                .on_press(Message::ButtonPressed { list: list_type, id: i.to_string() })
        );
    }
    area
}

fn create_row_elements(list_type: ListType, elements: &Vec<String>) -> Row<Message> {
    let mut area = Row::new()
        .spacing(20)
        .padding(2)
        .width(Length::Fill)
        .push(Space::with_width(Length::Fill));

    for i in elements {
        area = area.push(
            button(text(i))
                .width(Length::Shrink)
                .on_press(Message::ButtonPressed { list: list_type, id: i.to_string() })
        );
    }

    area.push(Space::with_width(Length::Fill))
}

struct Profiles { items: Vec<String> }
struct Presets { items: Vec<String> }
struct Mods { items: Vec<String> }
struct Toolbar { items: Vec<String> }

impl Profiles {
    fn view(&self) -> Element<Message> {
        Scrollable::new(create_column_elements(ListType::Profiles, &self.items))
            .width(Length::Fill)
            .into()
    }
}
impl Presets {
    fn view(&self) -> Element<Message> {
        Scrollable::new(create_column_elements(ListType::Presets, &self.items))
            .width(Length::Fill)
            .into()
    }
}
impl Mods {
    fn view(&self) -> Element<Message> {
        Scrollable::new(create_column_elements(ListType::Mods, &self.items))
            .width(Length::Fill)
            .into()
    }
}
impl Toolbar {
    fn view(&self) -> Element<Message> {
        container(
            create_row_elements(ListType::Toolbar, &self.items)
        )
        .style(iced::theme::Container::Custom(Box::new(ToolbarStyle)))
        .into()
    }
}

#[derive(Debug, Clone, Copy)]
struct ToolbarStyle;

impl container::StyleSheet for ToolbarStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border: Border {
                width: 2.0,
                color: Color::from_rgb(0.6, 0.6, 0.6),
                radius: 0.0.into(),
            },
            shadow: Default::default(),
            background: None,
            text_color: None
        }
    }
}

// MainWindow
struct MainWindow {
    profiles: Profiles,
    presets: Presets,
    mods: Mods,
    toolbar: Toolbar,
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
                toolbar: Toolbar { items: vec!["Save".into(), "Load".into()] },
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "AGM".into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        if let Message::ButtonPressed { list, id } = message {
            println!("Pressed {:?} id {}", list, id);
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

        let toolbar = self.toolbar.view();

        column![
            main_section,
            Space::with_height(Length::Fill),
            toolbar,
        ]
        .spacing(20)
        .into()
    }
}
