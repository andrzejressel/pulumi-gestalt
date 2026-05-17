#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterNodeConfigLinuxNodeConfigHugepagesConfig {
    /// Amount of 1G hugepages.
    #[builder(into)]
    #[serde(rename = "hugepageSize1g")]
    pub r#hugepage_size_1_g: Option<i32>,
    /// Amount of 2M hugepages.
    #[builder(into)]
    #[serde(rename = "hugepageSize2m")]
    pub r#hugepage_size_2_m: Option<i32>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterNodeConfigLinuxNodeConfigHugepagesConfig {
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
                "hugepage_size_1_g".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hugepage_size_1_g,
                )
                .await,
            );
            map.insert(
                "hugepage_size_2_m".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#hugepage_size_2_m,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterNodeConfigLinuxNodeConfigHugepagesConfig {
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
                    r#hugepage_size_1_g: {
                        let field_value = match fields_map.get("hugepage_size_1_g") {
                            Some(value) => value,
                            None => bail!("Missing field 'hugepage_size_1_g' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#hugepage_size_2_m: {
                        let field_value = match fields_map.get("hugepage_size_2_m") {
                            Some(value) => value,
                            None => bail!("Missing field 'hugepage_size_2_m' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
