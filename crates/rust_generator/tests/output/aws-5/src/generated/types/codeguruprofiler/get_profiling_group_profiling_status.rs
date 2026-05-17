#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetProfilingGroupProfilingStatus {
    #[builder(into)]
    #[serde(rename = "latestAgentOrchestratedAt")]
    pub r#latest_agent_orchestrated_at: String,
    #[builder(into)]
    #[serde(rename = "latestAgentProfileReportedAt")]
    pub r#latest_agent_profile_reported_at: String,
    #[builder(into)]
    #[serde(rename = "latestAggregatedProfiles")]
    pub r#latest_aggregated_profiles: Vec<super::super::types::codeguruprofiler::GetProfilingGroupProfilingStatusLatestAggregatedProfile>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetProfilingGroupProfilingStatus {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "latest_agent_orchestrated_at",
                    &self.r#latest_agent_orchestrated_at,
                ),
                to_pulumi_object_field(
                    "latest_agent_profile_reported_at",
                    &self.r#latest_agent_profile_reported_at,
                ),
                to_pulumi_object_field(
                    "latest_aggregated_profiles",
                    &self.r#latest_aggregated_profiles,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetProfilingGroupProfilingStatus {
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
                    r#latest_agent_orchestrated_at: {
                        let field_value = match fields_map.get("latest_agent_orchestrated_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'latest_agent_orchestrated_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#latest_agent_profile_reported_at: {
                        let field_value = match fields_map.get("latest_agent_profile_reported_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'latest_agent_profile_reported_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#latest_aggregated_profiles: {
                        let field_value = match fields_map.get("latest_aggregated_profiles") {
                            Some(value) => value,
                            None => bail!("Missing field 'latest_aggregated_profiles' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
