#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DataSourceDynamodbConfig {
    /// The DeltaSyncConfig for a versioned data source. See `delta_sync_config` Block for details.
    #[builder(into)]
    #[serde(rename = "deltaSyncConfig")]
    pub r#delta_sync_config: Option<Box<super::super::types::appsync::DataSourceDynamodbConfigDeltaSyncConfig>>,
    /// AWS region of the DynamoDB table. Defaults to current region.
    #[builder(into)]
    #[serde(rename = "region")]
    pub r#region: Option<String>,
    /// Name of the DynamoDB table.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: String,
    /// Set to `true` to use Amazon Cognito credentials with this data source.
    #[builder(into)]
    #[serde(rename = "useCallerCredentials")]
    pub r#use_caller_credentials: Option<bool>,
    /// Detects Conflict Detection and Resolution with this data source.
    #[builder(into)]
    #[serde(rename = "versioned")]
    pub r#versioned: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DataSourceDynamodbConfig {
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
                    "delta_sync_config",
                    &self.r#delta_sync_config,
                ),
                to_pulumi_object_field(
                    "region",
                    &self.r#region,
                ),
                to_pulumi_object_field(
                    "table_name",
                    &self.r#table_name,
                ),
                to_pulumi_object_field(
                    "use_caller_credentials",
                    &self.r#use_caller_credentials,
                ),
                to_pulumi_object_field(
                    "versioned",
                    &self.r#versioned,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DataSourceDynamodbConfig {
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
                    r#delta_sync_config: {
                        let field_value = match fields_map.get("delta_sync_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'delta_sync_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#region: {
                        let field_value = match fields_map.get("region") {
                            Some(value) => value,
                            None => bail!("Missing field 'region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#table_name: {
                        let field_value = match fields_map.get("table_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'table_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_caller_credentials: {
                        let field_value = match fields_map.get("use_caller_credentials") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_caller_credentials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#versioned: {
                        let field_value = match fields_map.get("versioned") {
                            Some(value) => value,
                            None => bail!("Missing field 'versioned' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
