#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct OsPolicyAssignmentInstanceFilter {
    /// Target all VMs in the project. If true, no other criteria
    /// is permitted.
    #[builder(into)]
    #[serde(rename = "all")]
    pub r#all: Option<bool>,
    /// List of label sets used for VM exclusion. If
    /// the list has more than one label set, the VM is excluded if any of the label
    /// sets are applicable for the VM. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "exclusionLabels")]
    pub r#exclusion_labels: Option<Vec<super::super::types::osconfig::OsPolicyAssignmentInstanceFilterExclusionLabel>>,
    /// List of label sets used for VM inclusion. If
    /// the list has more than one `LabelSet`, the VM is included if any of the
    /// label sets are applicable for the VM. Structure is
    /// documented below.
    #[builder(into)]
    #[serde(rename = "inclusionLabels")]
    pub r#inclusion_labels: Option<Vec<super::super::types::osconfig::OsPolicyAssignmentInstanceFilterInclusionLabel>>,
    /// List of inventories to select VMs. A VM is
    /// selected if its inventory data matches at least one of the following
    /// inventories. Structure is documented below.
    #[builder(into)]
    #[serde(rename = "inventories")]
    pub r#inventories: Option<Vec<super::super::types::osconfig::OsPolicyAssignmentInstanceFilterInventory>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for OsPolicyAssignmentInstanceFilter {
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
                    "all",
                    &self.r#all,
                ),
                to_pulumi_object_field(
                    "exclusion_labels",
                    &self.r#exclusion_labels,
                ),
                to_pulumi_object_field(
                    "inclusion_labels",
                    &self.r#inclusion_labels,
                ),
                to_pulumi_object_field(
                    "inventories",
                    &self.r#inventories,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for OsPolicyAssignmentInstanceFilter {
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
                    r#exclusion_labels: {
                        let field_value = match fields_map.get("exclusion_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'exclusion_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inclusion_labels: {
                        let field_value = match fields_map.get("inclusion_labels") {
                            Some(value) => value,
                            None => bail!("Missing field 'inclusion_labels' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#inventories: {
                        let field_value = match fields_map.get("inventories") {
                            Some(value) => value,
                            None => bail!("Missing field 'inventories' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
