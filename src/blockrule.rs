use std::collections::HashMap;

pub struct ViewState {
    id: String,
    tag: String,
    classes: Vec<String>,
    attrs: HashMap<String, String>
}

impl ViewState {
    pub fn new() -> ViewState {
        ViewState {
            id: String::new(),
            tag: String::new(),
            classes: Vec::new(),
            attrs: HashMap::new(),
        }
    }
}
