#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolStorageImageReference {
    /// Specifies the ID of the Custom Image which the virtual machines should be created from. Changing this forces a new resource to be created. See [official documentation](https://docs.microsoft.com/azure/batch/batch-custom-images) for more details.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<String>,
    /// Specifies the offer of the image used to create the virtual machines. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "offer")]
    pub r#offer: Option<String>,
    /// Specifies the publisher of the image used to create the virtual machines. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Option<String>,
    /// Specifies the SKU of the image used to create the virtual machines. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sku")]
    pub r#sku: Option<String>,
    /// Specifies the version of the image used to create the virtual machines. Changing this forces a new resource to be created.
    /// 
    /// To provision a Custom Image, the following fields are applicable:
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}
