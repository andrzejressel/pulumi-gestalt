#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ScaleSetStorageProfileImageReference {
    /// Specifies the ID of the (custom) image to use to create the virtual machine scale set, as in the example below.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Specifies the offer of the image used to create the virtual machines.
    #[builder(into)]
    #[serde(rename = "offer")]
    pub r#offer: Option<String>,
    /// Specifies the publisher of the image used to create the virtual machines.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Option<String>,
    /// Specifies the SKU of the image used to create the virtual machines.
    #[builder(into)]
    #[serde(rename = "sku")]
    pub r#sku: Option<String>,
    /// Specifies the version of the image used to create the virtual machines.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
