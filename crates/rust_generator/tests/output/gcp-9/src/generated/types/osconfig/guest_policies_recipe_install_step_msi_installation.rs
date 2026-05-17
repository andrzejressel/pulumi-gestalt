#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuestPoliciesRecipeInstallStepMsiInstallation {
    /// Return codes that indicate that the software installed or updated successfully. Behaviour defaults to [0]
    #[builder(into)]
    #[serde(rename = "allowedExitCodes")]
    pub r#allowed_exit_codes: Option<Vec<i32>>,
    /// The id of the relevant artifact in the recipe.
    #[builder(into)]
    #[serde(rename = "artifactId")]
    pub r#artifact_id: String,
    /// The flags to use when installing the MSI. Defaults to the install flag.
    #[builder(into)]
    #[serde(rename = "flags")]
    pub r#flags: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GuestPoliciesRecipeInstallStepMsiInstallation {
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
                "allowed_exit_codes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#allowed_exit_codes,
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
                "flags".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#flags,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GuestPoliciesRecipeInstallStepMsiInstallation {
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
                    r#artifact_id: {
                        let field_value = match fields_map.get("artifact_id") {
                            Some(value) => value,
                            None => bail!("Missing field 'artifact_id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#flags: {
                        let field_value = match fields_map.get("flags") {
                            Some(value) => value,
                            None => bail!("Missing field 'flags' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
