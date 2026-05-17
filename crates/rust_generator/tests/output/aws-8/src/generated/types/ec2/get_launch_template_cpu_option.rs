#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetLaunchTemplateCpuOption {
    #[builder(into)]
    #[serde(rename = "amdSevSnp")]
    pub r#amd_sev_snp: String,
    #[builder(into)]
    #[serde(rename = "coreCount")]
    pub r#core_count: i32,
    #[builder(into)]
    #[serde(rename = "threadsPerCore")]
    pub r#threads_per_core: i32,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetLaunchTemplateCpuOption {
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
                "amd_sev_snp".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#amd_sev_snp,
                )
                .await,
            );
            map.insert(
                "core_count".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#core_count,
                )
                .await,
            );
            map.insert(
                "threads_per_core".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#threads_per_core,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetLaunchTemplateCpuOption {
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
                    r#amd_sev_snp: {
                        let field_value = match fields_map.get("amd_sev_snp") {
                            Some(value) => value,
                            None => bail!("Missing field 'amd_sev_snp' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#core_count: {
                        let field_value = match fields_map.get("core_count") {
                            Some(value) => value,
                            None => bail!("Missing field 'core_count' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#threads_per_core: {
                        let field_value = match fields_map.get("threads_per_core") {
                            Some(value) => value,
                            None => bail!("Missing field 'threads_per_core' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
