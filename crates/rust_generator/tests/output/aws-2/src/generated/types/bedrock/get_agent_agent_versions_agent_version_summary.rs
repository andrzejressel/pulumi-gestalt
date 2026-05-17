#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAgentAgentVersionsAgentVersionSummary {
    /// Name of agent to which the version belongs.
    #[builder(into)]
    #[serde(rename = "agentName")]
    pub r#agent_name: String,
    /// Status of the agent to which the version belongs.
    #[builder(into)]
    #[serde(rename = "agentStatus")]
    pub r#agent_status: String,
    /// Version of the agent.
    #[builder(into)]
    #[serde(rename = "agentVersion")]
    pub r#agent_version: String,
    /// Time at which the version was created.
    #[builder(into)]
    #[serde(rename = "createdAt")]
    pub r#created_at: String,
    /// Description of the version of the agent.
    /// * `GuardrailConfiguration` - Details aout the guardrail associated with the agent. See Guardrail Configuration
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: String,
    #[builder(into)]
    #[serde(rename = "guardrailConfigurations")]
    pub r#guardrail_configurations: Option<Vec<super::super::types::bedrock::GetAgentAgentVersionsAgentVersionSummaryGuardrailConfiguration>>,
    /// Time at which the version was last updated.
    #[builder(into)]
    #[serde(rename = "updatedAt")]
    pub r#updated_at: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAgentAgentVersionsAgentVersionSummary {
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
                "agent_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#agent_name,
                )
                .await,
            );
            map.insert(
                "agent_status".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#agent_status,
                )
                .await,
            );
            map.insert(
                "agent_version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#agent_version,
                )
                .await,
            );
            map.insert(
                "created_at".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#created_at,
                )
                .await,
            );
            map.insert(
                "description".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#description,
                )
                .await,
            );
            map.insert(
                "guardrail_configurations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#guardrail_configurations,
                )
                .await,
            );
            map.insert(
                "updated_at".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#updated_at,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAgentAgentVersionsAgentVersionSummary {
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
                    r#agent_name: {
                        let field_value = match fields_map.get("agent_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'agent_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#agent_status: {
                        let field_value = match fields_map.get("agent_status") {
                            Some(value) => value,
                            None => bail!("Missing field 'agent_status' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#agent_version: {
                        let field_value = match fields_map.get("agent_version") {
                            Some(value) => value,
                            None => bail!("Missing field 'agent_version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#created_at: {
                        let field_value = match fields_map.get("created_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'created_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#guardrail_configurations: {
                        let field_value = match fields_map.get("guardrail_configurations") {
                            Some(value) => value,
                            None => bail!("Missing field 'guardrail_configurations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#updated_at: {
                        let field_value = match fields_map.get("updated_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'updated_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
