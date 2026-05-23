#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ExportExportDestinationConfigurationS3Destination {
    /// Name of the Amazon S3 bucket used as the destination of a data export file.
    #[builder(into)]
    pub r#s_3_bucket: String,
    /// Output configuration for the data export. See the `s3_output_configurations` argument reference below.
    #[builder(into)]
    pub r#s_3_output_configurations: Option<Vec<super::super::types::bcmdata::ExportExportDestinationConfigurationS3DestinationS3OutputConfiguration>>,
    /// S3 path prefix you want prepended to the name of your data export.
    #[builder(into)]
    pub r#s_3_prefix: String,
    /// S3 bucket region.
    #[builder(into)]
    pub r#s_3_region: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ExportExportDestinationConfigurationS3Destination {
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
                    "s3Bucket",
                    &self.r#s_3_bucket,
                ),
                to_pulumi_object_field(
                    "s3OutputConfigurations",
                    &self.r#s_3_output_configurations,
                ),
                to_pulumi_object_field(
                    "s3Prefix",
                    &self.r#s_3_prefix,
                ),
                to_pulumi_object_field(
                    "s3Region",
                    &self.r#s_3_region,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("s3Bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 's3Bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_output_configurations: {
                        let field_value = match fields_map.get("s3OutputConfigurations") {
                            Some(value) => value,
                            None => bail!("Missing field 's3OutputConfigurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_prefix: {
                        let field_value = match fields_map.get("s3Prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 's3Prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#s_3_region: {
                        let field_value = match fields_map.get("s3Region") {
                            Some(value) => value,
                            None => bail!("Missing field 's3Region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
