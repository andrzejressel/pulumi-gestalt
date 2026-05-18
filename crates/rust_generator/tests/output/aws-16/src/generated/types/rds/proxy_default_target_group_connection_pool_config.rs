#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProxyDefaultTargetGroupConnectionPoolConfig {
    /// The number of seconds for a proxy to wait for a connection to become available in the connection pool. Only applies when the proxy has opened its maximum number of connections and all connections are busy with client sessions.
    #[builder(into)]
    #[serde(rename = "connectionBorrowTimeout")]
    pub r#connection_borrow_timeout: Option<i32>,
    /// One or more SQL statements for the proxy to run when opening each new database connection. Typically used with `SET` statements to make sure that each connection has identical settings such as time zone and character set. This setting is empty by default. For multiple statements, use semicolons as the separator. You can also include multiple variables in a single `SET` statement, such as `SET x=1, y=2`.
    #[builder(into)]
    #[serde(rename = "initQuery")]
    pub r#init_query: Option<String>,
    /// The maximum size of the connection pool for each target in a target group. For Aurora MySQL, it is expressed as a percentage of the max_connections setting for the RDS DB instance or Aurora DB cluster used by the target group.
    #[builder(into)]
    #[serde(rename = "maxConnectionsPercent")]
    pub r#max_connections_percent: Option<i32>,
    /// Controls how actively the proxy closes idle database connections in the connection pool. A high value enables the proxy to leave a high percentage of idle connections open. A low value causes the proxy to close idle client connections and return the underlying database connections to the connection pool. For Aurora MySQL, it is expressed as a percentage of the max_connections setting for the RDS DB instance or Aurora DB cluster used by the target group.
    #[builder(into)]
    #[serde(rename = "maxIdleConnectionsPercent")]
    pub r#max_idle_connections_percent: Option<i32>,
    /// Each item in the list represents a class of SQL operations that normally cause all later statements in a session using a proxy to be pinned to the same underlying database connection. Including an item in the list exempts that class of SQL operations from the pinning behavior. This setting is only supported for MySQL engine family databases. Currently, the only allowed value is `EXCLUDE_VARIABLE_SETS`.
    #[builder(into)]
    #[serde(rename = "sessionPinningFilters")]
    pub r#session_pinning_filters: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProxyDefaultTargetGroupConnectionPoolConfig {
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
                    "connection_borrow_timeout",
                    &self.r#connection_borrow_timeout,
                ),
                to_pulumi_object_field(
                    "init_query",
                    &self.r#init_query,
                ),
                to_pulumi_object_field(
                    "max_connections_percent",
                    &self.r#max_connections_percent,
                ),
                to_pulumi_object_field(
                    "max_idle_connections_percent",
                    &self.r#max_idle_connections_percent,
                ),
                to_pulumi_object_field(
                    "session_pinning_filters",
                    &self.r#session_pinning_filters,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProxyDefaultTargetGroupConnectionPoolConfig {
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
                    r#connection_borrow_timeout: {
                        let field_value = match fields_map.get("connection_borrow_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'connection_borrow_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#init_query: {
                        let field_value = match fields_map.get("init_query") {
                            Some(value) => value,
                            None => bail!("Missing field 'init_query' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_connections_percent: {
                        let field_value = match fields_map.get("max_connections_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_connections_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_idle_connections_percent: {
                        let field_value = match fields_map.get("max_idle_connections_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_idle_connections_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#session_pinning_filters: {
                        let field_value = match fields_map.get("session_pinning_filters") {
                            Some(value) => value,
                            None => bail!("Missing field 'session_pinning_filters' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
