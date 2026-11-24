pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;
    #[derive(Debug, Default, Clone)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }
    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Default, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(name: &str) -> Self {
                    Self {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();

                    self
                }
                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(String::as_str)
                }
            }
        }
        pub mod edge {
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Default, Clone)]
            pub struct Edge {
                pub from: String,
                pub to: String,
                pub attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(from: &str, to: &str) -> Self {
                    Self {
                        from: from.to_string(),
                        to: to.to_string(),
                        ..Default::default()
                    }
                }
                pub fn with_attrs(mut self, attrs: &[(&str, &str)]) -> Self {
                    self.attrs = attrs
                        .iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect();

                    self
                }
                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(String::as_str)
                }
            }
        }
    }

    impl Graph {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn node(&self, name: &str) -> Option<&Node> {
            self.nodes.iter().find(|node| node.name == name)
        }
        pub fn with_nodes(mut self, no: &[Node]) -> Self {
            self.nodes = no.to_vec();
            self
        }

        pub fn with_edges(mut self, ed: &[Edge]) -> Self {
            self.edges = ed.to_vec();
            self
        }

        pub fn with_attrs(mut self, at: &[(&str, &str)]) -> Self {
            self.attrs = at
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            self
        }
    }
}
