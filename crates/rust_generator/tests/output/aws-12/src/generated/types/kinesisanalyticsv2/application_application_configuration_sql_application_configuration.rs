#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApplicationApplicationConfigurationSqlApplicationConfiguration {
    /// The input stream used by the application.
    #[builder(into)]
    #[serde(rename = "input")]
    pub r#input: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationInput>>,
    /// The destination streams used by the application.
    #[builder(into)]
    #[serde(rename = "outputs")]
    pub r#outputs: Option<Vec<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationOutput>>,
    /// The reference data source used by the application.
    #[builder(into)]
    #[serde(rename = "referenceDataSource")]
    pub r#reference_data_source: Option<Box<super::super::types::kinesisanalyticsv2::ApplicationApplicationConfigurationSqlApplicationConfigurationReferenceDataSource>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApplicationApplicationConfigurationSqlApplicationConfiguration {
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
                "input".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#input,
                )
                .await,
            );
            map.insert(
                "outputs".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#outputs,
                )
                .await,
            );
            map.insert(
                "reference_data_source".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reference_data_source,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApplicationApplicationConfigurationSqlApplicationConfiguration {
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
                    r#input: {
                        let field_value = match fields_map.get("input") {
                            Some(value) => value,
                            None => bail!("Missing field 'input' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#outputs: {
                        let field_value = match fields_map.get("outputs") {
                            Some(value) => value,
                            None => bail!("Missing field 'outputs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reference_data_source: {
                        let field_value = match fields_map.get("reference_data_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'reference_data_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
