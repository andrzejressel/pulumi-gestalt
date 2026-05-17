#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionHealthCheckSslHealthCheck {
    /// The TCP port number for the HTTP2 health check request.
    /// The default value is 443.
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
    /// If not specified, HTTP2 health check follows behavior specified in `port` and
    /// `portName` fields.
    /// Possible values are: `USE_FIXED_PORT`, `USE_NAMED_PORT`, `USE_SERVING_PORT`.
    #[builder(into)]
    #[serde(rename = "portSpecification")]
    pub r#port_specification: Option<String>,
    /// Specifies the type of proxy header to append before sending data to the
    /// backend.
    /// Default value is `NONE`.
    /// Possible values are: `NONE`, `PROXY_V1`.
    #[builder(into)]
    #[serde(rename = "proxyHeader")]
    pub r#proxy_header: Option<String>,
    /// The application data to send once the SSL connection has been
    /// established (default value is empty). If both request and response are
    /// empty, the connection establishment alone will indicate health. The request
    /// data can only be ASCII.
    #[builder(into)]
    #[serde(rename = "request")]
    pub r#request: Option<String>,
    /// The bytes to match against the beginning of the response data. If left empty
    /// (the default value), any response will indicate health. The response data
    /// can only be ASCII.
    #[builder(into)]
    #[serde(rename = "response")]
    pub r#response: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionHealthCheckSslHealthCheck {
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
            map.insert(
                "proxy_header".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#proxy_header,
                )
                .await,
            );
            map.insert(
                "request".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request,
                )
                .await,
            );
            map.insert(
                "response".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#response,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionHealthCheckSslHealthCheck {
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
                    r#proxy_header: {
                        let field_value = match fields_map.get("proxy_header") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxy_header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#request: {
                        let field_value = match fields_map.get("request") {
                            Some(value) => value,
                            None => bail!("Missing field 'request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#response: {
                        let field_value = match fields_map.get("response") {
                            Some(value) => value,
                            None => bail!("Missing field 'response' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
