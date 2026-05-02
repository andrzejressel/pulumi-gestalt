use pulumi_gestalt_schema::model::ElementId;
use crate::model::ElementIdExt;
use crate::utils::escape_rust_name;

pub fn pcl_property_name_to_rust(name: &str) -> String {
    escape_rust_name(&ElementId::create_valid_id(name)).into()
}