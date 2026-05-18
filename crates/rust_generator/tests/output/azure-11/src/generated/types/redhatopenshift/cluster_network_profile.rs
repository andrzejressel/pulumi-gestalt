#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNetworkProfile {
    /// The outbound (egress) routing method. Possible values are `Loadbalancer` and `UserDefinedRouting`. Defaults to `Loadbalancer`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "outboundType")]
    pub r#outbound_type: Option<String>,
    /// The CIDR to use for pod IP addresses. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "podCidr")]
    pub r#pod_cidr: String,
    /// Whether a preconfigured network security group is being used on the subnets.  Defaults to `false`.  Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "preconfiguredNetworkSecurityGroupEnabled")]
    pub r#preconfigured_network_security_group_enabled: Option<bool>,
    /// The network range used by the OpenShift service. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "serviceCidr")]
    pub r#service_cidr: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterNetworkProfile {
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
                    "outbound_type",
                    &self.r#outbound_type,
                ),
                to_pulumi_object_field(
                    "pod_cidr",
                    &self.r#pod_cidr,
                ),
                to_pulumi_object_field(
                    "preconfigured_network_security_group_enabled",
                    &self.r#preconfigured_network_security_group_enabled,
                ),
                to_pulumi_object_field(
                    "service_cidr",
                    &self.r#service_cidr,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterNetworkProfile {
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
                    r#outbound_type: {
                        let field_value = match fields_map.get("outbound_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'outbound_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pod_cidr: {
                        let field_value = match fields_map.get("pod_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'pod_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#preconfigured_network_security_group_enabled: {
                        let field_value = match fields_map.get("preconfigured_network_security_group_enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'preconfigured_network_security_group_enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#service_cidr: {
                        let field_value = match fields_map.get("service_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'service_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
