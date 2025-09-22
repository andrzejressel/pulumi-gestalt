#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetDomainOffPeakWindowOptionsOffPeakWindowWindowStartTime {
    /// Starting hour of the 10-hour window for updates
    #[builder(into)]
    #[serde(rename = "hours")]
    pub r#hours: i32,
    /// Starting minute of the 10-hour window for updates
    #[builder(into)]
    #[serde(rename = "minutes")]
    pub r#minutes: i32,
}
