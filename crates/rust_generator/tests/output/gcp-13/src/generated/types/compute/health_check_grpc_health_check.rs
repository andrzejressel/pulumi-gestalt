#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HealthCheckGrpcHealthCheck {
    /// The gRPC service name for the health check.
    /// The value of grpcServiceName has the following meanings by convention:
    /// - Empty serviceName means the overall status of all services at the backend.
    /// - Non-empty serviceName means the health of that gRPC service, as defined by the owner of the service.
    /// The grpcServiceName can only be ASCII.
    #[builder(into)]
    #[serde(rename = "grpcServiceName")]
    pub r#grpc_service_name: Option<String>,
    /// The port number for the health check request.
    /// Must be specified if portName and portSpecification are not set
    /// or if port_specification is USE_FIXED_PORT. Valid values are 1 through 65535.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Port name as defined in InstanceGroup#NamedPort#name. If both port and
    /// port_name are defined, port takes precedence.
    #[builder(into)]
    #[serde(rename = "portName")]
    pub r#port_name: Option<String>,
    /// Specifies how port is selected for health checking, can be one of the
    /// following values:
    /// * `USE_FIXED_PORT`: The port number in `port` is used for health checking.
    /// * `USE_NAMED_PORT`: The `portName` is used for health checking.
    /// * `USE_SERVING_PORT`: For NetworkEndpointGroup, the port specified for each
    /// network endpoint is used for health checking. For other backends, the
    /// port or named port specified in the Backend Service is used for health
    /// checking.
    /// If not specified, gRPC health check follows behavior specified in `port` and
    /// `portName` fields.
    /// Possible values are: `USE_FIXED_PORT`, `USE_NAMED_PORT`, `USE_SERVING_PORT`.
    #[builder(into)]
    #[serde(rename = "portSpecification")]
    pub r#port_specification: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HealthCheckGrpcHealthCheck {
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
                "grpc_service_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#grpc_service_name,
                )
                .await,
            );
            map.insert(
                "port".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#port,
                )
                .await,
            );
            map.insert(
                "port_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#port_name,
                )
                .await,
            );
            map.insert(
                "port_specification".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#port_specification,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HealthCheckGrpcHealthCheck {
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
                    r#grpc_service_name: {
                        let field_value = match fields_map.get("grpc_service_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'grpc_service_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port: {
                        let field_value = match fields_map.get("port") {
                            Some(value) => value,
                            None => bail!("Missing field 'port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port_name: {
                        let field_value = match fields_map.get("port_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'port_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#port_specification: {
                        let field_value = match fields_map.get("port_specification") {
                            Some(value) => value,
                            None => bail!("Missing field 'port_specification' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
