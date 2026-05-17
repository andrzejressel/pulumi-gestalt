#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DistributionOriginCustomOriginConfig {
    /// HTTP port the custom origin listens on.
    #[builder(into)]
    #[serde(rename = "httpPort")]
    pub r#http_port: i32,
    /// HTTPS port the custom origin listens on.
    #[builder(into)]
    #[serde(rename = "httpsPort")]
    pub r#https_port: i32,
    #[builder(into)]
    #[serde(rename = "originKeepaliveTimeout")]
    pub r#origin_keepalive_timeout: Option<i32>,
    /// Origin protocol policy to apply to your origin. One of `http-only`, `https-only`, or `match-viewer`.
    #[builder(into)]
    #[serde(rename = "originProtocolPolicy")]
    pub r#origin_protocol_policy: String,
    #[builder(into)]
    #[serde(rename = "originReadTimeout")]
    pub r#origin_read_timeout: Option<i32>,
    /// List of SSL/TLS protocols that CloudFront can use when connecting to your origin over HTTPS. Valid values: `SSLv3`, `TLSv1`, `TLSv1.1`, `TLSv1.2`. For more information, see [Minimum Origin SSL Protocol](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesOriginSSLProtocols) in the Amazon CloudFront Developer Guide.
    #[builder(into)]
    #[serde(rename = "originSslProtocols")]
    pub r#origin_ssl_protocols: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DistributionOriginCustomOriginConfig {
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
                    "http_port",
                    &self.r#http_port,
                ),
                to_pulumi_object_field(
                    "https_port",
                    &self.r#https_port,
                ),
                to_pulumi_object_field(
                    "origin_keepalive_timeout",
                    &self.r#origin_keepalive_timeout,
                ),
                to_pulumi_object_field(
                    "origin_protocol_policy",
                    &self.r#origin_protocol_policy,
                ),
                to_pulumi_object_field(
                    "origin_read_timeout",
                    &self.r#origin_read_timeout,
                ),
                to_pulumi_object_field(
                    "origin_ssl_protocols",
                    &self.r#origin_ssl_protocols,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DistributionOriginCustomOriginConfig {
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
                    r#http_port: {
                        let field_value = match fields_map.get("http_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#https_port: {
                        let field_value = match fields_map.get("https_port") {
                            Some(value) => value,
                            None => bail!("Missing field 'https_port' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_keepalive_timeout: {
                        let field_value = match fields_map.get("origin_keepalive_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_keepalive_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_protocol_policy: {
                        let field_value = match fields_map.get("origin_protocol_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_protocol_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_read_timeout: {
                        let field_value = match fields_map.get("origin_read_timeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_read_timeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_ssl_protocols: {
                        let field_value = match fields_map.get("origin_ssl_protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_ssl_protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
