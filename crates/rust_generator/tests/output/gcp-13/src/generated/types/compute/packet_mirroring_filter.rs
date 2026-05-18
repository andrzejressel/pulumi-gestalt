#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PacketMirroringFilter {
    /// IP CIDR ranges that apply as a filter on the source (ingress) or
    /// destination (egress) IP in the IP header. Only IPv4 is supported.
    #[builder(into)]
    #[serde(rename = "cidrRanges")]
    pub r#cidr_ranges: Option<Vec<String>>,
    /// Direction of traffic to mirror.
    /// Default value is `BOTH`.
    /// Possible values are: `INGRESS`, `EGRESS`, `BOTH`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Option<String>,
    /// Possible IP protocols including tcp, udp, icmp and esp
    #[builder(into)]
    #[serde(rename = "ipProtocols")]
    pub r#ip_protocols: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PacketMirroringFilter {
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
                    "cidr_ranges",
                    &self.r#cidr_ranges,
                ),
                to_pulumi_object_field(
                    "direction",
                    &self.r#direction,
                ),
                to_pulumi_object_field(
                    "ip_protocols",
                    &self.r#ip_protocols,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PacketMirroringFilter {
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
                    r#cidr_ranges: {
                        let field_value = match fields_map.get("cidr_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'cidr_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#direction: {
                        let field_value = match fields_map.get("direction") {
                            Some(value) => value,
                            None => bail!("Missing field 'direction' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_protocols: {
                        let field_value = match fields_map.get("ip_protocols") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_protocols' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
