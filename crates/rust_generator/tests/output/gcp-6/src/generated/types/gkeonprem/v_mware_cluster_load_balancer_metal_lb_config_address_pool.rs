#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct VMwareClusterLoadBalancerMetalLbConfigAddressPool {
    /// The addresses that are part of this pool. Each address
    /// must be either in the CIDR form (1.2.3.0/24) or range
    /// form (1.2.3.1-1.2.3.5).
    #[builder(into)]
    #[serde(rename = "addresses")]
    pub r#addresses: Vec<String>,
    /// If true, avoid using IPs ending in .0 or .255.
    /// This avoids buggy consumer devices mistakenly dropping IPv4 traffic for
    /// those special IP addresses.
    #[builder(into)]
    #[serde(rename = "avoidBuggyIps")]
    pub r#avoid_buggy_ips: Option<bool>,
    /// If true, prevent IP addresses from being automatically assigned.
    /// 
    /// <a name="nested_dataplane_v2"></a>The `dataplane_v2` block supports:
    #[builder(into)]
    #[serde(rename = "manualAssign")]
    pub r#manual_assign: Option<bool>,
    /// The name of the address pool.
    #[builder(into)]
    #[serde(rename = "pool")]
    pub r#pool: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for VMwareClusterLoadBalancerMetalLbConfigAddressPool {
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
                    "addresses",
                    &self.r#addresses,
                ),
                to_pulumi_object_field(
                    "avoid_buggy_ips",
                    &self.r#avoid_buggy_ips,
                ),
                to_pulumi_object_field(
                    "manual_assign",
                    &self.r#manual_assign,
                ),
                to_pulumi_object_field(
                    "pool",
                    &self.r#pool,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for VMwareClusterLoadBalancerMetalLbConfigAddressPool {
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
                    r#addresses: {
                        let field_value = match fields_map.get("addresses") {
                            Some(value) => value,
                            None => bail!("Missing field 'addresses' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#avoid_buggy_ips: {
                        let field_value = match fields_map.get("avoid_buggy_ips") {
                            Some(value) => value,
                            None => bail!("Missing field 'avoid_buggy_ips' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#manual_assign: {
                        let field_value = match fields_map.get("manual_assign") {
                            Some(value) => value,
                            None => bail!("Missing field 'manual_assign' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#pool: {
                        let field_value = match fields_map.get("pool") {
                            Some(value) => value,
                            None => bail!("Missing field 'pool' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
