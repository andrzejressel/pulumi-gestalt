#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RailsAppLayerLoadBasedAutoScaling {
    #[builder(into)]
    #[serde(rename = "downscaling")]
    pub r#downscaling: Option<Box<super::super::types::opsworks::RailsAppLayerLoadBasedAutoScalingDownscaling>>,
    #[builder(into)]
    #[serde(rename = "enable")]
    pub r#enable: Option<bool>,
    #[builder(into)]
    #[serde(rename = "upscaling")]
    pub r#upscaling: Option<Box<super::super::types::opsworks::RailsAppLayerLoadBasedAutoScalingUpscaling>>,
}
