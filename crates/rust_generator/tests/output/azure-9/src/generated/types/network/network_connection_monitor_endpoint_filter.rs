#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct NetworkConnectionMonitorEndpointFilter {
    /// A `item` block as defined below.
    #[builder(into)]
    #[serde(rename = "items")]
    pub r#items: Option<Vec<super::super::types::network::NetworkConnectionMonitorEndpointFilterItem>>,
    /// The behaviour type of this endpoint filter. Currently the only allowed value is `Include`. Defaults to `Include`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
