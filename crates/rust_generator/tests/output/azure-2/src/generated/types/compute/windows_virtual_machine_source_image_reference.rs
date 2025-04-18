#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WindowsVirtualMachineSourceImageReference {
    /// Specifies the offer of the image used to create the virtual machines. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "offer")]
    pub r#offer: Box<String>,
    /// Specifies the publisher of the image used to create the virtual machines. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
    /// Specifies the SKU of the image used to create the virtual machines. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sku")]
    pub r#sku: Box<String>,
    /// Specifies the version of the image used to create the virtual machines. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
