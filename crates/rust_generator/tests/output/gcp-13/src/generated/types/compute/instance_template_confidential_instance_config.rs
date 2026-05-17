#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct InstanceTemplateConfidentialInstanceConfig {
    /// Defines the confidential computing technology the instance uses. SEV is an AMD feature. TDX is an Intel feature. One of the following values is required: `SEV`, `SEV_SNP`, `TDX`. `on_host_maintenance` can be set to MIGRATE if `confidential_instance_type` is set to `SEV` and `min_cpu_platform` is set to `"AMD Milan"`. Otherwise, `on_host_maintenance` has to be set to TERMINATE or this will fail to create the VM. If `SEV_SNP`, currently `min_cpu_platform` has to be set to `"AMD Milan"` or this will fail to create the VM.
    #[builder(into)]
    #[serde(rename = "confidentialInstanceType")]
    pub r#confidential_instance_type: Option<String>,
    /// Defines whether the instance should have confidential compute enabled with AMD SEV. If enabled, `on_host_maintenance` can be set to MIGRATE if `min_cpu_platform` is set to `"AMD Milan"`. Otherwise, `on_host_maintenance` has to be set to TERMINATE or this will fail to create the VM.
    #[builder(into)]
    #[serde(rename = "enableConfidentialCompute")]
    pub r#enable_confidential_compute: Option<bool>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for InstanceTemplateConfidentialInstanceConfig {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "confidential_instance_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#confidential_instance_type,
                )
                .await,
            );
            map.insert(
                "enable_confidential_compute".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#enable_confidential_compute,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for InstanceTemplateConfidentialInstanceConfig {
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
                    r#confidential_instance_type: {
                        let field_value = match fields_map.get("confidential_instance_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'confidential_instance_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#enable_confidential_compute: {
                        let field_value = match fields_map.get("enable_confidential_compute") {
                            Some(value) => value,
                            None => bail!("Missing field 'enable_confidential_compute' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
