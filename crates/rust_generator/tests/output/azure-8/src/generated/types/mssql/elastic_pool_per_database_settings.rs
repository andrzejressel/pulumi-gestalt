#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ElasticPoolPerDatabaseSettings {
    /// The maximum capacity any one database can consume.
    #[builder(into)]
    #[serde(rename = "maxCapacity")]
    pub r#max_capacity: f64,
    /// The minimum capacity all databases are guaranteed.
    #[builder(into)]
    #[serde(rename = "minCapacity")]
    pub r#min_capacity: f64,
}
