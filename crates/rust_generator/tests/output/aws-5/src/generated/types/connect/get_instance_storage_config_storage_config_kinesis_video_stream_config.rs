#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetInstanceStorageConfigStorageConfigKinesisVideoStreamConfig {
    /// The encryption configuration. Documented below.
    #[builder(into)]
    #[serde(rename = "encryptionConfigs")]
    pub r#encryption_configs: Vec<super::super::types::connect::GetInstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig>,
    /// The prefix of the video stream. Minimum length of `1`. Maximum length of `128`. When read from the state, the value returned is `<prefix>-connect-<connect_instance_alias>-contact-` since the API appends additional details to the `prefix`.
    #[builder(into)]
    #[serde(rename = "prefix")]
    pub r#prefix: String,
    /// The number of hours to retain the data in a data store associated with the stream. Minimum value of `0`. Maximum value of `87600`. A value of `0` indicates that the stream does not persist data.
    #[builder(into)]
    #[serde(rename = "retentionPeriodHours")]
    pub r#retention_period_hours: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetInstanceStorageConfigStorageConfigKinesisVideoStreamConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("encryption_configs".to_string(), self.r#encryption_configs.to_pulumi_value().await);
            map.insert("prefix".to_string(), self.r#prefix.to_pulumi_value().await);
            map.insert("retention_period_hours".to_string(), self.r#retention_period_hours.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetInstanceStorageConfigStorageConfigKinesisVideoStreamConfig {
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
                    r#encryption_configs: {
                        let field_value = match fields_map.get("encryption_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'encryption_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Vec<super::super::types::connect::GetInstanceStorageConfigStorageConfigKinesisVideoStreamConfigEncryptionConfig> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#prefix: {
                        let field_value = match fields_map.get("prefix") {
                            Some(value) => value,
                            None => bail!("Missing field 'prefix' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#retention_period_hours: {
                        let field_value = match fields_map.get("retention_period_hours") {
                            Some(value) => value,
                            None => bail!("Missing field 'retention_period_hours' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <i32 as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
