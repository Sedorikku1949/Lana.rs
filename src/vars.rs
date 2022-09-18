use std::fs;

pub(in crate) const TOKEN: &str = "lx9lKTmvvrV8jXRfKPAeGq4FLtnqzEWy2h/6Zc8msPCgfosAIxGXGG8c4b9NE7QyBFZ3WE0HmNPAI6D4NMLT/S+QQMk3CV7uI+3Oz18Sne0=";
pub(in crate) const PREFIX: char = '&';

pub(in crate) const DEV_ID: [&str; 2] = ["oTuuZtBsFA/wdrV4nrbbgJQNYWFgVGENipXG5nobios=", "AuVhtZE48wqtcJMcEJPkLUebB4MLuGsg4emcJruQvn4="];

#[allow(dead_code)]
pub(in crate) struct Config<'a> {
    pub(in crate) maintenance: i8,
    pub(in crate) status: &'a str,
    pub(in crate) default_status: &'a str,
    pub(in crate) raw: Vec<(String, String)>,
}

#[allow(dead_code)]
impl Config<'_> {
    pub(in crate) fn edit(&mut self, key: &String, data: &String, save: bool) -> bool {
        let mut edited: bool = false;
        for e in &mut self.raw {
            if &e.0 == key {
                e.1 = data.to_owned();
                edited = true;
            }
        };
        if !edited { self.raw.insert(0 as usize, (key.to_owned(), data.to_owned())); };
        if save {  let _ = self.save(); };
        return true;
    }

    pub(in crate) fn get(&self, key: &String) -> Result<&String, ()> {
        for e in &self.raw {
            if &e.0 == key { return Ok(&e.1); }
        };
        Err(())
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let mut content: String = String::new();
        for (k, v) in &self.raw {
            let cnt = format!("{key}={value}\n", key = k.as_str(), value = v.as_str());
            content.push_str(&cnt);
        };
        content.push_str(format!("maintenance={}", &self.maintenance).as_str());
        return fs::write("config.lmay", content.trim())
    }
}

pub(in crate) static mut CONFIG: Config = Config {
    maintenance: -1,
    status: "🦀 Rusty crab wave in coming",
    default_status: "🦀 Rusty crab wave in coming",
    raw: vec![]
};