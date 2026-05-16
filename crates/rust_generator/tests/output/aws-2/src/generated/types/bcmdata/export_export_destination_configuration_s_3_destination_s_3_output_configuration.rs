#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExportExportDestinationConfigurationS3DestinationS3OutputConfiguration {
    /// Compression type for the data export. Valid values `GZIP`, `PARQUET`.
    #[builder(into)]
    #[serde(rename = "compression")]
    pub r#compression: String,
    /// File format for the data export. Valid values `TEXT_OR_CSV` or `PARQUET`.
    #[builder(into)]
    #[serde(rename = "format")]
    pub r#format: String,
    /// Output type for the data export. Valid value `CUSTOM`.
    #[builder(into)]
    #[serde(rename = "outputType")]
    pub r#output_type: String,
    /// The rule to follow when generating a version of the data export file. You have the choice to overwrite the previous version or to be delivered in addition to the previous versions. Overwriting exports can save on Amazon S3 storage costs. Creating new export versions allows you to track the changes in cost and usage data over time. Valid values `CREATE_NEW_REPORT` or `OVERWRITE_REPORT`.
    #[builder(into)]
    #[serde(rename = "overwrite")]
    pub r#overwrite: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExportExportDestinationConfigurationS3DestinationS3OutputConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("compression".to_string(), self.r#compression.to_pulumi_value().await);
            map.insert("format".to_string(), self.r#format.to_pulumi_value().await);
            map.insert("output_type".to_string(), self.r#output_type.to_pulumi_value().await);
            map.insert("overwrite".to_string(), self.r#overwrite.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExportExportDestinationConfigurationS3DestinationS3OutputConfiguration {
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
                    r#compression: {
                        let field_value = match fields_map.get("compression") {
                            Some(value) => value,
                            None => bail!("Missing field 'compression' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#format: {
                        let field_value = match fields_map.get("format") {
                            Some(value) => value,
                            None => bail!("Missing field 'format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#output_type: {
                        let field_value = match fields_map.get("output_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'output_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#overwrite: {
                        let field_value = match fields_map.get("overwrite") {
                            Some(value) => value,
                            None => bail!("Missing field 'overwrite' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
