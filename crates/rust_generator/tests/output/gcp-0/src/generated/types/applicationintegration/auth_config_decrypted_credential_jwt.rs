#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct AuthConfigDecryptedCredentialJwt {
    /// (Output)
    /// The token calculated by the header, payload and signature.
    #[builder(into)]
    #[serde(rename = "jwt")]
    pub r#jwt: Option<String>,
    /// Identifies which algorithm is used to generate the signature.
    #[builder(into)]
    #[serde(rename = "jwtHeader")]
    pub r#jwt_header: Option<String>,
    /// Contains a set of claims. The JWT specification defines seven Registered Claim Names which are the standard fields commonly included in tokens. Custom claims are usually also included, depending on the purpose of the token.
    #[builder(into)]
    #[serde(rename = "jwtPayload")]
    pub r#jwt_payload: Option<String>,
    /// User's pre-shared secret to sign the token.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Option<String>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for AuthConfigDecryptedCredentialJwt {
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
                "jwt".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#jwt,
                )
                .await,
            );
            map.insert(
                "jwt_header".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#jwt_header,
                )
                .await,
            );
            map.insert(
                "jwt_payload".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#jwt_payload,
                )
                .await,
            );
            map.insert(
                "secret".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#secret,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for AuthConfigDecryptedCredentialJwt {
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
                    r#jwt: {
                        let field_value = match fields_map.get("jwt") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwt' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jwt_header: {
                        let field_value = match fields_map.get("jwt_header") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwt_header' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jwt_payload: {
                        let field_value = match fields_map.get("jwt_payload") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwt_payload' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#secret: {
                        let field_value = match fields_map.get("secret") {
                            Some(value) => value,
                            None => bail!("Missing field 'secret' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
