#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct Container {
    #[builder(into)]
    #[serde(rename = "brightness")]
    pub r#brightness: Option<Box<super::types::ContainerBrightness>>,
    #[builder(into)]
    #[serde(rename = "color")]
    pub r#color: Option<pulumi_gestalt_rust::OneOf2<Box<super::types::ContainerColor>, String>>,
    #[builder(into)]
    #[serde(rename = "material")]
    pub r#material: Option<String>,
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<super::types::ContainerSize>,
}
