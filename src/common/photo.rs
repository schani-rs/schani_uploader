#[derive(Debug)]
pub struct Photo {
    path: String,
    title: String,
}

impl Photo {
    pub fn new(path: &String, title: &String) -> Photo {
        Photo {
            path: path.to_string(),
            title: title.to_string(),
        }
    }

    pub fn get_path(&self) -> String {
        self.path.to_string()
    }
}
