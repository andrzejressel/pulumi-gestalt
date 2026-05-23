#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionSpark {
    /// Dataproc Metastore Service configuration for the connection.
    /// Structure is documented below.
    #[builder(into)]
    pub r#metastore_service_config: Option<Box<super::super::types::bigquery::ConnectionSparkMetastoreServiceConfig>>,
    /// (Output)
    /// The account ID of the service created for the purpose of this connection.
    #[builder(into)]
    pub r#service_account_id: Option<String>,
    /// Spark History Server configuration for the connection.
    /// Structure is documented below.
    #[builder(into)]
    pub r#spark_history_server_config: Option<Box<super::super::types::bigquery::ConnectionSparkSparkHistoryServerConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionSpark {
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
                    "metastoreServiceConfig",
                    &self.r#metastore_service_config,
                ),
                to_pulumi_object_field(
                    "serviceAccountId",
                    &self.r#service_account_id,
                ),
                to_pulumi_object_field(
                    "sparkHistoryServerConfig",
                    &self.r#spark_history_server_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
                        let field_value = match fields_map.get("metastoreServiceConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'metastoreServiceConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_account_id: {
                        let field_value = match fields_map.get("serviceAccountId") {
                            Some(value) => value,
                            None => bail!("Missing field 'serviceAccountId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#spark_history_server_config: {
                        let field_value = match fields_map.get("sparkHistoryServerConfig") {
                            Some(value) => value,
                            None => bail!("Missing field 'sparkHistoryServerConfig' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
