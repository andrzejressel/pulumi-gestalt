#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LaunchTemplateEnclaveOptions {
    /// If set to `true`, Nitro Enclaves will be enabled on the instance.
    /// 
    /// For more information, see the documentation on [Nitro Enclaves](https://docs.aws.amazon.com/enclaves/latest/user/nitro-enclave.html).
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Option<bool>,
}
