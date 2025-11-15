#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalClusterStorageLvpShareConfig {
    /// Defines the machine path and storage class for the LVP Share.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "lvpConfig")]
    pub r#lvp_config: Box<super::super::types::gkeonprem::BareMetalClusterStorageLvpShareConfigLvpConfig>,
    /// The number of subdirectories to create under path.
    #[builder(into)]
    #[serde(rename = "sharedPathPvCount")]
    pub r#shared_path_pv_count: Option<i32>,
}
