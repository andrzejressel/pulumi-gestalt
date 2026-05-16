#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CatalogDatabaseCreateTableDefaultPermission {
    /// The permissions that are granted to the principal.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Option<Vec<String>>,
    /// The principal who is granted permissions.. See `principal` below.
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: Option<Box<super::super::types::glue::CatalogDatabaseCreateTableDefaultPermissionPrincipal>>,
}
