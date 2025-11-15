#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRestorableDatabaseAccountsAccount {
    /// The API type of the Cosmos DB Restorable Database Account.
    #[builder(into)]
    #[serde(rename = "apiType")]
    pub r#api_type: String,
    /// The creation time of the regional Cosmos DB Restorable Database Account.
    #[builder(into)]
    #[serde(rename = "creationTime")]
    pub r#creation_time: String,
    /// The deletion time of the regional Cosmos DB Restorable Database Account.
    #[builder(into)]
    #[serde(rename = "deletionTime")]
    pub r#deletion_time: String,
    /// The ID of the Cosmos DB Restorable Database Account.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// One or more `restorable_locations` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "restorableLocations")]
    pub r#restorable_locations: Vec<super::super::types::cosmosdb::GetRestorableDatabaseAccountsAccountRestorableLocation>,
}
