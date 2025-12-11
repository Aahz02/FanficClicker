use std::cmp;

use iced::font::{Font, Style};
use iced::time::{self, Instant, seconds};
use iced::widget::{Column, Row, button, column, container, row, text, tooltip};
use iced::{Center, Element, Fill, Subscription};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Category {
    Fluff,
    Angst,
    AU,
    Horror,
    Romance,
}

#[derive(Clone)]
struct Tag {
    name: String,
    categories: Vec<Category>,
    cost: usize,
    active: bool,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Clone)]
struct Upgrade {
    name: String,
    flavor_text: String,
    desc: String,
    multiplier: f64,
    count: usize,
    cost: usize,
}

impl PartialEq<String> for Upgrade {
    fn eq(&self, other: &String) -> bool {
        &self.name == other
    }
}

// allow is used here because time::every requires the message given to it to have an Instant
// attached to it but it doesn't get used anywhere
#[allow(dead_code)]
#[derive(Clone)]
enum Message {
    Upload,
    Tick(Instant),
    BuyTag(Tag),
    ToggleTag(Tag),
    BuyUpgrade(Upgrade),
}

struct GameState {
    all_tags: Vec<Tag>,
    unlocked_tags: Vec<Tag>,
    all_upgrades: Vec<Upgrade>,
    upgrades: Vec<Upgrade>,
    kudos: f64,
    highest_kudos: f64,
}

impl GameState {
    fn default() -> Self {
        GameState {
            all_tags: vec![
                Tag {
                    name: String::from("Confession"),
                    categories: vec![Category::Fluff, Category::Romance],
                    cost: 5,
                    active: false,
                },
                Tag {
                    name: String::from("Monsters"),
                    categories: vec![Category::Horror, Category::AU],
                    cost: 5,
                    active: false,
                },
                Tag {
                    name: String::from("Lover's Spat"),
                    categories: vec![Category::Angst, Category::Romance],
                    cost: 10,
                    active: false,
                },
                Tag {
                    name: String::from("Hallucinations"),
                    categories: vec![Category::Angst, Category::Horror],
                    cost: 10,
                    active: false,
                },
            ],
            unlocked_tags: Vec::new(),
            all_upgrades: vec![
                Upgrade {
                    name: String::from("Clone"),
                    flavor_text: String::from(
                        "Make a clone of yourself to write for you, still more ethical than AI!",
                    ),
                    desc: String::from(
                        "Automatically uploads works for you, 1 per stack every five seconds.",
                    ),
                    multiplier: 0.0,
                    count: 5,
                    cost: 15,
                },
                Upgrade {
                    name: String::from("Beta Reader"),
                    flavor_text: String::from(
                        "Get your friends to read your works before you upload them, free labor.",
                    ),
                    desc: String::from("Increases how many kudos you earn by 1% per stack."),
                    multiplier: 0.1,
                    count: 10,
                    cost: 25,
                },
            ],
            upgrades: Vec::new(),
            kudos: 0.0,
            highest_kudos: 0.0,
        }
    }

    fn calc_tag_bonus(&self) -> f64 {
        let mut found_categories: Vec<Category> = Vec::new();
        let mut category_counts: Vec<usize> = Vec::new();
        for tag in &self.unlocked_tags {
            if !tag.active {
                continue;
            }
            for category in &tag.categories {
                let mut index: usize = 0;
                while index < found_categories.len() {
                    if found_categories[index] == *category {
                        category_counts[index] += 1;
                        break;
                    }
                    index += 1;
                }
                if index == found_categories.len() {
                    found_categories.push(*category);
                    category_counts.push(1);
                }
            }
        }
        let mut sum: f64 = 0.0;
        for index in 0..category_counts.len() {
            if found_categories[index] == Category::AU {
                sum += cmp::max(10 - 2 * category_counts[index], 0) as f64;
            } else {
                sum += category_counts[index] as f64;
            }
        }
        sum * 0.1 + 0.5
    }

    fn get_upgrade_bonus(&self) -> f64 {
        let mut sum: f64 = 0.5;
        for upgrade in &self.upgrades {
            sum += upgrade.multiplier * upgrade.count as f64;
        }
        sum
    }

    fn upload_story(&mut self) {
        self.kudos += 1.0 * (self.calc_tag_bonus() + self.get_upgrade_bonus());
        if self.kudos > self.highest_kudos {
            self.highest_kudos = self.kudos;
        }
    }

