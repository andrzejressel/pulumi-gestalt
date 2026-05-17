#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RouterNatSubnetwork {
    /// Self-link of subnetwork to NAT
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// List of the secondary ranges of the subnetwork that are allowed
    /// to use NAT. This can be populated only if
    /// `LIST_OF_SECONDARY_IP_RANGES` is one of the values in
    /// sourceIpRangesToNat
    #[builder(into)]
    #[serde(rename = "secondaryIpRangeNames")]
    pub r#secondary_ip_range_names: Option<Vec<String>>,
    /// List of options for which source IPs in the subnetwork
    /// should have NAT enabled. Supported values include:
    /// `ALL_IP_RANGES`, `LIST_OF_SECONDARY_IP_RANGES`,
    /// `PRIMARY_IP_RANGE`.
    #[builder(into)]
    #[serde(rename = "sourceIpRangesToNats")]
    pub r#source_ip_ranges_to_nats: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RouterNatSubnetwork {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
                )
                .await,
            );
            map.insert(
                "secondary_ip_range_names".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secondary_ip_range_names,
                )
                .await,
            );
            map.insert(
                "source_ip_ranges_to_nats".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#source_ip_ranges_to_nats,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RouterNatSubnetwork {
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
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secondary_ip_range_names: {
                        let field_value = match fields_map.get("secondary_ip_range_names") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_ip_range_names' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_ip_ranges_to_nats: {
                        let field_value = match fields_map.get("source_ip_ranges_to_nats") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_ip_ranges_to_nats' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
