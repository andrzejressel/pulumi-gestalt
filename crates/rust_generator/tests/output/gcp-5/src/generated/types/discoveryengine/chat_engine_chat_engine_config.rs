#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ChatEngineChatEngineConfig {
    /// The configuration to generate the Dialogflow agent that is associated to this Engine.
    /// Exactly one of `agent_creation_config` or `dialogflow_agent_to_link` must be set.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "agentCreationConfig")]
    pub r#agent_creation_config: Option<Box<super::super::types::discoveryengine::ChatEngineChatEngineConfigAgentCreationConfig>>,
    /// The resource name of an existing Dialogflow agent to link to this Chat Engine. Format: `projects/<Project_ID>/locations/<Location_ID>/agents/<Agent_ID>`.
    /// Exactly one of `agent_creation_config` or `dialogflow_agent_to_link` must be set.
    #[builder(into)]
    #[serde(rename = "dialogflowAgentToLink")]
    pub r#dialogflow_agent_to_link: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ChatEngineChatEngineConfig {
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
                    "agent_creation_config",
                    &self.r#agent_creation_config,
                ),
                to_pulumi_object_field(
                    "dialogflow_agent_to_link",
                    &self.r#dialogflow_agent_to_link,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ChatEngineChatEngineConfig {
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
                    r#agent_creation_config: {
                        let field_value = match fields_map.get("agent_creation_config") {
                            Some(value) => value,
                            None => bail!("Missing field 'agent_creation_config' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#dialogflow_agent_to_link: {
                        let field_value = match fields_map.get("dialogflow_agent_to_link") {
                            Some(value) => value,
                            None => bail!("Missing field 'dialogflow_agent_to_link' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
