#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
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
