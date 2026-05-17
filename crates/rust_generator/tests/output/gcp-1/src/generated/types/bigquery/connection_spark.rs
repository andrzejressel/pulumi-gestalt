#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionSpark {
    /// Dataproc Metastore Service configuration for the connection.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "metastoreServiceConfig")]
    pub r#metastore_service_config: Option<Box<super::super::types::bigquery::ConnectionSparkMetastoreServiceConfig>>,
    /// (Output)
    /// The account ID of the service created for the purpose of this connection.
    #[builder(into)]
    #[serde(rename = "serviceAccountId")]
    pub r#service_account_id: Option<String>,
    /// Spark History Server configuration for the connection.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sparkHistoryServerConfig")]
    pub r#spark_history_server_config: Option<Box<super::super::types::bigquery::ConnectionSparkSparkHistoryServerConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionSpark {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "metastore_service_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#metastore_service_config,
                )
                .await,
            );
            map.insert(
                "service_account_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_account_id,
                )
                .await,
            );
            map.insert(
                "spark_history_server_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#spark_history_server_config,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionSpark {
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
                    r#metastore_service_config: {
                        let field_value = match fields_map.get("metastore_service_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'metastore_service_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_id: {
                        let field_value = match fields_map.get("service_account_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_account_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spark_history_server_config: {
                        let field_value = match fields_map.get("spark_history_server_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'spark_history_server_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
