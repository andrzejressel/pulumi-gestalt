#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PatchDeploymentPatchConfigZypper {
    /// Install only patches with these categories. Common categories include security, recommended, and feature.
    #[builder(into)]
    #[serde(rename = "categories")]
    pub r#categories: Option<Vec<String>>,
    /// List of packages to exclude from update.
    #[builder(into)]
    #[serde(rename = "excludes")]
    pub r#excludes: Option<Vec<String>>,
    /// An exclusive list of patches to be updated. These are the only patches that will be installed using 'zypper patch patch:' command.
    /// This field must not be used with any other patch configuration fields.
    #[builder(into)]
    #[serde(rename = "exclusivePatches")]
    pub r#exclusive_patches: Option<Vec<String>>,
    /// Install only patches with these severities. Common severities include critical, important, moderate, and low.
    #[builder(into)]
    #[serde(rename = "severities")]
    pub r#severities: Option<Vec<String>>,
    /// Adds the --with-optional flag to zypper patch.
    #[builder(into)]
    #[serde(rename = "withOptional")]
    pub r#with_optional: Option<bool>,
    /// Adds the --with-update flag, to zypper patch.
    #[builder(into)]
    #[serde(rename = "withUpdate")]
    pub r#with_update: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PatchDeploymentPatchConfigZypper {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "categories",
                    &self.r#categories,
                ),
                to_pulumi_object_field(
                    "excludes",
                    &self.r#excludes,
                ),
                to_pulumi_object_field(
                    "exclusive_patches",
                    &self.r#exclusive_patches,
                ),
                to_pulumi_object_field(
                    "severities",
                    &self.r#severities,
                ),
                to_pulumi_object_field(
                    "with_optional",
                    &self.r#with_optional,
                ),
                to_pulumi_object_field(
                    "with_update",
                    &self.r#with_update,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PatchDeploymentPatchConfigZypper {
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
                    r#categories: {
                        let field_value = match fields_map.get("categories") {
                            Some(value) => value,
                            None => bail!("Missing field 'categories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#severities: {
                        let field_value = match fields_map.get("severities") {
                            Some(value) => value,
                            None => bail!("Missing field 'severities' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#with_optional: {
                        let field_value = match fields_map.get("with_optional") {
                            Some(value) => value,
                            None => bail!("Missing field 'with_optional' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#with_update: {
                        let field_value = match fields_map.get("with_update") {
                            Some(value) => value,
                            None => bail!("Missing field 'with_update' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
