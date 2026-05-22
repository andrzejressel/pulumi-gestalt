#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsMediaStoreSettings {
    /// Number of seconds to wait before retrying connection to the flash media server if the connection is lost.
    #[builder(into)]
    #[serde(rename = "connectionRetryInterval")]
    pub r#connection_retry_interval: Option<i32>,
    #[builder(into)]
    #[serde(rename = "filecacheDuration")]
    pub r#filecache_duration: Option<i32>,
    #[builder(into)]
    #[serde(rename = "mediaStoreStorageClass")]
    pub r#media_store_storage_class: Option<String>,
    /// Number of retry attempts.
    #[builder(into)]
    #[serde(rename = "numRetries")]
    pub r#num_retries: Option<i32>,
    /// Number of seconds to wait until a restart is initiated.
    #[builder(into)]
    #[serde(rename = "restartDelay")]
    pub r#restart_delay: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsMediaStoreSettings {
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
                    "connectionRetryInterval",
                    &self.r#connection_retry_interval,
                ),
                to_pulumi_object_field(
                    "filecacheDuration",
                    &self.r#filecache_duration,
                ),
                to_pulumi_object_field(
                    "mediaStoreStorageClass",
                    &self.r#media_store_storage_class,
                ),
                to_pulumi_object_field(
                    "numRetries",
                    &self.r#num_retries,
                ),
                to_pulumi_object_field(
                    "restartDelay",
                    &self.r#restart_delay,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsMediaStoreSettings {
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
                    r#connection_retry_interval: {
                        let field_value = match fields_map.get("connectionRetryInterval") {
                            Some(value) => value,
                            None => bail!("Missing field 'connectionRetryInterval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#filecache_duration: {
                        let field_value = match fields_map.get("filecacheDuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'filecacheDuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#media_store_storage_class: {
                        let field_value = match fields_map.get("mediaStoreStorageClass") {
                            Some(value) => value,
                            None => bail!("Missing field 'mediaStoreStorageClass' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#num_retries: {
                        let field_value = match fields_map.get("numRetries") {
                            Some(value) => value,
                            None => bail!("Missing field 'numRetries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restart_delay: {
                        let field_value = match fields_map.get("restartDelay") {
                            Some(value) => value,
                            None => bail!("Missing field 'restartDelay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
