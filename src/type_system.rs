use chrono::prelude::*;

pub trait Meme {
    /// What the meme is called
    fn name(&self) -> String;
    fn state(&self) -> MemeState;
}

#[derive(Clone, Debug)]
pub enum MemeState {
    Alive,
    /// When the meme died
    Dead(DateTime<Utc>),
}

#[derive(Clone, Debug)]
pub struct Pope {
    name: String,
    state: MemeState,
}

impl Pope {
    pub fn new(name: &str, state: MemeState) -> Self {
        Self { name: name.to_owned(), state }
    }
    pub fn kill(&mut self) {
        if let MemeState::Alive = self.state {
            self.state = MemeState::Dead(Utc::now());
        } else {
            eprintln!("Could not kill pope {} who is already dead.", self.name);
        }
    }
}

impl Meme for Pope {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn state(&self) -> MemeState {
        self.state.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Doge;

impl Meme for Doge {
    fn name(&self) -> String {
        "doge".to_owned()
    }

    fn state(&self) -> MemeState {
        MemeState::Alive
    }
}

#[derive(Clone, Debug)]
pub struct Pepe {
    pub name: String,
    pub state: MemeState,
}

impl Meme for Pepe {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn state(&self) -> MemeState {
        self.state.clone()
    }
}
