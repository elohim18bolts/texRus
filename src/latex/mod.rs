type Size = usize;
static mut INSTANCE: bool = false;
pub trait Element {
    fn render(&self) -> String;
}
pub trait Document {
    fn create_document() -> Self;
    fn add(&mut self, e: &dyn Element) -> &mut Self;
}

pub struct LatexDoc {
    content: String,
}

impl Document for LatexDoc {
    fn create_document() -> Self {
        unsafe {
            if INSTANCE {
                panic!("Error: One document is enough.");
            }
            INSTANCE = true;
        }
        Self {
            content: String::new(),
        }
    }
    fn add(&mut self, e: &dyn Element) -> &mut Self {
        self.content.push_str(e.render().as_str());
        self.content.push_str("\n");
        self
    }
}
impl Element for LatexDoc {
    fn render(&self) -> String {
        self.content.clone()
    }
}

pub enum DocumentClass {
    Article(Size),
}
impl Element for DocumentClass {
    fn render(&self) -> String {
        match self {
            DocumentClass::Article(size) => format!("\\documentclass[{size}pt]{{Article}}"),
        }
    }
}

pub enum Package<'a> {
    Array(Option<&'a str>),
}
impl<'a> Element for Package<'a> {
    fn render(&self) -> String {
        match self {
            Package::Array(Some(options)) => {
                format!("\\usepackage[{options}]{{Array}}")
            }
            Package::Array(None) => format!("\\usepackage{{Array}}"),
        }
    }
}
