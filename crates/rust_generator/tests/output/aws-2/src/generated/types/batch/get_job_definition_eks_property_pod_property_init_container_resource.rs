#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetJobDefinitionEksPropertyPodPropertyInitContainerResource {
    /// The type and quantity of the resources to reserve for the container.
    #[builder(into)]
    #[serde(rename = "limits")]
    pub r#limits: std::collections::HashMap<String, String>,
    /// The type and quantity of the resources to request for the container.
    #[builder(into)]
    #[serde(rename = "requests")]
    pub r#requests: std::collections::HashMap<String, String>,
}
