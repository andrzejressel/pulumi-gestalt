#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FleetRuntimeConfiguration {
    /// Maximum amount of time (in seconds) that a game session can remain in status `ACTIVATING`.
    #[builder(into)]
    #[serde(rename = "gameSessionActivationTimeoutSeconds")]
    pub r#game_session_activation_timeout_seconds: Option<i32>,
    /// Maximum number of game sessions with status `ACTIVATING` to allow on an instance simultaneously.
    #[builder(into)]
    #[serde(rename = "maxConcurrentGameSessionActivations")]
    pub r#max_concurrent_game_session_activations: Option<i32>,
    /// Collection of server process configurations that describe which server processes to run on each instance in a fleet. See below.
    #[builder(into)]
    #[serde(rename = "serverProcesses")]
    pub r#server_processes: Option<Vec<super::super::types::gamelift::FleetRuntimeConfigurationServerProcess>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FleetRuntimeConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;

            let mut map: BTreeMap<String, pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue> = BTreeMap::new();
            map.insert("game_session_activation_timeout_seconds".to_string(), self.r#game_session_activation_timeout_seconds.to_pulumi_value().await);
            map.insert("max_concurrent_game_session_activations".to_string(), self.r#max_concurrent_game_session_activations.to_pulumi_value().await);
            map.insert("server_processes".to_string(), self.r#server_processes.to_pulumi_value().await);

            map.to_pulumi_value().await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FleetRuntimeConfiguration {
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
                    r#game_session_activation_timeout_seconds: {
                        let field_value = match fields_map.get("game_session_activation_timeout_seconds") {
                            Some(value) => value,
                            None => bail!("Missing field 'game_session_activation_timeout_seconds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#max_concurrent_game_session_activations: {
                        let field_value = match fields_map.get("max_concurrent_game_session_activations") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_concurrent_game_session_activations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<i32> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                    r#server_processes: {
                        let field_value = match fields_map.get("server_processes") {
                            Some(value) => value,
                            None => bail!("Missing field 'server_processes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        <Option<Vec<super::super::types::gamelift::FleetRuntimeConfigurationServerProcess>> as FromPulumiValue>::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
