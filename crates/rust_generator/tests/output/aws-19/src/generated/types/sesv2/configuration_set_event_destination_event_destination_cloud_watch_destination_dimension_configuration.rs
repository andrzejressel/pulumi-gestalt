#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration {
    /// The default value of the dimension that is published to Amazon CloudWatch if you don't provide the value of the dimension when you send an email.
    #[builder(into)]
    pub r#default_dimension_value: String,
    /// The name of an Amazon CloudWatch dimension associated with an email sending metric.
    #[builder(into)]
    pub r#dimension_name: String,
    /// The location where the Amazon SES API v2 finds the value of a dimension to publish to Amazon CloudWatch. Valid values: `MESSAGE_TAG`, `EMAIL_HEADER`, `LINK_TAG`.
    #[builder(into)]
    pub r#dimension_value_source: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration {
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
                    "defaultDimensionValue",
                    &self.r#default_dimension_value,
                ),
                to_pulumi_object_field(
                    "dimensionName",
                    &self.r#dimension_name,
                ),
                to_pulumi_object_field(
                    "dimensionValueSource",
                    &self.r#dimension_value_source,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration {
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
                    r#default_dimension_value: {
                        let field_value = match fields_map.get("defaultDimensionValue") {
                            Some(value) => value,
                            None => bail!("Missing field 'defaultDimensionValue' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dimension_name: {
                        let field_value = match fields_map.get("dimensionName") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimensionName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dimension_value_source: {
                        let field_value = match fields_map.get("dimensionValueSource") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimensionValueSource' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
