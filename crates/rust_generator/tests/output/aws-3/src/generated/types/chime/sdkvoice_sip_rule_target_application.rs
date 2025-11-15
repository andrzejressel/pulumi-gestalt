#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SdkvoiceSipRuleTargetApplication {
    /// The AWS Region of the target application.
    #[builder(into)]
    #[serde(rename = "awsRegion")]
    pub r#aws_region: String,
    /// Priority of the SIP media application in the target list.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: i32,
    /// The SIP media application ID.
    #[builder(into)]
    #[serde(rename = "sipMediaApplicationId")]
    pub r#sip_media_application_id: String,
}
