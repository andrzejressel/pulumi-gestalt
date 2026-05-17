#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterDiagnosticsConfig {
    /// The Blob Endpoint of the Storage Account.
    #[builder(into)]
    #[serde(rename = "blobEndpoint")]
    pub r#blob_endpoint: String,
    /// The protected diagnostics storage key name, such as `StorageAccountKey1`.
    #[builder(into)]
    #[serde(rename = "protectedAccountKeyName")]
    pub r#protected_account_key_name: String,
    /// The Queue Endpoint of the Storage Account.
    #[builder(into)]
    #[serde(rename = "queueEndpoint")]
    pub r#queue_endpoint: String,
    /// The name of the Storage Account where the Diagnostics should be sent to.
    #[builder(into)]
    #[serde(rename = "storageAccountName")]
    pub r#storage_account_name: String,
    /// The Table Endpoint of the Storage Account.
    #[builder(into)]
    #[serde(rename = "tableEndpoint")]
    pub r#table_endpoint: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterDiagnosticsConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "blob_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#blob_endpoint,
                )
                .await,
            );
            map.insert(
                "protected_account_key_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#protected_account_key_name,
                )
                .await,
            );
            map.insert(
                "queue_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#queue_endpoint,
                )
                .await,
            );
            map.insert(
                "storage_account_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_account_name,
                )
                .await,
            );
            map.insert(
                "table_endpoint".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#table_endpoint,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterDiagnosticsConfig {
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
                    r#blob_endpoint: {
                        let field_value = match fields_map.get("blob_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'blob_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#protected_account_key_name: {
                        let field_value = match fields_map.get("protected_account_key_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'protected_account_key_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#queue_endpoint: {
                        let field_value = match fields_map.get("queue_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'queue_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_account_name: {
                        let field_value = match fields_map.get("storage_account_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_account_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_endpoint: {
                        let field_value = match fields_map.get("table_endpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'table_endpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
