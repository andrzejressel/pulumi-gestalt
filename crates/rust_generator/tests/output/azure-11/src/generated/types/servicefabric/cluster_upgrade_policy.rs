#[derive(pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterUpgradePolicy {
    /// A `delta_health_policy` block as defined below
    #[builder(into)]
    pub r#delta_health_policy: Option<Box<super::super::types::servicefabric::ClusterUpgradePolicyDeltaHealthPolicy>>,
    /// Indicates whether to restart the Service Fabric node even if only dynamic configurations have changed.
    #[builder(into)]
    pub r#force_restart_enabled: Option<bool>,
    /// Specifies the duration, in "hh:mm:ss" string format, after which Service Fabric retries the health check if the previous health check fails. Defaults to `00:45:00`.
    #[builder(into)]
    pub r#health_check_retry_timeout: Option<String>,
    /// Specifies the duration, in "hh:mm:ss" string format, that Service Fabric waits in order to verify that the cluster is stable before it continues to the next upgrade domain or completes the upgrade. This wait duration prevents undetected changes of health right after the health check is performed. Defaults to `00:01:00`.
    #[builder(into)]
    pub r#health_check_stable_duration: Option<String>,
    /// Specifies the duration, in "hh:mm:ss" string format, that Service Fabric waits before it performs the initial health check after it finishes the upgrade on the upgrade domain. Defaults to `00:00:30`.
    #[builder(into)]
    pub r#health_check_wait_duration: Option<String>,
    /// A `health_policy` block as defined below
    #[builder(into)]
    pub r#health_policy: Option<Box<super::super::types::servicefabric::ClusterUpgradePolicyHealthPolicy>>,
    /// Specifies the duration, in "hh:mm:ss" string format, that Service Fabric takes to upgrade a single upgrade domain. After this period, the upgrade fails. Defaults to `02:00:00`.
    #[builder(into)]
    pub r#upgrade_domain_timeout: Option<String>,
    /// Specifies the duration, in "hh:mm:ss" string format, that Service Fabric waits for a replica set to reconfigure into a safe state, if it is not already in a safe state, before Service Fabric proceeds with the upgrade. Defaults to `10675199.02:48:05.4775807`.
    #[builder(into)]
    pub r#upgrade_replica_set_check_timeout: Option<String>,
    /// Specifies the duration, in "hh:mm:ss" string format, that Service Fabric takes for the entire upgrade. After this period, the upgrade fails. Defaults to `12:00:00`.
    #[builder(into)]
    pub r#upgrade_timeout: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterUpgradePolicy {
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
                    "deltaHealthPolicy",
                    &self.r#delta_health_policy,
                ),
                to_pulumi_object_field(
                    "forceRestartEnabled",
                    &self.r#force_restart_enabled,
                ),
                to_pulumi_object_field(
                    "healthCheckRetryTimeout",
                    &self.r#health_check_retry_timeout,
                ),
                to_pulumi_object_field(
                    "healthCheckStableDuration",
                    &self.r#health_check_stable_duration,
                ),
                to_pulumi_object_field(
                    "healthCheckWaitDuration",
                    &self.r#health_check_wait_duration,
                ),
                to_pulumi_object_field(
                    "healthPolicy",
                    &self.r#health_policy,
                ),
                to_pulumi_object_field(
                    "upgradeDomainTimeout",
                    &self.r#upgrade_domain_timeout,
                ),
                to_pulumi_object_field(
                    "upgradeReplicaSetCheckTimeout",
                    &self.r#upgrade_replica_set_check_timeout,
                ),
                to_pulumi_object_field(
                    "upgradeTimeout",
                    &self.r#upgrade_timeout,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterUpgradePolicy {
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
                    r#delta_health_policy: {
                        let field_value = match fields_map.get("deltaHealthPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'deltaHealthPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#force_restart_enabled: {
                        let field_value = match fields_map.get("forceRestartEnabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'forceRestartEnabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_check_retry_timeout: {
                        let field_value = match fields_map.get("healthCheckRetryTimeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthCheckRetryTimeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_check_stable_duration: {
                        let field_value = match fields_map.get("healthCheckStableDuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthCheckStableDuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_check_wait_duration: {
                        let field_value = match fields_map.get("healthCheckWaitDuration") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthCheckWaitDuration' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#health_policy: {
                        let field_value = match fields_map.get("healthPolicy") {
                            Some(value) => value,
                            None => bail!("Missing field 'healthPolicy' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upgrade_domain_timeout: {
                        let field_value = match fields_map.get("upgradeDomainTimeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'upgradeDomainTimeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upgrade_replica_set_check_timeout: {
                        let field_value = match fields_map.get("upgradeReplicaSetCheckTimeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'upgradeReplicaSetCheckTimeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#upgrade_timeout: {
                        let field_value = match fields_map.get("upgradeTimeout") {
                            Some(value) => value,
                            None => bail!("Missing field 'upgradeTimeout' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
