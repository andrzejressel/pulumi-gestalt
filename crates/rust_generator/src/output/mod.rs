use crate::model::ElementIdExt;
use crate::output::types::generate_types_code;
use crate::utils::escape_rust_name;
use convert_case::{Case, Casing};
use pulumi_gestalt_schema::model::{ElementId, Package};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{File, FileTimes};
use std::io::Write;
use std::time::SystemTime;

pub(crate) mod functions;
mod main;
pub(crate) mod provider;
pub(crate) mod resources;
pub(crate) mod types;

#[derive(Debug)]
enum TreeNode {
    Namespace(BTreeMap<String, TreeNode>, BTreeSet<ElementId>),
}

impl TreeNode {
    // Create an empty tree
    pub(crate) fn new() -> Self {
        TreeNode::Namespace(BTreeMap::new(), BTreeSet::new())
    }

    fn insert(&mut self, element: ElementId) {
        self.insert_priv(element, 0);
    }

    fn insert_priv(&mut self, element: ElementId, index: usize) {
        match self {
            TreeNode::Namespace(children, children_elements) => {
                if element.namespace.len() == index {
                    children_elements.insert(element.clone());
                } else {
                    let next_node = children
                        .entry(escape_rust_name(&element.namespace[index]).to_string())
                        .or_insert_with(TreeNode::new);
                    next_node.insert_priv(element, index + 1);
                }
            }
        }
    }
}

pub(crate) fn generate_combined_code(package: &Package, result_path: &std::path::Path) {
    generate_files(
        package,
        &result_path.join("functions"),
        &package.functions,
        &functions::generate_single_file,
    );
    generate_files(
        package,
        &result_path.join("resources"),
        &package.resources,
        &resources::generate_single_file,
    );
    generate_types_code(package, &result_path.join("types"));

    let provider_path = result_path.join("provider");
    std::fs::create_dir_all(&provider_path).unwrap();

    let provider_code = provider::generate_code(package);
    let mut provider_file = File::create(provider_path.join("provider.rs")).unwrap();
    provider_file.write_all(provider_code.as_bytes()).unwrap();
    let times = FileTimes::new().set_modified(SystemTime::UNIX_EPOCH);
    provider_file.set_times(times).unwrap();

    let main = main::generate(
        generate_includes("functions", &package.functions),
        generate_includes("resources", &package.resources),
        types::generate_module_imports(package),
        package,
    )
    .unwrap();

    let times = FileTimes::new().set_modified(SystemTime::UNIX_EPOCH);
    let mut file = File::create(result_path.join("lib.rs")).unwrap();
    file.write_all(main.as_bytes()).unwrap();
    file.set_times(times).unwrap();
}

fn generate_files<T>(
    package: &Package,
    result_path: &std::path::Path,
    objects: &BTreeMap<ElementId, T>,
    generator: &(impl Fn(&Package, &ElementId) -> String + 'static),
) {
    if objects.is_empty() {
        return;
    }
    let mut tree = TreeNode::new();

    for element_id in objects.keys() {
        tree.insert(element_id.clone());
    }

    generate_files_looper(package, &tree, result_path, &Box::new(&generator));
}

fn generate_files_looper(
    package: &Package,
    tree_node: &TreeNode,
    current_path: &std::path::Path,
    generator: &dyn Fn(&Package, &ElementId) -> String,
) {
    match tree_node {
        TreeNode::Namespace(ns, types) => {
            std::fs::create_dir_all(current_path).unwrap();
            for (name, node) in ns {
                generate_files_looper(package, node, &current_path.join(name), generator);
            }

            for type_ in types {
                let file_name = format!("{}.rs", type_.get_rust_struct_name().to_case(Case::Snake));
                let content = generator(package, type_);
                let mut file = File::create(current_path.join(file_name)).unwrap();
                file.write_all(content.as_bytes()).unwrap();
                let times = FileTimes::new().set_modified(SystemTime::UNIX_EPOCH);
                file.set_times(times).unwrap();
            }
        }
    }
}

fn generate_includes<T>(name: &str, objects: &BTreeMap<ElementId, T>) -> String {
    if objects.is_empty() {
        return "".to_string();
    }
    let mut tree = TreeNode::new();

    for element_id in objects.keys() {
        tree.insert(element_id.clone());
    }

    generate_includes_looper(&tree, std::path::Path::new(name))
}

fn generate_includes_looper(tree_node: &TreeNode, current_path: &std::path::Path) -> String {
    match tree_node {
        TreeNode::Namespace(ns, functions) => {
            let mut s = String::new();

            for (name, node) in ns {
                s.push_str(&format!("pub mod {}{{\n", name));
                s.push_str(&generate_includes_looper(node, &current_path.join(name)));
                s.push_str("}\n");
            }

            for function in functions {
                s.push_str(&format!(
                    "include!(\"{}/{}.rs\");\n",
                    current_path.to_str().unwrap().replace("\\", "/"),
                    function.get_rust_struct_name().to_case(Case::Snake)
                ));
            }

            s
        }
    }
}

pub(crate) fn get_register_interface(element_id: &ElementId) -> String {
    let depth = element_id.namespace.len() + 2;
    let prefix = if depth > 0 {
        "super::".repeat(depth)
    } else {
        "self::".to_string()
    };
    format!(
        "{}bindings::component::pulumi_gestalt::register_interface",
        prefix
    )
}
