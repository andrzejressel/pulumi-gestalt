#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ProfileAddress {
    /// The first line of a customer address.
    #[builder(into)]
    #[serde(rename = "address1")]
    pub r#address_1: Option<String>,
    /// The second line of a customer address.
    #[builder(into)]
    #[serde(rename = "address2")]
    pub r#address_2: Option<String>,
    /// The third line of a customer address.
    #[builder(into)]
    #[serde(rename = "address3")]
    pub r#address_3: Option<String>,
    /// The fourth line of a customer address.
    #[builder(into)]
    #[serde(rename = "address4")]
    pub r#address_4: Option<String>,
    /// The city in which a customer lives.
    #[builder(into)]
    #[serde(rename = "city")]
    pub r#city: Option<String>,
    /// The country in which a customer lives.
    #[builder(into)]
    #[serde(rename = "country")]
    pub r#country: Option<String>,
    /// The county in which a customer lives.
    #[builder(into)]
    #[serde(rename = "county")]
    pub r#county: Option<String>,
    /// The postal code of a customer address.
    #[builder(into)]
    #[serde(rename = "postalCode")]
    pub r#postal_code: Option<String>,
    /// The province in which a customer lives.
    #[builder(into)]
    #[serde(rename = "province")]
    pub r#province: Option<String>,
    /// The state in which a customer lives.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
}
