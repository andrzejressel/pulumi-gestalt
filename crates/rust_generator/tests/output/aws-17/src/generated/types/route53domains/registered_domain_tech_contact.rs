#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegisteredDomainTechContact {
    /// First line of the contact's address.
    #[builder(into)]
    #[serde(rename = "addressLine1")]
    pub r#address_line_1: Option<String>,
    /// Second line of contact's address, if any.
    #[builder(into)]
    #[serde(rename = "addressLine2")]
    pub r#address_line_2: Option<String>,
    /// The city of the contact's address.
    #[builder(into)]
    #[serde(rename = "city")]
    pub r#city: Option<String>,
    /// Indicates whether the contact is a person, company, association, or public organization. See the [AWS API documentation](https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_ContactDetail.html#Route53Domains-Type-domains_ContactDetail-ContactType) for valid values.
    #[builder(into)]
    #[serde(rename = "contactType")]
    pub r#contact_type: Option<String>,
    /// Code for the country of the contact's address. See the [AWS API documentation](https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_ContactDetail.html#Route53Domains-Type-domains_ContactDetail-CountryCode) for valid values.
    #[builder(into)]
    #[serde(rename = "countryCode")]
    pub r#country_code: Option<String>,
    /// Email address of the contact.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: Option<String>,
    /// A key-value map of parameters required by certain top-level domains.
    #[builder(into)]
    #[serde(rename = "extraParams")]
    pub r#extra_params: Option<std::collections::HashMap<String, String>>,
    /// Fax number of the contact. Phone number must be specified in the format "+[country dialing code].[number including any area code]".
    #[builder(into)]
    #[serde(rename = "fax")]
    pub r#fax: Option<String>,
    /// First name of contact.
    #[builder(into)]
    #[serde(rename = "firstName")]
    pub r#first_name: Option<String>,
    /// Last name of contact.
    #[builder(into)]
    #[serde(rename = "lastName")]
    pub r#last_name: Option<String>,
    /// Name of the organization for contact types other than `PERSON`.
    #[builder(into)]
    #[serde(rename = "organizationName")]
    pub r#organization_name: Option<String>,
    /// The phone number of the contact. Phone number must be specified in the format "+[country dialing code].[number including any area code]".
    #[builder(into)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: Option<String>,
    /// The state or province of the contact's city.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// The zip or postal code of the contact's address.
    #[builder(into)]
    #[serde(rename = "zipCode")]
    pub r#zip_code: Option<String>,
}
