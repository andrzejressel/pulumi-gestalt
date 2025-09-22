#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct HyperdriveConfigCaching {
    /// Disable caching for this Hyperdrive configuration.
    #[builder(into)]
    #[serde(rename = "disabled")]
    pub r#disabled: Option<bool>,
    /// Configure the `max_age` value of this Hyperdrive configuration.
    #[builder(into)]
    #[serde(rename = "maxAge")]
    pub r#max_age: Option<i32>,
    /// Disable caching for this Hyperdrive configuration.
    #[builder(into)]
    #[serde(rename = "staleWhileRevalidate")]
    pub r#stale_while_revalidate: Option<i32>,
}
