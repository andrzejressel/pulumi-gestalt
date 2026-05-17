#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ZeroTrustTunnelCloudflaredConfigConfig {
    /// Each incoming request received by cloudflared causes cloudflared to send a request to a local service. This section configures the rules that determine which requests are sent to which local services. Last rule must match all requests, e.g `service = "http_status:503"`. [Read more](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/tunnel-guide/local/local-management/ingress/).
    #[builder(into)]
    #[serde(rename = "ingressRules")]
    pub r#ingress_rules: Vec<super::types::ZeroTrustTunnelCloudflaredConfigConfigIngressRule>,
    #[builder(into)]
    #[serde(rename = "originRequest")]
    pub r#origin_request: Option<Box<super::types::ZeroTrustTunnelCloudflaredConfigConfigOriginRequest>>,
    /// If you're exposing a [private network](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/private-net/), you need to add the `warp-routing` key and set it to `true`.
    #[builder(into)]
    #[serde(rename = "warpRouting")]
    pub r#warp_routing: Option<Box<super::types::ZeroTrustTunnelCloudflaredConfigConfigWarpRouting>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ZeroTrustTunnelCloudflaredConfigConfig {
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
                "ingress_rules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ingress_rules,
                )
                .await,
            );
            map.insert(
                "origin_request".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#origin_request,
                )
                .await,
            );
            map.insert(
                "warp_routing".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#warp_routing,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ZeroTrustTunnelCloudflaredConfigConfig {
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
                    r#ingress_rules: {
                        let field_value = match fields_map.get("ingress_rules") {
                            Some(value) => value,
                            None => bail!("Missing field 'ingress_rules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#origin_request: {
                        let field_value = match fields_map.get("origin_request") {
                            Some(value) => value,
                            None => bail!("Missing field 'origin_request' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#warp_routing: {
                        let field_value = match fields_map.get("warp_routing") {
                            Some(value) => value,
                            None => bail!("Missing field 'warp_routing' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
