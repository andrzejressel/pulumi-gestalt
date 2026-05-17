#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RestorePlanRestoreConfigClusterResourceRestoreScope {
    /// If True, all valid cluster-scoped resources will be restored.
    /// Mutually exclusive to any other field in `clusterResourceRestoreScope`.
    #[builder(into)]
    #[serde(rename = "allGroupKinds")]
    pub r#all_group_kinds: Option<bool>,
    /// A list of cluster-scoped resource group kinds to NOT restore from the backup.
    /// If specified, all valid cluster-scoped resources will be restored except
    /// for those specified in the list.
    /// Mutually exclusive to any other field in `clusterResourceRestoreScope`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "excludedGroupKinds")]
    pub r#excluded_group_kinds: Option<Vec<super::super::types::gkebackup::RestorePlanRestoreConfigClusterResourceRestoreScopeExcludedGroupKind>>,
    /// If True, no cluster-scoped resources will be restored.
    /// Mutually exclusive to any other field in `clusterResourceRestoreScope`.
    #[builder(into)]
    #[serde(rename = "noGroupKinds")]
    pub r#no_group_kinds: Option<bool>,
    /// A list of cluster-scoped resource group kinds to restore from the backup.
    /// If specified, only the selected resources will be restored.
    /// Mutually exclusive to any other field in the `clusterResourceRestoreScope`.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "selectedGroupKinds")]
    pub r#selected_group_kinds: Option<Vec<super::super::types::gkebackup::RestorePlanRestoreConfigClusterResourceRestoreScopeSelectedGroupKind>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RestorePlanRestoreConfigClusterResourceRestoreScope {
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
                "all_group_kinds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#all_group_kinds,
                )
                .await,
            );
            map.insert(
                "excluded_group_kinds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#excluded_group_kinds,
                )
                .await,
            );
            map.insert(
                "no_group_kinds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#no_group_kinds,
                )
                .await,
            );
            map.insert(
                "selected_group_kinds".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#selected_group_kinds,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RestorePlanRestoreConfigClusterResourceRestoreScope {
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
                    r#all_group_kinds: {
                        let field_value = match fields_map.get("all_group_kinds") {
                            Some(value) => value,
                            None => bail!("Missing field 'all_group_kinds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excluded_group_kinds: {
                        let field_value = match fields_map.get("excluded_group_kinds") {
                            Some(value) => value,
                            None => bail!("Missing field 'excluded_group_kinds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_group_kinds: {
                        let field_value = match fields_map.get("no_group_kinds") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_group_kinds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#selected_group_kinds: {
                        let field_value = match fields_map.get("selected_group_kinds") {
                            Some(value) => value,
                            None => bail!("Missing field 'selected_group_kinds' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
