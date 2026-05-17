#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ServiceTaskSpec {
    /// The spec for each container
    #[builder(into)]
    #[serde(rename = "containerSpec")]
    pub r#container_spec: Box<super::types::ServiceTaskSpecContainerSpec>,
    /// A counter that triggers an update even if no relevant parameters have been changed. See the [spec](https://github.com/docker/swarmkit/blob/master/api/specs.proto#L126).
    #[builder(into)]
    #[serde(rename = "forceUpdate")]
    pub r#force_update: Option<i32>,
    /// Specifies the log driver to use for tasks created from this spec. If not present, the default one for the swarm will be used, finally falling back to the engine default if not specified
    #[builder(into)]
    #[serde(rename = "logDriver")]
    pub r#log_driver: Option<Box<super::types::ServiceTaskSpecLogDriver>>,
    /// The networks the container is attached to
    #[builder(into)]
    #[serde(rename = "networksAdvanceds")]
    pub r#networks_advanceds: Option<Vec<super::types::ServiceTaskSpecNetworksAdvanced>>,
    /// The placement preferences
    #[builder(into)]
    #[serde(rename = "placement")]
    pub r#placement: Option<Box<super::types::ServiceTaskSpecPlacement>>,
    /// Resource requirements which apply to each individual container created as part of the service
    #[builder(into)]
    #[serde(rename = "resources")]
    pub r#resources: Option<Box<super::types::ServiceTaskSpecResources>>,
    /// Specification for the restart policy which applies to containers created as part of this service.
    #[builder(into)]
    #[serde(rename = "restartPolicy")]
    pub r#restart_policy: Option<Box<super::types::ServiceTaskSpecRestartPolicy>>,
    /// Runtime is the type of runtime specified for the task executor. See the [types](https://github.com/moby/moby/blob/master/api/types/swarm/runtime.go).
    #[builder(into)]
    #[serde(rename = "runtime")]
    pub r#runtime: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ServiceTaskSpec {
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
                "container_spec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#container_spec,
                )
                .await,
            );
            map.insert(
                "force_update".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#force_update,
                )
                .await,
            );
            map.insert(
                "log_driver".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#log_driver,
                )
                .await,
            );
            map.insert(
                "networks_advanceds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#networks_advanceds,
                )
                .await,
            );
            map.insert(
                "placement".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#placement,
                )
                .await,
            );
            map.insert(
                "resources".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#resources,
                )
                .await,
            );
            map.insert(
                "restart_policy".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#restart_policy,
                )
                .await,
            );
            map.insert(
                "runtime".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#runtime,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ServiceTaskSpec {
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
                    r#container_spec: {
                        let field_value = match fields_map.get("container_spec") {
                            Some(value) => value,
                            None => bail!("Missing field 'container_spec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#force_update: {
                        let field_value = match fields_map.get("force_update") {
                            Some(value) => value,
                            None => bail!("Missing field 'force_update' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#log_driver: {
                        let field_value = match fields_map.get("log_driver") {
                            Some(value) => value,
                            None => bail!("Missing field 'log_driver' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#networks_advanceds: {
                        let field_value = match fields_map.get("networks_advanceds") {
                            Some(value) => value,
                            None => bail!("Missing field 'networks_advanceds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#placement: {
                        let field_value = match fields_map.get("placement") {
                            Some(value) => value,
                            None => bail!("Missing field 'placement' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resources: {
                        let field_value = match fields_map.get("resources") {
                            Some(value) => value,
                            None => bail!("Missing field 'resources' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#restart_policy: {
                        let field_value = match fields_map.get("restart_policy") {
                            Some(value) => value,
                            None => bail!("Missing field 'restart_policy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#runtime: {
                        let field_value = match fields_map.get("runtime") {
                            Some(value) => value,
                            None => bail!("Missing field 'runtime' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
