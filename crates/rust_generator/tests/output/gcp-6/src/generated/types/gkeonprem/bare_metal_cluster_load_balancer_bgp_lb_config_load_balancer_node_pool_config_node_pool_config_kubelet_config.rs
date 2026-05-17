#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct BareMetalClusterLoadBalancerBgpLbConfigLoadBalancerNodePoolConfigNodePoolConfigKubeletConfig {
    /// The maximum size of bursty pulls, temporarily allows pulls to burst to this
    /// number, while still not exceeding registry_pull_qps.
    /// The value must not be a negative number.
    /// Updating this field may impact scalability by changing the amount of
    /// traffic produced by image pulls.
    /// Defaults to 10.
    #[builder(into)]
    #[serde(rename = "registryBurst")]
    pub r#registry_burst: Option<i32>,
    /// The limit of registry pulls per second.
    /// Setting this value to 0 means no limit.
    /// Updating this field may impact scalability by changing the amount of
    /// traffic produced by image pulls.
    /// Defaults to 5.
    #[builder(into)]
    #[serde(rename = "registryPullQps")]
    pub r#registry_pull_qps: Option<i32>,
    /// Prevents the Kubelet from pulling multiple images at a time.
    /// We recommend *not* changing the default value on nodes that run docker
    /// daemon with version  < 1.9 or an Another Union File System (Aufs) storage
    /// backend. Issue https://github.com/kubernetes/kubernetes/issues/10959 has
    /// more details.
    #[builder(into)]
    #[serde(rename = "serializeImagePullsDisabled")]
    pub r#serialize_image_pulls_disabled: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for BareMetalClusterLoadBalancerBgpLbConfigLoadBalancerNodePoolConfigNodePoolConfigKubeletConfig {
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
                "registry_burst".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#registry_burst,
                )
                .await,
            );
            map.insert(
                "registry_pull_qps".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#registry_pull_qps,
                )
                .await,
            );
            map.insert(
                "serialize_image_pulls_disabled".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#serialize_image_pulls_disabled,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for BareMetalClusterLoadBalancerBgpLbConfigLoadBalancerNodePoolConfigNodePoolConfigKubeletConfig {
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
                    r#registry_burst: {
                        let field_value = match fields_map.get("registry_burst") {
                            Some(value) => value,
                            None => bail!("Missing field 'registry_burst' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#registry_pull_qps: {
                        let field_value = match fields_map.get("registry_pull_qps") {
                            Some(value) => value,
                            None => bail!("Missing field 'registry_pull_qps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#serialize_image_pulls_disabled: {
                        let field_value = match fields_map.get("serialize_image_pulls_disabled") {
                            Some(value) => value,
                            None => bail!("Missing field 'serialize_image_pulls_disabled' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
