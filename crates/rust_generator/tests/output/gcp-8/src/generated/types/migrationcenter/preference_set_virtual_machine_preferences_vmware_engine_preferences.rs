#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreferenceSetVirtualMachinePreferencesVmwareEnginePreferences {
    /// Commitment plan to consider when calculating costs for virtual machine insights and recommendations. If you are unsure which value to set, a 3 year commitment plan is often a good value to start with. Possible values: `COMMITMENT_PLAN_UNSPECIFIED`, `ON_DEMAND`, `COMMITMENT_1_YEAR_MONTHLY_PAYMENTS`, `COMMITMENT_3_YEAR_MONTHLY_PAYMENTS`, `COMMITMENT_1_YEAR_UPFRONT_PAYMENT`, `COMMITMENT_3_YEAR_UPFRONT_PAYMENT`,
    #[builder(into)]
    #[serde(rename = "commitmentPlan")]
    pub r#commitment_plan: Option<String>,
    /// CPU overcommit ratio. Acceptable values are between 1.0 and 8.0, with 0.1 increment.
    #[builder(into)]
    #[serde(rename = "cpuOvercommitRatio")]
    pub r#cpu_overcommit_ratio: Option<f64>,
    /// Memory overcommit ratio. Acceptable values are 1.0, 1.25, 1.5, 1.75 and 2.0.
    #[builder(into)]
    #[serde(rename = "memoryOvercommitRatio")]
    pub r#memory_overcommit_ratio: Option<f64>,
    /// The Deduplication and Compression ratio is based on the logical (Used Before) space required to store data before applying deduplication and compression, in relation to the physical (Used After) space required after applying deduplication and compression. Specifically, the ratio is the Used Before space divided by the Used After space. For example, if the Used Before space is 3 GB, but the physical Used After space is 1 GB, the deduplication and compression ratio is 3x. Acceptable values are between 1.0 and 4.0.
    #[builder(into)]
    #[serde(rename = "storageDeduplicationCompressionRatio")]
    pub r#storage_deduplication_compression_ratio: Option<f64>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreferenceSetVirtualMachinePreferencesVmwareEnginePreferences {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;

        async move {
            use std::collections::BTreeMap;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue;
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue;

            let mut map: BTreeMap<String, PulumiValue> = BTreeMap::new();
            map.insert(
                "commitment_plan".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#commitment_plan,
                )
                .await,
            );
            map.insert(
                "cpu_overcommit_ratio".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#cpu_overcommit_ratio,
                )
                .await,
            );
            map.insert(
                "memory_overcommit_ratio".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#memory_overcommit_ratio,
                )
                .await,
            );
            map.insert(
                "storage_deduplication_compression_ratio".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#storage_deduplication_compression_ratio,
                )
                .await,
            );

            ToPulumiValue::to_pulumi_value(
                &map,
            )
            .await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreferenceSetVirtualMachinePreferencesVmwareEnginePreferences {
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
                    r#commitment_plan: {
                        let field_value = match fields_map.get("commitment_plan") {
                            Some(value) => value,
                            None => bail!("Missing field 'commitment_plan' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#cpu_overcommit_ratio: {
                        let field_value = match fields_map.get("cpu_overcommit_ratio") {
                            Some(value) => value,
                            None => bail!("Missing field 'cpu_overcommit_ratio' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#memory_overcommit_ratio: {
                        let field_value = match fields_map.get("memory_overcommit_ratio") {
                            Some(value) => value,
                            None => bail!("Missing field 'memory_overcommit_ratio' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#storage_deduplication_compression_ratio: {
                        let field_value = match fields_map.get("storage_deduplication_compression_ratio") {
                            Some(value) => value,
                            None => bail!("Missing field 'storage_deduplication_compression_ratio' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
