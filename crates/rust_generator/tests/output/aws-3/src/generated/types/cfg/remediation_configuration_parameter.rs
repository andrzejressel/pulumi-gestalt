#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RemediationConfigurationParameter {
    /// Name of the attribute.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Value is dynamic and changes at run-time.
    #[builder(into)]
    #[serde(rename = "resourceValue")]
    pub r#resource_value: Option<String>,
    /// Value is static and does not change at run-time.
    #[builder(into)]
    #[serde(rename = "staticValue")]
    pub r#static_value: Option<String>,
    /// List of static values.
    #[builder(into)]
    #[serde(rename = "staticValues")]
    pub r#static_values: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RemediationConfigurationParameter {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("name".to_string(), self.r#name.to_pulumi_value().await);
            map.insert("resource_value".to_string(), self.r#resource_value.to_pulumi_value().await);
            map.insert("static_value".to_string(), self.r#static_value.to_pulumi_value().await);
            map.insert("static_values".to_string(), self.r#static_values.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RemediationConfigurationParameter {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#resource_value: {
                        let field_value = match fields_map.get("resource_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#static_value: {
                        let field_value = match fields_map.get("static_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'static_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#static_values: {
                        let field_value = match fields_map.get("static_values") {
                            Some(value) => value,
                            None => bail!("Missing field 'static_values' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<String>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
