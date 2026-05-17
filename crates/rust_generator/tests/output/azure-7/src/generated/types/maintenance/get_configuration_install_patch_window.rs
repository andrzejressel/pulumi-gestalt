#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetConfigurationInstallPatchWindow {
    /// List of Classification category of patches to be patched.
    #[builder(into)]
    #[serde(rename = "classificationsToIncludes")]
    pub r#classifications_to_includes: Vec<String>,
    /// List of KB numbers to be excluded from patching.
    #[builder(into)]
    #[serde(rename = "kbNumbersToExcludes")]
    pub r#kb_numbers_to_excludes: Vec<String>,
    /// List of KB numbers to be included for patching.
    #[builder(into)]
    #[serde(rename = "kbNumbersToIncludes")]
    pub r#kb_numbers_to_includes: Vec<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetConfigurationInstallPatchWindow {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_concurrent, to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "classifications_to_includes",
                    &self.r#classifications_to_includes,
                ),
                to_pulumi_object_field(
                    "kb_numbers_to_excludes",
                    &self.r#kb_numbers_to_excludes,
                ),
                to_pulumi_object_field(
                    "kb_numbers_to_includes",
                    &self.r#kb_numbers_to_includes,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetConfigurationInstallPatchWindow {
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
                    r#classifications_to_includes: {
                        let field_value = match fields_map.get("classifications_to_includes") {
                            Some(value) => value,
                            None => bail!("Missing field 'classifications_to_includes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kb_numbers_to_excludes: {
                        let field_value = match fields_map.get("kb_numbers_to_excludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'kb_numbers_to_excludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kb_numbers_to_includes: {
                        let field_value = match fields_map.get("kb_numbers_to_includes") {
                            Some(value) => value,
                            None => bail!("Missing field 'kb_numbers_to_includes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
