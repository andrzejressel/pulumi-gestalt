#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetQuickConnectQuickConnectConfig {
    /// Phone configuration of the Quick Connect. This is returned only if `quick_connect_type` is `PHONE_NUMBER`. The `phone_config` block is documented below.
    #[builder(into)]
    #[serde(rename = "phoneConfigs")]
    pub r#phone_configs: Vec<super::super::types::connect::GetQuickConnectQuickConnectConfigPhoneConfig>,
    /// Queue configuration of the Quick Connect. This is returned only if `quick_connect_type` is `QUEUE`. The `queue_config` block is documented below.
    #[builder(into)]
    #[serde(rename = "queueConfigs")]
    pub r#queue_configs: Vec<super::super::types::connect::GetQuickConnectQuickConnectConfigQueueConfig>,
    /// Configuration type of the Quick Connect. Valid values are `PHONE_NUMBER`, `QUEUE`, `USER`.
    #[builder(into)]
    #[serde(rename = "quickConnectType")]
    pub r#quick_connect_type: String,
    /// User configuration of the Quick Connect. This is returned only if `quick_connect_type` is `USER`. The `user_config` block is documented below.
    #[builder(into)]
    #[serde(rename = "userConfigs")]
    pub r#user_configs: Vec<super::super::types::connect::GetQuickConnectQuickConnectConfigUserConfig>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetQuickConnectQuickConnectConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "phone_configs",
                    &self.r#phone_configs,
                ),
                to_pulumi_object_field(
                    "queue_configs",
                    &self.r#queue_configs,
                ),
                to_pulumi_object_field(
                    "quick_connect_type",
                    &self.r#quick_connect_type,
                ),
                to_pulumi_object_field(
                    "user_configs",
                    &self.r#user_configs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetQuickConnectQuickConnectConfig {
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
                    r#phone_configs: {
                        let field_value = match fields_map.get("phone_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'phone_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#queue_configs: {
                        let field_value = match fields_map.get("queue_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'queue_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#quick_connect_type: {
                        let field_value = match fields_map.get("quick_connect_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'quick_connect_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#user_configs: {
                        let field_value = match fields_map.get("user_configs") {
                            Some(value) => value,
                            None => bail!("Missing field 'user_configs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
