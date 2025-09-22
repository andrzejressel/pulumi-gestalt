#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GenericServiceBasicService {
    /// Labels that specify the resource that emits the monitoring data
    /// which is used for SLO reporting of this `Service`.
    #[builder(into)]
    #[serde(rename = "serviceLabels")]
    pub r#service_labels: Option<std::collections::HashMap<String, String>>,
    /// The type of service that this basic service defines, e.g.
    /// APP_ENGINE service type
    #[builder(into)]
    #[serde(rename = "serviceType")]
    pub r#service_type: Option<String>,
}
