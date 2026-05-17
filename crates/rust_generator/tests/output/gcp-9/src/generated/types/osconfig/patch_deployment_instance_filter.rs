#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PatchDeploymentInstanceFilter {
    /// Target all VM instances in the project. If true, no other criteria is permitted.
    #[builder(into)]
    #[serde(rename = "all")]
    pub r#all: Option<bool>,
    /// Targets VM instances matching ANY of these GroupLabels. This allows targeting of disparate groups of VM instances.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "groupLabels")]
    pub r#group_labels: Option<Vec<super::super::types::osconfig::PatchDeploymentInstanceFilterGroupLabel>>,
    /// Targets VMs whose name starts with one of these prefixes. Similar to labels, this is another way to group
    /// VMs when targeting configs, for example prefix="prod-".
    #[builder(into)]
    #[serde(rename = "instanceNamePrefixes")]
    pub r#instance_name_prefixes: Option<Vec<String>>,
    /// Targets any of the VM instances specified. Instances are specified by their URI in the `form zones/{{zone}}/instances/{{instance_name}}`,
    /// `projects/{{project_id}}/zones/{{zone}}/instances/{{instance_name}}`, or
    /// `https://www.googleapis.com/compute/v1/projects/{{project_id}}/zones/{{zone}}/instances/{{instance_name}}`
    #[builder(into)]
    #[serde(rename = "instances")]
    pub r#instances: Option<Vec<String>>,
    /// Targets VM instances in ANY of these zones. Leave empty to target VM instances in any zone.
    #[builder(into)]
    #[serde(rename = "zones")]
    pub r#zones: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PatchDeploymentInstanceFilter {
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
                    "all",
                    &self.r#all,
                ),
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
                    "zones",
                    &self.r#zones,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PatchDeploymentInstanceFilter {
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
                    r#all: {
                        let field_value = match fields_map.get("all") {
                            Some(value) => value,
                            None => bail!("Missing field 'all' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
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
