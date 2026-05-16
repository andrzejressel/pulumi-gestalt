#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration {
    /// The default value of the dimension that is published to Amazon CloudWatch if you don't provide the value of the dimension when you send an email.
    #[builder(into)]
    #[serde(rename = "defaultDimensionValue")]
    pub r#default_dimension_value: String,
    /// The name of an Amazon CloudWatch dimension associated with an email sending metric.
    #[builder(into)]
    #[serde(rename = "dimensionName")]
    pub r#dimension_name: String,
    /// The location where the Amazon SES API v2 finds the value of a dimension to publish to Amazon CloudWatch. Valid values: `MESSAGE_TAG`, `EMAIL_HEADER`, `LINK_TAG`.
    #[builder(into)]
    #[serde(rename = "dimensionValueSource")]
    pub r#dimension_value_source: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("default_dimension_value".to_string(), self.r#default_dimension_value.to_pulumi_value().await);
            map.insert("dimension_name".to_string(), self.r#dimension_name.to_pulumi_value().await);
            map.insert("dimension_value_source".to_string(), self.r#dimension_value_source.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConfigurationSetEventDestinationEventDestinationCloudWatchDestinationDimensionConfiguration {
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
                    r#default_dimension_value: {
                        let field_value = match fields_map.get("default_dimension_value") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_dimension_value' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dimension_name: {
                        let field_value = match fields_map.get("dimension_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimension_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#dimension_value_source: {
                        let field_value = match fields_map.get("dimension_value_source") {
                            Some(value) => value,
                            None => bail!("Missing field 'dimension_value_source' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
