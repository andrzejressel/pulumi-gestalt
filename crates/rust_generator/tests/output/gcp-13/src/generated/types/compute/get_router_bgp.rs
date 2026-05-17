#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRouterBgp {
    /// User-specified flag to indicate which mode to use for advertisement. Default value: "DEFAULT" Possible values: ["DEFAULT", "CUSTOM"]
    #[builder(into)]
    #[serde(rename = "advertiseMode")]
    pub r#advertise_mode: String,
    /// User-specified list of prefix groups to advertise in custom mode.
    /// This field can only be populated if advertiseMode is CUSTOM and
    /// is advertised to all peers of the router. These groups will be
    /// advertised in addition to any specified prefixes. Leave this field
    /// blank to advertise no custom groups.
    /// 
    /// This enum field has the one valid value: ALL_SUBNETS
    #[builder(into)]
    #[serde(rename = "advertisedGroups")]
    pub r#advertised_groups: Vec<String>,
    /// User-specified list of individual IP ranges to advertise in
    /// custom mode. This field can only be populated if advertiseMode
    /// is CUSTOM and is advertised to all peers of the router. These IP
    /// ranges will be advertised in addition to any specified groups.
    /// Leave this field blank to advertise no custom IP ranges.
    #[builder(into)]
    #[serde(rename = "advertisedIpRanges")]
    pub r#advertised_ip_ranges: Vec<super::super::types::compute::GetRouterBgpAdvertisedIpRange>,
    /// Local BGP Autonomous System Number (ASN). Must be an RFC6996
    /// private ASN, either 16-bit or 32-bit. The value will be fixed for
    /// this router resource. All VPN tunnels that link to this router
    /// will have the same local ASN.
    #[builder(into)]
    #[serde(rename = "asn")]
    pub r#asn: i32,
    /// Explicitly specifies a range of valid BGP Identifiers for this Router.
    /// It is provided as a link-local IPv4 range (from 169.254.0.0/16), of
    /// size at least /30, even if the BGP sessions are over IPv6. It must
    /// not overlap with any IPv4 BGP session ranges. Other vendors commonly
    /// call this router ID.
    #[builder(into)]
    #[serde(rename = "identifierRange")]
    pub r#identifier_range: String,
    /// The interval in seconds between BGP keepalive messages that are sent
    /// to the peer. Hold time is three times the interval at which keepalive
    /// messages are sent, and the hold time is the maximum number of seconds
    /// allowed to elapse between successive keepalive messages that BGP
    /// receives from a peer.
    /// 
    /// BGP will use the smaller of either the local hold time value or the
    /// peer's hold time value as the hold time for the BGP connection
    /// between the two peers. If set, this value must be between 20 and 60.
    /// The default is 20.
    #[builder(into)]
    #[serde(rename = "keepaliveInterval")]
    pub r#keepalive_interval: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRouterBgp {
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
                    "advertise_mode",
                    &self.r#advertise_mode,
                ),
                to_pulumi_object_field(
                    "advertised_groups",
                    &self.r#advertised_groups,
                ),
                to_pulumi_object_field(
                    "advertised_ip_ranges",
                    &self.r#advertised_ip_ranges,
                ),
                to_pulumi_object_field(
                    "asn",
                    &self.r#asn,
                ),
                to_pulumi_object_field(
                    "identifier_range",
                    &self.r#identifier_range,
                ),
                to_pulumi_object_field(
                    "keepalive_interval",
                    &self.r#keepalive_interval,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRouterBgp {
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
                    r#advertise_mode: {
                        let field_value = match fields_map.get("advertise_mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'advertise_mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#advertised_groups: {
                        let field_value = match fields_map.get("advertised_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'advertised_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#advertised_ip_ranges: {
                        let field_value = match fields_map.get("advertised_ip_ranges") {
                            Some(value) => value,
                            None => bail!("Missing field 'advertised_ip_ranges' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#asn: {
                        let field_value = match fields_map.get("asn") {
                            Some(value) => value,
                            None => bail!("Missing field 'asn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identifier_range: {
                        let field_value = match fields_map.get("identifier_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'identifier_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#keepalive_interval: {
                        let field_value = match fields_map.get("keepalive_interval") {
                            Some(value) => value,
                            None => bail!("Missing field 'keepalive_interval' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
