#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServicePerimetersServicePerimeterStatusVpcAccessibleServices {
    /// The list of APIs usable within the Service Perimeter.
    /// Must be empty unless `enableRestriction` is True.
    #[builder(into)]
    #[serde(rename = "allowedServices")]
    pub r#allowed_services: Option<Vec<String>>,
    /// Whether to restrict API calls within the Service Perimeter to the
    /// list of APIs specified in 'allowedServices'.
    #[builder(into)]
    #[serde(rename = "enableRestriction")]
    pub r#enable_restriction: Option<bool>,
}
