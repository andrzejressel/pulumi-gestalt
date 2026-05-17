#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetExpressRouteCircuitServiceProviderProperty {
    /// The bandwidth in Mbps of the ExpressRoute circuit.
    #[builder(into)]
    #[serde(rename = "bandwidthInMbps")]
    pub r#bandwidth_in_mbps: i32,
    /// The name of the peering location and **not** the Azure resource location.
    #[builder(into)]
    #[serde(rename = "peeringLocation")]
    pub r#peering_location: String,
    /// The name of the ExpressRoute Service Provider.
    #[builder(into)]
    #[serde(rename = "serviceProviderName")]
    pub r#service_provider_name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetExpressRouteCircuitServiceProviderProperty {
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
                    "bandwidth_in_mbps",
                    &self.r#bandwidth_in_mbps,
                ),
                to_pulumi_object_field(
                    "peering_location",
                    &self.r#peering_location,
                ),
                to_pulumi_object_field(
                    "service_provider_name",
                    &self.r#service_provider_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetExpressRouteCircuitServiceProviderProperty {
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
                    r#bandwidth_in_mbps: {
                        let field_value = match fields_map.get("bandwidth_in_mbps") {
                            Some(value) => value,
                            None => bail!("Missing field 'bandwidth_in_mbps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#peering_location: {
                        let field_value = match fields_map.get("peering_location") {
                            Some(value) => value,
                            None => bail!("Missing field 'peering_location' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_provider_name: {
                        let field_value = match fields_map.get("service_provider_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_provider_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
