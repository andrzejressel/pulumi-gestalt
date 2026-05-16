#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionDestinationConfigDestination {
    /// Host
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Option<String>,
    /// port number
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Service Attachment
    #[builder(into)]
    #[serde(rename = "serviceAttachment")]
    pub r#service_attachment: Option<String>,
}
