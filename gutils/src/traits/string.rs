use regex::Regex;

#[allow(unused)]
pub trait ReString {
    fn replace_re(&self, re: &Regex, to: &str) -> Self;
    fn replace_re_all(&self, re: &Regex, to: &str) -> Self;
    fn to_title_case(&self) -> Self;
    fn splice(&self, size: usize) -> Self;
}

impl ReString for String {
    fn replace_re(&self, re: &Regex, to: &str) -> Self {
        re.replace(self, to).to_string()
    }

    fn replace_re_all(&self, re: &Regex, to: &str) -> Self {
        re.replace_all(self, to).to_string()
    }

    fn to_title_case(&self) -> Self {
        match self.len() {
            0 => self.to_string(),
            1 => self.to_uppercase(),
            _ => {
                let mut cha = self.chars();
                match cha.next() {
                    None => String::new(),
                    Some(f) => {
                        f.to_uppercase().collect::<String>()
                            + cha.collect::<String>().to_lowercase().as_str()
                    }
                }
            }
        }
    }

    fn splice(&self, index: usize) -> Self {
        self.split_at(if index > self.len() {
            self.len()
        } else {
            index
        })
        .0
        .to_string()
    }
}
