#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct NetworkConnectionMonitorEndpointFilterItem {
    /// The address of the filter item.
    #[builder(into)]
    #[serde(rename = "address")]
    pub r#address: Option<String>,
    /// The type of items included in the filter. Possible values are `AgentAddress`. Defaults to `AgentAddress`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
