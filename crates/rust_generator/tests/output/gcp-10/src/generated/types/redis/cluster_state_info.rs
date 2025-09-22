#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClusterStateInfo {
    /// A nested object resource.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "updateInfo")]
    pub r#update_info: Option<Box<super::super::types::redis::ClusterStateInfoUpdateInfo>>,
}
