#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PatchDeploymentPatchConfigWindowsUpdate {
    /// Only apply updates of these windows update classifications. If empty, all updates are applied.
    /// Each value may be one of: `CRITICAL`, `SECURITY`, `DEFINITION`, `DRIVER`, `FEATURE_PACK`, `SERVICE_PACK`, `TOOL`, `UPDATE_ROLLUP`, `UPDATE`.
    #[builder(into)]
    #[serde(rename = "classifications")]
    pub r#classifications: Option<Vec<String>>,
    /// List of KBs to exclude from update.
    #[builder(into)]
    #[serde(rename = "excludes")]
    pub r#excludes: Option<Vec<String>>,
    /// An exclusive list of kbs to be updated. These are the only patches that will be updated.
    /// This field must not be used with other patch configurations.
    #[builder(into)]
    #[serde(rename = "exclusivePatches")]
    pub r#exclusive_patches: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PatchDeploymentPatchConfigWindowsUpdate {
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
                "classifications".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#classifications,
                )
                .await,
            );
            map.insert(
                "excludes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#excludes,
                )
                .await,
            );
            map.insert(
                "exclusive_patches".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#exclusive_patches,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PatchDeploymentPatchConfigWindowsUpdate {
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
                    r#classifications: {
                        let field_value = match fields_map.get("classifications") {
                            Some(value) => value,
                            None => bail!("Missing field 'classifications' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#excludes: {
                        let field_value = match fields_map.get("excludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'excludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclusive_patches: {
                        let field_value = match fields_map.get("exclusive_patches") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusive_patches' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
