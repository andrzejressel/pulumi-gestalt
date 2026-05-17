#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct UptimeCheckConfigHttpCheck {
    /// If present, the check will only pass if the HTTP response status code is in this set of status codes. If empty, the HTTP status code will only pass if the HTTP status code is 200-299.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "acceptedResponseStatusCodes")]
    pub r#accepted_response_status_codes: Option<Vec<super::super::types::monitoring::UptimeCheckConfigHttpCheckAcceptedResponseStatusCode>>,
    /// The authentication information using username and password. Optional when creating an HTTP check; defaults to empty. Do not use with other authentication fields.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "authInfo")]
    pub r#auth_info: Option<Box<super::super::types::monitoring::UptimeCheckConfigHttpCheckAuthInfo>>,
    /// The request body associated with the HTTP POST request. If `content_type` is `URL_ENCODED`, the body passed in must be URL-encoded. Users can provide a `Content-Length` header via the `headers` field or the API will do so. If the `request_method` is `GET` and `body` is not empty, the API will return an error. The maximum byte size is 1 megabyte. Note - As with all bytes fields JSON representations are base64 encoded. e.g. `foo=bar` in URL-encoded form is `foo%3Dbar` and in base64 encoding is `Zm9vJTI1M0RiYXI=`.
    #[builder(into)]
    #[serde(rename = "body")]
    pub r#body: Option<String>,
    /// The content type to use for the check.
    /// Possible values are: `TYPE_UNSPECIFIED`, `URL_ENCODED`, `USER_PROVIDED`.
    #[builder(into)]
    #[serde(rename = "contentType")]
    pub r#content_type: Option<String>,
    /// A user provided content type header to use for the check. The invalid configurations outlined in the `content_type` field apply to custom_content_type`, as well as the following 1. `content_type` is `URL_ENCODED` and `custom_content_type` is set. 2. `content_type` is `USER_PROVIDED` and `custom_content_type` is not set.
    #[builder(into)]
    #[serde(rename = "customContentType")]
    pub r#custom_content_type: Option<String>,
    /// The list of headers to send as part of the uptime check request. If two headers have the same key and different values, they should be entered as a single header, with the value being a comma-separated list of all the desired values as described in [RFC 2616 (page 31)](https://www.w3.org/Protocols/rfc2616/rfc2616.txt). Entering two separate headers with the same key in a Create call will cause the first to be overwritten by the second. The maximum number of headers allowed is 100.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Option<std::collections::HashMap<String, String>>,
    /// Boolean specifying whether to encrypt the header information. Encryption should be specified for any headers related to authentication that you do not wish to be seen when retrieving the configuration. The server will be responsible for encrypting the headers. On Get/List calls, if `mask_headers` is set to `true` then the headers will be obscured with `******`.
    #[builder(into)]
    #[serde(rename = "maskHeaders")]
    pub r#mask_headers: Option<bool>,
    /// The path to the page to run the check against. Will be combined with the host (specified within the MonitoredResource) and port to construct the full URL. If the provided path does not begin with `/`, a `/` will be prepended automatically. Optional (defaults to `/`).
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Option<String>,
    /// Contains information needed to add pings to an HTTP check.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pingConfig")]
    pub r#ping_config: Option<Box<super::super::types::monitoring::UptimeCheckConfigHttpCheckPingConfig>>,
    /// The port to the page to run the check against. Will be combined with `host` (specified within the `monitored_resource`) and path to construct the full URL. Optional (defaults to 80 without SSL, or 443 with SSL).
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// The HTTP request method to use for the check. If set to `METHOD_UNSPECIFIED` then `request_method` defaults to `GET`.
    /// Default value is `GET`.
    /// Possible values are: `METHOD_UNSPECIFIED`, `GET`, `POST`.
    #[builder(into)]
    #[serde(rename = "requestMethod")]
    pub r#request_method: Option<String>,
    /// The authentication information using the Monitoring Service Agent. Optional when creating an HTTPS check; defaults to empty. Do not use with other authentication fields.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "serviceAgentAuthentication")]
    pub r#service_agent_authentication: Option<Box<super::super::types::monitoring::UptimeCheckConfigHttpCheckServiceAgentAuthentication>>,
    /// If true, use HTTPS instead of HTTP to run the check.
    #[builder(into)]
    #[serde(rename = "useSsl")]
    pub r#use_ssl: Option<bool>,
    /// Boolean specifying whether to include SSL certificate validation as a part of the Uptime check. Only applies to checks where `monitored_resource` is set to `uptime_url`. If `use_ssl` is `false`, setting `validate_ssl` to `true` has no effect.
    #[builder(into)]
    #[serde(rename = "validateSsl")]
    pub r#validate_ssl: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for UptimeCheckConfigHttpCheck {
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
                "accepted_response_status_codes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#accepted_response_status_codes,
                )
                .await,
            );
            map.insert(
                "auth_info".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#auth_info,
                )
                .await,
            );
            map.insert(
                "body".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#body,
                )
                .await,
            );
            map.insert(
                "content_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#content_type,
                )
                .await,
            );
            map.insert(
                "custom_content_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#custom_content_type,
                )
                .await,
            );
            map.insert(
                "headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#headers,
                )
                .await,
            );
            map.insert(
                "mask_headers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#mask_headers,
                )
                .await,
            );
            map.insert(
                "path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#path,
                )
                .await,
            );
            map.insert(
                "ping_config".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ping_config,
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
                "request_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#request_method,
                )
                .await,
            );
            map.insert(
                "service_agent_authentication".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#service_agent_authentication,
                )
                .await,
            );
            map.insert(
                "use_ssl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_ssl,
                )
                .await,
            );
            map.insert(
                "validate_ssl".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#validate_ssl,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for UptimeCheckConfigHttpCheck {
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
                    r#accepted_response_status_codes: {
                        let field_value = match fields_map.get("accepted_response_status_codes") {
                            Some(value) => value,
                            None => bail!("Missing field 'accepted_response_status_codes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#auth_info: {
                        let field_value = match fields_map.get("auth_info") {
                            Some(value) => value,
                            None => bail!("Missing field 'auth_info' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#body: {
                        let field_value = match fields_map.get("body") {
                            Some(value) => value,
                            None => bail!("Missing field 'body' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#content_type: {
                        let field_value = match fields_map.get("content_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'content_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#custom_content_type: {
                        let field_value = match fields_map.get("custom_content_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'custom_content_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#headers: {
                        let field_value = match fields_map.get("headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mask_headers: {
                        let field_value = match fields_map.get("mask_headers") {
                            Some(value) => value,
                            None => bail!("Missing field 'mask_headers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#path: {
                        let field_value = match fields_map.get("path") {
                            Some(value) => value,
                            None => bail!("Missing field 'path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ping_config: {
                        let field_value = match fields_map.get("ping_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'ping_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#request_method: {
                        let field_value = match fields_map.get("request_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'request_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_agent_authentication: {
                        let field_value = match fields_map.get("service_agent_authentication") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_agent_authentication' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_ssl: {
                        let field_value = match fields_map.get("use_ssl") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_ssl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validate_ssl: {
                        let field_value = match fields_map.get("validate_ssl") {
                            Some(value) => value,
                            None => bail!("Missing field 'validate_ssl' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
