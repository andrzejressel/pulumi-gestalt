#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RecordGeolocationRoutingPolicy {
    /// A two-letter continent code. See http://docs.aws.amazon.com/Route53/latest/APIReference/API_GetGeoLocation.html for code details. Either `continent` or `country` must be specified.
    #[builder(into)]
    #[serde(rename = "continent")]
    pub r#continent: Option<String>,
    /// A two-character country code or `*` to indicate a default resource record set.
    #[builder(into)]
    #[serde(rename = "country")]
    pub r#country: Option<String>,
    /// A subdivision code for a country.
    #[builder(into)]
    #[serde(rename = "subdivision")]
    pub r#subdivision: Option<String>,
}
