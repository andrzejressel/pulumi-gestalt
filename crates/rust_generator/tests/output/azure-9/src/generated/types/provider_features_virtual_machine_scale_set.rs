#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ProviderFeaturesVirtualMachineScaleSet {
    #[builder(into)]
    #[serde(rename = "forceDelete")]
    pub r#force_delete: Option<bool>,
    #[builder(into)]
    #[serde(rename = "reimageOnManualUpgrade")]
    pub r#reimage_on_manual_upgrade: Option<bool>,
    #[builder(into)]
    #[serde(rename = "rollInstancesWhenRequired")]
    pub r#roll_instances_when_required: Option<bool>,
    #[builder(into)]
    #[serde(rename = "scaleToZeroBeforeDeletion")]
    pub r#scale_to_zero_before_deletion: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ProviderFeaturesVirtualMachineScaleSet {
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
                    "force_delete",
                    &self.r#force_delete,
                ),
                to_pulumi_object_field(
                    "reimage_on_manual_upgrade",
                    &self.r#reimage_on_manual_upgrade,
                ),
                to_pulumi_object_field(
                    "roll_instances_when_required",
                    &self.r#roll_instances_when_required,
                ),
                to_pulumi_object_field(
                    "scale_to_zero_before_deletion",
                    &self.r#scale_to_zero_before_deletion,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ProviderFeaturesVirtualMachineScaleSet {
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
                    r#force_delete: {
                        let field_value = match fields_map.get("force_delete") {
                            Some(value) => value,
                            None => bail!("Missing field 'force_delete' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reimage_on_manual_upgrade: {
                        let field_value = match fields_map.get("reimage_on_manual_upgrade") {
                            Some(value) => value,
                            None => bail!("Missing field 'reimage_on_manual_upgrade' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#roll_instances_when_required: {
                        let field_value = match fields_map.get("roll_instances_when_required") {
                            Some(value) => value,
                            None => bail!("Missing field 'roll_instances_when_required' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#scale_to_zero_before_deletion: {
                        let field_value = match fields_map.get("scale_to_zero_before_deletion") {
                            Some(value) => value,
                            None => bail!("Missing field 'scale_to_zero_before_deletion' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
