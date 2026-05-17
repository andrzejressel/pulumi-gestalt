#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataQualityJobDefinitionDataQualityBaselineConfig {
    /// The constraints resource for a monitoring job. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "constraintsResource")]
    pub r#constraints_resource: Option<Box<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityBaselineConfigConstraintsResource>>,
    /// The statistics resource for a monitoring job. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "statisticsResource")]
    pub r#statistics_resource: Option<Box<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityBaselineConfigStatisticsResource>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataQualityJobDefinitionDataQualityBaselineConfig {
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
                "constraints_resource".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#constraints_resource,
                )
                .await,
            );
            map.insert(
                "statistics_resource".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#statistics_resource,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataQualityJobDefinitionDataQualityBaselineConfig {
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
                    r#constraints_resource: {
                        let field_value = match fields_map.get("constraints_resource") {
                            Some(value) => value,
                            None => bail!("Missing field 'constraints_resource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#statistics_resource: {
                        let field_value = match fields_map.get("statistics_resource") {
                            Some(value) => value,
                            None => bail!("Missing field 'statistics_resource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
