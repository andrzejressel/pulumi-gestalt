#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ClusterSku {
    /// Specifies the node count for the cluster. Boundaries depend on the SKU name.
    /// 
    /// > **NOTE:** If no `optimized_auto_scale` block is defined, then the capacity is required.
    /// > **NOTE:** If an `optimized_auto_scale` block is defined and no capacity is set, then the capacity is initially set to the value of `minimum_instances`.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: Option<i32>,
    /// The name of the SKU. Possible values are `Dev(No SLA)_Standard_D11_v2`, `Dev(No SLA)_Standard_E2a_v4`, `Standard_D14_v2`, `Standard_D11_v2`, `Standard_D16d_v5`, `Standard_D13_v2`, `Standard_D12_v2`, `Standard_DS14_v2+4TB_PS`, `Standard_DS14_v2+3TB_PS`, `Standard_DS13_v2+1TB_PS`, `Standard_DS13_v2+2TB_PS`, `Standard_D32d_v5`, `Standard_D32d_v4`, `Standard_EC8ads_v5`, `Standard_EC8as_v5+1TB_PS`, `Standard_EC8as_v5+2TB_PS`, `Standard_EC16ads_v5`, `Standard_EC16as_v5+4TB_PS`, `Standard_EC16as_v5+3TB_PS`, `Standard_E80ids_v4`, `Standard_E8a_v4`, `Standard_E8ads_v5`, `Standard_E8as_v5+1TB_PS`, `Standard_E8as_v5+2TB_PS`, `Standard_E8as_v4+1TB_PS`, `Standard_E8as_v4+2TB_PS`, `Standard_E8d_v5`, `Standard_E8d_v4`, `Standard_E8s_v5+1TB_PS`, `Standard_E8s_v5+2TB_PS`, `Standard_E8s_v4+1TB_PS`, `Standard_E8s_v4+2TB_PS`, `Standard_E4a_v4`, `Standard_E4ads_v5`, `Standard_E4d_v5`, `Standard_E4d_v4`, `Standard_E16a_v4`, `Standard_E16ads_v5`, `Standard_E16as_v5+4TB_PS`, `Standard_E16as_v5+3TB_PS`, `Standard_E16as_v4+4TB_PS`, `Standard_E16as_v4+3TB_PS`, `Standard_E16d_v5`, `Standard_E16d_v4`, `Standard_E16s_v5+4TB_PS`, `Standard_E16s_v5+3TB_PS`, `Standard_E16s_v4+4TB_PS`, `Standard_E16s_v4+3TB_PS`, `Standard_E64i_v3`, `Standard_E2a_v4`, `Standard_E2ads_v5`, `Standard_E2d_v5`, `Standard_E2d_v4`, `Standard_L8as_v3`, `Standard_L8s`, `Standard_L8s_v3`, `Standard_L8s_v2`, `Standard_L4s`, `Standard_L16as_v3`, `Standard_L16s`, `Standard_L16s_v3`, `Standard_L16s_v2`, `Standard_L32as_v3` and `Standard_L32s_v3`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ClusterSku {
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
                "capacity".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#capacity,
                )
                .await,
            );
            map.insert(
                "name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ClusterSku {
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
                    r#capacity: {
                        let field_value = match fields_map.get("capacity") {
                            Some(value) => value,
                            None => bail!("Missing field 'capacity' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#name: {
                        let field_value = match fields_map.get("name") {
                            Some(value) => value,
                            None => bail!("Missing field 'name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
