#[derive(Debug, Clone, PartialEq)]
pub enum NodeType {
    Database,

    Table,

    Section,

    Column,

    Index,

    ForeignKey,

    Constraint,
}

#[derive(Debug, Clone)]
pub struct Node {

    pub name: String,

    pub node_type: NodeType,

    pub value: Option<String>,

    pub children: Vec<Node>,
}

impl Node {

    pub fn new(
        name: &str,
        node_type: NodeType,
        value: Option<&str>,
    ) -> Self {

        Self {
            name: name.to_string(),

            node_type,

            value: value.map(|v| v.to_string()),

            children: vec![],
        }
    }

    pub fn add_child(
        &mut self,
        child: Node,
    ) {

        self.children.push(child);
    }
}