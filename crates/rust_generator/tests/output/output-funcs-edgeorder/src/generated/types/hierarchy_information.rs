#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct HierarchyInformation {
    /// Represents configuration name that uniquely identifies configuration
    #[builder(into)]
    #[serde(rename = "configurationName")]
    pub r#configuration_name: Option<String>,
    /// Represents product family name that uniquely identifies product family
    #[builder(into)]
    #[serde(rename = "productFamilyName")]
    pub r#product_family_name: Option<String>,
    /// Represents product line name that uniquely identifies product line
    #[builder(into)]
    #[serde(rename = "productLineName")]
    pub r#product_line_name: Option<String>,
    /// Represents product name that uniquely identifies product
    #[builder(into)]
    #[serde(rename = "productName")]
    pub r#product_name: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for HierarchyInformation {
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
                "configuration_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#configuration_name,
                )
                .await,
            );
            map.insert(
                "product_family_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#product_family_name,
                )
                .await,
            );
            map.insert(
                "product_line_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#product_line_name,
                )
                .await,
            );
            map.insert(
                "product_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#product_name,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for HierarchyInformation {
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
                    r#configuration_name: {
                        let field_value = match fields_map.get("configuration_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'configuration_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#product_family_name: {
                        let field_value = match fields_map.get("product_family_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'product_family_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#product_line_name: {
                        let field_value = match fields_map.get("product_line_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'product_line_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#product_name: {
                        let field_value = match fields_map.get("product_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'product_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
