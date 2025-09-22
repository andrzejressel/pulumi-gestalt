#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceTypeInstanceDisk {
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: i32,
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: i32,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}
