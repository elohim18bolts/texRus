pub mod document {
    pub trait Document {
        fn new() -> Self;
        fn title(&mut self, title: &str) -> &mut Self;
        fn new_paragraph(&mut self, par: &str) -> &mut Self;
    }
}

pub mod latex {
    use super::document::Document;
    static mut INSTANCE: bool = false;

    type FontSize = usize;

    pub enum DocumentClass {
        Article(FontSize),
    }
    pub trait Alignment {
        fn left(&self) -> String;
        fn center(&self) -> String;
    }
    pub trait LatexDocument {
        fn print(&self);
        fn create_document(&mut self, class: &DocumentClass) -> &mut Self;
        fn end_document(&mut self) -> &mut Self;
        fn begin_document(&mut self) -> &mut Self;
        fn section(&mut self, title: &str) -> &mut Self;
        fn unordered_list(&mut self, list: &Vec<&str>) -> &mut Self;
        fn ordered_list(&mut self, list: &Vec<&str>) -> &mut Self;
        fn toc(&mut self) -> &mut Self;
        fn new_page(&mut self) -> &mut Self;
    }
    pub struct LatexDoc {
        initialized: bool,
        preamble: String,
        content: String,
    }
    impl Document for LatexDoc {
        fn new() -> Self {
            unsafe {
                if INSTANCE {
                    panic!("Error: Can not create more than one instance of a latex document.");
                }
                INSTANCE = true;
            }
            Self {
                initialized: false,
                preamble: String::new(),
                content: String::new(),
            }
        }
        fn title(&mut self, title: &str) -> &mut Self {
            if self.initialized {
                panic!("Can not call title if the document has been initialized.");
            }
            self.initialized = true;
            self.preamble.push_str(&format!("\\title{{{title}}}\n"));
            self.preamble.push_str("\\today\n");
            self.content.push_str("\\begin{document}\n");
            self.content.push_str("\\maketitle\n");
            self.content.push_str("\\newpage\n");
            self
        }

        fn new_paragraph(&mut self, par: &str) -> &mut Self {
            if !self.initialized {
                panic!("Document not initialized.");
            }
            self.content.push_str("\\paragraph{}\n");
            self.content.push_str(par);
            self.content.push_str("\n");
            self
        }
    }
    impl LatexDocument for LatexDoc {
        fn print(&self) {
            println!("{}", self.preamble);
            println!("{}", self.content);
        }
        fn create_document(&mut self, class: &DocumentClass) -> &mut Self {
            self.preamble.push_str("\\documentclass[");
            match class {
                DocumentClass::Article(size) => {
                    self.preamble.push_str(&format!("{size}pt]{{article}}\n"))
                }
            }
            self
        }
        fn end_document(&mut self) -> &mut Self {
            if !self.initialized {
                panic!("Error: The document is not initialize, the end_document function has not meaning.");
            }
            self.content.push_str("\\end{document}\n");
            self
        }
        fn begin_document(&mut self) -> &mut Self {
            if self.initialized {
                panic!("Can not initialize an already initialized document.");
            }
            self.initialized = true;
            self.content.push_str("\\begin{document}\n");
            self
        }
        fn section(&mut self, title: &str) -> &mut Self {
            if !self.initialized {
                panic!("Error: Can not build section for an unintialized document.");
            }
            self.content.push_str(&format!("\\section{{{title}}}\n"));
            self
        }
        fn unordered_list(&mut self, list: &Vec<&str>) -> &mut Self {
            if !self.initialized {
                panic!("Can not create list if the document is not initialized.");
            }
            self.content.push_str("\\begin{itemize}\n");
            for item in list {
                self.content.push_str(&format!("\\item {item}\n"));
            }
            self.content.push_str("\\end{itemize}\n");
            self
        }
        fn ordered_list(&mut self, list: &Vec<&str>) -> &mut Self {
            if !self.initialized {
                panic!("Can not create list if the document is not initialized.");
            }
            self.content.push_str("\\begin{enumerate}\n");
            for item in list {
                self.content.push_str(&format!("\\item {item}\n"));
            }
            self.content.push_str("\\end{enumerate}\n");
            self
        }
        fn toc(&mut self) -> &mut Self {
            self.content.push_str("\\tableofcontents\n");
            self
        }
        fn new_page(&mut self) -> &mut Self {
            self.content.push_str("\\clearpage\n");
            self
        }
    }
    impl Alignment for str {
        fn left(&self) -> String {
            format!("\\begin{{flushleft}}\n{self}\n\\end{{flushleft}}\n")
        }
        fn center(&self) -> String {
            format!("\\begin{{center}}\n{self}\n\\end{{center}}\n")
        }
    }
}
