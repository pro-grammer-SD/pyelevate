use crate::models::Package;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

pub struct DependencyResolver {
    graph: DiGraph<String, ()>,
    nodes: HashMap<String, NodeIndex>,
}

#[derive(Debug, Clone)]
pub struct Conflict {
    pub package: String,
    pub reason: String,
    pub current: String,
    pub required: String,
}

impl DependencyResolver {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn add_package(&mut self, name: &str) {
        if !self.nodes.contains_key(name) {
            let node = self.graph.add_node(name.to_string());
            self.nodes.insert(name.to_string(), node);
        }
    }

    pub fn add_dependency(&mut self, from: &str, to: &str) {
        self.add_package(from);
        self.add_package(to);

        if let (Some(&from_idx), Some(&to_idx)) = (self.nodes.get(from), self.nodes.get(to)) {
            self.graph.add_edge(from_idx, to_idx, ());
        }
    }

    pub fn detect_conflicts(&self, packages: &[Package]) -> Vec<Conflict> {
        let mut conflicts = Vec::new();

        for pkg in packages {
            for dep in &pkg.dependencies {
                if let Some(dep_pkg) = packages.iter().find(|p| &p.name == dep) {
                    if let Some(latest) = &dep_pkg.latest_version {
                        if latest > &dep_pkg.current_version {
                            conflicts.push(Conflict {
                                package: pkg.name.clone(),
                                reason: format!(
                                    "Requires {} but upgrade to {} may break compatibility",
                                    dep, latest
                                ),
                                current: dep_pkg.current_version.clone(),
                                required: latest.clone(),
                            });
                        }
                    }
                }
            }
        }

        conflicts
    }

    pub fn get_dependents(&self, package: &str) -> Vec<String> {
        if let Some(&node_idx) = self.nodes.get(package) {
            self.graph
                .neighbors_directed(node_idx, petgraph::Direction::Incoming)
                .filter_map(|idx| self.graph.node_weight(idx).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn get_dependencies(&self, package: &str) -> Vec<String> {
        if let Some(&node_idx) = self.nodes.get(package) {
            self.graph
                .neighbors_directed(node_idx, petgraph::Direction::Outgoing)
                .filter_map(|idx| self.graph.node_weight(idx).cloned())
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl Default for DependencyResolver {
    fn default() -> Self {
        Self::new()
    }
}
