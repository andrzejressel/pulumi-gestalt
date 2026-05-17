#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ApiMetaData {
    /// Time at which the API proxy was created, in milliseconds since epoch.
    #[builder(into)]
    #[serde(rename = "createdAt")]
    pub r#created_at: Option<String>,
    /// Time at which the API proxy was most recently modified, in milliseconds since epoch.
    #[builder(into)]
    #[serde(rename = "lastModifiedAt")]
    pub r#last_modified_at: Option<String>,
    /// The type of entity described
    #[builder(into)]
    #[serde(rename = "subType")]
    pub r#sub_type: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ApiMetaData {
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
                "created_at".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#created_at,
                )
                .await,
            );
            map.insert(
                "last_modified_at".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#last_modified_at,
                )
                .await,
            );
            map.insert(
                "sub_type".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#sub_type,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ApiMetaData {
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
                    r#created_at: {
                        let field_value = match fields_map.get("created_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'created_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#last_modified_at: {
                        let field_value = match fields_map.get("last_modified_at") {
                            Some(value) => value,
                            None => bail!("Missing field 'last_modified_at' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#sub_type: {
                        let field_value = match fields_map.get("sub_type") {
                            Some(value) => value,
                            None => bail!("Missing field 'sub_type' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
