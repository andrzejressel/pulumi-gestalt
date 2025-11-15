#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTrafficPolicyDocumentEndpoint {
    /// ID of an endpoint you want to assign.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// To route traffic to an Amazon S3 bucket that is configured as a website endpoint, specify the region in which you created the bucket for `region`.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    /// Type of the endpoint. Valid values are `value`, `cloudfront`, `elastic-load-balancer`, `s3-website`, `application-load-balancer`, `network-load-balancer` and `elastic-beanstalk`
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
    /// Value of the `type`.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Option<String>,
}
