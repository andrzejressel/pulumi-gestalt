#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderFeaturesVirtualMachine {
    #[builder(into)]
    #[serde(rename = "deleteOsDiskOnDeletion")]
    pub r#delete_os_disk_on_deletion: Option<bool>,
    #[builder(into)]
    #[serde(rename = "detachImplicitDataDiskOnDeletion")]
    pub r#detach_implicit_data_disk_on_deletion: Option<bool>,
    #[builder(into)]
    #[serde(rename = "gracefulShutdown")]
    pub r#graceful_shutdown: Option<bool>,
    #[builder(into)]
    #[serde(rename = "skipShutdownAndForceDelete")]
    pub r#skip_shutdown_and_force_delete: Option<bool>,
}
