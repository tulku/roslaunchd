use xml::reader::{EventReader, XmlEvent};

use std::fs::File;
use std::io::BufReader;

use std::env;

use roslaunchd::Tree;
use roslaunchd::Tag;


fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).unwrap();
    let file = BufReader::new(file);

    let event_reader = EventReader::new(file);

    let mut tree = Tree::new(Tag::Launch);
    let mut parent_references = vec![&mut tree];

    for event in event_reader {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, namespace: _ }) => {
                // println!("{}+{} - {:?}", indent(depth), name, attributes);
                let opening_tag = Tag::new(&name.local_name[..], &attributes);
                if let Tag::Launch = opening_tag {
                    println!("Found tag {}, skipping.", name);
                } else {
                    let current_node = parent_references.last_mut().unwrap();
                    let new_parent = current_node.add_child(Tree::new(opening_tag));
                    // parent_references.push(new_parent);
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                // depth -= 1;
                // println!("{}-{}", indent(depth), name);
            }
            Err(error) => {
                println!("Error: {}", error);
                break;
            }
            _ => {}
        }
    }
    // TODO: tree.as_ref().unwrap() or &tree.unwrap() ??
    println!("{:?}", tree);
}
