#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataLakeConfiguration {
    /// Provides encryption details of Amazon Security Lake object.
    #[builder(into)]
    #[serde(rename = "encryptionConfigurations")]
    pub r#encryption_configurations: Option<Vec<super::super::types::securitylake::DataLakeConfigurationEncryptionConfiguration>>,
    /// Provides lifecycle details of Amazon Security Lake object.
    #[builder(into)]
    #[serde(rename = "lifecycleConfiguration")]
    pub r#lifecycle_configuration: Option<Box<super::super::types::securitylake::DataLakeConfigurationLifecycleConfiguration>>,
    /// The AWS Regions where Security Lake is automatically enabled.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: String,
    /// Provides replication details of Amazon Security Lake object.
    #[builder(into)]
    #[serde(rename = "replicationConfiguration")]
    pub r#replication_configuration: Option<Box<super::super::types::securitylake::DataLakeConfigurationReplicationConfiguration>>,
}
