#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegisteredDomainAdminContact {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegisteredDomainAdminContact {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "address_line_1",
                    &self.r#address_line_1,
                ),
                to_pulumi_object_field(
                    "address_line_2",
                    &self.r#address_line_2,
                ),
                to_pulumi_object_field(
                    "city",
                    &self.r#city,
                ),
                to_pulumi_object_field(
                    "contact_type",
                    &self.r#contact_type,
                ),
                to_pulumi_object_field(
                    "country_code",
                    &self.r#country_code,
                ),
                to_pulumi_object_field(
                    "email",
                    &self.r#email,
                ),
                to_pulumi_object_field(
                    "extra_params",
                    &self.r#extra_params,
                ),
                to_pulumi_object_field(
                    "fax",
                    &self.r#fax,
                ),
                to_pulumi_object_field(
                    "first_name",
                    &self.r#first_name,
                ),
                to_pulumi_object_field(
                    "last_name",
                    &self.r#last_name,
                ),
                to_pulumi_object_field(
                    "organization_name",
                    &self.r#organization_name,
                ),
                to_pulumi_object_field(
                    "phone_number",
                    &self.r#phone_number,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
                to_pulumi_object_field(
                    "zip_code",
                    &self.r#zip_code,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegisteredDomainAdminContact {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::Result<Self> {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::rootcause::bail;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;

        match value.content {
            PulumiValueContent::Object(ref _obj) => {
                use std::collections::BTreeMap;
                let fields_map: BTreeMap<String, PulumiValue> =
                    _obj.iter().cloned().collect();

                Ok(Self {
                    r#address_line_1: {
                        let field_value = match fields_map.get("address_line_1") {
                            Some(value) => value,
                            None => bail!("Missing field 'address_line_1' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#address_line_2: {
                        let field_value = match fields_map.get("address_line_2") {
                            Some(value) => value,
                            None => bail!("Missing field 'address_line_2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#city: {
                        let field_value = match fields_map.get("city") {
                            Some(value) => value,
                            None => bail!("Missing field 'city' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#contact_type: {
                        let field_value = match fields_map.get("contact_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'contact_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#country_code: {
                        let field_value = match fields_map.get("country_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'country_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#email: {
                        let field_value = match fields_map.get("email") {
                            Some(value) => value,
                            None => bail!("Missing field 'email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#extra_params: {
                        let field_value = match fields_map.get("extra_params") {
                            Some(value) => value,
                            None => bail!("Missing field 'extra_params' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fax: {
                        let field_value = match fields_map.get("fax") {
                            Some(value) => value,
                            None => bail!("Missing field 'fax' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#first_name: {
                        let field_value = match fields_map.get("first_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'first_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_name: {
                        let field_value = match fields_map.get("last_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organization_name: {
                        let field_value = match fields_map.get("organization_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'organization_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#phone_number: {
                        let field_value = match fields_map.get("phone_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'phone_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zip_code: {
                        let field_value = match fields_map.get("zip_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'zip_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
