#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamSourceConfig {
    /// MySQL data source configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mysqlSourceConfig")]
    pub r#mysql_source_config: Option<Box<super::super::types::datastream::StreamSourceConfigMysqlSourceConfig>>,
    /// MySQL data source configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oracleSourceConfig")]
    pub r#oracle_source_config: Option<Box<super::super::types::datastream::StreamSourceConfigOracleSourceConfig>>,
    /// PostgreSQL data source configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "postgresqlSourceConfig")]
    pub r#postgresql_source_config: Option<Box<super::super::types::datastream::StreamSourceConfigPostgresqlSourceConfig>>,
    /// Source connection profile resource. Format: projects/{project}/locations/{location}/connectionProfiles/{name}
    #[builder(into)]
    #[serde(rename = "sourceConnectionProfile")]
    pub r#source_connection_profile: String,
    /// SQL Server data source configuration.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sqlServerSourceConfig")]
    pub r#sql_server_source_config: Option<Box<super::super::types::datastream::StreamSourceConfigSqlServerSourceConfig>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StreamSourceConfig {
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
                    "mysql_source_config",
                    &self.r#mysql_source_config,
                ),
                to_pulumi_object_field(
                    "oracle_source_config",
                    &self.r#oracle_source_config,
                ),
                to_pulumi_object_field(
                    "postgresql_source_config",
                    &self.r#postgresql_source_config,
                ),
                to_pulumi_object_field(
                    "source_connection_profile",
                    &self.r#source_connection_profile,
                ),
                to_pulumi_object_field(
                    "sql_server_source_config",
                    &self.r#sql_server_source_config,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StreamSourceConfig {
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
                    r#mysql_source_config: {
                        let field_value = match fields_map.get("mysql_source_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'mysql_source_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oracle_source_config: {
                        let field_value = match fields_map.get("oracle_source_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'oracle_source_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#postgresql_source_config: {
                        let field_value = match fields_map.get("postgresql_source_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'postgresql_source_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_connection_profile: {
                        let field_value = match fields_map.get("source_connection_profile") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_connection_profile' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_server_source_config: {
                        let field_value = match fields_map.get("sql_server_source_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'sql_server_source_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
