#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegisteredDomainBillingContact {
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegisteredDomainBillingContact {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "address_line_1".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#address_line_1,
                )
                .await,
            );
            map.insert(
                "address_line_2".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#address_line_2,
                )
                .await,
            );
            map.insert(
                "city".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#city,
                )
                .await,
            );
            map.insert(
                "contact_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#contact_type,
                )
                .await,
            );
            map.insert(
                "country_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#country_code,
                )
                .await,
            );
            map.insert(
                "email".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email,
                )
                .await,
            );
            map.insert(
                "extra_params".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#extra_params,
                )
                .await,
            );
            map.insert(
                "fax".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fax,
                )
                .await,
            );
            map.insert(
                "first_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#first_name,
                )
                .await,
            );
            map.insert(
                "last_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_name,
                )
                .await,
            );
            map.insert(
                "organization_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#organization_name,
                )
                .await,
            );
            map.insert(
                "phone_number".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#phone_number,
                )
                .await,
            );
            map.insert(
                "state".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#state,
                )
                .await,
            );
            map.insert(
                "zip_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#zip_code,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegisteredDomainBillingContact {
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
