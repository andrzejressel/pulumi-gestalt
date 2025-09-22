#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SharedImageGallerySharingCommunityGallery {
    /// The End User Licence Agreement for the Shared Image Gallery. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "eula")]
    pub r#eula: String,
    /// Specifies the name of the Shared Image Gallery. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Prefix of the community public name for the Shared Image Gallery. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: String,
    /// Email of the publisher for the Shared Image Gallery. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publisherEmail")]
    pub r#publisher_email: String,
    /// URI of the publisher for the Shared Image Gallery. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "publisherUri")]
    pub r#publisher_uri: String,
}
