#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CustomTargetTypeCustomActions {
    /// The Skaffold custom action responsible for deploy operations.
    #[builder(into)]
    #[serde(rename = "deployAction")]
    pub r#deploy_action: String,
    /// List of Skaffold modules Cloud Deploy will include in the Skaffold Config as required before performing diagnose.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "includeSkaffoldModules")]
    pub r#include_skaffold_modules: Option<Vec<super::super::types::clouddeploy::CustomTargetTypeCustomActionsIncludeSkaffoldModule>>,
    /// The Skaffold custom action responsible for render operations. If not provided then Cloud Deploy will perform the render operations via `skaffold render`.
    #[builder(into)]
    #[serde(rename = "renderAction")]
    pub r#render_action: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CustomTargetTypeCustomActions {
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
                "deploy_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#deploy_action,
                )
                .await,
            );
            map.insert(
                "include_skaffold_modules".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#include_skaffold_modules,
                )
                .await,
            );
            map.insert(
                "render_action".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#render_action,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CustomTargetTypeCustomActions {
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
                    r#deploy_action: {
                        let field_value = match fields_map.get("deploy_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'deploy_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#include_skaffold_modules: {
                        let field_value = match fields_map.get("include_skaffold_modules") {
                            Some(value) => value,
                            None => bail!("Missing field 'include_skaffold_modules' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#render_action: {
                        let field_value = match fields_map.get("render_action") {
                            Some(value) => value,
                            None => bail!("Missing field 'render_action' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
