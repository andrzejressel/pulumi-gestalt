#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PatchDeploymentPatchConfigYum {
    /// List of packages to exclude from update. These packages will be excluded.
    #[builder(into)]
    #[serde(rename = "excludes")]
    pub r#excludes: Option<Vec<String>>,
    /// An exclusive list of packages to be updated. These are the only packages that will be updated.
    /// If these packages are not installed, they will be ignored. This field cannot be specified with
    /// any other patch configuration fields.
    #[builder(into)]
    #[serde(rename = "exclusivePackages")]
    pub r#exclusive_packages: Option<Vec<String>>,
    /// Will cause patch to run yum update-minimal instead.
    #[builder(into)]
    #[serde(rename = "minimal")]
    pub r#minimal: Option<bool>,
    /// Adds the --security flag to yum update. Not supported on all platforms.
    #[builder(into)]
    #[serde(rename = "security")]
    pub r#security: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PatchDeploymentPatchConfigYum {
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
                    "excludes",
                    &self.r#excludes,
                ),
                to_pulumi_object_field(
                    "exclusive_packages",
                    &self.r#exclusive_packages,
                ),
                to_pulumi_object_field(
                    "minimal",
                    &self.r#minimal,
                ),
                to_pulumi_object_field(
                    "security",
                    &self.r#security,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PatchDeploymentPatchConfigYum {
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
                    r#excludes: {
                        let field_value = match fields_map.get("excludes") {
                            Some(value) => value,
                            None => bail!("Missing field 'excludes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#exclusive_packages: {
                        let field_value = match fields_map.get("exclusive_packages") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusive_packages' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#minimal: {
                        let field_value = match fields_map.get("minimal") {
                            Some(value) => value,
                            None => bail!("Missing field 'minimal' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#security: {
                        let field_value = match fields_map.get("security") {
                            Some(value) => value,
                            None => bail!("Missing field 'security' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
