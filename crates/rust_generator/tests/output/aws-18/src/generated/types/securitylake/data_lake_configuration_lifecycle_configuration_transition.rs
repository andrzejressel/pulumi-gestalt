#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataLakeConfigurationLifecycleConfigurationTransition {
    /// Number of days before data transition to a different S3 Storage Class in the Amazon Security Lake object.
    #[builder(into)]
    #[serde(rename = "days")]
    pub r#days: Option<i32>,
    /// The range of storage classes that you can choose from based on the data access, resiliency, and cost requirements of your workloads.
    #[builder(into)]
    #[serde(rename = "storageClass")]
    pub r#storage_class: Option<String>,
}
