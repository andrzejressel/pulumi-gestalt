#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExportExportDestinationConfigurationS3Destination {
    /// Name of the Amazon S3 bucket used as the destination of a data export file.
    #[builder(into)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: String,
    /// Output configuration for the data export. See the `s3_output_configurations` argument reference below.
    #[builder(into)]
    #[serde(rename = "s3OutputConfigurations")]
    pub r#s_3_output_configurations: Option<Vec<super::super::types::bcmdata::ExportExportDestinationConfigurationS3DestinationS3OutputConfiguration>>,
    /// S3 path prefix you want prepended to the name of your data export.
    #[builder(into)]
    #[serde(rename = "s3Prefix")]
    pub r#s_3_prefix: String,
    /// S3 bucket region.
    #[builder(into)]
    #[serde(rename = "s3Region")]
    pub r#s_3_region: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExportExportDestinationConfigurationS3Destination {
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
                "s_3_bucket".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_bucket,
                )
                .await,
            );
            map.insert(
                "s_3_output_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_output_configurations,
                )
                .await,
            );
            map.insert(
                "s_3_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_prefix,
                )
                .await,
            );
            map.insert(
                "s_3_region".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#s_3_region,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ExportExportDestinationConfigurationS3Destination {
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
                    r#s_3_bucket: {
                        let field_value = match fields_map.get("s_3_bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_output_configurations: {
                        let field_value = match fields_map.get("s_3_output_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_output_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_prefix: {
                        let field_value = match fields_map.get("s_3_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_region: {
                        let field_value = match fields_map.get("s_3_region") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
