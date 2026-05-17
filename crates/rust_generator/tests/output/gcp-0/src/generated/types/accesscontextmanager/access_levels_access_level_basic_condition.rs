#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AccessLevelsAccessLevelBasicCondition {
    /// Device specific restrictions, all restrictions must hold for
    /// the Condition to be true. If not specified, all devices are
    /// allowed.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "devicePolicy")]
    pub r#device_policy: Option<Box<super::super::types::accesscontextmanager::AccessLevelsAccessLevelBasicConditionDevicePolicy>>,
    /// A list of CIDR block IP subnetwork specification. May be IPv4
    /// or IPv6.
    /// Note that for a CIDR IP address block, the specified IP address
    /// portion must be properly truncated (i.e. all the host bits must
    /// be zero) or the input is considered malformed. For example,
    /// "192.0.2.0/24" is accepted but "192.0.2.1/24" is not. Similarly,
    /// for IPv6, "2001:db8::/32" is accepted whereas "2001:db8::1/32"
    /// is not. The originating IP of a request must be in one of the
    /// listed subnets in order for this Condition to be true.
    /// If empty, all IP addresses are allowed.
    #[builder(into)]
    #[serde(rename = "ipSubnetworks")]
    pub r#ip_subnetworks: Option<Vec<String>>,
    /// An allowed list of members (users, service accounts).
    /// Using groups is not supported yet.
    /// The signed-in user originating the request must be a part of one
    /// of the provided members. If not specified, a request may come
    /// from any user (logged in/not logged in, not present in any
    /// groups, etc.).
    /// Formats: `user:{emailid}`, `serviceAccount:{emailid}`
    #[builder(into)]
    #[serde(rename = "members")]
    pub r#members: Option<Vec<String>>,
    /// Whether to negate the Condition. If true, the Condition becomes
    /// a NAND over its non-empty fields, each field must be false for
    /// the Condition overall to be satisfied. Defaults to false.
    #[builder(into)]
    #[serde(rename = "negate")]
    pub r#negate: Option<bool>,
    /// The request must originate from one of the provided
    /// countries/regions.
    /// Format: A valid ISO 3166-1 alpha-2 code.
    #[builder(into)]
    #[serde(rename = "regions")]
    pub r#regions: Option<Vec<String>>,
    /// A list of other access levels defined in the same Policy,
    /// referenced by resource name. Referencing an AccessLevel which
    /// does not exist is an error. All access levels listed must be
    /// granted for the Condition to be true.
    /// Format: accessPolicies/{policy_id}/accessLevels/{short_name}
    #[builder(into)]
    #[serde(rename = "requiredAccessLevels")]
    pub r#required_access_levels: Option<Vec<String>>,
    /// The request must originate from one of the provided VPC networks in Google Cloud. Cannot specify this field together with `ip_subnetworks`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "vpcNetworkSources")]
    pub r#vpc_network_sources: Option<Vec<super::super::types::accesscontextmanager::AccessLevelsAccessLevelBasicConditionVpcNetworkSource>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AccessLevelsAccessLevelBasicCondition {
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
                "device_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#device_policy,
                )
                .await,
            );
            map.insert(
                "ip_subnetworks".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_subnetworks,
                )
                .await,
            );
            map.insert(
                "members".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#members,
                )
                .await,
            );
            map.insert(
                "negate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#negate,
                )
                .await,
            );
            map.insert(
                "regions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#regions,
                )
                .await,
            );
            map.insert(
                "required_access_levels".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#required_access_levels,
                )
                .await,
            );
            map.insert(
                "vpc_network_sources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vpc_network_sources,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AccessLevelsAccessLevelBasicCondition {
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
                    r#device_policy: {
                        let field_value = match fields_map.get("device_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'device_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#ip_subnetworks: {
                        let field_value = match fields_map.get("ip_subnetworks") {
                            Some(value) => value,
                            None => bail!("Missing field 'ip_subnetworks' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#members: {
                        let field_value = match fields_map.get("members") {
                            Some(value) => value,
                            None => bail!("Missing field 'members' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#negate: {
                        let field_value = match fields_map.get("negate") {
                            Some(value) => value,
                            None => bail!("Missing field 'negate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#regions: {
                        let field_value = match fields_map.get("regions") {
                            Some(value) => value,
                            None => bail!("Missing field 'regions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#required_access_levels: {
                        let field_value = match fields_map.get("required_access_levels") {
                            Some(value) => value,
                            None => bail!("Missing field 'required_access_levels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vpc_network_sources: {
                        let field_value = match fields_map.get("vpc_network_sources") {
                            Some(value) => value,
                            None => bail!("Missing field 'vpc_network_sources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
