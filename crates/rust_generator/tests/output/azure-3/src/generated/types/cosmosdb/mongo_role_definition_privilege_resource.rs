#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct MongoRoleDefinitionPrivilegeResource {
    /// The name of the Mongo DB Collection that the Role Definition is applied.
    #[builder(into)]
    #[serde(rename = "collectionName")]
    pub r#collection_name: Option<String>,
    /// The name of the Mongo DB that the Role Definition is applied.
    #[builder(into)]
    #[serde(rename = "dbName")]
    pub r#db_name: Option<String>,
}
