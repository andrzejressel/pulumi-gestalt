#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GalleryApplicationVersionManageAction {
    /// The command to install the Gallery Application. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "install")]
    pub r#install: String,
    /// The command to remove the Gallery Application. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "remove")]
    pub r#remove: String,
    /// The command to update the Gallery Application. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "update")]
    pub r#update: Option<String>,
}
