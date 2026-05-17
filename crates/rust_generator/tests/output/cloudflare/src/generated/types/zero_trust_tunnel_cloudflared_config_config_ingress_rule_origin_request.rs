#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustTunnelCloudflaredConfigConfigIngressRuleOriginRequest {
    /// Access rules for the ingress service.
    #[builder(into)]
    #[serde(rename = "access")]
    pub r#access: Option<Box<super::types::ZeroTrustTunnelCloudflaredConfigConfigIngressRuleOriginRequestAccess>>,
    /// Runs as jump host.
    #[builder(into)]
    #[serde(rename = "bastionMode")]
    pub r#bastion_mode: Option<bool>,
    /// Path to the certificate authority (CA) for the certificate of your origin. This option should be used only if your certificate is not signed by Cloudflare. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "caPool")]
    pub r#ca_pool: Option<String>,
    /// Timeout for establishing a new TCP connection to your origin server. This excludes the time taken to establish TLS, which is controlled by `tlsTimeout`. Defaults to `30s`.
    #[builder(into)]
    #[serde(rename = "connectTimeout")]
    pub r#connect_timeout: Option<String>,
    /// Disables chunked transfer encoding. Useful if you are running a Web Server Gateway Interface (WSGI) server. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "disableChunkedEncoding")]
    pub r#disable_chunked_encoding: Option<bool>,
    /// Enables HTTP/2 support for the origin connection. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "http2Origin")]
    pub r#http_2_origin: Option<bool>,
    /// Sets the HTTP Host header on requests sent to the local service. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "httpHostHeader")]
    pub r#http_host_header: Option<String>,
    /// IP rules for the proxy service.
    #[builder(into)]
    #[serde(rename = "ipRules")]
    pub r#ip_rules: Option<Vec<super::types::ZeroTrustTunnelCloudflaredConfigConfigIngressRuleOriginRequestIpRule>>,
    /// Maximum number of idle keepalive connections between Tunnel and your origin. This does not restrict the total number of concurrent connections. Defaults to `100`.
    #[builder(into)]
    #[serde(rename = "keepAliveConnections")]
    pub r#keep_alive_connections: Option<i32>,
    /// Timeout after which an idle keepalive connection can be discarded. Defaults to `1m30s`.
    #[builder(into)]
    #[serde(rename = "keepAliveTimeout")]
    pub r#keep_alive_timeout: Option<String>,
    /// Disable the “happy eyeballs” algorithm for IPv4/IPv6 fallback if your local network has misconfigured one of the protocols. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "noHappyEyeballs")]
    pub r#no_happy_eyeballs: Option<bool>,
    /// Disables TLS verification of the certificate presented by your origin. Will allow any certificate from the origin to be accepted. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "noTlsVerify")]
    pub r#no_tls_verify: Option<bool>,
    /// Hostname that cloudflared should expect from your origin server certificate. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "originServerName")]
    pub r#origin_server_name: Option<String>,
    /// cloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen address for that proxy. Defaults to `127.0.0.1`.
    #[builder(into)]
    #[serde(rename = "proxyAddress")]
    pub r#proxy_address: Option<String>,
    /// cloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures the listen port for that proxy. If set to zero, an unused port will randomly be chosen. Defaults to `0`.
    #[builder(into)]
    #[serde(rename = "proxyPort")]
    pub r#proxy_port: Option<i32>,
    /// cloudflared starts a proxy server to translate HTTP traffic into TCP when proxying, for example, SSH or RDP. This configures what type of proxy will be started. Available values: `""`, `socks`. Defaults to `""`.
    #[builder(into)]
    #[serde(rename = "proxyType")]
    pub r#proxy_type: Option<String>,
    /// The timeout after which a TCP keepalive packet is sent on a connection between Tunnel and the origin server. Defaults to `30s`.
    #[builder(into)]
    #[serde(rename = "tcpKeepAlive")]
    pub r#tcp_keep_alive: Option<String>,
    /// Timeout for completing a TLS handshake to your origin server, if you have chosen to connect Tunnel to an HTTPS server. Defaults to `10s`.
    #[builder(into)]
    #[serde(rename = "tlsTimeout")]
    pub r#tls_timeout: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZeroTrustTunnelCloudflaredConfigConfigIngressRuleOriginRequest {
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
                    "access",
                    &self.r#access,
                ),
                to_pulumi_object_field(
                    "bastion_mode",
                    &self.r#bastion_mode,
                ),
                to_pulumi_object_field(
                    "ca_pool",
                    &self.r#ca_pool,
                ),
                to_pulumi_object_field(
                    "connect_timeout",
                    &self.r#connect_timeout,
                ),
                to_pulumi_object_field(
                    "disable_chunked_encoding",
                    &self.r#disable_chunked_encoding,
                ),
                to_pulumi_object_field(
                    "http_2_origin",
                    &self.r#http_2_origin,
                ),
                to_pulumi_object_field(
                    "http_host_header",
                    &self.r#http_host_header,
                ),
                to_pulumi_object_field(
                    "ip_rules",
                    &self.r#ip_rules,
                ),
                to_pulumi_object_field(
                    "keep_alive_connections",
                    &self.r#keep_alive_connections,
                ),
                to_pulumi_object_field(
                    "keep_alive_timeout",
                    &self.r#keep_alive_timeout,
                ),
                to_pulumi_object_field(
                    "no_happy_eyeballs",
                    &self.r#no_happy_eyeballs,
                ),
                to_pulumi_object_field(
                    "no_tls_verify",
                    &self.r#no_tls_verify,
                ),
                to_pulumi_object_field(
                    "origin_server_name",
                    &self.r#origin_server_name,
                ),
                to_pulumi_object_field(
                    "proxy_address",
                    &self.r#proxy_address,
                ),
                to_pulumi_object_field(
                    "proxy_port",
                    &self.r#proxy_port,
                ),
                to_pulumi_object_field(
                    "proxy_type",
                    &self.r#proxy_type,
                ),
                to_pulumi_object_field(
                    "tcp_keep_alive",
                    &self.r#tcp_keep_alive,
                ),
                to_pulumi_object_field(
                    "tls_timeout",
                    &self.r#tls_timeout,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZeroTrustTunnelCloudflaredConfigConfigIngressRuleOriginRequest {
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
                    r#access: {
                        let field_value = match fields_map.get("access") {
                            Some(value) => value,
                            None => bail!("Missing field 'access' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#bastion_mode: {
                        let field_value = match fields_map.get("bastion_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'bastion_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ca_pool: {
                        let field_value = match fields_map.get("ca_pool") {
                            Some(value) => value,
                            None => bail!("Missing field 'ca_pool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#connect_timeout: {
                        let field_value = match fields_map.get("connect_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'connect_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#disable_chunked_encoding: {
                        let field_value = match fields_map.get("disable_chunked_encoding") {
                            Some(value) => value,
                            None => bail!("Missing field 'disable_chunked_encoding' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_2_origin: {
                        let field_value = match fields_map.get("http_2_origin") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_2_origin' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#http_host_header: {
                        let field_value = match fields_map.get("http_host_header") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_host_header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_rules: {
                        let field_value = match fields_map.get("ip_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#keep_alive_connections: {
                        let field_value = match fields_map.get("keep_alive_connections") {
                            Some(value) => value,
                            None => bail!("Missing field 'keep_alive_connections' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#keep_alive_timeout: {
                        let field_value = match fields_map.get("keep_alive_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'keep_alive_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_happy_eyeballs: {
                        let field_value = match fields_map.get("no_happy_eyeballs") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_happy_eyeballs' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_tls_verify: {
                        let field_value = match fields_map.get("no_tls_verify") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_tls_verify' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_server_name: {
                        let field_value = match fields_map.get("origin_server_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_server_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proxy_address: {
                        let field_value = match fields_map.get("proxy_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxy_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proxy_port: {
                        let field_value = match fields_map.get("proxy_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxy_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#proxy_type: {
                        let field_value = match fields_map.get("proxy_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'proxy_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tcp_keep_alive: {
                        let field_value = match fields_map.get("tcp_keep_alive") {
                            Some(value) => value,
                            None => bail!("Missing field 'tcp_keep_alive' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tls_timeout: {
                        let field_value = match fields_map.get("tls_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'tls_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
