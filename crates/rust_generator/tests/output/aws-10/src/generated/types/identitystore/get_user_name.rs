#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetUserName {
    /// The family name of the user.
    #[builder(into)]
    #[serde(rename = "familyName")]
    pub r#family_name: String,
    /// The name that is typically displayed when the name is shown for display.
    #[builder(into)]
    #[serde(rename = "formatted")]
    pub r#formatted: String,
    /// The given name of the user.
    #[builder(into)]
    #[serde(rename = "givenName")]
    pub r#given_name: String,
    /// The honorific prefix of the user.
    #[builder(into)]
    #[serde(rename = "honorificPrefix")]
    pub r#honorific_prefix: String,
    /// The honorific suffix of the user.
    #[builder(into)]
    #[serde(rename = "honorificSuffix")]
    pub r#honorific_suffix: String,
    /// The middle name of the user.
    #[builder(into)]
    #[serde(rename = "middleName")]
    pub r#middle_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetUserName {
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
                "family_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#family_name,
                )
                .await,
            );
            map.insert(
                "formatted".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#formatted,
                )
                .await,
            );
            map.insert(
                "given_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#given_name,
                )
                .await,
            );
            map.insert(
                "honorific_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#honorific_prefix,
                )
                .await,
            );
            map.insert(
                "honorific_suffix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#honorific_suffix,
                )
                .await,
            );
            map.insert(
                "middle_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#middle_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetUserName {
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
                    r#family_name: {
                        let field_value = match fields_map.get("family_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'family_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#formatted: {
                        let field_value = match fields_map.get("formatted") {
                            Some(value) => value,
                            None => bail!("Missing field 'formatted' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#given_name: {
                        let field_value = match fields_map.get("given_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'given_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#honorific_prefix: {
                        let field_value = match fields_map.get("honorific_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'honorific_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#honorific_suffix: {
                        let field_value = match fields_map.get("honorific_suffix") {
                            Some(value) => value,
                            None => bail!("Missing field 'honorific_suffix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#middle_name: {
                        let field_value = match fields_map.get("middle_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'middle_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
