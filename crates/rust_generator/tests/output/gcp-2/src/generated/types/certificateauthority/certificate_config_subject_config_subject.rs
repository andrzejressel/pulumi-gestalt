#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateConfigSubjectConfigSubject {
    /// The common name of the distinguished name.
    #[builder(into)]
    #[serde(rename = "commonName")]
    pub r#common_name: String,
    /// The country code of the subject.
    #[builder(into)]
    #[serde(rename = "countryCode")]
    pub r#country_code: Option<String>,
    /// The locality or city of the subject.
    #[builder(into)]
    #[serde(rename = "locality")]
    pub r#locality: Option<String>,
    /// The organization of the subject.
    #[builder(into)]
    #[serde(rename = "organization")]
    pub r#organization: String,
    /// The organizational unit of the subject.
    #[builder(into)]
    #[serde(rename = "organizationalUnit")]
    pub r#organizational_unit: Option<String>,
    /// The postal code of the subject.
    #[builder(into)]
    #[serde(rename = "postalCode")]
    pub r#postal_code: Option<String>,
    /// The province, territory, or regional state of the subject.
    #[builder(into)]
    #[serde(rename = "province")]
    pub r#province: Option<String>,
    /// The street address of the subject.
    #[builder(into)]
    #[serde(rename = "streetAddress")]
    pub r#street_address: Option<String>,
}
