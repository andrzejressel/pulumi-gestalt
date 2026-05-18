#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuestPoliciesRecipe {
    /// Resources available to be used in the steps in the recipe.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "artifacts")]
    pub r#artifacts: Option<Vec<super::super::types::osconfig::GuestPoliciesRecipeArtifact>>,
    /// Default is INSTALLED. The desired state the agent should maintain for this recipe.
    /// INSTALLED: The software recipe is installed on the instance but won't be updated to new versions.
    /// INSTALLED_KEEP_UPDATED: The software recipe is installed on the instance. The recipe is updated to a higher version,
    /// if a higher version of the recipe is assigned to this instance.
    /// REMOVE: Remove is unsupported for software recipes and attempts to create or update a recipe to the REMOVE state is rejected.
    /// Default value is `INSTALLED`.
    /// Possible values are: `INSTALLED`, `UPDATED`, `REMOVED`.
    #[builder(into)]
    #[serde(rename = "desiredState")]
    pub r#desired_state: Option<String>,
    /// Actions to be taken for installing this recipe. On failure it stops executing steps and does not attempt another installation.
    /// Any steps taken (including partially completed steps) are not rolled back.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "installSteps")]
    pub r#install_steps: Option<Vec<super::super::types::osconfig::GuestPoliciesRecipeInstallStep>>,
    /// Unique identifier for the recipe. Only one recipe with a given name is installed on an instance.
    /// Names are also used to identify resources which helps to determine whether guest policies have conflicts.
    /// This means that requests to create multiple recipes with the same name and version are rejected since they
    /// could potentially have conflicting assignments.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
    /// Actions to be taken for updating this recipe. On failure it stops executing steps and does not attempt another update for this recipe.
    /// Any steps taken (including partially completed steps) are not rolled back.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "updateSteps")]
    pub r#update_steps: Option<Vec<super::super::types::osconfig::GuestPoliciesRecipeUpdateStep>>,
    /// The version of this software recipe. Version can be up to 4 period separated numbers (e.g. 12.34.56.78).
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GuestPoliciesRecipe {
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
                    "artifacts",
                    &self.r#artifacts,
                ),
                to_pulumi_object_field(
                    "desired_state",
                    &self.r#desired_state,
                ),
                to_pulumi_object_field(
                    "install_steps",
                    &self.r#install_steps,
                ),
                to_pulumi_object_field(
                    "name",
                    &self.r#name,
                ),
                to_pulumi_object_field(
                    "update_steps",
                    &self.r#update_steps,
                ),
                to_pulumi_object_field(
                    "version",
                    &self.r#version,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GuestPoliciesRecipe {
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
                    r#artifacts: {
                        let field_value = match fields_map.get("artifacts") {
                            Some(value) => value,
                            None => bail!("Missing field 'artifacts' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#desired_state: {
                        let field_value = match fields_map.get("desired_state") {
                            Some(value) => value,
                            None => bail!("Missing field 'desired_state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#install_steps: {
                        let field_value = match fields_map.get("install_steps") {
                            Some(value) => value,
                            None => bail!("Missing field 'install_steps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#update_steps: {
                        let field_value = match fields_map.get("update_steps") {
                            Some(value) => value,
                            None => bail!("Missing field 'update_steps' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#version: {
                        let field_value = match fields_map.get("version") {
                            Some(value) => value,
                            None => bail!("Missing field 'version' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
