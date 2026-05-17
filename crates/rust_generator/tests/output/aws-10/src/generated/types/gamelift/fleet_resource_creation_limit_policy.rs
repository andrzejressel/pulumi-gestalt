#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetResourceCreationLimitPolicy {
    /// Maximum number of game sessions that an individual can create during the policy period.
    #[builder(into)]
    #[serde(rename = "newGameSessionsPerCreator")]
    pub r#new_game_sessions_per_creator: Option<i32>,
    /// Time span used in evaluating the resource creation limit policy.
    #[builder(into)]
    #[serde(rename = "policyPeriodInMinutes")]
    pub r#policy_period_in_minutes: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FleetResourceCreationLimitPolicy {
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
                "new_game_sessions_per_creator".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#new_game_sessions_per_creator,
                )
                .await,
            );
            map.insert(
                "policy_period_in_minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#policy_period_in_minutes,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FleetResourceCreationLimitPolicy {
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
                    r#new_game_sessions_per_creator: {
                        let field_value = match fields_map.get("new_game_sessions_per_creator") {
                            Some(value) => value,
                            None => bail!("Missing field 'new_game_sessions_per_creator' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_period_in_minutes: {
                        let field_value = match fields_map.get("policy_period_in_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_period_in_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
