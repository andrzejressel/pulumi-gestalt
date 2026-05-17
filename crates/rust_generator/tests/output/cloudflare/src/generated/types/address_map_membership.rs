#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AddressMapMembership {
    /// Controls whether the membership can be deleted via the API or not.
    #[builder(into)]
    #[serde(rename = "canDelete")]
    pub r#can_delete: Option<bool>,
    /// Identifier of the account or zone.
    #[builder(into)]
    #[serde(rename = "identifier")]
    pub r#identifier: String,
    /// The type of the membership.
    #[builder(into)]
    #[serde(rename = "kind")]
    pub r#kind: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AddressMapMembership {
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
                "can_delete".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#can_delete,
                )
                .await,
            );
            map.insert(
                "identifier".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#identifier,
                )
                .await,
            );
            map.insert(
                "kind".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#kind,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AddressMapMembership {
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
                    r#can_delete: {
                        let field_value = match fields_map.get("can_delete") {
                            Some(value) => value,
                            None => bail!("Missing field 'can_delete' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#identifier: {
                        let field_value = match fields_map.get("identifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'identifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#kind: {
                        let field_value = match fields_map.get("kind") {
                            Some(value) => value,
                            None => bail!("Missing field 'kind' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
