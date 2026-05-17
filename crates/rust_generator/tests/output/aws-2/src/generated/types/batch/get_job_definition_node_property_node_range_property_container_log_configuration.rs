#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetJobDefinitionNodePropertyNodeRangePropertyContainerLogConfiguration {
    /// The log driver to use for the container.
    #[builder(into)]
    #[serde(rename = "logDriver")]
    pub r#log_driver: String,
    /// The configuration options to send to the log driver.
    #[builder(into)]
    #[serde(rename = "options")]
    pub r#options: std::collections::HashMap<String, String>,
    /// The secrets to pass to the log configuration.
    #[builder(into)]
    #[serde(rename = "secretOptions")]
    pub r#secret_options: Vec<super::super::types::batch::GetJobDefinitionNodePropertyNodeRangePropertyContainerLogConfigurationSecretOption>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainerLogConfiguration {
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
                "log_driver".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_driver,
                )
                .await,
            );
            map.insert(
                "options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#options,
                )
                .await,
            );
            map.insert(
                "secret_options".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret_options,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetJobDefinitionNodePropertyNodeRangePropertyContainerLogConfiguration {
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
                    r#log_driver: {
                        let field_value = match fields_map.get("log_driver") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_driver' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#options: {
                        let field_value = match fields_map.get("options") {
                            Some(value) => value,
                            None => bail!("Missing field 'options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret_options: {
                        let field_value = match fields_map.get("secret_options") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret_options' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
