#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ThreeTierVirtualInstanceThreeTierConfigurationDatabaseServerConfigurationVirtualMachineConfigurationImage {
    /// Specifies the offer of the platform image or marketplace image used to create the virtual machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "offer")]
    pub r#offer: String,
    /// The publisher of the Image. Possible values are `RedHat` and `SUSE`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: String,
    /// The SKU of the Image. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sku")]
    pub r#sku: String,
    /// Specifies the version of the platform image or marketplace image used to create the virtual machine. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: String,
}
