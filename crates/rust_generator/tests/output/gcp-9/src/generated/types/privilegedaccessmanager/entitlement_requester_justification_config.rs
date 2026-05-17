#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct EntitlementRequesterJustificationConfig {
    /// The justification is not mandatory but can be provided in any of the supported formats.
    #[builder(into)]
    #[serde(rename = "notMandatory")]
    pub r#not_mandatory: Option<Box<super::super::types::privilegedaccessmanager::EntitlementRequesterJustificationConfigNotMandatory>>,
    /// The requester has to provide a justification in the form of free flowing text.
    /// 
    /// - - -
    #[builder(into)]
    #[serde(rename = "unstructured")]
    pub r#unstructured: Option<Box<super::super::types::privilegedaccessmanager::EntitlementRequesterJustificationConfigUnstructured>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for EntitlementRequesterJustificationConfig {
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
                "not_mandatory".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#not_mandatory,
                )
                .await,
            );
            map.insert(
                "unstructured".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#unstructured,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for EntitlementRequesterJustificationConfig {
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
                    r#not_mandatory: {
                        let field_value = match fields_map.get("not_mandatory") {
                            Some(value) => value,
                            None => bail!("Missing field 'not_mandatory' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#unstructured: {
                        let field_value = match fields_map.get("unstructured") {
                            Some(value) => value,
                            None => bail!("Missing field 'unstructured' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
