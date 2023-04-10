#[derive(Debug)]
enum NodeType {
    INTEGER
}

#[derive(Debug)]
struct Node {
    n_type: NodeType,
    value: String,
    left_child: Option<Box<Node>>,
    right_child: Option<Box<Node>>,
}

impl Node {
    pub fn new(n_type: NodeType, value: String) -> Self {
        Self {
            n_type,
            value,
            left_child: None,
            right_child: None,
        }
    }

    pub fn insert_left_child(&mut self, child: Node) -> () {
        self.left_child = Some(Box::from(child));
    }

    pub fn insert_right_child(&mut self, child: Node) -> () {
        self.right_child = Some(Box::from(child));
    }

    pub fn scan_node(node: &Node) -> () {
        println!("Scaning node...");
        println!("Node type: {:?}", node.n_type);
        println!("Node value: {}", node.value);
        println!("");

        if node.left_child.is_some() {
            Self::scan_node(&node.left_child.as_ref().unwrap());
        }

        if node.right_child.is_some() {
            Self::scan_node(&node.right_child.as_ref().unwrap());
        }
    }
}

fn main() {
   let mut node: Node = Node::new(NodeType::INTEGER, String::from("123"));
   let left_child: Node = Node::new(NodeType::INTEGER, String::from("124"));
   let right_child: Node = Node::new(NodeType::INTEGER, String::from("125"));

   node.insert_left_child(left_child);
   node.insert_right_child(right_child);

   Node::scan_node(&node);
}
