#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceStorageConfigStorageConfigS3Config {
    /// The S3 bucket name.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: String,
    /// The S3 bucket prefix.
    #[builder(into)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: String,
    /// The encryption configuration. Documented below.
    #[builder(into)]
    #[serde(rename = "encryptionConfig")]
    pub r#encryption_config: Option<Box<super::super::types::connect::InstanceStorageConfigStorageConfigS3ConfigEncryptionConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceStorageConfigStorageConfigS3Config {
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
                "bucket_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bucket_name,
                )
                .await,
            );
            map.insert(
                "bucket_prefix".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#bucket_prefix,
                )
                .await,
            );
            map.insert(
                "encryption_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#encryption_config,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceStorageConfigStorageConfigS3Config {
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
                    r#bucket_name: {
                        let field_value = match fields_map.get("bucket_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bucket_prefix: {
                        let field_value = match fields_map.get("bucket_prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'bucket_prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#encryption_config: {
                        let field_value = match fields_map.get("encryption_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
