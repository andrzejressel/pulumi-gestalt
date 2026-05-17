#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct LbTrafficExtensionExtensionChainExtension {
    /// The :authority header in the gRPC request sent from Envoy to the extension service.
    #[builder(into)]
    #[serde(rename = "authority")]
    pub r#authority: Option<String>,
    /// Determines how the proxy behaves if the call to the extension fails or times out.
    /// When set to TRUE, request or response processing continues without error.
    /// Any subsequent extensions in the extension chain are also executed.
    /// When set to FALSE: * If response headers have not been delivered to the downstream client,
    /// a generic 500 error is returned to the client. The error response can be tailored by
    /// configuring a custom error response in the load balancer.
    #[builder(into)]
    #[serde(rename = "failOpen")]
    pub r#fail_open: Option<bool>,
    /// List of the HTTP headers to forward to the extension (from the client or backend).
    /// If omitted, all headers are sent. Each element is a string indicating the header name.
    #[builder(into)]
    #[serde(rename = "forwardHeaders")]
    pub r#forward_headers: Option<Vec<String>>,
    /// The name for this extension. The name is logged as part of the HTTP request logs.
    /// The name must conform with RFC-1034, is restricted to lower-cased letters, numbers and hyphens,
    /// and can have a maximum length of 63 characters. Additionally, the first character must be a letter
    /// and the last a letter or a number.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// The reference to the service that runs the extension. Must be a reference to a backend service
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: String,
    /// A set of events during request or response processing for which this extension is called.
    /// This field is required for the LbTrafficExtension resource. It's not relevant for the LbRouteExtension
    /// resource. Possible values:`EVENT_TYPE_UNSPECIFIED`, `REQUEST_HEADERS`, `REQUEST_BODY`, `RESPONSE_HEADERS`,
    /// `RESPONSE_BODY`, `RESPONSE_BODY` and `RESPONSE_BODY`.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "supportedEvents")]
    pub r#supported_events: Option<Vec<String>>,
    /// Specifies the timeout for each individual message on the stream. The timeout must be between 10-1000 milliseconds.
    /// A duration in seconds with up to nine fractional digits, ending with 's'. Example: "3.5s".
    #[builder(into)]
    #[serde(rename = "timeout")]
    pub r#timeout: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for LbTrafficExtensionExtensionChainExtension {
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
                "authority".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#authority,
                )
                .await,
            );
            map.insert(
                "fail_open".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#fail_open,
                )
                .await,
            );
            map.insert(
                "forward_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#forward_headers,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "service".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service,
                )
                .await,
            );
            map.insert(
                "supported_events".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#supported_events,
                )
                .await,
            );
            map.insert(
                "timeout".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timeout,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for LbTrafficExtensionExtensionChainExtension {
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
                    r#authority: {
                        let field_value = match fields_map.get("authority") {
                            Some(value) => value,
                            None => bail!("Missing field 'authority' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#fail_open: {
                        let field_value = match fields_map.get("fail_open") {
                            Some(value) => value,
                            None => bail!("Missing field 'fail_open' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#forward_headers: {
                        let field_value = match fields_map.get("forward_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'forward_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service: {
                        let field_value = match fields_map.get("service") {
                            Some(value) => value,
                            None => bail!("Missing field 'service' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#supported_events: {
                        let field_value = match fields_map.get("supported_events") {
                            Some(value) => value,
                            None => bail!("Missing field 'supported_events' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timeout: {
                        let field_value = match fields_map.get("timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
