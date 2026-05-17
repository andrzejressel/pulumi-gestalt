#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterHttpProxyConfig {
    /// The proxy address to be used when communicating over HTTP.
    #[builder(into)]
    #[serde(rename = "httpProxy")]
    pub r#http_proxy: Option<String>,
    /// The proxy address to be used when communicating over HTTPS.
    #[builder(into)]
    #[serde(rename = "httpsProxy")]
    pub r#https_proxy: Option<String>,
    #[builder(into)]
    #[serde(rename = "noProxies")]
    pub r#no_proxies: Option<Vec<String>>,
    /// The base64 encoded alternative CA certificate content in PEM format.
    #[builder(into)]
    #[serde(rename = "trustedCa")]
    pub r#trusted_ca: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterHttpProxyConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "http_proxy",
                    &self.r#http_proxy,
                ),
                to_pulumi_object_field(
                    "https_proxy",
                    &self.r#https_proxy,
                ),
                to_pulumi_object_field(
                    "no_proxies",
                    &self.r#no_proxies,
                ),
                to_pulumi_object_field(
                    "trusted_ca",
                    &self.r#trusted_ca,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterHttpProxyConfig {
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
                    r#http_proxy: {
                        let field_value = match fields_map.get("http_proxy") {
                            Some(value) => value,
                            None => bail!("Missing field 'http_proxy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#https_proxy: {
                        let field_value = match fields_map.get("https_proxy") {
                            Some(value) => value,
                            None => bail!("Missing field 'https_proxy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_proxies: {
                        let field_value = match fields_map.get("no_proxies") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_proxies' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#trusted_ca: {
                        let field_value = match fields_map.get("trusted_ca") {
                            Some(value) => value,
                            None => bail!("Missing field 'trusted_ca' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
