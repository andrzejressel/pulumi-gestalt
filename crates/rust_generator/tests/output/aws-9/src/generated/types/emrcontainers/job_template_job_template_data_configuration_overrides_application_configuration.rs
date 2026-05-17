#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct JobTemplateJobTemplateDataConfigurationOverridesApplicationConfiguration {
    /// The classification within a configuration.
    #[builder(into)]
    #[serde(rename = "classification")]
    pub r#classification: String,
    /// A list of additional configurations to apply within a configuration object.
    #[builder(into)]
    #[serde(rename = "configurations")]
    pub r#configurations: Option<Vec<super::super::types::emrcontainers::JobTemplateJobTemplateDataConfigurationOverridesApplicationConfigurationConfiguration>>,
    /// A set of properties specified within a configuration classification.
    #[builder(into)]
    #[serde(rename = "properties")]
    pub r#properties: Option<std::collections::HashMap<String, String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for JobTemplateJobTemplateDataConfigurationOverridesApplicationConfiguration {
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
                "classification".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#classification,
                )
                .await,
            );
            map.insert(
                "configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#configurations,
                )
                .await,
            );
            map.insert(
                "properties".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#properties,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for JobTemplateJobTemplateDataConfigurationOverridesApplicationConfiguration {
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
                    r#classification: {
                        let field_value = match fields_map.get("classification") {
                            Some(value) => value,
                            None => bail!("Missing field 'classification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#configurations: {
                        let field_value = match fields_map.get("configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#properties: {
                        let field_value = match fields_map.get("properties") {
                            Some(value) => value,
                            None => bail!("Missing field 'properties' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
