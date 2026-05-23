#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BackendServiceBackend {
    /// Specifies the balancing mode for this backend.
    /// For global HTTP(S) or TCP/SSL load balancing, the default is
    /// UTILIZATION. Valid values are UTILIZATION, RATE (for HTTP(S))
    /// and CONNECTION (for TCP/SSL).
    /// See the [Backend Services Overview](https://cloud.google.com/load-balancing/docs/backend-service#balancing-mode)
    /// for an explanation of load balancing modes.
    /// Default value is `UTILIZATION`.
    /// Possible values are: `UTILIZATION`, `RATE`, `CONNECTION`.
    #[builder(into)]
    pub r#balancing_mode: Option<String>,
    /// A multiplier applied to the group's maximum servicing capacity
    /// (based on UTILIZATION, RATE or CONNECTION).
    /// Default value is 1, which means the group will serve up to 100%
    /// of its configured capacity (depending on balancingMode). A
    /// setting of 0 means the group is completely drained, offering
    /// 0% of its available Capacity. Valid range is [0.0,1.0].
    #[builder(into)]
    pub r#capacity_scaler: Option<f64>,
    /// An optional description of this resource.
    /// Provide this property when you create the resource.
    #[builder(into)]
    pub r#description: Option<String>,
    /// The fully-qualified URL of an Instance Group or Network Endpoint
    /// Group resource. In case of instance group this defines the list
    /// of instances that serve traffic. Member virtual machine
    /// instances from each instance group must live in the same zone as
    /// the instance group itself. No two backends in a backend service
    /// are allowed to use same Instance Group resource.
    /// For Network Endpoint Groups this defines list of endpoints. All
    /// endpoints of Network Endpoint Group must be hosted on instances
    /// located in the same zone as the Network Endpoint Group.
    /// Backend services cannot mix Instance Group and
    /// Network Endpoint Group backends.
    /// Note that you must specify an Instance Group or Network Endpoint
    /// Group resource using the fully-qualified URL, rather than a
    /// partial URL.
    #[builder(into)]
    pub r#group: String,
    /// The max number of simultaneous connections for the group. Can
    /// be used with either CONNECTION or UTILIZATION balancing modes.
    /// For CONNECTION mode, either maxConnections or one
    /// of maxConnectionsPerInstance or maxConnectionsPerEndpoint,
    /// as appropriate for group type, must be set.
    #[builder(into)]
    pub r#max_connections: Option<i32>,
    /// The max number of simultaneous connections that a single backend
    /// network endpoint can handle. This is used to calculate the
    /// capacity of the group. Can be used in either CONNECTION or
    /// UTILIZATION balancing modes.
    /// For CONNECTION mode, either
    /// maxConnections or maxConnectionsPerEndpoint must be set.
    #[builder(into)]
    pub r#max_connections_per_endpoint: Option<i32>,
    /// The max number of simultaneous connections that a single
    /// backend instance can handle. This is used to calculate the
    /// capacity of the group. Can be used in either CONNECTION or
    /// UTILIZATION balancing modes.
    /// For CONNECTION mode, either maxConnections or
    /// maxConnectionsPerInstance must be set.
    #[builder(into)]
    pub r#max_connections_per_instance: Option<i32>,
    /// The max requests per second (RPS) of the group.
    /// Can be used with either RATE or UTILIZATION balancing modes,
    /// but required if RATE mode. For RATE mode, either maxRate or one
    /// of maxRatePerInstance or maxRatePerEndpoint, as appropriate for
    /// group type, must be set.
    #[builder(into)]
    pub r#max_rate: Option<i32>,
    /// The max requests per second (RPS) that a single backend network
    /// endpoint can handle. This is used to calculate the capacity of
    /// the group. Can be used in either balancing mode. For RATE mode,
    /// either maxRate or maxRatePerEndpoint must be set.
    #[builder(into)]
    pub r#max_rate_per_endpoint: Option<f64>,
    /// The max requests per second (RPS) that a single backend
    /// instance can handle. This is used to calculate the capacity of
    /// the group. Can be used in either balancing mode. For RATE mode,
    /// either maxRate or maxRatePerInstance must be set.
    #[builder(into)]
    pub r#max_rate_per_instance: Option<f64>,
    /// Used when balancingMode is UTILIZATION. This ratio defines the
    /// CPU utilization target for the group. Valid range is [0.0, 1.0].
    #[builder(into)]
    pub r#max_utilization: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BackendServiceBackend {
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
                    "balancingMode",
                    &self.r#balancing_mode,
                ),
                to_pulumi_object_field(
                    "capacityScaler",
                    &self.r#capacity_scaler,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "group",
                    &self.r#group,
                ),
                to_pulumi_object_field(
                    "maxConnections",
                    &self.r#max_connections,
                ),
                to_pulumi_object_field(
                    "maxConnectionsPerEndpoint",
                    &self.r#max_connections_per_endpoint,
                ),
                to_pulumi_object_field(
                    "maxConnectionsPerInstance",
                    &self.r#max_connections_per_instance,
                ),
                to_pulumi_object_field(
                    "maxRate",
                    &self.r#max_rate,
                ),
                to_pulumi_object_field(
                    "maxRatePerEndpoint",
                    &self.r#max_rate_per_endpoint,
                ),
                to_pulumi_object_field(
                    "maxRatePerInstance",
                    &self.r#max_rate_per_instance,
                ),
                to_pulumi_object_field(
                    "maxUtilization",
                    &self.r#max_utilization,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BackendServiceBackend {
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
                    r#balancing_mode: {
                        let field_value = match fields_map.get("balancingMode") {
                            Some(value) => value,
                            None => bail!("Missing field 'balancingMode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#capacity_scaler: {
                        let field_value = match fields_map.get("capacityScaler") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacityScaler' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#group: {
                        let field_value = match fields_map.get("group") {
                            Some(value) => value,
                            None => bail!("Missing field 'group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_connections: {
                        let field_value = match fields_map.get("maxConnections") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxConnections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_connections_per_endpoint: {
                        let field_value = match fields_map.get("maxConnectionsPerEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxConnectionsPerEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_connections_per_instance: {
                        let field_value = match fields_map.get("maxConnectionsPerInstance") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxConnectionsPerInstance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_rate: {
                        let field_value = match fields_map.get("maxRate") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxRate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_rate_per_endpoint: {
                        let field_value = match fields_map.get("maxRatePerEndpoint") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxRatePerEndpoint' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_rate_per_instance: {
                        let field_value = match fields_map.get("maxRatePerInstance") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxRatePerInstance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_utilization: {
                        let field_value = match fields_map.get("maxUtilization") {
                            Some(value) => value,
                            None => bail!("Missing field 'maxUtilization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
