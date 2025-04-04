#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LinuxVirtualMachineScaleSetBootDiagnostics {
    /// The Primary/Secondary Endpoint for the Azure Storage Account which should be used to store Boot Diagnostics, including Console Output and Screenshots from the Hypervisor.
    /// 
    /// > **Note:** Passing a null value will utilize a Managed Storage Account to store Boot Diagnostics.
    #[builder(into, default)]
    #[serde(rename = "storageAccountUri")]
    pub r#storage_account_uri: Box<Option<String>>,
}
