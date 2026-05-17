#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPrivateCloudCircuit {
    /// The ID of the ExpressRoute Circuit.
    #[builder(into)]
    #[serde(rename = "expressRouteId")]
    pub r#express_route_id: String,
    /// The ID of the ExpressRoute Circuit private peering.
    #[builder(into)]
    #[serde(rename = "expressRoutePrivatePeeringId")]
    pub r#express_route_private_peering_id: String,
    /// The CIDR of the primary subnet.
    #[builder(into)]
    #[serde(rename = "primarySubnetCidr")]
    pub r#primary_subnet_cidr: String,
    /// The CIDR of the secondary subnet.
    #[builder(into)]
    #[serde(rename = "secondarySubnetCidr")]
    pub r#secondary_subnet_cidr: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPrivateCloudCircuit {
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
                    "express_route_id",
                    &self.r#express_route_id,
                ),
                to_pulumi_object_field(
                    "express_route_private_peering_id",
                    &self.r#express_route_private_peering_id,
                ),
                to_pulumi_object_field(
                    "primary_subnet_cidr",
                    &self.r#primary_subnet_cidr,
                ),
                to_pulumi_object_field(
                    "secondary_subnet_cidr",
                    &self.r#secondary_subnet_cidr,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPrivateCloudCircuit {
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
