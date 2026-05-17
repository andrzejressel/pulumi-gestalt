#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OsPolicyAssignmentOsPolicy {
    /// This flag determines the OS
    /// policy compliance status when none of the resource groups within the policy
    /// are applicable for a VM. Set this value to `true` if the policy needs to be
    /// reported as compliant even if the policy has nothing to validate or enforce.
    #[builder(into)]
    #[serde(rename = "allowNoResourceGroupMatch")]
    pub r#allow_no_resource_group_match: Option<bool>,
    /// Policy description. Length of the description is
    /// limited to 1024 characters.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Option<String>,
    /// The id of the OS policy with the following restrictions:
    /// 
    /// *   Must contain only lowercase letters, numbers, and hyphens.
    /// *   Must start with a letter.
    /// *   Must be between 1-63 characters.
    /// *   Must end with a number or a letter.
    /// *   Must be unique within the assignment.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: String,
    /// Policy mode Possible values are: `MODE_UNSPECIFIED`,
    /// `VALIDATION`, `ENFORCEMENT`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: String,
    /// List of resource groups for the policy. For a
    /// particular VM, resource groups are evaluated in the order specified and the
    /// first resource group that is applicable is selected and the rest are
    /// ignored. If none of the resource groups are applicable for a VM, the VM is
    /// considered to be non-compliant w.r.t this policy. This behavior can be
    /// toggled by the flag `allow_no_resource_group_match` Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "resourceGroups")]
    pub r#resource_groups: Vec<super::super::types::osconfig::OsPolicyAssignmentOsPolicyResourceGroup>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OsPolicyAssignmentOsPolicy {
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
                    "allow_no_resource_group_match",
                    &self.r#allow_no_resource_group_match,
                ),
                to_pulumi_object_field(
                    "description",
                    &self.r#description,
                ),
                to_pulumi_object_field(
                    "id",
                    &self.r#id,
                ),
                to_pulumi_object_field(
                    "mode",
                    &self.r#mode,
                ),
                to_pulumi_object_field(
                    "resource_groups",
                    &self.r#resource_groups,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OsPolicyAssignmentOsPolicy {
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
                    r#allow_no_resource_group_match: {
                        let field_value = match fields_map.get("allow_no_resource_group_match") {
                            Some(value) => value,
                            None => bail!("Missing field 'allow_no_resource_group_match' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#description: {
                        let field_value = match fields_map.get("description") {
                            Some(value) => value,
                            None => bail!("Missing field 'description' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mode: {
                        let field_value = match fields_map.get("mode") {
                            Some(value) => value,
                            None => bail!("Missing field 'mode' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#resource_groups: {
                        let field_value = match fields_map.get("resource_groups") {
                            Some(value) => value,
                            None => bail!("Missing field 'resource_groups' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
