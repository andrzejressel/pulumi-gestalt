#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GuestPoliciesAssignment {
    /// Targets instances matching at least one of these label sets. This allows an assignment to target disparate groups,
    /// for example "env=prod or env=staging".
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "groupLabels")]
    pub r#group_labels: Option<Vec<super::super::types::osconfig::GuestPoliciesAssignmentGroupLabel>>,
    /// Targets VM instances whose name starts with one of these prefixes.
    /// Like labels, this is another way to group VM instances when targeting configs,
    /// for example prefix="prod-".
    /// Only supported for project-level policies.
    #[builder(into)]
    #[serde(rename = "instanceNamePrefixes")]
    pub r#instance_name_prefixes: Option<Vec<String>>,
    /// Targets any of the instances specified. Instances are specified by their URI in the form
    /// zones/[ZONE]/instances/[INSTANCE_NAME].
    /// Instance targeting is uncommon and is supported to facilitate the management of changes
    /// by the instance or to target specific VM instances for development and testing.
    /// Only supported for project-level policies and must reference instances within this project.
    #[builder(into)]
    #[serde(rename = "instances")]
    pub r#instances: Option<Vec<String>>,
    /// Targets VM instances matching at least one of the following OS types.
    /// VM instances must match all supplied criteria for a given OsType to be included.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "osTypes")]
    pub r#os_types: Option<Vec<super::super::types::osconfig::GuestPoliciesAssignmentOsType>>,
    /// Targets instances in any of these zones. Leave empty to target instances in any zone.
    /// Zonal targeting is uncommon and is supported to facilitate the management of changes by zone.
    #[builder(into)]
    #[serde(rename = "zones")]
    pub r#zones: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GuestPoliciesAssignment {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "group_labels",
                    &self.r#group_labels,
                ),
                to_pulumi_object_field(
                    "instance_name_prefixes",
                    &self.r#instance_name_prefixes,
                ),
                to_pulumi_object_field(
                    "instances",
                    &self.r#instances,
                ),
                to_pulumi_object_field(
                    "os_types",
                    &self.r#os_types,
                ),
                to_pulumi_object_field(
                    "zones",
                    &self.r#zones,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GuestPoliciesAssignment {
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
                    r#group_labels: {
                        let field_value = match fields_map.get("group_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'group_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instance_name_prefixes: {
                        let field_value = match fields_map.get("instance_name_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'instance_name_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#instances: {
                        let field_value = match fields_map.get("instances") {
                            Some(value) => value,
                            None => bail!("Missing field 'instances' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#os_types: {
                        let field_value = match fields_map.get("os_types") {
                            Some(value) => value,
                            None => bail!("Missing field 'os_types' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#zones: {
                        let field_value = match fields_map.get("zones") {
                            Some(value) => value,
                            None => bail!("Missing field 'zones' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
