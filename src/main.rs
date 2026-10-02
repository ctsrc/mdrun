use comrak::{
    Arena, Options,
    arena_tree::NodeEdge,
    format_html,
    nodes::{Ast, NodeHeading, NodeValue, Sourcepos},
    parse_document,
};
use std::cell::Ref;

#[derive(Debug)]
pub struct SectionBuilder<'a> {
    pub heading: NodeHeading,
    pub heading_sourcepos: Sourcepos,
    pub heading_content: Vec<Ref<'a, Ast>>,
    pub section_content: Vec<SectionContent<'a>>,
}

#[derive(Debug)]
pub enum SectionContent<'a> {
    /// A fenced code block. For example zsh, text, python3, etc
    CodeBlock(Ref<'a, Ast>),
    /// A non-code piece of content.
    Other(Ref<'a, Ast>),
}

#[derive(Debug)]
pub struct Section<'a> {
    heading: NodeHeading,
    heading_sourcepos: Sourcepos,
    heading_content: Vec<Ref<'a, Ast>>,
    section_content: Vec<SectionContent<'a>>,
}

impl<'a> SectionBuilder<'a> {
    pub fn new(heading: NodeHeading, heading_sourcepos: Sourcepos) -> Self {
        Self {
            heading,
            heading_sourcepos,
            heading_content: Default::default(),
            section_content: Default::default(),
        }
    }
    pub fn build(self) -> Section<'a> {
        Section {
            heading: self.heading,
            heading_sourcepos: self.heading_sourcepos,
            heading_content: self.heading_content,
            section_content: self.section_content,
        }
    }
}

fn main() {
    let md_document = "# `cool` project ## yeah!\n\nExample readme of an awesome project\n\n```zsh\ntrue\n```\n\n```text\n```\n\n##\n\n## How are you?\n\nHello\n\n### Sub\n\nsub\n\n## Next\n\nnext\n\n### Sub\n\nsub 2\n";

    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
    let arena = Arena::new();

    // Parse the document into a root `AstNode`
    let root = parse_document(&arena, md_document, &Options::default());

    let mut sections = vec![];
    let mut current_section: Option<SectionBuilder> = None;
    let mut current_heading_is_open = false;

    // Traverse edges and gather sections of the document from AST.
    // A section is the part of a document from the beginning of a heading,
    // up until the beginning of the next heading, regardless of the level of the headings.
    //
    // For example, the following document consists of four sections:
    //
    // ```markdown
    // # Eef freef
    //
    // ## Schmeep leep
    //
    // Zorp dorp.
    //
    // ### Burga borp
    //
    // Bazinga!
    //
    // ##
    //
    // Oops I forgor put title in above heading..
    // ```
    for edge in root.traverse() {
        match edge {
            NodeEdge::Start(node) => {
                let data = node.data();
                //println!("Entering node: {:?}", data);
                if let NodeValue::Heading(heading) = data.value {
                    // Reached start of a heading, and thus the start of a new section.

                    // Headings do not "nest"; for example, `# hello ## world` on one line is *not* a h2 "inside" a h1.
                    if current_heading_is_open {
                        panic!(
                            "{}: Got new heading start while already inside a heading. Malformed document?",
                            data.sourcepos
                        );
                    }

                    // We are finished with previous section if there was any.
                    if let Some(previous_section) = current_section {
                        sections.push(previous_section.build());
                    }

                    // Starting current section.
                    current_section = Some(SectionBuilder::new(heading, data.sourcepos));
                    current_heading_is_open = true;
                } else if current_heading_is_open {
                    // Currently inside a heading.

                    current_section
                        .as_mut()
                        .expect("Current section is already some at this point, because we are inside a heading.")
                        .heading_content
                        .push(data);
                } else if let Some(current_section) = current_section.as_mut() {
                    // Inside a section, after the heading of the section has been closed.

                    let section_content = match data.value {
                        NodeValue::CodeBlock(_) => SectionContent::CodeBlock(data),
                        _ => SectionContent::Other(data),
                    };
                    current_section.section_content.push(section_content);
                }
            }
            NodeEdge::End(node) => {
                let data = node.data();
                //println!("Leaving node: {:?}", data);
                if let NodeValue::Heading(_) = data.value {
                    // Reached end of a heading. Keep in mind, this does not end the section itself.
                    // It is only the end of the beginning :^)

                    if !current_heading_is_open {
                        panic!(
                            "{}: Got end of heading while not yet inside a heading. Malformed document?",
                            data.sourcepos
                        );
                    }
                    current_heading_is_open = false;
                }
            }
        }
    }
    if let Some(current_section) = current_section {
        sections.push(current_section.build());
    }

    //println!();

    let mut html = String::new();
    format_html(root, &Options::default(), &mut html).unwrap();

    println!("{}", html);
    println!();
    println!("{:#?}", sections);
    println!();

    //
    //
    //

    for section in sections {
        println!("Section heading: {:#?}", section.heading);
        println!(
            "Section heading source position: {:#?}",
            section.heading_sourcepos
        );
        println!("Section heading content: {:?}", section.heading_content);
        for content in section.section_content {
            match content {
                SectionContent::CodeBlock(code_block) => {
                    println!("{:?}", code_block);
                }
                other => {
                    println!("{:?}", other);
                }
            }
        }
        println!();
    }
}
