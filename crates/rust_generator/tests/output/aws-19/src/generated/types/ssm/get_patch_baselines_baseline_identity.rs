#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetPatchBaselinesBaselineIdentity {
    /// Description of the patch baseline.
    #[builder(into)]
    #[serde(rename = "baselineDescription")]
    pub r#baseline_description: String,
    /// ID of the patch baseline.
    #[builder(into)]
    #[serde(rename = "baselineId")]
    pub r#baseline_id: String,
    /// Name of the patch baseline.
    #[builder(into)]
    #[serde(rename = "baselineName")]
    pub r#baseline_name: String,
    /// Indicates whether this is the default baseline. AWS Systems Manager supports creating multiple default patch baselines. For example, you can create a default patch baseline for each operating system.
    #[builder(into)]
    #[serde(rename = "defaultBaseline")]
    pub r#default_baseline: bool,
    /// Operating system the patch baseline applies to.
    #[builder(into)]
    #[serde(rename = "operatingSystem")]
    pub r#operating_system: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetPatchBaselinesBaselineIdentity {
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
                    "baselineDescription",
                    &self.r#baseline_description,
                ),
                to_pulumi_object_field(
                    "baselineId",
                    &self.r#baseline_id,
                ),
                to_pulumi_object_field(
                    "baselineName",
                    &self.r#baseline_name,
                ),
                to_pulumi_object_field(
                    "defaultBaseline",
                    &self.r#default_baseline,
                ),
                to_pulumi_object_field(
                    "operatingSystem",
                    &self.r#operating_system,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetPatchBaselinesBaselineIdentity {
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
                    r#baseline_description: {
                        let field_value = match fields_map.get("baselineDescription") {
                            Some(value) => value,
                            None => bail!("Missing field 'baselineDescription' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#baseline_id: {
                        let field_value = match fields_map.get("baselineId") {
                            Some(value) => value,
                            None => bail!("Missing field 'baselineId' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#baseline_name: {
                        let field_value = match fields_map.get("baselineName") {
                            Some(value) => value,
                            None => bail!("Missing field 'baselineName' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#default_baseline: {
                        let field_value = match fields_map.get("defaultBaseline") {
                            Some(value) => value,
                            None => bail!("Missing field 'defaultBaseline' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#operating_system: {
                        let field_value = match fields_map.get("operatingSystem") {
                            Some(value) => value,
                            None => bail!("Missing field 'operatingSystem' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
