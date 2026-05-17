#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HostPoolScheduledAgentUpdates {
    /// Enables or disables scheduled updates of the AVD agent components (RDAgent, Geneva Monitoring agent, and side-by-side stack) on session hosts. If this is enabled then up to two `schedule` blocks must be defined. Default is `false`.
    /// 
    /// > **NOTE:** if `enabled` is set to `true` then at least one and a maximum of two `schedule` blocks must be provided.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
    /// A `schedule` block as defined below. A maximum of two blocks can be added.
    #[builder(into)]
    #[serde(rename = "schedules")]
    pub r#schedules: Option<Vec<super::super::types::desktopvirtualization::HostPoolScheduledAgentUpdatesSchedule>>,
    /// Specifies the time zone in which the agent update schedule will apply, [the possible values are defined here](https://jackstromberg.com/2017/01/list-of-time-zones-consumed-by-azure/). If `use_session_host_timezone` is enabled then it will override this setting. Default is `UTC`
    #[builder(into)]
    #[serde(rename = "timezone")]
    pub r#timezone: Option<String>,
    /// Specifies whether scheduled agent updates should be applied based on the timezone of the affected session host. If configured then this setting overrides `timezone`. Default is `false`.
    #[builder(into)]
    #[serde(rename = "useSessionHostTimezone")]
    pub r#use_session_host_timezone: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HostPoolScheduledAgentUpdates {
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
                "enabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enabled,
                )
                .await,
            );
            map.insert(
                "schedules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#schedules,
                )
                .await,
            );
            map.insert(
                "timezone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#timezone,
                )
                .await,
            );
            map.insert(
                "use_session_host_timezone".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#use_session_host_timezone,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HostPoolScheduledAgentUpdates {
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
                    r#enabled: {
                        let field_value = match fields_map.get("enabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'enabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#schedules: {
                        let field_value = match fields_map.get("schedules") {
                            Some(value) => value,
                            None => bail!("Missing field 'schedules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#timezone: {
                        let field_value = match fields_map.get("timezone") {
                            Some(value) => value,
                            None => bail!("Missing field 'timezone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#use_session_host_timezone: {
                        let field_value = match fields_map.get("use_session_host_timezone") {
                            Some(value) => value,
                            None => bail!("Missing field 'use_session_host_timezone' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
