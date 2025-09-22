#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserAddresses {
    /// The country that this address is in.
    #[builder(into)]
    #[serde(rename = "country")]
    pub r#country: Option<String>,
    /// The name that is typically displayed when the address is shown for display.
    #[builder(into)]
    #[serde(rename = "formatted")]
    pub r#formatted: Option<String>,
    /// The address locality.
    #[builder(into)]
    #[serde(rename = "locality")]
    pub r#locality: Option<String>,
    /// The postal code of the address.
    #[builder(into)]
    #[serde(rename = "postalCode")]
    pub r#postal_code: Option<String>,
    /// When `true`, this is the primary address associated with the user.
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Option<bool>,
    /// The region of the address.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    /// The street of the address.
    #[builder(into)]
    #[serde(rename = "streetAddress")]
    pub r#street_address: Option<String>,
    /// The type of address.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}
