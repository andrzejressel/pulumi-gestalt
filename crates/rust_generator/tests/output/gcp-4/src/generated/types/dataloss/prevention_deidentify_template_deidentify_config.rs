#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct PreventionDeidentifyTemplateDeidentifyConfig {
    /// Treat the dataset as an image and redact.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "imageTransformations")]
    pub r#image_transformations: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigImageTransformations>>,
    /// Treat the dataset as free-form text and apply the same free text transformation everywhere
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "infoTypeTransformations")]
    pub r#info_type_transformations: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigInfoTypeTransformations>>,
    /// Treat the dataset as structured. Transformations can be applied to specific locations within structured datasets, such as transforming a column within a table.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "recordTransformations")]
    pub r#record_transformations: Option<Box<super::super::types::dataloss::PreventionDeidentifyTemplateDeidentifyConfigRecordTransformations>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for PreventionDeidentifyTemplateDeidentifyConfig {
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
                "image_transformations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#image_transformations,
                )
                .await,
            );
            map.insert(
                "info_type_transformations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#info_type_transformations,
                )
                .await,
            );
            map.insert(
                "record_transformations".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#record_transformations,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for PreventionDeidentifyTemplateDeidentifyConfig {
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
                    r#image_transformations: {
                        let field_value = match fields_map.get("image_transformations") {
                            Some(value) => value,
                            None => bail!("Missing field 'image_transformations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#info_type_transformations: {
                        let field_value = match fields_map.get("info_type_transformations") {
                            Some(value) => value,
                            None => bail!("Missing field 'info_type_transformations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#record_transformations: {
                        let field_value = match fields_map.get("record_transformations") {
                            Some(value) => value,
                            None => bail!("Missing field 'record_transformations' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
