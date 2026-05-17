#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct WorkspaceFeatureStore {
    /// The version of Spark runtime.
    #[builder(into)]
    #[serde(rename = "computerSparkRuntimeVersion")]
    pub r#computer_spark_runtime_version: Option<String>,
    /// The name of offline store connection.
    #[builder(into)]
    #[serde(rename = "offlineConnectionName")]
    pub r#offline_connection_name: Option<String>,
    /// The name of online store connection.
    /// 
    /// > **Note:** `feature_store` must be set when`kind` is `FeatureStore`
    #[builder(into)]
    #[serde(rename = "onlineConnectionName")]
    pub r#online_connection_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for WorkspaceFeatureStore {
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
                    "computer_spark_runtime_version",
                    &self.r#computer_spark_runtime_version,
                ),
                to_pulumi_object_field(
                    "offline_connection_name",
                    &self.r#offline_connection_name,
                ),
                to_pulumi_object_field(
                    "online_connection_name",
                    &self.r#online_connection_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for WorkspaceFeatureStore {
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
                    r#computer_spark_runtime_version: {
                        let field_value = match fields_map.get("computer_spark_runtime_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'computer_spark_runtime_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#offline_connection_name: {
                        let field_value = match fields_map.get("offline_connection_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'offline_connection_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#online_connection_name: {
                        let field_value = match fields_map.get("online_connection_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'online_connection_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
