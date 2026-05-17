#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegistrationContactSettingsTechnicalContact {
    /// Required. Email address of the contact.
    #[builder(into)]
    #[serde(rename = "email")]
    pub r#email: String,
    /// Fax number of the contact in international format. For example, "+1-800-555-0123".
    #[builder(into)]
    #[serde(rename = "faxNumber")]
    pub r#fax_number: Option<String>,
    /// Required. Phone number of the contact in international format. For example, "+1-800-555-0123".
    #[builder(into)]
    #[serde(rename = "phoneNumber")]
    pub r#phone_number: String,
    /// Required. Postal address of the contact.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "postalAddress")]
    pub r#postal_address: Box<super::super::types::clouddomains::RegistrationContactSettingsTechnicalContactPostalAddress>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegistrationContactSettingsTechnicalContact {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "email".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#email,
                )
                .await,
            );
            map.insert(
                "fax_number".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fax_number,
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
                "postal_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#postal_address,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegistrationContactSettingsTechnicalContact {
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
                    r#email: {
                        let field_value = match fields_map.get("email") {
                            Some(value) => value,
                            None => bail!("Missing field 'email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fax_number: {
                        let field_value = match fields_map.get("fax_number") {
                            Some(value) => value,
                            None => bail!("Missing field 'fax_number' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#postal_address: {
                        let field_value = match fields_map.get("postal_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'postal_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
