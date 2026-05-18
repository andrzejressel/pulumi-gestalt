#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionInstanceTemplateNetworkInterfaceAliasIpRange {
    /// The IP CIDR range represented by this alias IP range. This IP CIDR range
    /// must belong to the specified subnetwork and cannot contain IP addresses reserved by
    /// system or used by other network interfaces. At the time of writing only a
    /// netmask (e.g. /24) may be supplied, with a CIDR format resulting in an API
    /// error.
    #[builder(into)]
    #[serde(rename = "ipCidrRange")]
    pub r#ip_cidr_range: String,
    /// The subnetwork secondary range name specifying
    /// the secondary range from which to allocate the IP CIDR range for this alias IP
    /// range. If left unspecified, the primary range of the subnetwork will be used.
    #[builder(into)]
    #[serde(rename = "subnetworkRangeName")]
    pub r#subnetwork_range_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionInstanceTemplateNetworkInterfaceAliasIpRange {
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
                    "ip_cidr_range",
                    &self.r#ip_cidr_range,
                ),
                to_pulumi_object_field(
                    "subnetwork_range_name",
                    &self.r#subnetwork_range_name,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionInstanceTemplateNetworkInterfaceAliasIpRange {
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
                    r#ip_cidr_range: {
                        let field_value = match fields_map.get("ip_cidr_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_cidr_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnetwork_range_name: {
                        let field_value = match fields_map.get("subnetwork_range_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnetwork_range_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
