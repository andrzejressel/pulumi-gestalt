#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketReplicationConfigRuleDestinationReplicationTime {
    /// Status of the Replication Time Control. Either `"Enabled"` or `"Disabled"`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    /// Configuration block specifying the time by which replication should be complete for all objects and operations on objects. See below.
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: Box<super::super::types::s3::BucketReplicationConfigRuleDestinationReplicationTimeTime>,
}
