#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccountBlobProperties {
    /// Is the blob service properties for change feed events enabled? Default to `false`.
    /// 
    /// > **Note:** This field cannot be configured when `kind` is set to `Storage` (V1).
    #[builder(into)]
    #[serde(rename = "changeFeedEnabled")]
    pub r#change_feed_enabled: Option<bool>,
    /// The duration of change feed events retention in days. The possible values are between 1 and 146000 days (400 years). Setting this to null (or omit this in the configuration file) indicates an infinite retention of the change feed.
    /// 
    /// > **Note:** This field cannot be configured when `kind` is set to `Storage` (V1).
    #[builder(into)]
    #[serde(rename = "changeFeedRetentionInDays")]
    pub r#change_feed_retention_in_days: Option<i32>,
    /// A `container_delete_retention_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "containerDeleteRetentionPolicy")]
    pub r#container_delete_retention_policy: Option<Box<super::super::types::storage::AccountBlobPropertiesContainerDeleteRetentionPolicy>>,
    /// A `cors_rule` block as defined below.
    #[builder(into)]
    #[serde(rename = "corsRules")]
    pub r#cors_rules: Option<Vec<super::super::types::storage::AccountBlobPropertiesCorsRule>>,
    /// The API Version which should be used by default for requests to the Data Plane API if an incoming request doesn't specify an API Version.
    #[builder(into)]
    #[serde(rename = "defaultServiceVersion")]
    pub r#default_service_version: Option<String>,
    /// A `delete_retention_policy` block as defined below.
    #[builder(into)]
    #[serde(rename = "deleteRetentionPolicy")]
    pub r#delete_retention_policy: Option<Box<super::super::types::storage::AccountBlobPropertiesDeleteRetentionPolicy>>,
    /// Is the last access time based tracking enabled? Default to `false`.
    /// 
    /// > **Note:** This field cannot be configured when `kind` is set to `Storage` (V1).
    #[builder(into)]
    #[serde(rename = "lastAccessTimeEnabled")]
    pub r#last_access_time_enabled: Option<bool>,
    /// A `restore_policy` block as defined below. This must be used together with `delete_retention_policy` set, `versioning_enabled` and `change_feed_enabled` set to `true`.
    /// 
    /// > **Note:** This field cannot be configured when `kind` is set to `Storage` (V1).
    /// 
    /// > **Note:** `restore_policy` can not be configured when `dns_endpoint_type` is `AzureDnsZone`.
    #[builder(into)]
    #[serde(rename = "restorePolicy")]
    pub r#restore_policy: Option<Box<super::super::types::storage::AccountBlobPropertiesRestorePolicy>>,
    /// Is versioning enabled? Default to `false`.
    /// 
    /// > **Note:** This field cannot be configured when `kind` is set to `Storage` (V1).
    #[builder(into)]
    #[serde(rename = "versioningEnabled")]
    pub r#versioning_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccountBlobProperties {
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
                    "change_feed_enabled",
                    &self.r#change_feed_enabled,
                ),
                to_pulumi_object_field(
                    "change_feed_retention_in_days",
                    &self.r#change_feed_retention_in_days,
                ),
                to_pulumi_object_field(
                    "container_delete_retention_policy",
                    &self.r#container_delete_retention_policy,
                ),
                to_pulumi_object_field(
                    "cors_rules",
                    &self.r#cors_rules,
                ),
                to_pulumi_object_field(
                    "default_service_version",
                    &self.r#default_service_version,
                ),
                to_pulumi_object_field(
                    "delete_retention_policy",
                    &self.r#delete_retention_policy,
                ),
                to_pulumi_object_field(
                    "last_access_time_enabled",
                    &self.r#last_access_time_enabled,
                ),
                to_pulumi_object_field(
                    "restore_policy",
                    &self.r#restore_policy,
                ),
                to_pulumi_object_field(
                    "versioning_enabled",
                    &self.r#versioning_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccountBlobProperties {
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
                    r#change_feed_enabled: {
                        let field_value = match fields_map.get("change_feed_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'change_feed_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#change_feed_retention_in_days: {
                        let field_value = match fields_map.get("change_feed_retention_in_days") {
                            Some(value) => value,
                            None => bail!("Missing field 'change_feed_retention_in_days' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#container_delete_retention_policy: {
                        let field_value = match fields_map.get("container_delete_retention_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_delete_retention_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cors_rules: {
                        let field_value = match fields_map.get("cors_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'cors_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_service_version: {
                        let field_value = match fields_map.get("default_service_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_service_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#delete_retention_policy: {
                        let field_value = match fields_map.get("delete_retention_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'delete_retention_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_access_time_enabled: {
                        let field_value = match fields_map.get("last_access_time_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_access_time_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restore_policy: {
                        let field_value = match fields_map.get("restore_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'restore_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#versioning_enabled: {
                        let field_value = match fields_map.get("versioning_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'versioning_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
