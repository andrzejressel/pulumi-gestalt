#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct StackStorageConnector {
    /// Type of storage connector.
    /// Valid values are `HOMEFOLDERS`, `GOOGLE_DRIVE`, or `ONE_DRIVE`.
    #[builder(into)]
    #[serde(rename = "connectorType")]
    pub r#connector_type: Box<String>,
    /// Names of the domains for the account.
    #[builder(into, default)]
    #[serde(rename = "domains")]
    pub r#domains: Box<Option<Vec<String>>>,
    /// ARN of the storage connector.
    #[builder(into, default)]
    #[serde(rename = "resourceIdentifier")]
    pub r#resource_identifier: Box<Option<String>>,
}
