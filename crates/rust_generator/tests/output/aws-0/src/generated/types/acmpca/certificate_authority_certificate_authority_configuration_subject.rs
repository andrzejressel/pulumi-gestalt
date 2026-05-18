#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct CertificateAuthorityCertificateAuthorityConfigurationSubject {
    /// Fully qualified domain name (FQDN) associated with the certificate subject. Must be less than or equal to 64 characters in length.
    #[builder(into)]
    #[serde(rename = "commonName")]
    pub r#common_name: Option<String>,
    /// Two digit code that specifies the country in which the certificate subject located. Must be less than or equal to 2 characters in length.
    #[builder(into)]
    #[serde(rename = "country")]
    pub r#country: Option<String>,
    /// Disambiguating information for the certificate subject. Must be less than or equal to 64 characters in length.
    #[builder(into)]
    #[serde(rename = "distinguishedNameQualifier")]
    pub r#distinguished_name_qualifier: Option<String>,
    /// Typically a qualifier appended to the name of an individual. Examples include Jr. for junior, Sr. for senior, and III for third. Must be less than or equal to 3 characters in length.
    #[builder(into)]
    #[serde(rename = "generationQualifier")]
    pub r#generation_qualifier: Option<String>,
    /// First name. Must be less than or equal to 16 characters in length.
    #[builder(into)]
    #[serde(rename = "givenName")]
    pub r#given_name: Option<String>,
    /// Concatenation that typically contains the first letter of the `given_name`, the first letter of the middle name if one exists, and the first letter of the `surname`. Must be less than or equal to 5 characters in length.
    #[builder(into)]
    #[serde(rename = "initials")]
    pub r#initials: Option<String>,
    /// Locality (such as a city or town) in which the certificate subject is located. Must be less than or equal to 128 characters in length.
    #[builder(into)]
    #[serde(rename = "locality")]
    pub r#locality: Option<String>,
    /// Legal name of the organization with which the certificate subject is affiliated. Must be less than or equal to 64 characters in length.
    #[builder(into)]
    #[serde(rename = "organization")]
    pub r#organization: Option<String>,
    /// Subdivision or unit of the organization (such as sales or finance) with which the certificate subject is affiliated. Must be less than or equal to 64 characters in length.
    #[builder(into)]
    #[serde(rename = "organizationalUnit")]
    pub r#organizational_unit: Option<String>,
    /// Typically a shortened version of a longer `given_name`. For example, Jonathan is often shortened to John. Elizabeth is often shortened to Beth, Liz, or Eliza. Must be less than or equal to 128 characters in length.
    #[builder(into)]
    #[serde(rename = "pseudonym")]
    pub r#pseudonym: Option<String>,
    /// State in which the subject of the certificate is located. Must be less than or equal to 128 characters in length.
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Option<String>,
    /// Family name. In the US and the UK for example, the surname of an individual is ordered last. In Asian cultures the surname is typically ordered first. Must be less than or equal to 40 characters in length.
    #[builder(into)]
    #[serde(rename = "surname")]
    pub r#surname: Option<String>,
    /// Title such as Mr. or Ms. which is pre-pended to the name to refer formally to the certificate subject. Must be less than or equal to 64 characters in length.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for CertificateAuthorityCertificateAuthorityConfigurationSubject {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > + Send {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "common_name",
                    &self.r#common_name,
                ),
                to_pulumi_object_field(
                    "country",
                    &self.r#country,
                ),
                to_pulumi_object_field(
                    "distinguished_name_qualifier",
                    &self.r#distinguished_name_qualifier,
                ),
                to_pulumi_object_field(
                    "generation_qualifier",
                    &self.r#generation_qualifier,
                ),
                to_pulumi_object_field(
                    "given_name",
                    &self.r#given_name,
                ),
                to_pulumi_object_field(
                    "initials",
                    &self.r#initials,
                ),
                to_pulumi_object_field(
                    "locality",
                    &self.r#locality,
                ),
                to_pulumi_object_field(
                    "organization",
                    &self.r#organization,
                ),
                to_pulumi_object_field(
                    "organizational_unit",
                    &self.r#organizational_unit,
                ),
                to_pulumi_object_field(
                    "pseudonym",
                    &self.r#pseudonym,
                ),
                to_pulumi_object_field(
                    "state",
                    &self.r#state,
                ),
                to_pulumi_object_field(
                    "surname",
                    &self.r#surname,
                ),
                to_pulumi_object_field(
                    "title",
                    &self.r#title,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for CertificateAuthorityCertificateAuthorityConfigurationSubject {
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
                    r#country: {
                        let field_value = match fields_map.get("country") {
                            Some(value) => value,
                            None => bail!("Missing field 'country' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#distinguished_name_qualifier: {
                        let field_value = match fields_map.get("distinguished_name_qualifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'distinguished_name_qualifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#generation_qualifier: {
                        let field_value = match fields_map.get("generation_qualifier") {
                            Some(value) => value,
                            None => bail!("Missing field 'generation_qualifier' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#given_name: {
                        let field_value = match fields_map.get("given_name") {
                            Some(value) => value,
                            None => bail!("Missing field 'given_name' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#initials: {
                        let field_value = match fields_map.get("initials") {
                            Some(value) => value,
                            None => bail!("Missing field 'initials' while converting PulumiValue to {}", std::any::type_name::<Self>()),
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
                    r#pseudonym: {
                        let field_value = match fields_map.get("pseudonym") {
                            Some(value) => value,
                            None => bail!("Missing field 'pseudonym' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#state: {
                        let field_value = match fields_map.get("state") {
                            Some(value) => value,
                            None => bail!("Missing field 'state' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#surname: {
                        let field_value = match fields_map.get("surname") {
                            Some(value) => value,
                            None => bail!("Missing field 'surname' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#title: {
                        let field_value = match fields_map.get("title") {
                            Some(value) => value,
                            None => bail!("Missing field 'title' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
