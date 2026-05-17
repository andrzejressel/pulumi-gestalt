#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PrivateCloudCircuit {
    /// The ID of the ExpressRoute Circuit.
    #[builder(into)]
    #[serde(rename = "expressRouteId")]
    pub r#express_route_id: Option<String>,
    /// The ID of the ExpressRoute Circuit private peering.
    #[builder(into)]
    #[serde(rename = "expressRoutePrivatePeeringId")]
    pub r#express_route_private_peering_id: Option<String>,
    /// The CIDR of the primary subnet.
    #[builder(into)]
    #[serde(rename = "primarySubnetCidr")]
    pub r#primary_subnet_cidr: Option<String>,
    /// The CIDR of the secondary subnet.
    #[builder(into)]
    #[serde(rename = "secondarySubnetCidr")]
    pub r#secondary_subnet_cidr: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PrivateCloudCircuit {
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
                "express_route_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#express_route_id,
                )
                .await,
            );
            map.insert(
                "express_route_private_peering_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#express_route_private_peering_id,
                )
                .await,
            );
            map.insert(
                "primary_subnet_cidr".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#primary_subnet_cidr,
                )
                .await,
            );
            map.insert(
                "secondary_subnet_cidr".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secondary_subnet_cidr,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PrivateCloudCircuit {
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
                    r#express_route_id: {
                        let field_value = match fields_map.get("express_route_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'express_route_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#express_route_private_peering_id: {
                        let field_value = match fields_map.get("express_route_private_peering_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'express_route_private_peering_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary_subnet_cidr: {
                        let field_value = match fields_map.get("primary_subnet_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary_subnet_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secondary_subnet_cidr: {
                        let field_value = match fields_map.get("secondary_subnet_cidr") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary_subnet_cidr' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
