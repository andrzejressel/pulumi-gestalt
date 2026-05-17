#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KeystoresAliasesSelfSignedCertSubject {
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
                "common_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#common_name,
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
                "locality".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#locality,
                )
                .await,
            );
            map.insert(
                "org".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#org,
                )
                .await,
            );
            map.insert(
                "org_unit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#org_unit,
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

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KeystoresAliasesSelfSignedCertSubject {
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
                    r#common_name: {
                        let field_value = match fields_map.get("common_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'common_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#locality: {
                        let field_value = match fields_map.get("locality") {
                            Some(value) => value,
                            None => bail!("Missing field 'locality' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#org: {
                        let field_value = match fields_map.get("org") {
                            Some(value) => value,
                            None => bail!("Missing field 'org' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#org_unit: {
                        let field_value = match fields_map.get("org_unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'org_unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