    fn tick(&mut self) {
        for upgrade in self.upgrades.clone() {
            if upgrade == String::from("Clone") {
                for _ in 0..upgrade.count {
                    self.upload_story();
                }
            }
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Upload => {
                self.upload_story();
            }
            Message::Tick(_) => {
                self.tick();
            }
            Message::BuyTag(tag) => {
                if tag.name == "???" {
                    return;
                }
                if self.kudos >= tag.cost as f64 {
                    self.kudos -= tag.cost as f64;
                    self.unlocked_tags.push(tag);
                }
            }
            Message::ToggleTag(tag) => {
                for index in 0..self.unlocked_tags.len() {
                    if self.unlocked_tags[index].name == tag.name {
                        self.unlocked_tags[index].active = !self.unlocked_tags[index].active;
                    }
                }
            }
            Message::BuyUpgrade(mut upgrade) => {
                if upgrade.name == "???" {
                    return;
                }
                for index in 0..self.upgrades.len() {
                    if self.upgrades[index] == upgrade.name {
                        upgrade.cost += 5 * self.upgrades[index].count;

                        if self.kudos >= upgrade.cost as f64
                            && self.upgrades[index].count < upgrade.count
                        {
                            self.upgrades[index].count += 1;
                            self.kudos -= upgrade.cost as f64;
                        }
                        return;
                    }
                }
                if self.kudos >= upgrade.cost as f64 {
                    self.kudos -= upgrade.cost as f64;
                    upgrade.count = 1;
                    self.upgrades.push(upgrade);
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        for upgrade in self.upgrades.clone() {
            if upgrade == String::from("Clone") {
                return time::every(seconds(5)).map(Message::Tick);
            }
        }
        Subscription::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let buyable = |name: String| button(text(name));
        let upload = button("Upload Work").on_press(Message::Upload);
        let mut upgrades: Column<'_, Message> = Column::new();
        let mut curr_row = Vec::new();
        for mut upgrade in self.all_upgrades.clone() {
            if self.highest_kudos < upgrade.cost as f64 {
                upgrade = Upgrade {
                    name: String::from("???"),
                    flavor_text: String::from(""),
                    desc: String::from(""),
                    multiplier: 0.0,
                    count: 0,
                    cost: 0,
                }
            }
            let mut count: usize = 0;
            for index in 0..self.upgrades.len() {
                if self.upgrades[index] == upgrade.name {
                    count = self.upgrades[index].count;
                }
            }
            let style = if count == upgrade.count && upgrade.name != "???" {
                button::secondary
            } else {
                button::primary
            };
            let cost = upgrade.cost + 5 * count;
            if curr_row.len() < 3 {
                curr_row.push(
                    tooltip(
                        buyable(upgrade.name.clone())
                            .on_press(Message::BuyUpgrade(upgrade.clone()))
                            .style(style),
                        container(column![
                            text(upgrade.flavor_text.clone()).font(Font {
                                style: Style::Italic,
                                ..Default::default()
                            }),
                            text(upgrade.desc.clone()),
                            text(if upgrade.name == "???" {
                                "".to_string()
                            } else if count != upgrade.count {
                                format!("Owned: {}/{} Price: {}", count, upgrade.count, cost)
                            } else {
                                "Sold Out".to_string()
                            })
                        ])
                        .style(container::bordered_box),
                        tooltip::Position::Bottom,
                    )
                    .into(),
                );
            } else {
                upgrades = upgrades.push(Row::from_vec(curr_row));
                curr_row = vec![
                    tooltip(
                        buyable(upgrade.name.clone())
                            .on_press(Message::BuyUpgrade(upgrade.clone()))
                            .style(style),
                        container(column![
                            text(upgrade.flavor_text.clone()).font(Font {
                                style: Style::Italic,
                                ..Default::default()
                            }),
                            text(upgrade.desc.clone()),
                        ])
                        .style(container::bordered_box),
                        tooltip::Position::Bottom,
                    )
                    .into(),
                ];
            }
            if upgrade.name == "???" {
                break;
            }
        }
        if !curr_row.is_empty() {
            upgrades = upgrades.push(Row::from_vec(curr_row));
        }
        let mut unbought_tags: Column<'_, Message> = Column::new();
        curr_row = Vec::new();
        for mut tag in self.all_tags.clone() {
            if self.unlocked_tags.contains(&tag) {
                continue;
            }
            if self.highest_kudos < tag.cost as f64 {
                tag = Tag {
                    name: String::from("???"),
                    categories: Vec::new(),
                    cost: 0,
                    active: false,
                }
            }
            if curr_row.len() < 3 {
                curr_row.push(
                    tooltip(
                        buyable(tag.name.clone()).on_press(Message::BuyTag(tag.clone())),
                        container(text!("Price: {}", tag.cost)).style(container::bordered_box),
                        tooltip::Position::Bottom,
                    )
                    .into(),
                );
            } else {
                unbought_tags = unbought_tags.push(Row::from_vec(curr_row));
                curr_row = vec![
                    tooltip(
                        buyable(tag.name.clone()).on_press(Message::BuyTag(tag.clone())),
                        container(text!("Price: {}", tag.cost)).style(container::bordered_box),
                        tooltip::Position::Bottom,
                    )
                    .into(),
                ];
            }
            if tag.name == "???" {
                break;
            }
        }
        if !curr_row.is_empty() {
            unbought_tags = unbought_tags.push(Row::from_vec(curr_row));
        }
        let mut tags: Column<'_, Message> = Column::new();
        curr_row = Vec::new();
        for tag in self.unlocked_tags.clone() {
            let style = if !tag.active {
                button::secondary
            } else {
                button::primary
            };
            if curr_row.len() < 3 {
                curr_row.push(
                    buyable(tag.name.clone())
                        .on_press(Message::ToggleTag(tag.clone()))
                        .style(style)
                        .into(),
                );
            } else {
                tags = tags.push(Row::from_vec(curr_row));
                curr_row = vec![
                    buyable(tag.name.clone())
                        .on_press(Message::ToggleTag(tag.clone()))
                        .style(style)
                        .into(),
                ];
            }
        }
        if !curr_row.is_empty() {
            tags = tags.push(Row::from_vec(curr_row));
        }
        let kudos = text!("{:.0} kudos", self.kudos);
        let content = column![
            text("Fanfic Clicker"),
            row![
                container(column![kudos, upload]).align_left(Fill),
                container(column![text("Tags:"), tags]).align_x(Center),
                container(column![
                    text("Upgrade Shop:"),
                    upgrades,
                    text("Tag Shop:"),
                    unbought_tags
                ])
                .align_right(Fill)
            ]
        ];
        content.into()
    }
}

fn main() -> iced::Result {
    iced::application(GameState::default, GameState::update, GameState::view)
        .subscription(GameState::subscription)
        .run()
}
