#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomerSubscriptionDetails {
    /// Location placement Id of a subscription
    #[builder(into)]
    #[serde(rename = "locationPlacementId")]
    pub r#location_placement_id: Option<String>,
    /// Quota ID of a subscription
    #[builder(into)]
    #[serde(rename = "quotaId")]
    pub r#quota_id: String,
    /// List of registered feature flags for subscription
    #[builder(into)]
    #[serde(rename = "registeredFeatures")]
    pub r#registered_features: Option<Vec<super::types::CustomerSubscriptionRegisteredFeatures>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomerSubscriptionDetails {
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
                "location_placement_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#location_placement_id,
                )
                .await,
            );
            map.insert(
                "quota_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#quota_id,
                )
                .await,
            );
            map.insert(
                "registered_features".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#registered_features,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CustomerSubscriptionDetails {
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
                    r#location_placement_id: {
                        let field_value = match fields_map.get("location_placement_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'location_placement_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#quota_id: {
                        let field_value = match fields_map.get("quota_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'quota_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#registered_features: {
                        let field_value = match fields_map.get("registered_features") {
                            Some(value) => value,
                            None => bail!("Missing field 'registered_features' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
