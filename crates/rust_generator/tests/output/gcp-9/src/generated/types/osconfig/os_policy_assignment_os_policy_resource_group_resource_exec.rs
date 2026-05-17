#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OsPolicyAssignmentOsPolicyResourceGroupResourceExec {
    /// What to run to bring this resource into the desired
    /// state. An exit code of 100 indicates "success", any other exit code
    /// indicates a failure running enforce. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "enforce")]
    pub r#enforce: Option<Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceExecEnforce>>,
    /// What to run to validate this resource is in the
    /// desired state. An exit code of 100 indicates "in desired state", and exit
    /// code of 101 indicates "not in desired state". Any other exit code indicates
    /// a failure running validate. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "validate")]
    pub r#validate: Box<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroupResourceExecValidate>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OsPolicyAssignmentOsPolicyResourceGroupResourceExec {
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
                "enforce".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enforce,
                )
                .await,
            );
            map.insert(
                "validate".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#validate,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OsPolicyAssignmentOsPolicyResourceGroupResourceExec {
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
                    r#enforce: {
                        let field_value = match fields_map.get("enforce") {
                            Some(value) => value,
                            None => bail!("Missing field 'enforce' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#validate: {
                        let field_value = match fields_map.get("validate") {
                            Some(value) => value,
                            None => bail!("Missing field 'validate' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
