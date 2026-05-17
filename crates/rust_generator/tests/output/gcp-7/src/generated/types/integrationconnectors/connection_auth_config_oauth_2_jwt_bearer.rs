#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct ConnectionAuthConfigOauth2JwtBearer {
    /// Secret version reference containing a PKCS#8 PEM-encoded private key associated with the Client Certificate.
    /// This private key will be used to sign JWTs used for the jwt-bearer authorization grant.
    /// Specified in the form as: projects/*/secrets/*/versions/*.
    #[builder(into)]
    #[serde(rename = "clientKey")]
    pub r#client_key: Option<Box<super::super::types::integrationconnectors::ConnectionAuthConfigOauth2JwtBearerClientKey>>,
    /// JwtClaims providers fields to generate the token.
    #[builder(into)]
    #[serde(rename = "jwtClaims")]
    pub r#jwt_claims: Option<Box<super::super::types::integrationconnectors::ConnectionAuthConfigOauth2JwtBearerJwtClaims>>,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for ConnectionAuthConfigOauth2JwtBearer {
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
                "client_key".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#client_key,
                )
                .await,
            );
            map.insert(
                "jwt_claims".to_string(),
                ToPulumiValue::to_pulumi_value(
                    &self.r#jwt_claims,
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

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for ConnectionAuthConfigOauth2JwtBearer {
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
                    r#client_key: {
                        let field_value = match fields_map.get("client_key") {
                            Some(value) => value,
                            None => bail!("Missing field 'client_key' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#jwt_claims: {
                        let field_value = match fields_map.get("jwt_claims") {
                            Some(value) => value,
                            None => bail!("Missing field 'jwt_claims' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
