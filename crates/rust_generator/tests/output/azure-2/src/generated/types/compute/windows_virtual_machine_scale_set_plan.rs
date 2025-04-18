#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsVirtualMachineScaleSetPlan {
    /// Specifies the name of the image from the marketplace. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the product of the image from the marketplace. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "product")]
    pub r#product: Box<String>,
    /// Specifies the publisher of the image. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
}
