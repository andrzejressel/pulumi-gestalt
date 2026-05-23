#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegisteredDomainRegistrantContact {
    /// First line of the contact's address.
    #[builder(into)]
    pub r#address_line_1: Option<String>,
    /// Second line of contact's address, if any.
    #[builder(into)]
    pub r#address_line_2: Option<String>,
    /// The city of the contact's address.
    #[builder(into)]
    pub r#city: Option<String>,
    /// Indicates whether the contact is a person, company, association, or public organization. See the [AWS API documentation](https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_ContactDetail.html#Route53Domains-Type-domains_ContactDetail-ContactType) for valid values.
    #[builder(into)]
    pub r#contact_type: Option<String>,
    /// Code for the country of the contact's address. See the [AWS API documentation](https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_ContactDetail.html#Route53Domains-Type-domains_ContactDetail-CountryCode) for valid values.
    #[builder(into)]
    pub r#country_code: Option<String>,
    /// Email address of the contact.
    #[builder(into)]
    pub r#email: Option<String>,
    /// A key-value map of parameters required by certain top-level domains.
    #[builder(into)]
    pub r#extra_params: Option<std::collections::HashMap<String, String>>,
    /// Fax number of the contact. Phone number must be specified in the format "+[country dialing code].[number including any area code]".
    #[builder(into)]
    pub r#fax: Option<String>,
    /// First name of contact.
    #[builder(into)]
    pub r#first_name: Option<String>,
    /// Last name of contact.
    #[builder(into)]
    pub r#last_name: Option<String>,
    /// Name of the organization for contact types other than `PERSON`.
    #[builder(into)]
    pub r#organization_name: Option<String>,
    /// The phone number of the contact. Phone number must be specified in the format "+[country dialing code].[number including any area code]".
    #[builder(into)]
    pub r#phone_number: Option<String>,
    /// The state or province of the contact's city.
    #[builder(into)]
    pub r#state: Option<String>,
    /// The zip or postal code of the contact's address.
    #[builder(into)]
    pub r#zip_code: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegisteredDomainRegistrantContact {
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
                    "addressLine1",
                    &self.r#address_line_1,
                ),
                to_pulumi_object_field(
                    "addressLine2",
                    &self.r#address_line_2,
                ),
                to_pulumi_object_field(
                    "city",
                    &self.r#city,
                ),
                to_pulumi_object_field(
                    "contactType",
                    &self.r#contact_type,
                ),
                to_pulumi_object_field(
                    "countryCode",
                    &self.r#country_code,
                ),
                to_pulumi_object_field(
                    "email",
                    &self.r#email,
                ),
                to_pulumi_object_field(
                    "extraParams",
                    &self.r#extra_params,
                ),
                to_pulumi_object_field(
                    "fax",
                    &self.r#fax,
                ),
                to_pulumi_object_field(
                    "firstName",
                    &self.r#first_name,
                ),
                to_pulumi_object_field(
                    "lastName",
                    &self.r#last_name,
                ),
                to_pulumi_object_field(
                    "organizationName",
                    &self.r#organization_name,
                ),
                to_pulumi_object_field(
                    "phoneNumber",
                    &self.r#phone_number,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
                to_pulumi_object_field(
                    "zipCode",
                    &self.r#zip_code,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegisteredDomainRegistrantContact {
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
                        let field_value = match fields_map.get("addressLine1") {
                            Some(value) => value,
                            None => bail!("Missing field 'addressLine1' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#address_line_2: {
                        let field_value = match fields_map.get("addressLine2") {
                            Some(value) => value,
                            None => bail!("Missing field 'addressLine2' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("contactType") {
                            Some(value) => value,
                            None => bail!("Missing field 'contactType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#country_code: {
                        let field_value = match fields_map.get("countryCode") {
                            Some(value) => value,
                            None => bail!("Missing field 'countryCode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("extraParams") {
                            Some(value) => value,
                            None => bail!("Missing field 'extraParams' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("firstName") {
                            Some(value) => value,
                            None => bail!("Missing field 'firstName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_name: {
                        let field_value = match fields_map.get("lastName") {
                            Some(value) => value,
                            None => bail!("Missing field 'lastName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organization_name: {
                        let field_value = match fields_map.get("organizationName") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizationName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#phone_number: {
                        let field_value = match fields_map.get("phoneNumber") {
                            Some(value) => value,
                            None => bail!("Missing field 'phoneNumber' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                        let field_value = match fields_map.get("zipCode") {
                            Some(value) => value,
                            None => bail!("Missing field 'zipCode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
