#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments, clippy::should_implement_trait)]
pub struct RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfiguration {
    /// Email template used when a detected risk event is blocked. See notify email type below.
    #[builder(into)]
    #[serde(rename = "blockEmail")]
    pub r#block_email: Option<Box<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationBlockEmail>>,
    /// The email address that is sending the email. The address must be either individually verified with Amazon Simple Email Service, or from a domain that has been verified with Amazon SES.
    #[builder(into)]
    #[serde(rename = "from")]
    pub r#from: Option<String>,
    /// The multi-factor authentication (MFA) email template used when MFA is challenged as part of a detected risk. See notify email type below.
    #[builder(into)]
    #[serde(rename = "mfaEmail")]
    pub r#mfa_email: Option<Box<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationMfaEmail>>,
    /// The email template used when a detected risk event is allowed. See notify email type below.
    #[builder(into)]
    #[serde(rename = "noActionEmail")]
    pub r#no_action_email: Option<Box<super::super::types::cognito::RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfigurationNoActionEmail>>,
    /// The destination to which the receiver of an email should reply to.
    #[builder(into)]
    #[serde(rename = "replyTo")]
    pub r#reply_to: Option<String>,
    /// The Amazon Resource Name (ARN) of the identity that is associated with the sending authorization policy. This identity permits Amazon Cognito to send for the email address specified in the From parameter.
    #[builder(into)]
    #[serde(rename = "sourceArn")]
    pub r#source_arn: String,
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::ToPulumiValue for RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfiguration {
    fn to_pulumi_value(
        &self,
    ) -> impl std::future::Future<
        Output = pulumi_gestalt_rust::__private::pulumi_gestalt_model::PulumiValue,
    > {
        use pulumi_gestalt_rust::__private::futures::FutureExt;
        use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::to_pulumi_object_concurrent;
        async move {
            use pulumi_gestalt_rust::__private::pulumi_gestalt_model::__private::{
                to_pulumi_object_field, ToPulumiObjectFieldFuture,
            };
            let field_futures: Vec<ToPulumiObjectFieldFuture<'_>> = vec![
                to_pulumi_object_field(
                    "block_email",
                    &self.r#block_email,
                ),
                to_pulumi_object_field(
                    "from",
                    &self.r#from,
                ),
                to_pulumi_object_field(
                    "mfa_email",
                    &self.r#mfa_email,
                ),
                to_pulumi_object_field(
                    "no_action_email",
                    &self.r#no_action_email,
                ),
                to_pulumi_object_field(
                    "reply_to",
                    &self.r#reply_to,
                ),
                to_pulumi_object_field(
                    "source_arn",
                    &self.r#source_arn,
                ),
            ];
            to_pulumi_object_concurrent(field_futures).await
        }
        .boxed_local()
    }
}

impl pulumi_gestalt_rust::__private::pulumi_gestalt_model::FromPulumiValue for RiskConfigurationAccountTakeoverRiskConfigurationNotifyConfiguration {
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
                    r#block_email: {
                        let field_value = match fields_map.get("block_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'block_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#from: {
                        let field_value = match fields_map.get("from") {
                            Some(value) => value,
                            None => bail!("Missing field 'from' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#mfa_email: {
                        let field_value = match fields_map.get("mfa_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'mfa_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#no_action_email: {
                        let field_value = match fields_map.get("no_action_email") {
                            Some(value) => value,
                            None => bail!("Missing field 'no_action_email' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#reply_to: {
                        let field_value = match fields_map.get("reply_to") {
                            Some(value) => value,
                            None => bail!("Missing field 'reply_to' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                    r#source_arn: {
                        let field_value = match fields_map.get("source_arn") {
                            Some(value) => value,
                            None => bail!("Missing field 'source_arn' while converting PulumiValue to {}", std::any::type_name::<Self>()),
                        };
                        FromPulumiValue::from_pulumi_value(field_value)?
                    },
                })
            }
            _ => bail!("Expected Object, got {:?}", value.content),
        }
    }
}
