#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IntegrationRuntimeSsisPackageStore {
    /// Name of the Linked Service to associate with the packages.
    #[builder(into)]
    #[serde(rename = "linkedServiceName")]
    pub r#linked_service_name: Box<String>,
    /// Name of the package store.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
