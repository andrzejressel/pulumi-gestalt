#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct FeatureMembershipConfigmanagement {
    /// (Optional, Deprecated)
    /// Binauthz configuration for the cluster. Structure is documented below.
    /// This field will be ignored and should not be set.
    #[builder(into)]
    #[serde(rename = "binauthz")]
    pub r#binauthz: Option<Box<super::super::types::gkehub::FeatureMembershipConfigmanagementBinauthz>>,
    /// Config Sync configuration for the cluster. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "configSync")]
    pub r#config_sync: Option<Box<super::super::types::gkehub::FeatureMembershipConfigmanagementConfigSync>>,
    /// Hierarchy Controller configuration for the cluster. Structure is documented below.
    /// Configuring Hierarchy Controller through the configmanagement feature is no longer recommended.
    /// Use open source Kubernetes [Hierarchical Namespace Controller (HNC)](https://github.com/kubernetes-sigs/hierarchical-namespaces) instead.
    /// Follow the [instructions](https://cloud.google.com/kubernetes-engine/enterprise/config-sync/docs/how-to/migrate-hierarchy-controller)
    /// to migrate from Hierarchy Controller to HNC.
    #[builder(into)]
    #[serde(rename = "hierarchyController")]
    pub r#hierarchy_controller: Option<Box<super::super::types::gkehub::FeatureMembershipConfigmanagementHierarchyController>>,
    /// Set this field to MANAGEMENT_AUTOMATIC to enable Config Sync auto-upgrades, and set this field to MANAGEMENT_MANUAL or MANAGEMENT_UNSPECIFIED to disable Config Sync auto-upgrades.
    #[builder(into)]
    #[serde(rename = "management")]
    pub r#management: Option<String>,
    /// Policy Controller configuration for the cluster. Structure is documented below.
    /// Configuring Policy Controller through the configmanagement feature is no longer recommended.
    /// Use the policycontroller feature instead.
    #[builder(into)]
    #[serde(rename = "policyController")]
    pub r#policy_controller: Option<Box<super::super::types::gkehub::FeatureMembershipConfigmanagementPolicyController>>,
    /// Version of Config Sync installed.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for FeatureMembershipConfigmanagement {
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
                "binauthz".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#binauthz,
                )
                .await,
            );
            map.insert(
                "config_sync".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#config_sync,
                )
                .await,
            );
            map.insert(
                "hierarchy_controller".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hierarchy_controller,
                )
                .await,
            );
            map.insert(
                "management".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#management,
                )
                .await,
            );
            map.insert(
                "policy_controller".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#policy_controller,
                )
                .await,
            );
            map.insert(
                "version".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#version,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for FeatureMembershipConfigmanagement {
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
                    r#binauthz: {
                        let field_value = match fields_map.get("binauthz") {
                            Some(value) => value,
                            None => bail!("Missing field 'binauthz' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#config_sync: {
                        let field_value = match fields_map.get("config_sync") {
                            Some(value) => value,
                            None => bail!("Missing field 'config_sync' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hierarchy_controller: {
                        let field_value = match fields_map.get("hierarchy_controller") {
                            Some(value) => value,
                            None => bail!("Missing field 'hierarchy_controller' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#management: {
                        let field_value = match fields_map.get("management") {
                            Some(value) => value,
                            None => bail!("Missing field 'management' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#policy_controller: {
                        let field_value = match fields_map.get("policy_controller") {
                            Some(value) => value,
                            None => bail!("Missing field 'policy_controller' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
