#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceStateInfo {
    /// (Output)
    /// Represents information about instance with state UPDATING.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "updateInfos")]
    pub r#update_infos: Box<Option<Vec<super::super::types::memorystore::InstanceStateInfoUpdateInfo>>>,
}
