#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsAkamaiSettings {
    /// Number of seconds to wait before retrying connection to the flash media server if the connection is lost.
    #[builder(into)]
    #[serde(rename = "connectionRetryInterval")]
    pub r#connection_retry_interval: Option<i32>,
    #[builder(into)]
    #[serde(rename = "filecacheDuration")]
    pub r#filecache_duration: Option<i32>,
    #[builder(into)]
    #[serde(rename = "httpTransferMode")]
    pub r#http_transfer_mode: Option<String>,
    /// Number of retry attempts.
    #[builder(into)]
    #[serde(rename = "numRetries")]
    pub r#num_retries: Option<i32>,
    /// Number of seconds to wait until a restart is initiated.
    #[builder(into)]
    #[serde(rename = "restartDelay")]
    pub r#restart_delay: Option<i32>,
    #[builder(into)]
    #[serde(rename = "salt")]
    pub r#salt: Option<String>,
    #[builder(into)]
    #[serde(rename = "token")]
    pub r#token: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsAkamaiSettings {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("connection_retry_interval".to_string(), self.r#connection_retry_interval.to_pulumi_value().await);
            map.insert("filecache_duration".to_string(), self.r#filecache_duration.to_pulumi_value().await);
            map.insert("http_transfer_mode".to_string(), self.r#http_transfer_mode.to_pulumi_value().await);
            map.insert("num_retries".to_string(), self.r#num_retries.to_pulumi_value().await);
            map.insert("restart_delay".to_string(), self.r#restart_delay.to_pulumi_value().await);
            map.insert("salt".to_string(), self.r#salt.to_pulumi_value().await);
            map.insert("token".to_string(), self.r#token.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChannelEncoderSettingsOutputGroupOutputGroupSettingsHlsGroupSettingsHlsCdnSettingHlsAkamaiSettings {
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
                    r#connection_retry_interval: {
                        let field_value = match fields_map.get("connection_retry_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_retry_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#filecache_duration: {
                        let field_value = match fields_map.get("filecache_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'filecache_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#http_transfer_mode: {
                        let field_value = match fields_map.get("http_transfer_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_transfer_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#num_retries: {
                        let field_value = match fields_map.get("num_retries") {
                            Some(value) => value,
                            None => bail!("Missing field 'num_retries' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#restart_delay: {
                        let field_value = match fields_map.get("restart_delay") {
                            Some(value) => value,
                            None => bail!("Missing field 'restart_delay' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#salt: {
                        let field_value = match fields_map.get("salt") {
                            Some(value) => value,
                            None => bail!("Missing field 'salt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#token: {
                        let field_value = match fields_map.get("token") {
                            Some(value) => value,
                            None => bail!("Missing field 'token' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
