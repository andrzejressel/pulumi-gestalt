#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct SsisEnvironmentReferenceResponse {
    /// Environment folder name.
    #[builder(into)]
    #[serde(rename = "environmentFolderName")]
    pub r#environment_folder_name: Option<String>,
    /// Environment name.
    #[builder(into)]
    #[serde(rename = "environmentName")]
    pub r#environment_name: Option<String>,
    /// Environment reference id.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Option<f64>,
    /// Reference type
    #[builder(into)]
    #[serde(rename = "referenceType")]
    pub r#reference_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for SsisEnvironmentReferenceResponse {
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
                "environment_folder_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#environment_folder_name,
                )
                .await,
            );
            map.insert(
                "environment_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#environment_name,
                )
                .await,
            );
            map.insert(
                "id".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#id,
                )
                .await,
            );
            map.insert(
                "reference_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#reference_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for SsisEnvironmentReferenceResponse {
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
                    r#environment_folder_name: {
                        let field_value = match fields_map.get("environment_folder_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment_folder_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#environment_name: {
                        let field_value = match fields_map.get("environment_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'environment_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#id: {
                        let field_value = match fields_map.get("id") {
                            Some(value) => value,
                            None => bail!("Missing field 'id' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reference_type: {
                        let field_value = match fields_map.get("reference_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'reference_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
