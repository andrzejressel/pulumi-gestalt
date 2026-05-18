#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResolverRuleTargetIp {
    /// One IP address that you want to forward DNS queries to. You can specify only IPv4 addresses.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Option<String>,
    /// One IPv6 address that you want to forward DNS queries to.
    #[builder(into)]
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Option<String>,
    /// Port at `ip` that you want to forward DNS queries to. Default value is `53`.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Option<i32>,
    /// Protocol for the resolver endpoint. Valid values can be found in the [AWS documentation](https://docs.aws.amazon.com/Route53/latest/APIReference/API_route53resolver_TargetAddress.html). Default value is `Do53`.
    #[builder(into)]
    #[serde(rename = "protocol")]
    pub r#protocol: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResolverRuleTargetIp {
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
                    "ip",
                    &self.r#ip,
                ),
                to_pulumi_object_field(
                    "ipv_6",
                    &self.r#ipv_6,
                ),
                to_pulumi_object_field(
                    "port",
                    &self.r#port,
                ),
                to_pulumi_object_field(
                    "protocol",
                    &self.r#protocol,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResolverRuleTargetIp {
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
                    r#ip: {
                        let field_value = match fields_map.get("ip") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ipv_6: {
                        let field_value = match fields_map.get("ipv_6") {
                            Some(value) => value,
                            None => bail!("Missing field 'ipv_6' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#protocol: {
                        let field_value = match fields_map.get("protocol") {
                            Some(value) => value,
                            None => bail!("Missing field 'protocol' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
