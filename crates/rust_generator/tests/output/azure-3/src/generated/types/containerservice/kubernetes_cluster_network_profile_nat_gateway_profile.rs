#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct KubernetesClusterNetworkProfileNatGatewayProfile {
    /// The outcome (resource IDs) of the specified arguments.
    #[builder(into)]
    pub r#effective_outbound_ips: Option<Vec<String>>,
    /// Desired outbound flow idle timeout in minutes for the managed nat gateway. Must be between `4` and `120` inclusive. Defaults to `4`.
    #[builder(into)]
    pub r#idle_timeout_in_minutes: Option<i32>,
    /// Count of desired managed outbound IPs for the managed nat gateway. Must be between `1` and `16` inclusive.
    #[builder(into)]
    pub r#managed_outbound_ip_count: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for KubernetesClusterNetworkProfileNatGatewayProfile {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "effectiveOutboundIps",
                    &self.r#effective_outbound_ips,
                ),
                to_pulumi_object_field(
                    "idleTimeoutInMinutes",
                    &self.r#idle_timeout_in_minutes,
                ),
                to_pulumi_object_field(
                    "managedOutboundIpCount",
                    &self.r#managed_outbound_ip_count,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for KubernetesClusterNetworkProfileNatGatewayProfile {
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
                    r#effective_outbound_ips: {
                        let field_value = match fields_map.get("effectiveOutboundIps") {
                            Some(value) => value,
                            None => bail!("Missing field 'effectiveOutboundIps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#idle_timeout_in_minutes: {
                        let field_value = match fields_map.get("idleTimeoutInMinutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'idleTimeoutInMinutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#managed_outbound_ip_count: {
                        let field_value = match fields_map.get("managedOutboundIpCount") {
                            Some(value) => value,
                            None => bail!("Missing field 'managedOutboundIpCount' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
