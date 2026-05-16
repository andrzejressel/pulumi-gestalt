#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInputDestination {
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: String,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: String,
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: String,
    #[builder(into)]
    #[serde(rename = "vpcs")]
    pub r#vpcs: Vec<super::super::types::medialive::GetInputDestinationVpc>,
}
