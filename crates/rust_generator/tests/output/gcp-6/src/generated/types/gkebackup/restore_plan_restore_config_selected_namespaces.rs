#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RestorePlanRestoreConfigSelectedNamespaces {
    /// A list of Kubernetes Namespaces.
    #[builder(into)]
    #[serde(rename = "namespaces")]
    pub r#namespaces: Vec<String>,
}
