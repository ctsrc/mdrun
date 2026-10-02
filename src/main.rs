use comrak::{Arena, Options, arena_tree::NodeEdge, format_html, nodes::NodeValue, parse_document};

fn main() {
    let md_document = "# `cool` project\n\nExample readme of an awesome project\n\n```zsh\ntrue\n```\n\n```text\n```\n\n## How are you?\n\nHello\n\n### Sub\n\nsub\n\n## Next\n\nnext\n\n### Sub\n\nsub 2\n";

    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    // Parse the document into a root `AstNode`
    let root = parse_document(&arena, md_document, &Options::default());

    // Iterate over all the descendants of root.
    for node in root.descendants() {
        let data = node.data();
        println!("Data: {:#?}", data);
        if let NodeValue::Heading(_) = data.value {
            println!("was a heading");
        }
    }

    println!();

    // Traverse edges. Alternative to walking descendants.
    for edge in root.traverse() {
        match edge {
            NodeEdge::Start(node) => {
                println!("Entering node: {:?}", node.data());
            }
            NodeEdge::End(node) => {
                println!("Leaving node: {:?}", node.data());
            }
        }
    }
    
    println!();

    let mut html = String::new();
    format_html(root, &Options::default(), &mut html).unwrap();

    println!("{}", html);
}
