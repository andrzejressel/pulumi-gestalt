#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DistributionOriginGroupFailoverCriteria {
    /// List of HTTP status codes for the origin group.
    #[builder(into)]
    #[serde(rename = "statusCodes")]
    pub r#status_codes: Vec<i32>,
}
