#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue, pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAppIngressCustomDomain {
    /// The Binding type. Possible values include `Disabled` and `SniEnabled`. Defaults to `Disabled`.
    #[builder(into)]
    #[serde(rename = "certificateBindingType")]
    pub r#certificate_binding_type: String,
    /// The ID of the Container App Environment Certificate.
    #[builder(into)]
    #[serde(rename = "certificateId")]
    pub r#certificate_id: String,
    /// The name of the Container App.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}
