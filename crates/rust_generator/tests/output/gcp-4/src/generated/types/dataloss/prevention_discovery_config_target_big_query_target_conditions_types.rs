#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PreventionDiscoveryConfigTargetBigQueryTargetConditionsTypes {
    /// A set of BiqQuery table types
    /// Each value may be one of: `BIG_QUERY_TABLE_TYPE_TABLE`, `BIG_QUERY_TABLE_TYPE_EXTERNAL_BIG_LAKE`.
    #[builder(into)]
    #[serde(rename = "types")]
    pub r#types: Option<Vec<String>>,
}
