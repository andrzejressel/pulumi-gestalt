#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SubnetworkSecondaryIpRange {
    /// The range of IP addresses belonging to this subnetwork secondary
    /// range. Provide this property when you create the subnetwork.
    /// Ranges must be unique and non-overlapping with all primary and
    /// secondary IP ranges within a network. Only IPv4 is supported.
    /// Field is optional when `reserved_internal_range` is defined, otherwise required.
    #[builder(into)]
    #[serde(rename = "ipCidrRange")]
    pub r#ip_cidr_range: Option<String>,
    /// The name associated with this subnetwork secondary range, used
    /// when adding an alias IP range to a VM instance. The name must
    /// be 1-63 characters long, and comply with RFC1035. The name
    /// must be unique within the subnetwork.
    #[builder(into)]
    #[serde(rename = "rangeName")]
    pub r#range_name: String,
    /// The ID of the reserved internal range. Must be prefixed with `networkconnectivity.googleapis.com`
    /// E.g. `networkconnectivity.googleapis.com/projects/{project}/locations/global/internalRanges/{rangeId}`
    #[builder(into)]
    #[serde(rename = "reservedInternalRange")]
    pub r#reserved_internal_range: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SubnetworkSecondaryIpRange {
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
                "ip_cidr_range".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#ip_cidr_range,
                )
                .await,
            );
            map.insert(
                "range_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#range_name,
                )
                .await,
            );
            map.insert(
                "reserved_internal_range".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reserved_internal_range,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SubnetworkSecondaryIpRange {
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
                    r#range_name: {
                        let field_value = match fields_map.get("range_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'range_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reserved_internal_range: {
                        let field_value = match fields_map.get("reserved_internal_range") {
                            Some(value) => value,
                            None => bail!("Missing field 'reserved_internal_range' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
