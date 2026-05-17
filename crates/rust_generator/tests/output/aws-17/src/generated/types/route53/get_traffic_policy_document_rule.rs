#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetTrafficPolicyDocumentRule {
    /// Configuration block for when you add a geoproximity rule, you configure Amazon Route 53 to route traffic to your resources based on the geographic location of your resources. Only valid for `geoproximity` type. See below
    #[builder(into)]
    #[serde(rename = "geoProximityLocations")]
    pub r#geo_proximity_locations: Option<Vec<super::super::types::route53::GetTrafficPolicyDocumentRuleGeoProximityLocation>>,
    /// ID of a rule you want to assign.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Configuration block for when you add a multivalue answer rule, you configure your traffic policy to route traffic approximately randomly to your healthy resources.  Only valid for `multivalue` type. See below
    #[builder(into)]
    #[serde(rename = "items")]
    pub r#items: Option<Vec<super::super::types::route53::GetTrafficPolicyDocumentRuleItem>>,
    /// Configuration block for when you add a geolocation rule, you configure your traffic policy to route your traffic based on the geographic location of your users.  Only valid for `geo` type. See below
    #[builder(into)]
    #[serde(rename = "locations")]
    pub r#locations: Option<Vec<super::super::types::route53::GetTrafficPolicyDocumentRuleLocation>>,
    /// Configuration block for the settings for the rule or endpoint that you want to route traffic to whenever the corresponding resources are available. Only valid for `failover` type. See below
    #[builder(into)]
    #[serde(rename = "primary")]
    pub r#primary: Option<Box<super::super::types::route53::GetTrafficPolicyDocumentRulePrimary>>,
    #[builder(into)]
    #[serde(rename = "regions")]
    pub r#regions: Option<Vec<super::super::types::route53::GetTrafficPolicyDocumentRuleRegion>>,
    /// Configuration block for the rule or endpoint that you want to route traffic to whenever the primary resources are not available. Only valid for `failover` type. See below
    #[builder(into)]
    #[serde(rename = "secondary")]
    pub r#secondary: Option<Box<super::super::types::route53::GetTrafficPolicyDocumentRuleSecondary>>,
    /// Type of the rule.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetTrafficPolicyDocumentRule {
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
                "geo_proximity_locations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#geo_proximity_locations,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "items".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#items,
                )
                .await,
            );
            map.insert(
                "locations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#locations,
                )
                .await,
            );
            map.insert(
                "primary".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#primary,
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
                "secondary".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secondary,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetTrafficPolicyDocumentRule {
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
                    r#geo_proximity_locations: {
                        let field_value = match fields_map.get("geo_proximity_locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'geo_proximity_locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#items: {
                        let field_value = match fields_map.get("items") {
                            Some(value) => value,
                            None => bail!("Missing field 'items' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#locations: {
                        let field_value = match fields_map.get("locations") {
                            Some(value) => value,
                            None => bail!("Missing field 'locations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#primary: {
                        let field_value = match fields_map.get("primary") {
                            Some(value) => value,
                            None => bail!("Missing field 'primary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#secondary: {
                        let field_value = match fields_map.get("secondary") {
                            Some(value) => value,
                            None => bail!("Missing field 'secondary' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
