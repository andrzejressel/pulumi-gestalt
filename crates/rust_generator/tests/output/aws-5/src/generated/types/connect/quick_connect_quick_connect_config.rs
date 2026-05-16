#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct QuickConnectQuickConnectConfig {
    /// Specifies the phone configuration of the Quick Connect. This is required only if `quick_connect_type` is `PHONE_NUMBER`. The `phone_config` block is documented below.
    #[builder(into)]
    #[serde(rename = "phoneConfigs")]
    pub r#phone_configs: Option<Vec<super::super::types::connect::QuickConnectQuickConnectConfigPhoneConfig>>,
    /// Specifies the queue configuration of the Quick Connect. This is required only if `quick_connect_type` is `QUEUE`. The `queue_config` block is documented below.
    #[builder(into)]
    #[serde(rename = "queueConfigs")]
    pub r#queue_configs: Option<Vec<super::super::types::connect::QuickConnectQuickConnectConfigQueueConfig>>,
    /// Specifies the configuration type of the quick connect. valid values are `PHONE_NUMBER`, `QUEUE`, `USER`.
    #[builder(into)]
    #[serde(rename = "quickConnectType")]
    pub r#quick_connect_type: String,
    /// Specifies the user configuration of the Quick Connect. This is required only if `quick_connect_type` is `USER`. The `user_config` block is documented below.
    #[builder(into)]
    #[serde(rename = "userConfigs")]
    pub r#user_configs: Option<Vec<super::super::types::connect::QuickConnectQuickConnectConfigUserConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for QuickConnectQuickConnectConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("phone_configs".to_string(), self.r#phone_configs.to_pulumi_value().await);
            map.insert("queue_configs".to_string(), self.r#queue_configs.to_pulumi_value().await);
            map.insert("quick_connect_type".to_string(), self.r#quick_connect_type.to_pulumi_value().await);
            map.insert("user_configs".to_string(), self.r#user_configs.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for QuickConnectQuickConnectConfig {
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
                    r#phone_configs: {
                        let field_value = match fields_map.get("phone_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'phone_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::connect::QuickConnectQuickConnectConfigPhoneConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#queue_configs: {
                        let field_value = match fields_map.get("queue_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'queue_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::connect::QuickConnectQuickConnectConfigQueueConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#quick_connect_type: {
                        let field_value = match fields_map.get("quick_connect_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'quick_connect_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <String as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#user_configs: {
                        let field_value = match fields_map.get("user_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::connect::QuickConnectQuickConnectConfigUserConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
