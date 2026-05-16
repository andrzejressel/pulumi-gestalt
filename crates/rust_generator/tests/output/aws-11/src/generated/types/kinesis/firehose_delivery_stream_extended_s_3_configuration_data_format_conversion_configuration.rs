#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfiguration {
    /// Defaults to `true`. Set it to `false` if you want to disable format conversion while preserving the configuration details.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// Specifies the deserializer that you want Kinesis Data Firehose to use to convert the format of your data from JSON. See `input_format_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "inputFormatConfiguration")]
    pub r#input_format_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationInputFormatConfiguration>,
    /// Specifies the serializer that you want Kinesis Data Firehose to use to convert the format of your data to the Parquet or ORC format. See `output_format_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "outputFormatConfiguration")]
    pub r#output_format_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationOutputFormatConfiguration>,
    /// Specifies the AWS Glue Data Catalog table that contains the column information. See `schema_configuration` block below for details.
    #[builder(into)]
    #[serde(rename = "schemaConfiguration")]
    pub r#schema_configuration: Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationSchemaConfiguration>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("enabled".to_string(), self.r#enabled.to_pulumi_value().await);
            map.insert("input_format_configuration".to_string(), self.r#input_format_configuration.to_pulumi_value().await);
            map.insert("output_format_configuration".to_string(), self.r#output_format_configuration.to_pulumi_value().await);
            map.insert("schema_configuration".to_string(), self.r#schema_configuration.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfiguration {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#input_format_configuration: {
                        let field_value = match fields_map.get("input_format_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'input_format_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationInputFormatConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#output_format_configuration: {
                        let field_value = match fields_map.get("output_format_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_format_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationOutputFormatConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#schema_configuration: {
                        let field_value = match fields_map.get("schema_configuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'schema_configuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Box<super::super::types::kinesis::FirehoseDeliveryStreamExtendedS3ConfigurationDataFormatConversionConfigurationSchemaConfiguration> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
