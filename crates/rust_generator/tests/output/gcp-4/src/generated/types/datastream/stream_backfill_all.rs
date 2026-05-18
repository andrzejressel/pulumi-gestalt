#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct StreamBackfillAll {
    /// MySQL data source objects to avoid backfilling.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "mysqlExcludedObjects")]
    pub r#mysql_excluded_objects: Option<Box<super::super::types::datastream::StreamBackfillAllMysqlExcludedObjects>>,
    /// PostgreSQL data source objects to avoid backfilling.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "oracleExcludedObjects")]
    pub r#oracle_excluded_objects: Option<Box<super::super::types::datastream::StreamBackfillAllOracleExcludedObjects>>,
    /// PostgreSQL data source objects to avoid backfilling.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "postgresqlExcludedObjects")]
    pub r#postgresql_excluded_objects: Option<Box<super::super::types::datastream::StreamBackfillAllPostgresqlExcludedObjects>>,
    /// SQL Server data source objects to avoid backfilling.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "sqlServerExcludedObjects")]
    pub r#sql_server_excluded_objects: Option<Box<super::super::types::datastream::StreamBackfillAllSqlServerExcludedObjects>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for StreamBackfillAll {
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
                    "mysql_excluded_objects",
                    &self.r#mysql_excluded_objects,
                ),
                to_pulumi_object_field(
                    "oracle_excluded_objects",
                    &self.r#oracle_excluded_objects,
                ),
                to_pulumi_object_field(
                    "postgresql_excluded_objects",
                    &self.r#postgresql_excluded_objects,
                ),
                to_pulumi_object_field(
                    "sql_server_excluded_objects",
                    &self.r#sql_server_excluded_objects,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for StreamBackfillAll {
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
                    r#mysql_excluded_objects: {
                        let field_value = match fields_map.get("mysql_excluded_objects") {
                            Some(value) => value,
                            None => bail!("Missing field 'mysql_excluded_objects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#oracle_excluded_objects: {
                        let field_value = match fields_map.get("oracle_excluded_objects") {
                            Some(value) => value,
                            None => bail!("Missing field 'oracle_excluded_objects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#postgresql_excluded_objects: {
                        let field_value = match fields_map.get("postgresql_excluded_objects") {
                            Some(value) => value,
                            None => bail!("Missing field 'postgresql_excluded_objects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sql_server_excluded_objects: {
                        let field_value = match fields_map.get("sql_server_excluded_objects") {
                            Some(value) => value,
                            None => bail!("Missing field 'sql_server_excluded_objects' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
