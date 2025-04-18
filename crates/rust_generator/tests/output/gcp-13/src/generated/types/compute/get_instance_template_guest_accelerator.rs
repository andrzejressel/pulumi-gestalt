#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetInstanceTemplateGuestAccelerator {
    /// The number of the guest accelerator cards exposed to this instance.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// The accelerator type resource to expose to this instance. E.g. `nvidia-tesla-k80`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
