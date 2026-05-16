#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RecordGeoproximityRoutingPolicy {
    /// A AWS region where the resource is present.
    #[builder(into)]
    #[serde(rename = "awsRegion")]
    pub r#aws_region: Option<String>,
    /// Route more traffic or less traffic to the resource by specifying a value ranges between -90 to 90. See https://docs.aws.amazon.com/Route53/latest/DeveloperGuide/routing-policy-geoproximity.html for bias details.
    #[builder(into)]
    #[serde(rename = "bias")]
    pub r#bias: Option<i32>,
    /// Specify `latitude` and `longitude` for routing traffic to non-AWS resources.
    #[builder(into)]
    #[serde(rename = "coordinates")]
    pub r#coordinates: Option<Vec<super::super::types::route53::RecordGeoproximityRoutingPolicyCoordinate>>,
    /// A AWS local zone group where the resource is present. See https://docs.aws.amazon.com/local-zones/latest/ug/available-local-zones.html for local zone group list.
    #[builder(into)]
    #[serde(rename = "localZoneGroup")]
    pub r#local_zone_group: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RecordGeoproximityRoutingPolicy {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("aws_region".to_string(), self.r#aws_region.to_pulumi_value().await);
            map.insert("bias".to_string(), self.r#bias.to_pulumi_value().await);
            map.insert("coordinates".to_string(), self.r#coordinates.to_pulumi_value().await);
            map.insert("local_zone_group".to_string(), self.r#local_zone_group.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RecordGeoproximityRoutingPolicy {
    fn from_pulumi_value(
        value: &pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    ) -> pulumi_gestalt_rust::__private::rootcause::Result<Self> {
        use std::collections::BTreeMap;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValueContent;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue;
        use pulumi_gestalt_rust::__private::rootcause::bail;

        match value.content {
            PulumiValueContent::Object(ref obj) => {
                let fields_map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> =
                    obj.iter().cloned().collect();

                Ok(Self {
                    r#aws_region: {
                        let field_value = match fields_map.get("aws_region") {
                            Some(value) => value,
                            None => bail!("Missing field 'aws_region' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#bias: {
                        let field_value = match fields_map.get("bias") {
                            Some(value) => value,
                            None => bail!("Missing field 'bias' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#coordinates: {
                        let field_value = match fields_map.get("coordinates") {
                            Some(value) => value,
                            None => bail!("Missing field 'coordinates' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::route53::RecordGeoproximityRoutingPolicyCoordinate>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#local_zone_group: {
                        let field_value = match fields_map.get("local_zone_group") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_zone_group' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<String> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
