use latex::latex::{Document, DocumentClass, Element, LatexDoc, Package};

fn main() {
    let mut doc: LatexDoc = Document::create_document();
    doc.add(&DocumentClass::Article(13));
    let package = Package::Array(None);
    doc.add(&package);
    println!("{}", doc.render());
}
