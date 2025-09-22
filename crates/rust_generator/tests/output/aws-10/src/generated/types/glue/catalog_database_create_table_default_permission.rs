#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct CatalogDatabaseCreateTableDefaultPermission {
    /// The permissions that are granted to the principal.
    #[builder(into)]
    #[serde(rename = "permissions")]
    pub r#permissions: Option<Vec<String>>,
    /// The principal who is granted permissions.. See `principal` below.
    #[builder(into)]
    #[serde(rename = "principal")]
    pub r#principal: Box<Option<super::super::types::glue::CatalogDatabaseCreateTableDefaultPermissionPrincipal>>,
}
