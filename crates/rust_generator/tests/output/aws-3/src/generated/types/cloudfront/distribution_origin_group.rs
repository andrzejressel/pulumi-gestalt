#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionOriginGroup {
    /// The failover criteria for when to failover to the secondary origin.
    #[builder(into)]
    #[serde(rename = "failoverCriteria")]
    pub r#failover_criteria: Box<super::super::types::cloudfront::DistributionOriginGroupFailoverCriteria>,
    /// Ordered member configuration blocks assigned to the origin group, where the first member is the primary origin. You must specify two members.
    #[builder(into)]
    #[serde(rename = "members")]
    pub r#members: Vec<super::super::types::cloudfront::DistributionOriginGroupMember>,
    #[builder(into)]
    #[serde(rename = "originId")]
    pub r#origin_id: String,
}
