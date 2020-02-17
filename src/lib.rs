use std::collections::HashMap;
// use std::fmt;
// use std::str::FromStr;
//
//
// #[derive(Debug)]
// pub struct Arena<T> {
//     data: std::Vec<T>;
// }

pub struct Arena<T> {
    content: Vec<T>,
}


#[derive(Debug)]
pub struct Tree{
    tag: Tag,
    children: Vec<Tree>,
    // parent: &Tree  // self if root
}

impl Tree {
    pub fn new(tag: Tag) -> Self {
        Self {
            tag,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Tree) -> &mut Tree {
        // child.parent = &self;
        self.children.push(child);
        self.children.last_mut().unwrap()
    }
}

// impl fmt::Display for Tree {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         fn indent(size: usize) -> String {
//             const INDENT: &'static str = "    ";
//             (0..size).map(|_| INDENT)
//                      .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
//         }



//         write!(f, "Tag: {:?}", self.tag)
//         write!(f, indent(1))
//     }
// }

#[derive(Debug)]
pub enum Tag {
    Launch,
    Include {
        file: String,
    },
    Group {
        name: String,
    },
    Remap {
        from: String,
        to: String,
    },
    Rosparam {
        command: String,
        file: String,
    },
    Param {
        name: String,
        value: String,
    },
    Node {
        name: String,
        package: String,
        node_type: String,
    }
}

fn getAttributeByName(attribute_name: &str, attributes: &Vec<xml::attribute::OwnedAttribute>) -> String {
    for attribute in attributes {
        if attribute.name.local_name == attribute_name {
            return attribute.value.clone();
        }
    }
    panic!("Required attribute {} not found in {:?}", attribute_name, attributes);
}

impl Tag {
    pub fn new(name: &str, attributes: &Vec<xml::attribute::OwnedAttribute>) -> Self {
        // TODO: lowercase name just in case?
        match name {
            "launch" => Self::Launch,
            "include" => {
                Self::newInclude(attributes)
            },
            "group" => {
                Self::newGroup(attributes)
            },
            "remap" => {
                Self::newRemap(attributes)
            },
            "rosparam" => {
                Self::newRosparam(attributes)
            },
            "param" => {
                Self::newParam(attributes)
            },
            "node" => {
                Self::newNode(attributes)
            },
            _ => panic!("Tag type {} not sopported", name)
        }
    }

    fn newGroup(attributes: &Vec<xml::attribute::OwnedAttribute>) -> Self {
        let name = getAttributeByName("name", attributes);
        Self::Group {
            name,
        }
    }

    fn newInclude(attributes: &Vec<xml::attribute::OwnedAttribute>) -> Self {
        let file = getAttributeByName("file", attributes);
        Self::Include {
            file,
        }
    }

    fn newRemap(attributes: &Vec<xml::attribute::OwnedAttribute>) -> Self {
        let from = getAttributeByName("from", attributes);
        let to = getAttributeByName("to", attributes);
        Self::Remap {
            to,
            from,
        }
    }

    fn newRosparam(attributes: &Vec<xml::attribute::OwnedAttribute>) -> Self {
        let file = getAttributeByName("file", attributes);
        let command = getAttributeByName("command", attributes);
        Self::Rosparam {
            file,
            command,
        }
    }

    fn newParam(attributes: &Vec<xml::attribute::OwnedAttribute>) -> Self {
        let name = getAttributeByName("name", attributes);
        let value = getAttributeByName("value", attributes);
        Self::Param {
            name,
            value,
        }
    }

    fn newNode(attributes: &Vec<xml::attribute::OwnedAttribute>) -> Self {
        let name = getAttributeByName("name", attributes);
        let package = getAttributeByName("pkg", attributes);
        let node_type = getAttributeByName("type", attributes);
        Self::Node {
            name,
            package,
            node_type,
        }
    }

}

struct Node {
    /// Name of the package
    package: String,
    /// Node type
    node_type: String,
    /// Name of the node in the ros graph.
    name: String,
    /// Namespace for the node.
    namespace: String,
    /// Machine where the node will run.
    /// Not used. All nodes run in localhost.
    machine: String,
    /// Command line arguments passed to the executable.
    args: Vec<String>,
    /// If true respawn if the node dies.
    respawn: bool,
    /// If respawn is true, do it after this delay in seconds.
    respawn_delay: f64,
    /// Map of (from, to) remapping arguments.
    remap_args: HashMap<String, String>,
    /// Map of (variable, value) of extra environment variables set for the node
    env_args: HashMap<String, String>,
    process_name: String,
    output: String,
    cwd: String,
    launch_prefix: String,
    required: bool,
    /// Name of the file the node was parsed from.
    filename: String,
}

impl Node {
    fn new(package: &str, node_type: &str, name: &str, respawn: bool) -> Node {
        Node {
            package: package.to_owned(),
            node_type: node_type.to_owned(),
            name: name.to_owned(),
            namespace: String::new(),
            machine: String::new(),
            args: Vec::new(),
            respawn,
            respawn_delay: 0.1,
            remap_args: HashMap::new(),
            env_args: HashMap::new(),
            process_name: String::new(),
            output: String::new(),
            cwd: String::new(),
            launch_prefix: String::new(),
            required: true,
            filename: String::new(),
        }
    }
}
