#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct DomainDefaultUserSettingsJupyterLabAppSettingsAppLifecycleManagementIdleSettings {
    /// The time that SageMaker waits after the application becomes idle before shutting it down. Valid values are between `60` and `525600`.
    #[builder(into)]
    #[serde(rename = "idleTimeoutInMinutes")]
    pub r#idle_timeout_in_minutes: Option<i32>,
    /// Indicates whether idle shutdown is activated for the application type. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into)]
    #[serde(rename = "lifecycleManagement")]
    pub r#lifecycle_management: Option<String>,
    /// The maximum value in minutes that custom idle shutdown can be set to by the user. Valid values are between `60` and `525600`.
    #[builder(into)]
    #[serde(rename = "maxIdleTimeoutInMinutes")]
    pub r#max_idle_timeout_in_minutes: Option<i32>,
    /// The minimum value in minutes that custom idle shutdown can be set to by the user. Valid values are between `60` and `525600`.
    #[builder(into)]
    #[serde(rename = "minIdleTimeoutInMinutes")]
    pub r#min_idle_timeout_in_minutes: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for DomainDefaultUserSettingsJupyterLabAppSettingsAppLifecycleManagementIdleSettings {
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
                "idle_timeout_in_minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#idle_timeout_in_minutes,
                )
                .await,
            );
            map.insert(
                "lifecycle_management".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#lifecycle_management,
                )
                .await,
            );
            map.insert(
                "max_idle_timeout_in_minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_idle_timeout_in_minutes,
                )
                .await,
            );
            map.insert(
                "min_idle_timeout_in_minutes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_idle_timeout_in_minutes,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for DomainDefaultUserSettingsJupyterLabAppSettingsAppLifecycleManagementIdleSettings {
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
                    r#idle_timeout_in_minutes: {
                        let field_value = match fields_map.get("idle_timeout_in_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'idle_timeout_in_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#lifecycle_management: {
                        let field_value = match fields_map.get("lifecycle_management") {
                            Some(value) => value,
                            None => bail!("Missing field 'lifecycle_management' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_idle_timeout_in_minutes: {
                        let field_value = match fields_map.get("max_idle_timeout_in_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_idle_timeout_in_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_idle_timeout_in_minutes: {
                        let field_value = match fields_map.get("min_idle_timeout_in_minutes") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_idle_timeout_in_minutes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
