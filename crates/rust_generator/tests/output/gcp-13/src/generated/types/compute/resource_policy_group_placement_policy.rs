#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ResourcePolicyGroupPlacementPolicy {
    /// The number of availability domains instances will be spread across. If two instances are in different
    /// availability domain, they will not be put in the same low latency network
    #[builder(into)]
    #[serde(rename = "availabilityDomainCount")]
    pub r#availability_domain_count: Option<i32>,
    /// Collocation specifies whether to place VMs inside the same availability domain on the same low-latency network.
    /// Specify `COLLOCATED` to enable collocation. Can only be specified with `vm_count`. If compute instances are created
    /// with a COLLOCATED policy, then exactly `vm_count` instances must be created at the same time with the resource policy
    /// attached.
    /// Possible values are: `COLLOCATED`.
    #[builder(into)]
    #[serde(rename = "collocation")]
    pub r#collocation: Option<String>,
    /// Specifies the number of max logical switches.
    #[builder(into)]
    #[serde(rename = "maxDistance")]
    pub r#max_distance: Option<i32>,
    /// Number of VMs in this placement group. Google does not recommend that you use this field
    /// unless you use a compact policy and you want your policy to work only if it contains this
    /// exact number of VMs.
    #[builder(into)]
    #[serde(rename = "vmCount")]
    pub r#vm_count: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ResourcePolicyGroupPlacementPolicy {
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
                "availability_domain_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#availability_domain_count,
                )
                .await,
            );
            map.insert(
                "collocation".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#collocation,
                )
                .await,
            );
            map.insert(
                "max_distance".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_distance,
                )
                .await,
            );
            map.insert(
                "vm_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#vm_count,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ResourcePolicyGroupPlacementPolicy {
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
                    r#availability_domain_count: {
                        let field_value = match fields_map.get("availability_domain_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'availability_domain_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#collocation: {
                        let field_value = match fields_map.get("collocation") {
                            Some(value) => value,
                            None => bail!("Missing field 'collocation' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_distance: {
                        let field_value = match fields_map.get("max_distance") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_distance' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#vm_count: {
                        let field_value = match fields_map.get("vm_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'vm_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
