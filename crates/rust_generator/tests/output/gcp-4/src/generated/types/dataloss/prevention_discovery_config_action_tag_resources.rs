#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDiscoveryConfigActionTagResources {
    /// Whether applying a tag to a resource should lower the risk of the profile for that resource. For example, in conjunction with an [IAM deny policy](https://cloud.google.com/iam/docs/deny-overview), you can deny all principals a permission if a tag value is present, mitigating the risk of the resource. This also lowers the data risk of resources at the lower levels of the resource hierarchy. For example, reducing the data risk of a table data profile also reduces the data risk of the constituent column data profiles.
    #[builder(into)]
    #[serde(rename = "lowerDataRiskToLow")]
    pub r#lower_data_risk_to_low: Option<bool>,
    /// The profile generations for which the tag should be attached to resources. If you attach a tag to only new profiles, then if the sensitivity score of a profile subsequently changes, its tag doesn't change. By default, this field includes only new profiles. To include both new and updated profiles for tagging, this field should explicitly include both `PROFILE_GENERATION_NEW` and `PROFILE_GENERATION_UPDATE`.
    /// Each value may be one of: `PROFILE_GENERATION_NEW`, `PROFILE_GENERATION_UPDATE`.
    #[builder(into)]
    #[serde(rename = "profileGenerationsToTags")]
    pub r#profile_generations_to_tags: Option<Vec<String>>,
    /// The tags to associate with different conditions.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "tagConditions")]
    pub r#tag_conditions: Option<Vec<super::super::types::dataloss::PreventionDiscoveryConfigActionTagResourcesTagCondition>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDiscoveryConfigActionTagResources {
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
                "lower_data_risk_to_low".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lower_data_risk_to_low,
                )
                .await,
            );
            map.insert(
                "profile_generations_to_tags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#profile_generations_to_tags,
                )
                .await,
            );
            map.insert(
                "tag_conditions".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#tag_conditions,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDiscoveryConfigActionTagResources {
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
                    r#lower_data_risk_to_low: {
                        let field_value = match fields_map.get("lower_data_risk_to_low") {
                            Some(value) => value,
                            None => bail!("Missing field 'lower_data_risk_to_low' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#profile_generations_to_tags: {
                        let field_value = match fields_map.get("profile_generations_to_tags") {
                            Some(value) => value,
                            None => bail!("Missing field 'profile_generations_to_tags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#tag_conditions: {
                        let field_value = match fields_map.get("tag_conditions") {
                            Some(value) => value,
                            None => bail!("Missing field 'tag_conditions' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
