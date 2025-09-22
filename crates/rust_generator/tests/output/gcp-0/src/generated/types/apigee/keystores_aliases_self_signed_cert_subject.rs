#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KeystoresAliasesSelfSignedCertSubject {
    /// Common name of the organization. Maximum length is 64 characters.
    #[builder(into)]
    #[serde(rename = "commonName")]
    pub r#common_name: Option<String>,
    /// Two-letter country code. Example, IN for India, US for United States of America.
    #[builder(into)]
    #[serde(rename = "countryCode")]
    pub r#country_code: Option<String>,
    /// Email address. Max 255 characters.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// City or town name. Maximum length is 128 characters.
    #[builder(into)]
    #[serde(rename = "locality")]
    pub r#locality: Option<String>,
    /// Organization name. Maximum length is 64 characters.
    #[builder(into)]
    #[serde(rename = "org")]
    pub r#org: Option<String>,
    /// Organization team name. Maximum length is 64 characters.
    #[builder(into)]
    #[serde(rename = "orgUnit")]
    pub r#org_unit: Option<String>,
    /// State or district name. Maximum length is 128 characters.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}
