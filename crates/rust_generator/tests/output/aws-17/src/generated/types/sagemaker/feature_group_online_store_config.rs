#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureGroupOnlineStoreConfig {
    /// Set to `true` to disable the automatic creation of an AWS Glue table when configuring an OfflineStore.
    #[builder(into)]
    #[serde(rename = "enableOnlineStore")]
    pub r#enable_online_store: Option<bool>,
    /// Security config for at-rest encryption of your OnlineStore. See Security Config Below.
    #[builder(into)]
    #[serde(rename = "securityConfig")]
    pub r#security_config: Option<Box<super::super::types::sagemaker::FeatureGroupOnlineStoreConfigSecurityConfig>>,
    /// Option for different tiers of low latency storage for real-time data retrieval. Valid values are `Standard`, or `InMemory`.
    #[builder(into)]
    #[serde(rename = "storageType")]
    pub r#storage_type: Option<String>,
    /// Time to live duration, where the record is hard deleted after the expiration time is reached; ExpiresAt = EventTime + TtlDuration.. See TTl Duration Below.
    #[builder(into)]
    #[serde(rename = "ttlDuration")]
    pub r#ttl_duration: Option<Box<super::super::types::sagemaker::FeatureGroupOnlineStoreConfigTtlDuration>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureGroupOnlineStoreConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("enable_online_store".to_string(), self.r#enable_online_store.to_pulumi_value().await);
            map.insert("security_config".to_string(), self.r#security_config.to_pulumi_value().await);
            map.insert("storage_type".to_string(), self.r#storage_type.to_pulumi_value().await);
            map.insert("ttl_duration".to_string(), self.r#ttl_duration.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureGroupOnlineStoreConfig {
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
                    r#enable_online_store: {
                        let field_value = match fields_map.get("enable_online_store") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_online_store' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<bool> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#security_config: {
                        let field_value = match fields_map.get("security_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'security_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::FeatureGroupOnlineStoreConfigSecurityConfig>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#storage_type: {
                        let field_value = match fields_map.get("storage_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#ttl_duration: {
                        let field_value = match fields_map.get("ttl_duration") {
                            Some(value) => value,
                            None => bail!("Missing field 'ttl_duration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Box<super::super::types::sagemaker::FeatureGroupOnlineStoreConfigTtlDuration>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
