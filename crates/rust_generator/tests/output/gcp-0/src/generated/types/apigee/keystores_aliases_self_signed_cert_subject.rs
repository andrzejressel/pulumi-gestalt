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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "common_name",
                    &self.r#common_name,
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
                    "locality",
                    &self.r#locality,
                ),
                to_pulumi_object_field(
                    "org",
                    &self.r#org,
                ),
                to_pulumi_object_field(
                    "org_unit",
                    &self.r#org_unit,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
