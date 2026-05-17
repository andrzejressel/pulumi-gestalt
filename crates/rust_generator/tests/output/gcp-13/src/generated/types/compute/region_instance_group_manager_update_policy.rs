#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RegionInstanceGroupManagerUpdatePolicy {
    /// The instance redistribution policy for regional managed instance groups. Valid values are: `"PROACTIVE"`, `"NONE"`. If `PROACTIVE` (default), the group attempts to maintain an even distribution of VM instances across zones in the region. If `NONE`, proactive redistribution is disabled.
    #[builder(into)]
    #[serde(rename = "instanceRedistributionType")]
    pub r#instance_redistribution_type: Option<String>,
    /// , Specifies a fixed number of VM instances. This must be a positive integer. Conflicts with `max_surge_percent`. Both cannot be 0.
    #[builder(into)]
    #[serde(rename = "maxSurgeFixed")]
    pub r#max_surge_fixed: Option<i32>,
    /// , Specifies a percentage of instances between 0 to 100%, inclusive. For example, specify 80 for 80%. Conflicts with `max_surge_fixed`.
    #[builder(into)]
    #[serde(rename = "maxSurgePercent")]
    pub r#max_surge_percent: Option<i32>,
    /// , Specifies a fixed number of VM instances. This must be a positive integer.
    #[builder(into)]
    #[serde(rename = "maxUnavailableFixed")]
    pub r#max_unavailable_fixed: Option<i32>,
    /// , Specifies a percentage of instances between 0 to 100%, inclusive. For example, specify 80 for 80%..
    #[builder(into)]
    #[serde(rename = "maxUnavailablePercent")]
    pub r#max_unavailable_percent: Option<i32>,
    /// , Minimum number of seconds to wait for after a newly created instance becomes available. This value must be from range [0, 3600]
    #[builder(into)]
    #[serde(rename = "minReadySec")]
    pub r#min_ready_sec: Option<i32>,
    /// Minimal action to be taken on an instance. You can specify either `NONE` to forbid any actions, `REFRESH` to update without stopping instances, `RESTART` to restart existing instances or `REPLACE` to delete and create new instances from the target template. If you specify a `REFRESH`, the Updater will attempt to perform that action only. However, if the Updater determines that the minimal action you specify is not enough to perform the update, it might perform a more disruptive action.
    #[builder(into)]
    #[serde(rename = "minimalAction")]
    pub r#minimal_action: String,
    /// Most disruptive action that is allowed to be taken on an instance. You can specify either NONE to forbid any actions, REFRESH to allow actions that do not need instance restart, RESTART to allow actions that can be applied without instance replacing or REPLACE to allow all possible actions. If the Updater determines that the minimal update action needed is more disruptive than most disruptive allowed action you specify it will not perform the update at all.
    #[builder(into)]
    #[serde(rename = "mostDisruptiveAllowedAction")]
    pub r#most_disruptive_allowed_action: Option<String>,
    /// , The instance replacement method for managed instance groups. Valid values are: "RECREATE", "SUBSTITUTE". If SUBSTITUTE (default), the group replaces VM instances with new instances that have randomly generated names. If RECREATE, instance names are preserved.  You must also set max_unavailable_fixed or max_unavailable_percent to be greater than 0.
    /// - - -
    #[builder(into)]
    #[serde(rename = "replacementMethod")]
    pub r#replacement_method: Option<String>,
    /// The type of update process. You can specify either `PROACTIVE` so that the instance group manager proactively executes actions in order to bring instances to their target versions or `OPPORTUNISTIC` so that no action is proactively executed but the update will be performed as part of other actions (for example, resizes or recreateInstances calls).
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RegionInstanceGroupManagerUpdatePolicy {
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
                "instance_redistribution_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#instance_redistribution_type,
                )
                .await,
            );
            map.insert(
                "max_surge_fixed".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_surge_fixed,
                )
                .await,
            );
            map.insert(
                "max_surge_percent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_surge_percent,
                )
                .await,
            );
            map.insert(
                "max_unavailable_fixed".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_unavailable_fixed,
                )
                .await,
            );
            map.insert(
                "max_unavailable_percent".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#max_unavailable_percent,
                )
                .await,
            );
            map.insert(
                "min_ready_sec".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#min_ready_sec,
                )
                .await,
            );
            map.insert(
                "minimal_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#minimal_action,
                )
                .await,
            );
            map.insert(
                "most_disruptive_allowed_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#most_disruptive_allowed_action,
                )
                .await,
            );
            map.insert(
                "replacement_method".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#replacement_method,
                )
                .await,
            );
            map.insert(
                "type_".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#type_,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RegionInstanceGroupManagerUpdatePolicy {
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
                    r#instance_redistribution_type: {
                        let field_value = match fields_map.get("instance_redistribution_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_redistribution_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_surge_fixed: {
                        let field_value = match fields_map.get("max_surge_fixed") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_surge_fixed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_surge_percent: {
                        let field_value = match fields_map.get("max_surge_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_surge_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_unavailable_fixed: {
                        let field_value = match fields_map.get("max_unavailable_fixed") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_unavailable_fixed' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#max_unavailable_percent: {
                        let field_value = match fields_map.get("max_unavailable_percent") {
                            Some(value) => value,
                            None => bail!("Missing field 'max_unavailable_percent' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#min_ready_sec: {
                        let field_value = match fields_map.get("min_ready_sec") {
                            Some(value) => value,
                            None => bail!("Missing field 'min_ready_sec' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimal_action: {
                        let field_value = match fields_map.get("minimal_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimal_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#most_disruptive_allowed_action: {
                        let field_value = match fields_map.get("most_disruptive_allowed_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'most_disruptive_allowed_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#replacement_method: {
                        let field_value = match fields_map.get("replacement_method") {
                            Some(value) => value,
                            None => bail!("Missing field 'replacement_method' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#type_: {
                        let field_value = match fields_map.get("type_") {
                            Some(value) => value,
                            None => bail!("Missing field 'type_' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
