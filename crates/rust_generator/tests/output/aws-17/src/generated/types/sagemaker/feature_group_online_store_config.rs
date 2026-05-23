#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureGroupOnlineStoreConfig {
    /// Set to `true` to disable the automatic creation of an AWS Glue table when configuring an OfflineStore.
    #[builder(into)]
    pub r#enable_online_store: Option<bool>,
    /// Security config for at-rest encryption of your OnlineStore. See Security Config Below.
    #[builder(into)]
    pub r#security_config: Option<Box<super::super::types::sagemaker::FeatureGroupOnlineStoreConfigSecurityConfig>>,
    /// Option for different tiers of low latency storage for real-time data retrieval. Valid values are `Standard`, or `InMemory`.
    #[builder(into)]
    pub r#storage_type: Option<String>,
    /// Time to live duration, where the record is hard deleted after the expiration time is reached; ExpiresAt = EventTime + TtlDuration.. See TTl Duration Below.
    #[builder(into)]
    pub r#ttl_duration: Option<Box<super::super::types::sagemaker::FeatureGroupOnlineStoreConfigTtlDuration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureGroupOnlineStoreConfig {
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
                    "enableOnlineStore",
                    &self.r#enable_online_store,
                ),
                to_pulumi_object_field(
                    "securityConfig",
                    &self.r#security_config,
                ),
                to_pulumi_object_field(
                    "storageType",
                    &self.r#storage_type,
                ),
                to_pulumi_object_field(
                    "ttlDuration",
                    &self.r#ttl_duration,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureGroupOnlineStoreConfig {
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
                    r#enable_online_store: {
                        let field_value = match fields_map.get("enableOnlineStore") {
                            Some(value) => value,
                            None => bail!("Missing field 'enableOnlineStore' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security_config: {
                        let field_value = match fields_map.get("securityConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'securityConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_type: {
                        let field_value = match fields_map.get("storageType") {
                            Some(value) => value,
                            None => bail!("Missing field 'storageType' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ttl_duration: {
                        let field_value = match fields_map.get("ttlDuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'ttlDuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
