#[allow(
    clippy::doc_lazy_continuation,
    clippy::tabs_in_doc_comments,
    clippy::should_implement_trait
)]
pub mod get_serial_console_access {
    #[allow(dead_code)]
    pub struct GetSerialConsoleAccessResult {
        /// Whether or not serial console access is enabled. Returns as `true` or `false`.
        pub enabled: pulumi_gestalt_rust::Output<bool>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::Context,
    ) -> GetSerialConsoleAccessResult {
        let request = pulumi_gestalt_rust::InvokeResourceRequest {
            token: "aws:ec2/getSerialConsoleAccess:getSerialConsoleAccess".into(),
            version: super::super::super::get_version(),
            object: &[],
        };
        let o = context.invoke_resource(request);
        GetSerialConsoleAccessResult {
            enabled: o.get_field("enabled"),
            id: o.get_field("id"),
        }
    }
}
