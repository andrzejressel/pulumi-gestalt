#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CacheNfsTargetNamespaceJunction {
    /// The name of the access policy applied to this target. Defaults to `default`.
    #[builder(into)]
    #[serde(rename = "accessPolicyName")]
    pub r#access_policy_name: Option<String>,
    /// The client-facing file path of this NFS target within the HPC Cache NFS Target.
    #[builder(into)]
    #[serde(rename = "namespacePath")]
    pub r#namespace_path: String,
    /// The NFS export of this NFS target within the HPC Cache NFS Target.
    #[builder(into)]
    #[serde(rename = "nfsExport")]
    pub r#nfs_export: String,
    /// The relative subdirectory path from the `nfs_export` to map to the `namespace_path`. Defaults to `""`, in which case the whole `nfs_export` is exported.
    #[builder(into)]
    #[serde(rename = "targetPath")]
    pub r#target_path: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CacheNfsTargetNamespaceJunction {
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
                "access_policy_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#access_policy_name,
                )
                .await,
            );
            map.insert(
                "namespace_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#namespace_path,
                )
                .await,
            );
            map.insert(
                "nfs_export".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#nfs_export,
                )
                .await,
            );
            map.insert(
                "target_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#target_path,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CacheNfsTargetNamespaceJunction {
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
                    r#access_policy_name: {
                        let field_value = match fields_map.get("access_policy_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'access_policy_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#namespace_path: {
                        let field_value = match fields_map.get("namespace_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'namespace_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#nfs_export: {
                        let field_value = match fields_map.get("nfs_export") {
                            Some(value) => value,
                            None => bail!("Missing field 'nfs_export' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#target_path: {
                        let field_value = match fields_map.get("target_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'target_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
