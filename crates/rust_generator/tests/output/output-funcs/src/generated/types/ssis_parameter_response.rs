#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SsisParameterResponse {
    /// Parameter type.
    #[builder(into)]
    #[serde(rename = "dataType")]
    pub r#data_type: Option<String>,
    /// Default value of parameter.
    #[builder(into)]
    #[serde(rename = "defaultValue")]
    pub r#default_value: Option<String>,
    /// Parameter description.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// Design default value of parameter.
    #[builder(into)]
    #[serde(rename = "designDefaultValue")]
    pub r#design_default_value: Option<String>,
    /// Parameter id.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<f64>,
    /// Parameter name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Option<String>,
    /// Whether parameter is required.
    #[builder(into)]
    #[serde(rename = "required")]
    pub r#required: Option<bool>,
    /// Whether parameter is sensitive.
    #[builder(into)]
    #[serde(rename = "sensitive")]
    pub r#sensitive: Option<bool>,
    /// Default sensitive value of parameter.
    #[builder(into)]
    #[serde(rename = "sensitiveDefaultValue")]
    pub r#sensitive_default_value: Option<String>,
    /// Parameter value set.
    #[builder(into)]
    #[serde(rename = "valueSet")]
    pub r#value_set: Option<bool>,
    /// Parameter value type.
    #[builder(into)]
    #[serde(rename = "valueType")]
    pub r#value_type: Option<String>,
    /// Parameter reference variable.
    #[builder(into)]
    #[serde(rename = "variable")]
    pub r#variable: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SsisParameterResponse {
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
                "data_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#data_type,
                )
                .await,
            );
            map.insert(
                "default_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#default_value,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "design_default_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#design_default_value,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "required".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#required,
                )
                .await,
            );
            map.insert(
                "sensitive".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sensitive,
                )
                .await,
            );
            map.insert(
                "sensitive_default_value".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sensitive_default_value,
                )
                .await,
            );
            map.insert(
                "value_set".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#value_set,
                )
                .await,
            );
            map.insert(
                "value_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#value_type,
                )
                .await,
            );
            map.insert(
                "variable".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#variable,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SsisParameterResponse {
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
                    r#data_type: {
                        let field_value = match fields_map.get("data_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'data_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_value: {
                        let field_value = match fields_map.get("default_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#design_default_value: {
                        let field_value = match fields_map.get("design_default_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'design_default_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#required: {
                        let field_value = match fields_map.get("required") {
                            Some(value) => value,
                            None => bail!("Missing field 'required' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sensitive: {
                        let field_value = match fields_map.get("sensitive") {
                            Some(value) => value,
                            None => bail!("Missing field 'sensitive' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sensitive_default_value: {
                        let field_value = match fields_map.get("sensitive_default_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'sensitive_default_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value_set: {
                        let field_value = match fields_map.get("value_set") {
                            Some(value) => value,
                            None => bail!("Missing field 'value_set' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#value_type: {
                        let field_value = match fields_map.get("value_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'value_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#variable: {
                        let field_value = match fields_map.get("variable") {
                            Some(value) => value,
                            None => bail!("Missing field 'variable' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
