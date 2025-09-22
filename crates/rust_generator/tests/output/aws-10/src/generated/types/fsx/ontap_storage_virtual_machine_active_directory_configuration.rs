#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OntapStorageVirtualMachineActiveDirectoryConfiguration {
    /// The NetBIOS name of the Active Directory computer object that will be created for your SVM. This is often the same as the SVM name but can be different. AWS limits to 15 characters because of standard NetBIOS naming limits.
    #[builder(into)]
    #[serde(rename = "netbiosName")]
    pub r#netbios_name: Option<String>,
    #[builder(into)]
    #[serde(rename = "selfManagedActiveDirectoryConfiguration")]
    pub r#self_managed_active_directory_configuration: Option<Box<super::super::types::fsx::OntapStorageVirtualMachineActiveDirectoryConfigurationSelfManagedActiveDirectoryConfiguration>>,
}
