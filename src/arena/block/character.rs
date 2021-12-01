uses! {
    super::BlockMut;
    super::util::Pack;
    regex::Regex;
}

block! {
    [pub ChatPallet(constructor, pack)]
    data: String = String::from("");
    defs: Vec<(Regex, String)> = vec![];
    index: Vec<(String, Vec<String>)> = vec![];
    match_index: Regex = Regex::new(r"\A//---(.*)(\n|\z)").unwrap();
    match_def: Regex = Regex::new(r"\A//(.+)=((.*\\\n)*(.*))(\n|\z)") .unwrap();
    match_line: Regex = Regex::new(r"\A(.*)(\n|\z)").unwrap();
    match_nl: Regex = Regex::new(r"([^\\])(\\\\)*\\n").unwrap();
}

impl ChatPallet {
    pub fn data_set(&mut self, mut data: String) {
        self.data = data.clone();
        self.index.clear();

        let mut index_name = String::from("");
        let mut index_items = vec![];

        while !data.is_empty() {
            if let Some(captures) = self.match_index.captures(&data) {
                if !index_items.is_empty() || !index_name.is_empty() {
                    self.index.push((index_name, index_items));
                }

                index_name = String::from(captures.get(1).unwrap().as_str());
                index_items = vec![];

                data = self.match_index.replace(&data, "").into();
            } else if let Some(captures) = self.match_def.captures(&data) {
                let regex = String::from(r"\A") + captures.get(1).unwrap().as_str() + r"\z";
                if let Ok(regex) = Regex::new(regex.as_str()) {
                    self.defs
                        .push((regex, String::from(captures.get(2).unwrap().as_str())));
                }

                data = self.match_def.replace(&data, "").into();
            } else if let Some(captures) = self.match_line.captures(&data) {
                let item = self
                    .match_nl
                    .replace_all(captures.get(1).unwrap().as_str(), "$1\n");
                index_items.push(item.into());

                data = self.match_line.replace(&data, "").into();
            } else {
                break;
            }
        }

        self.index.push((index_name, index_items));
    }

    pub fn index(&self) -> &Vec<(String, Vec<String>)> {
        &self.index
    }

    pub fn defs(&self) -> &Vec<(Regex, String)> {
        &self.defs
    }
}

block! {
    [pub Character(constructor, pack)]
    name: String = String::from("新規キャラクター");
    display_name: (String, String) = (String::from("新規キャラクター"), String::from(""));
}

impl Character {
    pub fn name(&self) -> &String {
        &self.name
    }
    pub fn display_name(&self) -> &(String, String) {
        &self.display_name
    }
}
