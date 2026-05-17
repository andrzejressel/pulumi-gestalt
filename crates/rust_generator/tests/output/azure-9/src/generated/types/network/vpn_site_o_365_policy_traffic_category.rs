#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VpnSiteO365PolicyTrafficCategory {
    /// Is allow endpoint enabled? The `Allow` endpoint is required for connectivity to specific O365 services and features, but are not as sensitive to network performance and latency as other endpoint types. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "allowEndpointEnabled")]
    pub r#allow_endpoint_enabled: Option<bool>,
    /// Is default endpoint enabled? The `Default` endpoint represents O365 services and dependencies that do not require any optimization, and can be treated by customer networks as normal Internet bound traffic. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "defaultEndpointEnabled")]
    pub r#default_endpoint_enabled: Option<bool>,
    /// Is optimize endpoint enabled? The `Optimize` endpoint is required for connectivity to every O365 service and represents the O365 scenario that is the most sensitive to network performance, latency, and availability. Defaults to `false`.
    #[builder(into)]
    #[serde(rename = "optimizeEndpointEnabled")]
    pub r#optimize_endpoint_enabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VpnSiteO365PolicyTrafficCategory {
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
                    "allow_endpoint_enabled",
                    &self.r#allow_endpoint_enabled,
                ),
                to_pulumi_object_field(
                    "default_endpoint_enabled",
                    &self.r#default_endpoint_enabled,
                ),
                to_pulumi_object_field(
                    "optimize_endpoint_enabled",
                    &self.r#optimize_endpoint_enabled,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VpnSiteO365PolicyTrafficCategory {
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
                    r#allow_endpoint_enabled: {
                        let field_value = match fields_map.get("allow_endpoint_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_endpoint_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_endpoint_enabled: {
                        let field_value = match fields_map.get("default_endpoint_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'default_endpoint_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#optimize_endpoint_enabled: {
                        let field_value = match fields_map.get("optimize_endpoint_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'optimize_endpoint_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
