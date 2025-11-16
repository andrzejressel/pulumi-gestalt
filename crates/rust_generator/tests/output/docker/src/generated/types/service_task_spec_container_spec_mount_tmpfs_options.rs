#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTaskSpecContainerSpecMountTmpfsOptions {
    /// The permission mode for the tmpfs mount in an integer
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Option<i32>,
    /// The size for the tmpfs mount in bytes
    #[builder(into)]
    #[serde(rename = "sizeBytes")]
    pub r#size_bytes: Option<i32>,
}
