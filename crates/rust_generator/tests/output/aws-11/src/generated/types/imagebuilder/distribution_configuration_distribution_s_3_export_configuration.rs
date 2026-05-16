#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionConfigurationDistributionS3ExportConfiguration {
    /// The disk image format of the exported image (`RAW`, `VHD`, or `VMDK`)
    #[builder(into)]
    #[serde(rename = "diskImageFormat")]
    pub r#disk_image_format: String,
    /// The name of the IAM role to use for exporting.
    #[builder(into)]
    #[serde(rename = "roleName")]
    pub r#role_name: String,
    /// The name of the S3 bucket to store the exported image in. The bucket needs to exist before the export configuration is created.
    #[builder(into)]
    #[serde(rename = "s3Bucket")]
    pub r#s_3_bucket: String,
    /// The prefix for the exported image.
    #[builder(into)]
    #[serde(rename = "s3Prefix")]
    pub r#s_3_prefix: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionConfigurationDistributionS3ExportConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("disk_image_format".to_string(), self.r#disk_image_format.to_pulumi_value().await);
            map.insert("role_name".to_string(), self.r#role_name.to_pulumi_value().await);
            map.insert("s_3_bucket".to_string(), self.r#s_3_bucket.to_pulumi_value().await);
            map.insert("s_3_prefix".to_string(), self.r#s_3_prefix.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionConfigurationDistributionS3ExportConfiguration {
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
                    r#disk_image_format: {
                        let field_value = match fields_map.get("disk_image_format") {
                            Some(value) => value,
                            None => bail!("Missing field 'disk_image_format' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#role_name: {
                        let field_value = match fields_map.get("role_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'role_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_bucket: {
                        let field_value = match fields_map.get("s_3_bucket") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_bucket' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#s_3_prefix: {
                        let field_value = match fields_map.get("s_3_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 's_3_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
