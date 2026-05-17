#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuestPoliciesRecipeInstallStepFileExec {
    /// A list of possible return values that the program can return to indicate a success. Defaults to [0].
    #[builder(into)]
    #[serde(rename = "allowedExitCodes")]
    pub r#allowed_exit_codes: Option<String>,
    /// Arguments to be passed to the provided executable.
    #[builder(into)]
    #[serde(rename = "args")]
    pub r#args: Option<Vec<String>>,
    /// The id of the relevant artifact in the recipe.
    #[builder(into)]
    #[serde(rename = "artifactId")]
    pub r#artifact_id: Option<String>,
    /// The absolute path of the file on the local filesystem.
    #[builder(into)]
    #[serde(rename = "localPath")]
    pub r#local_path: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GuestPoliciesRecipeInstallStepFileExec {
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
                "allowed_exit_codes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_exit_codes,
                )
                .await,
            );
            map.insert(
                "args".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#args,
                )
                .await,
            );
            map.insert(
                "artifact_id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#artifact_id,
                )
                .await,
            );
            map.insert(
                "local_path".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#local_path,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GuestPoliciesRecipeInstallStepFileExec {
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
                    r#allowed_exit_codes: {
                        let field_value = match fields_map.get("allowed_exit_codes") {
                            Some(value) => value,
                            None => bail!("Missing field 'allowed_exit_codes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#args: {
                        let field_value = match fields_map.get("args") {
                            Some(value) => value,
                            None => bail!("Missing field 'args' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#artifact_id: {
                        let field_value = match fields_map.get("artifact_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'artifact_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#local_path: {
                        let field_value = match fields_map.get("local_path") {
                            Some(value) => value,
                            None => bail!("Missing field 'local_path' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
