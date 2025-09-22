#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AiFeatureOnlineStoreFeatureviewBigQuerySource {
    /// Columns to construct entityId / row keys. Start by supporting 1 only.
    #[builder(into)]
    #[serde(rename = "entityIdColumns")]
    pub r#entity_id_columns: Vec<String>,
    /// The BigQuery view URI that will be materialized on each sync trigger based on FeatureView.SyncConfig.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: String,
}
