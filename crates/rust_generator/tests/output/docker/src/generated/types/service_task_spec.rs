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
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "container_spec",
                    &self.r#container_spec,
                ),
                to_pulumi_object_field(
                    "force_update",
                    &self.r#force_update,
                ),
                to_pulumi_object_field(
                    "log_driver",
                    &self.r#log_driver,
                ),
                to_pulumi_object_field(
                    "networks_advanceds",
                    &self.r#networks_advanceds,
                ),
                to_pulumi_object_field(
                    "placement",
                    &self.r#placement,
                ),
                to_pulumi_object_field(
                    "resources",
                    &self.r#resources,
                ),
                to_pulumi_object_field(
                    "restart_policy",
                    &self.r#restart_policy,
                ),
                to_pulumi_object_field(
                    "runtime",
                    &self.r#runtime,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
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
