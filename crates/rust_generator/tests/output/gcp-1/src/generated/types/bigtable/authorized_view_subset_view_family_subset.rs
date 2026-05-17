#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthorizedViewSubsetViewFamilySubset {
    /// Name of the column family to be included in the authorized view. The specified column family must exist in the parent table of this authorized view.
    #[builder(into)]
    #[serde(rename = "familyName")]
    pub r#family_name: String,
    /// A list of Base64-encoded prefixes for qualifiers of the column family to be included in the authorized view.
    /// Every qualifier starting with one of these prefixes is included in the authorized view. To provide access to all qualifiers, include the empty string as a prefix ("").
    #[builder(into)]
    #[serde(rename = "qualifierPrefixes")]
    pub r#qualifier_prefixes: Option<Vec<String>>,
    /// A list of Base64-encoded individual exact column qualifiers of the column family to be included in the authorized view.
    #[builder(into)]
    #[serde(rename = "qualifiers")]
    pub r#qualifiers: Option<Vec<String>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AuthorizedViewSubsetViewFamilySubset {
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
                "family_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#family_name,
                )
                .await,
            );
            map.insert(
                "qualifier_prefixes".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#qualifier_prefixes,
                )
                .await,
            );
            map.insert(
                "qualifiers".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#qualifiers,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AuthorizedViewSubsetViewFamilySubset {
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
                    r#family_name: {
                        let field_value = match fields_map.get("family_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'family_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#qualifier_prefixes: {
                        let field_value = match fields_map.get("qualifier_prefixes") {
                            Some(value) => value,
                            None => bail!("Missing field 'qualifier_prefixes' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#qualifiers: {
                        let field_value = match fields_map.get("qualifiers") {
                            Some(value) => value,
                            None => bail!("Missing field 'qualifiers' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
