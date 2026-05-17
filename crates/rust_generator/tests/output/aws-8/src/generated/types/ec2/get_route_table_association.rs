#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetRouteTableAssociation {
    /// ID of an Internet Gateway or Virtual Private Gateway which is connected to the Route Table (not exported if not passed as a parameter).
    #[builder(into)]
    #[serde(rename = "gatewayId")]
    pub r#gateway_id: String,
    /// Whether the association is due to the main route table.
    #[builder(into)]
    #[serde(rename = "main")]
    pub r#main: bool,
    /// Association ID.
    #[builder(into)]
    #[serde(rename = "routeTableAssociationId")]
    pub r#route_table_association_id: String,
    /// ID of the specific Route Table to retrieve.
    #[builder(into)]
    #[serde(rename = "routeTableId")]
    pub r#route_table_id: String,
    /// ID of a Subnet which is connected to the Route Table (not exported if not passed as a parameter).
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetRouteTableAssociation {
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
                "gateway_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#gateway_id,
                )
                .await,
            );
            map.insert(
                "main".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#main,
                )
                .await,
            );
            map.insert(
                "route_table_association_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#route_table_association_id,
                )
                .await,
            );
            map.insert(
                "route_table_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#route_table_id,
                )
                .await,
            );
            map.insert(
                "subnet_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#subnet_id,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetRouteTableAssociation {
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
                    r#gateway_id: {
                        let field_value = match fields_map.get("gateway_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'gateway_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#main: {
                        let field_value = match fields_map.get("main") {
                            Some(value) => value,
                            None => bail!("Missing field 'main' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_table_association_id: {
                        let field_value = match fields_map.get("route_table_association_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_table_association_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#route_table_id: {
                        let field_value = match fields_map.get("route_table_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'route_table_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#subnet_id: {
                        let field_value = match fields_map.get("subnet_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'subnet_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
