#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct GetAuthorityConfigSubjectConfigSubject {
    /// The common name of the distinguished name.
    #[builder(into)]
    #[serde(rename = "commonName")]
    pub r#common_name: String,
    /// The country code of the subject.
    #[builder(into)]
    #[serde(rename = "countryCode")]
    pub r#country_code: String,
    /// The locality or city of the subject.
    #[builder(into)]
    #[serde(rename = "locality")]
    pub r#locality: String,
    /// The organization of the subject.
    #[builder(into)]
    #[serde(rename = "organization")]
    pub r#organization: String,
    /// The organizational unit of the subject.
    #[builder(into)]
    #[serde(rename = "organizationalUnit")]
    pub r#organizational_unit: String,
    /// The postal code of the subject.
    #[builder(into)]
    #[serde(rename = "postalCode")]
    pub r#postal_code: String,
    /// The province, territory, or regional state of the subject.
    #[builder(into)]
    #[serde(rename = "province")]
    pub r#province: String,
    /// The street address of the subject.
    #[builder(into)]
    #[serde(rename = "streetAddress")]
    pub r#street_address: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for GetAuthorityConfigSubjectConfigSubject {
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
                "common_name".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#common_name,
                )
                .await,
            );
            map.insert(
                "country_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#country_code,
                )
                .await,
            );
            map.insert(
                "locality".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#locality,
                )
                .await,
            );
            map.insert(
                "organization".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#organization,
                )
                .await,
            );
            map.insert(
                "organizational_unit".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#organizational_unit,
                )
                .await,
            );
            map.insert(
                "postal_code".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#postal_code,
                )
                .await,
            );
            map.insert(
                "province".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#province,
                )
                .await,
            );
            map.insert(
                "street_address".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#street_address,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for GetAuthorityConfigSubjectConfigSubject {
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
                    r#common_name: {
                        let field_value = match fields_map.get("common_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'common_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#country_code: {
                        let field_value = match fields_map.get("country_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'country_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#locality: {
                        let field_value = match fields_map.get("locality") {
                            Some(value) => value,
                            None => bail!("Missing field 'locality' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organization: {
                        let field_value = match fields_map.get("organization") {
                            Some(value) => value,
                            None => bail!("Missing field 'organization' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#organizational_unit: {
                        let field_value = match fields_map.get("organizational_unit") {
                            Some(value) => value,
                            None => bail!("Missing field 'organizational_unit' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#postal_code: {
                        let field_value = match fields_map.get("postal_code") {
                            Some(value) => value,
                            None => bail!("Missing field 'postal_code' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#province: {
                        let field_value = match fields_map.get("province") {
                            Some(value) => value,
                            None => bail!("Missing field 'province' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#street_address: {
                        let field_value = match fields_map.get("street_address") {
                            Some(value) => value,
                            None => bail!("Missing field 'street_address' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
