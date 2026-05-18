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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "constraints_resource",
                    &self.r#constraints_resource,
                ),
                to_pulumi_object_field(
                    "statistics_resource",
                    &self.r#statistics_resource,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
