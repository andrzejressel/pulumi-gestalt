#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataCollectionRuleDestinations {
    /// A `azure_monitor_metrics` block as defined above.
    #[builder(into)]
    #[serde(rename = "azureMonitorMetrics")]
    pub r#azure_monitor_metrics: Option<Box<super::super::types::monitoring::DataCollectionRuleDestinationsAzureMonitorMetrics>>,
    /// One or more `event_hub` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "eventHub")]
    pub r#event_hub: Option<Box<super::super::types::monitoring::DataCollectionRuleDestinationsEventHub>>,
    /// One or more `event_hub` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "eventHubDirect")]
    pub r#event_hub_direct: Option<Box<super::super::types::monitoring::DataCollectionRuleDestinationsEventHubDirect>>,
    /// One or more `log_analytics` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "logAnalytics")]
    pub r#log_analytics: Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsLogAnalytic>>,
    /// One or more `monitor_account` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "monitorAccounts")]
    pub r#monitor_accounts: Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsMonitorAccount>>,
    /// One or more `storage_blob_direct` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "storageBlobDirects")]
    pub r#storage_blob_directs: Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsStorageBlobDirect>>,
    /// One or more `storage_blob` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "storageBlobs")]
    pub r#storage_blobs: Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsStorageBlob>>,
    /// One or more `storage_table_direct` blocks as defined below.
    /// 
    /// > **NOTE** `event_hub_direct`, `storage_blob_direct`, and `storage_table_direct` are only available for rules of kind `AgentDirectToStore`.
    /// 
    /// > **NOTE** At least one of `azure_monitor_metrics`, `event_hub`, `event_hub_direct`, `log_analytics`, `monitor_account`, `storage_blob`, `storage_blob_direct`,and `storage_table_direct` blocks must be specified.
    #[builder(into)]
    #[serde(rename = "storageTableDirects")]
    pub r#storage_table_directs: Option<Vec<super::super::types::monitoring::DataCollectionRuleDestinationsStorageTableDirect>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataCollectionRuleDestinations {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "azure_monitor_metrics",
                    &self.r#azure_monitor_metrics,
                ),
                to_pulumi_object_field(
                    "event_hub",
                    &self.r#event_hub,
                ),
                to_pulumi_object_field(
                    "event_hub_direct",
                    &self.r#event_hub_direct,
                ),
                to_pulumi_object_field(
                    "log_analytics",
                    &self.r#log_analytics,
                ),
                to_pulumi_object_field(
                    "monitor_accounts",
                    &self.r#monitor_accounts,
                ),
                to_pulumi_object_field(
                    "storage_blob_directs",
                    &self.r#storage_blob_directs,
                ),
                to_pulumi_object_field(
                    "storage_blobs",
                    &self.r#storage_blobs,
                ),
                to_pulumi_object_field(
                    "storage_table_directs",
                    &self.r#storage_table_directs,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataCollectionRuleDestinations {
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
                    r#azure_monitor_metrics: {
                        let field_value = match fields_map.get("azure_monitor_metrics") {
                            Some(value) => value,
                            None => bail!("Missing field 'azure_monitor_metrics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_hub: {
                        let field_value = match fields_map.get("event_hub") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_hub' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#event_hub_direct: {
                        let field_value = match fields_map.get("event_hub_direct") {
                            Some(value) => value,
                            None => bail!("Missing field 'event_hub_direct' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_analytics: {
                        let field_value = match fields_map.get("log_analytics") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_analytics' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#monitor_accounts: {
                        let field_value = match fields_map.get("monitor_accounts") {
                            Some(value) => value,
                            None => bail!("Missing field 'monitor_accounts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_blob_directs: {
                        let field_value = match fields_map.get("storage_blob_directs") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_blob_directs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_blobs: {
                        let field_value = match fields_map.get("storage_blobs") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_blobs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_table_directs: {
                        let field_value = match fields_map.get("storage_table_directs") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_table_directs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
