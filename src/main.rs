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
}

#[derive(Clone)]
struct Upgrade {
    name: String,
    multiplier: f64,
    count: usize,
}

impl PartialEq<String> for Upgrade {
    fn eq(&self, other: &String) -> bool {
        &self.name == other
    }
}

struct GameState {
    unlocked_tags: Vec<Tag>,
    active_tags: Vec<Tag>,
    upgrades: Vec<Upgrade>,
    kudos: f64,
}

impl GameState {
    fn new() -> Self {
        GameState { unlocked_tags: Vec::new(), active_tags: Vec::new(), upgrades: Vec::new(), kudos: 0.0 }
    }

    fn calc_tag_bonus(&self) -> f64 {
        let mut found_categories: Vec<Category> = Vec::new();
        let mut category_counts: Vec<usize> = Vec::new();
        for tag in &self.active_tags {
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
        for count in category_counts {
            sum += count as f64;
        }
        sum * 0.1
    }

    fn get_upgrade_bonus(&self) -> f64 {
        let mut sum: f64 = 0.0;
        for upgrade in &self.upgrades {
            sum += upgrade.multiplier * upgrade.count as f64;
        }
        sum
    }

    fn upload_story(mut self) -> Self {
        self.kudos += 1.0 * (self.calc_tag_bonus() + self.get_upgrade_bonus());
        self
    }
}

fn tick(mut gamestate: GameState) -> GameState {
    for upgrade in gamestate.upgrades.clone() {
        if upgrade == String::from("Clone") {
            for _ in 0..upgrade.count {
                gamestate = gamestate.upload_story();
            }
        }
    }
    gamestate
}

fn main() {
    let mut gamestate: GameState = GameState::new();


}
