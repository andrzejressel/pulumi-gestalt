
awsAWS"6.66.2*Àô
O
ssoadminAccountAssignment0aws:ssoadmin/accountAssignment:AccountAssignmentæéProvides a Single Sign-On (SSO) Account Assignment resource

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const exampleGetPermissionSet = example.then(example => aws.ssoadmin.getPermissionSet({
    instanceArn: example.arns?.[0],
    name: "AWSReadOnlyAccess",
}));
const exampleGetGroup = example.then(example => aws.identitystore.getGroup({
    identityStoreId: example.identityStoreIds?.[0],
    alternateIdentifier: {
        uniqueAttribute: {
            attributePath: "DisplayName",
            attributeValue: "ExampleGroup",
        },
    },
}));
const exampleAccountAssignment = new aws.ssoadmin.AccountAssignment("example", {
    instanceArn: example.then(example => example.arns?.[0]),
    permissionSetArn: exampleGetPermissionSet.then(exampleGetPermissionSet => exampleGetPermissionSet.arn),
    principalId: exampleGetGroup.then(exampleGetGroup => exampleGetGroup.groupId),
    principalType: "GROUP",
    targetId: "123456789012",
    targetType: "AWS_ACCOUNT",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_get_permission_set = aws.ssoadmin.get_permission_set(instance_arn=example.arns[0],
    name="AWSReadOnlyAccess")
example_get_group = aws.identitystore.get_group(identity_store_id=example.identity_store_ids[0],
    alternate_identifier={
        "unique_attribute": {
            "attribute_path": "DisplayName",
            "attribute_value": "ExampleGroup",
        },
    })
example_account_assignment = aws.ssoadmin.AccountAssignment("example",
    instance_arn=example.arns[0],
    permission_set_arn=example_get_permission_set.arn,
    principal_id=example_get_group.group_id,
    principal_type="GROUP",
    target_id="123456789012",
    target_type="AWS_ACCOUNT")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var exampleGetPermissionSet = Aws.SsoAdmin.GetPermissionSet.Invoke(new()
    {
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        Name = "AWSReadOnlyAccess",
    });

    var exampleGetGroup = Aws.IdentityStore.GetGroup.Invoke(new()
    {
        IdentityStoreId = example.Apply(getInstancesResult => getInstancesResult.IdentityStoreIds[0]),
        AlternateIdentifier = new Aws.IdentityStore.Inputs.GetGroupAlternateIdentifierInputArgs
        {
            UniqueAttribute = new Aws.IdentityStore.Inputs.GetGroupAlternateIdentifierUniqueAttributeInputArgs
            {
                AttributePath = "DisplayName",
                AttributeValue = "ExampleGroup",
            },
        },
    });

    var exampleAccountAssignment = new Aws.SsoAdmin.AccountAssignment("example", new()
    {
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        PermissionSetArn = exampleGetPermissionSet.Apply(getPermissionSetResult => getPermissionSetResult.Arn),
        PrincipalId = exampleGetGroup.Apply(getGroupResult => getGroupResult.GroupId),
        PrincipalType = "GROUP",
        TargetId = "123456789012",
        TargetType = "AWS_ACCOUNT",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/identitystore"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		exampleGetPermissionSet, err := ssoadmin.LookupPermissionSet(ctx, &ssoadmin.LookupPermissionSetArgs{
			InstanceArn: example.Arns[0],
			Name:        pulumi.StringRef("AWSReadOnlyAccess"),
		}, nil)
		if err != nil {
			return err
		}
		exampleGetGroup, err := identitystore.LookupGroup(ctx, &identitystore.LookupGroupArgs{
			IdentityStoreId: example.IdentityStoreIds[0],
			AlternateIdentifier: identitystore.GetGroupAlternateIdentifier{
				UniqueAttribute: identitystore.GetGroupAlternateIdentifierUniqueAttribute{
					AttributePath:  "DisplayName",
					AttributeValue: "ExampleGroup",
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewAccountAssignment(ctx, "example", &ssoadmin.AccountAssignmentArgs{
			InstanceArn:      pulumi.String(example.Arns[0]),
			PermissionSetArn: pulumi.String(exampleGetPermissionSet.Arn),
			PrincipalId:      pulumi.String(exampleGetGroup.GroupId),
			PrincipalType:    pulumi.String("GROUP"),
			TargetId:         pulumi.String("123456789012"),
			TargetType:       pulumi.String("AWS_ACCOUNT"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.inputs.GetPermissionSetArgs;
import com.pulumi.aws.identitystore.IdentitystoreFunctions;
import com.pulumi.aws.identitystore.inputs.GetGroupArgs;
import com.pulumi.aws.identitystore.inputs.GetGroupAlternateIdentifierArgs;
import com.pulumi.aws.identitystore.inputs.GetGroupAlternateIdentifierUniqueAttributeArgs;
import com.pulumi.aws.ssoadmin.AccountAssignment;
import com.pulumi.aws.ssoadmin.AccountAssignmentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        final var exampleGetPermissionSet = SsoadminFunctions.getPermissionSet(GetPermissionSetArgs.builder()
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .name("AWSReadOnlyAccess")
            .build());

        final var exampleGetGroup = IdentitystoreFunctions.getGroup(GetGroupArgs.builder()
            .identityStoreId(example.applyValue(getInstancesResult -> getInstancesResult.identityStoreIds()[0]))
            .alternateIdentifier(GetGroupAlternateIdentifierArgs.builder()
                .uniqueAttribute(GetGroupAlternateIdentifierUniqueAttributeArgs.builder()
                    .attributePath("DisplayName")
                    .attributeValue("ExampleGroup")
                    .build())
                .build())
            .build());

        var exampleAccountAssignment = new AccountAssignment("exampleAccountAssignment", AccountAssignmentArgs.builder()
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .permissionSetArn(exampleGetPermissionSet.applyValue(getPermissionSetResult -> getPermissionSetResult.arn()))
            .principalId(exampleGetGroup.applyValue(getGroupResult -> getGroupResult.groupId()))
            .principalType("GROUP")
            .targetId("123456789012")
            .targetType("AWS_ACCOUNT")
            .build());

    }
}
```
```yaml
resources:
  exampleAccountAssignment:
    type: aws:ssoadmin:AccountAssignment
    name: example
    properties:
      instanceArn: ${example.arns[0]}
      permissionSetArn: ${exampleGetPermissionSet.arn}
      principalId: ${exampleGetGroup.groupId}
      principalType: GROUP
      targetId: '123456789012'
      targetType: AWS_ACCOUNT
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
  exampleGetPermissionSet:
    fn::invoke:
      function: aws:ssoadmin:getPermissionSet
      arguments:
        instanceArn: ${example.arns[0]}
        name: AWSReadOnlyAccess
  exampleGetGroup:
    fn::invoke:
      function: aws:identitystore:getGroup
      arguments:
        identityStoreId: ${example.identityStoreIds[0]}
        alternateIdentifier:
          uniqueAttribute:
            attributePath: DisplayName
            attributeValue: ExampleGroup
```
<!--End PulumiCodeChooser -->

### With Managed Policy Attachment

> Because destruction of a managed policy attachment resource also re-provisions the associated permission set to all accounts, explicitly indicating the dependency with the account assignment resource via the `depends_on` meta argument is necessary to ensure proper deletion order when these resources are used together.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const examplePermissionSet = new aws.ssoadmin.PermissionSet("example", {
    name: "Example",
    instanceArn: example.then(example => example.arns?.[0]),
});
const exampleGroup = new aws.identitystore.Group("example", {
    identityStoreId: example.then(example => example.identityStoreIds?.[0]),
    displayName: "Admin",
    description: "Admin Group",
});
const accountAssignment = new aws.ssoadmin.AccountAssignment("account_assignment", {
    instanceArn: example.then(example => example.arns?.[0]),
    permissionSetArn: examplePermissionSet.arn,
    principalId: exampleGroup.groupId,
    principalType: "GROUP",
    targetId: "123456789012",
    targetType: "AWS_ACCOUNT",
});
const exampleManagedPolicyAttachment = new aws.ssoadmin.ManagedPolicyAttachment("example", {
    instanceArn: example.then(example => example.arns?.[0]),
    managedPolicyArn: "arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup",
    permissionSetArn: examplePermissionSet.arn,
}, {
    dependsOn: [exampleAwsSsoadminAccountAssignment],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_permission_set = aws.ssoadmin.PermissionSet("example",
    name="Example",
    instance_arn=example.arns[0])
example_group = aws.identitystore.Group("example",
    identity_store_id=example.identity_store_ids[0],
    display_name="Admin",
    description="Admin Group")
account_assignment = aws.ssoadmin.AccountAssignment("account_assignment",
    instance_arn=example.arns[0],
    permission_set_arn=example_permission_set.arn,
    principal_id=example_group.group_id,
    principal_type="GROUP",
    target_id="123456789012",
    target_type="AWS_ACCOUNT")
example_managed_policy_attachment = aws.ssoadmin.ManagedPolicyAttachment("example",
    instance_arn=example.arns[0],
    managed_policy_arn="arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup",
    permission_set_arn=example_permission_set.arn,
    opts = pulumi.ResourceOptions(depends_on=[example_aws_ssoadmin_account_assignment]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var examplePermissionSet = new Aws.SsoAdmin.PermissionSet("example", new()
    {
        Name = "Example",
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
    });

    var exampleGroup = new Aws.IdentityStore.Group("example", new()
    {
        IdentityStoreId = example.Apply(getInstancesResult => getInstancesResult.IdentityStoreIds[0]),
        DisplayName = "Admin",
        Description = "Admin Group",
    });

    var accountAssignment = new Aws.SsoAdmin.AccountAssignment("account_assignment", new()
    {
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        PermissionSetArn = examplePermissionSet.Arn,
        PrincipalId = exampleGroup.GroupId,
        PrincipalType = "GROUP",
        TargetId = "123456789012",
        TargetType = "AWS_ACCOUNT",
    });

    var exampleManagedPolicyAttachment = new Aws.SsoAdmin.ManagedPolicyAttachment("example", new()
    {
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        ManagedPolicyArn = "arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup",
        PermissionSetArn = examplePermissionSet.Arn,
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsSsoadminAccountAssignment,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/identitystore"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		examplePermissionSet, err := ssoadmin.NewPermissionSet(ctx, "example", &ssoadmin.PermissionSetArgs{
			Name:        pulumi.String("Example"),
			InstanceArn: pulumi.String(example.Arns[0]),
		})
		if err != nil {
			return err
		}
		exampleGroup, err := identitystore.NewGroup(ctx, "example", &identitystore.GroupArgs{
			IdentityStoreId: pulumi.String(example.IdentityStoreIds[0]),
			DisplayName:     pulumi.String("Admin"),
			Description:     pulumi.String("Admin Group"),
		})
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewAccountAssignment(ctx, "account_assignment", &ssoadmin.AccountAssignmentArgs{
			InstanceArn:      pulumi.String(example.Arns[0]),
			PermissionSetArn: examplePermissionSet.Arn,
			PrincipalId:      exampleGroup.GroupId,
			PrincipalType:    pulumi.String("GROUP"),
			TargetId:         pulumi.String("123456789012"),
			TargetType:       pulumi.String("AWS_ACCOUNT"),
		})
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewManagedPolicyAttachment(ctx, "example", &ssoadmin.ManagedPolicyAttachmentArgs{
			InstanceArn:      pulumi.String(example.Arns[0]),
			ManagedPolicyArn: pulumi.String("arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup"),
			PermissionSetArn: examplePermissionSet.Arn,
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsSsoadminAccountAssignment,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.PermissionSet;
import com.pulumi.aws.ssoadmin.PermissionSetArgs;
import com.pulumi.aws.identitystore.Group;
import com.pulumi.aws.identitystore.GroupArgs;
import com.pulumi.aws.ssoadmin.AccountAssignment;
import com.pulumi.aws.ssoadmin.AccountAssignmentArgs;
import com.pulumi.aws.ssoadmin.ManagedPolicyAttachment;
import com.pulumi.aws.ssoadmin.ManagedPolicyAttachmentArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        var examplePermissionSet = new PermissionSet("examplePermissionSet", PermissionSetArgs.builder()
            .name("Example")
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .build());

        var exampleGroup = new Group("exampleGroup", GroupArgs.builder()
            .identityStoreId(example.applyValue(getInstancesResult -> getInstancesResult.identityStoreIds()[0]))
            .displayName("Admin")
            .description("Admin Group")
            .build());

        var accountAssignment = new AccountAssignment("accountAssignment", AccountAssignmentArgs.builder()
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .permissionSetArn(examplePermissionSet.arn())
            .principalId(exampleGroup.groupId())
            .principalType("GROUP")
            .targetId("123456789012")
            .targetType("AWS_ACCOUNT")
            .build());

        var exampleManagedPolicyAttachment = new ManagedPolicyAttachment("exampleManagedPolicyAttachment", ManagedPolicyAttachmentArgs.builder()
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .managedPolicyArn("arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup")
            .permissionSetArn(examplePermissionSet.arn())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsSsoadminAccountAssignment)
                .build());

    }
}
```
```yaml
resources:
  examplePermissionSet:
    type: aws:ssoadmin:PermissionSet
    name: example
    properties:
      name: Example
      instanceArn: ${example.arns[0]}
  exampleGroup:
    type: aws:identitystore:Group
    name: example
    properties:
      identityStoreId: ${example.identityStoreIds[0]}
      displayName: Admin
      description: Admin Group
  accountAssignment:
    type: aws:ssoadmin:AccountAssignment
    name: account_assignment
    properties:
      instanceArn: ${example.arns[0]}
      permissionSetArn: ${examplePermissionSet.arn}
      principalId: ${exampleGroup.groupId}
      principalType: GROUP
      targetId: '123456789012'
      targetType: AWS_ACCOUNT
  exampleManagedPolicyAttachment:
    type: aws:ssoadmin:ManagedPolicyAttachment
    name: example
    properties:
      instanceArn: ${example.arns[0]}
      managedPolicyArn: arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup
      permissionSetArn: ${examplePermissionSet.arn}
    options:
      dependsOn:
        - ${exampleAwsSsoadminAccountAssignment}
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSO Account Assignments using the `principal_id`, `principal_type`, `target_id`, `target_type`, `permission_set_arn`, `instance_arn` separated by commas (`,`). For example:

```sh
$ pulumi import aws:ssoadmin/accountAssignment:AccountAssignment example f81d4fae-7dec-11d0-a765-00a0c91e6bf6,GROUP,1234567890,AWS_ACCOUNT,arn:aws:sso:::permissionSet/ssoins-0123456789abcdef/ps-0123456789abcdef,arn:aws:sso:::instance/ssoins-0123456789abcdef
```
G
instanceArn" 4The Amazon Resource Name (ARN) of the SSO Instance.
Ñ
permissionSetArn" lThe Amazon Resource Name (ARN) of the Permission Set that the admin wants to grant the principal access to.
ü
principalId" ãAn identifier for an object in SSO, such as a user or group. PrincipalIds are GUIDs (For example, `f81d4fae-7dec-11d0-a765-00a0c91e6bf6`).
n
principalType" YThe entity type for which the assignment will be created. Valid values: `USER`, `GROUP`.
K
targetId" ;An AWS account identifier, typically a 10-12 digit string.
k

targetTypeB" WThe entity type for which the assignment will be created. Valid values: `AWS_ACCOUNT`.
"G
instanceArn" 4The Amazon Resource Name (ARN) of the SSO Instance.
"Ñ
permissionSetArn" lThe Amazon Resource Name (ARN) of the Permission Set that the admin wants to grant the principal access to.
"ü
principalId" ãAn identifier for an object in SSO, such as a user or group. PrincipalIds are GUIDs (For example, `f81d4fae-7dec-11d0-a765-00a0c91e6bf6`).
"n
principalType" YThe entity type for which the assignment will be created. Valid values: `USER`, `GROUP`.
"K
targetId" ;An AWS account identifier, typically a 10-12 digit string.
"k

targetTypeB" WThe entity type for which the assignment will be created. Valid values: `AWS_ACCOUNT`.
*ŸS
=
ssoadminApplication$aws:ssoadmin/application:ApplicationÒBResource for managing an AWS SSO Admin Application.

> The `CreateApplication` API only supports custom OAuth 2.0 applications.
Creation of 3rd party SAML or OAuth 2.0 applications require setup to be done through the associated app service or AWS console.
See this issue for additional context.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const exampleApplication = new aws.ssoadmin.Application("example", {
    name: "example",
    applicationProviderArn: "arn:aws:sso::aws:applicationProvider/custom",
    instanceArn: example.then(example => example.arns?.[0]),
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_application = aws.ssoadmin.Application("example",
    name="example",
    application_provider_arn="arn:aws:sso::aws:applicationProvider/custom",
    instance_arn=example.arns[0])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var exampleApplication = new Aws.SsoAdmin.Application("example", new()
    {
        Name = "example",
        ApplicationProviderArn = "arn:aws:sso::aws:applicationProvider/custom",
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewApplication(ctx, "example", &ssoadmin.ApplicationArgs{
			Name:                   pulumi.String("example"),
			ApplicationProviderArn: pulumi.String("arn:aws:sso::aws:applicationProvider/custom"),
			InstanceArn:            pulumi.String(example.Arns[0]),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.Application;
import com.pulumi.aws.ssoadmin.ApplicationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        var exampleApplication = new Application("exampleApplication", ApplicationArgs.builder()
            .name("example")
            .applicationProviderArn("arn:aws:sso::aws:applicationProvider/custom")
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .build());

    }
}
```
```yaml
resources:
  exampleApplication:
    type: aws:ssoadmin:Application
    name: example
    properties:
      name: example
      applicationProviderArn: arn:aws:sso::aws:applicationProvider/custom
      instanceArn: ${example.arns[0]}
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
```
<!--End PulumiCodeChooser -->

### With Portal Options

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const exampleApplication = new aws.ssoadmin.Application("example", {
    name: "example",
    applicationProviderArn: "arn:aws:sso::aws:applicationProvider/custom",
    instanceArn: example.then(example => example.arns?.[0]),
    portalOptions: {
        visibility: "ENABLED",
        signInOptions: {
            applicationUrl: "http://example.com",
            origin: "APPLICATION",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_application = aws.ssoadmin.Application("example",
    name="example",
    application_provider_arn="arn:aws:sso::aws:applicationProvider/custom",
    instance_arn=example.arns[0],
    portal_options={
        "visibility": "ENABLED",
        "sign_in_options": {
            "application_url": "http://example.com",
            "origin": "APPLICATION",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var exampleApplication = new Aws.SsoAdmin.Application("example", new()
    {
        Name = "example",
        ApplicationProviderArn = "arn:aws:sso::aws:applicationProvider/custom",
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        PortalOptions = new Aws.SsoAdmin.Inputs.ApplicationPortalOptionsArgs
        {
            Visibility = "ENABLED",
            SignInOptions = new Aws.SsoAdmin.Inputs.ApplicationPortalOptionsSignInOptionsArgs
            {
                ApplicationUrl = "http://example.com",
                Origin = "APPLICATION",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewApplication(ctx, "example", &ssoadmin.ApplicationArgs{
			Name:                   pulumi.String("example"),
			ApplicationProviderArn: pulumi.String("arn:aws:sso::aws:applicationProvider/custom"),
			InstanceArn:            pulumi.String(example.Arns[0]),
			PortalOptions: &ssoadmin.ApplicationPortalOptionsArgs{
				Visibility: pulumi.String("ENABLED"),
				SignInOptions: &ssoadmin.ApplicationPortalOptionsSignInOptionsArgs{
					ApplicationUrl: pulumi.String("http://example.com"),
					Origin:         pulumi.String("APPLICATION"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.Application;
import com.pulumi.aws.ssoadmin.ApplicationArgs;
import com.pulumi.aws.ssoadmin.inputs.ApplicationPortalOptionsArgs;
import com.pulumi.aws.ssoadmin.inputs.ApplicationPortalOptionsSignInOptionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        var exampleApplication = new Application("exampleApplication", ApplicationArgs.builder()
            .name("example")
            .applicationProviderArn("arn:aws:sso::aws:applicationProvider/custom")
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .portalOptions(ApplicationPortalOptionsArgs.builder()
                .visibility("ENABLED")
                .signInOptions(ApplicationPortalOptionsSignInOptionsArgs.builder()
                    .applicationUrl("http://example.com")
                    .origin("APPLICATION")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  exampleApplication:
    type: aws:ssoadmin:Application
    name: example
    properties:
      name: example
      applicationProviderArn: arn:aws:sso::aws:applicationProvider/custom
      instanceArn: ${example.arns[0]}
      portalOptions:
        visibility: ENABLED
        signInOptions:
          applicationUrl: http://example.com
          origin: APPLICATION
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSO Admin Application using the `id`. For example:

```sh
$ pulumi import aws:ssoadmin/application:Application example arn:aws:sso::123456789012:application/id-12345678
```
?
applicationProviderArn" !ARN of the application provider.
ù
clientTokenB" áA unique, case-sensitive ID that you provide to ensure the idempotency of the request. AWS generates a random value when not provided.
5
descriptionB"  Description of the application.
?
instanceArn" ,ARN of the instance of IAM Identity Center.
N
nameB" @Name of the application.

The following arguments are optional:
–
portalOptionsjBh:f
d
ssoadminApplicationPortalOptions>aws:ssoadmin/ApplicationPortalOptions:ApplicationPortalOptionsSOptions for the portal associated with an application. See `portal_options` below.
V
statusB" FStatus of the application. Valid values are `ENABLED` and `DISABLED`.
À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"*
applicationAccount" AWS account ID.
".
applicationArn" ARN of the application.
"?
applicationProviderArn" !ARN of the application provider.
"ù
clientTokenB" áA unique, case-sensitive ID that you provide to ensure the idempotency of the request. AWS generates a random value when not provided.
"5
descriptionB"  Description of the application.
"?
instanceArn" ,ARN of the instance of IAM Identity Center.
"L
name" @Name of the application.

The following arguments are optional:
"–
portalOptionsjBh:f
d
ssoadminApplicationPortalOptions>aws:ssoadmin/ApplicationPortalOptions:ApplicationPortalOptionsSOptions for the portal associated with an application. See `portal_options` below.
"T
status" FStatus of the application. Valid values are `ENABLED` and `DISABLED`.
"À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*ï3
^
ssoadminApplicationAccessScope:aws:ssoadmin/applicationAccessScope:ApplicationAccessScope∏,Resource for managing an AWS SSO Admin Application Access Scope.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const exampleApplication = new aws.ssoadmin.Application("example", {
    name: "example",
    applicationProviderArn: "arn:aws:sso::aws:applicationProvider/custom",
    instanceArn: example.then(example => example.arns?.[0]),
});
const exampleApplicationAccessScope = new aws.ssoadmin.ApplicationAccessScope("example", {
    applicationArn: exampleApplication.applicationArn,
    authorizedTargets: ["arn:aws:sso::123456789012:application/ssoins-123456789012/apl-123456789012"],
    scope: "sso:account:access",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_application = aws.ssoadmin.Application("example",
    name="example",
    application_provider_arn="arn:aws:sso::aws:applicationProvider/custom",
    instance_arn=example.arns[0])
example_application_access_scope = aws.ssoadmin.ApplicationAccessScope("example",
    application_arn=example_application.application_arn,
    authorized_targets=["arn:aws:sso::123456789012:application/ssoins-123456789012/apl-123456789012"],
    scope="sso:account:access")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var exampleApplication = new Aws.SsoAdmin.Application("example", new()
    {
        Name = "example",
        ApplicationProviderArn = "arn:aws:sso::aws:applicationProvider/custom",
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
    });

    var exampleApplicationAccessScope = new Aws.SsoAdmin.ApplicationAccessScope("example", new()
    {
        ApplicationArn = exampleApplication.ApplicationArn,
        AuthorizedTargets = new[]
        {
            "arn:aws:sso::123456789012:application/ssoins-123456789012/apl-123456789012",
        },
        Scope = "sso:account:access",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		exampleApplication, err := ssoadmin.NewApplication(ctx, "example", &ssoadmin.ApplicationArgs{
			Name:                   pulumi.String("example"),
			ApplicationProviderArn: pulumi.String("arn:aws:sso::aws:applicationProvider/custom"),
			InstanceArn:            pulumi.String(example.Arns[0]),
		})
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewApplicationAccessScope(ctx, "example", &ssoadmin.ApplicationAccessScopeArgs{
			ApplicationArn: exampleApplication.ApplicationArn,
			AuthorizedTargets: pulumi.StringArray{
				pulumi.String("arn:aws:sso::123456789012:application/ssoins-123456789012/apl-123456789012"),
			},
			Scope: pulumi.String("sso:account:access"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.Application;
import com.pulumi.aws.ssoadmin.ApplicationArgs;
import com.pulumi.aws.ssoadmin.ApplicationAccessScope;
import com.pulumi.aws.ssoadmin.ApplicationAccessScopeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        var exampleApplication = new Application("exampleApplication", ApplicationArgs.builder()
            .name("example")
            .applicationProviderArn("arn:aws:sso::aws:applicationProvider/custom")
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .build());

        var exampleApplicationAccessScope = new ApplicationAccessScope("exampleApplicationAccessScope", ApplicationAccessScopeArgs.builder()
            .applicationArn(exampleApplication.applicationArn())
            .authorizedTargets("arn:aws:sso::123456789012:application/ssoins-123456789012/apl-123456789012")
            .scope("sso:account:access")
            .build());

    }
}
```
```yaml
resources:
  exampleApplication:
    type: aws:ssoadmin:Application
    name: example
    properties:
      name: example
      applicationProviderArn: arn:aws:sso::aws:applicationProvider/custom
      instanceArn: ${example.arns[0]}
  exampleApplicationAccessScope:
    type: aws:ssoadmin:ApplicationAccessScope
    name: example
    properties:
      applicationArn: ${exampleApplication.applicationArn}
      authorizedTargets:
        - arn:aws:sso::123456789012:application/ssoins-123456789012/apl-123456789012
      scope: sso:account:access
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSO Admin Application Access Scope using the `id`. For example:

```sh
$ pulumi import aws:ssoadmin/applicationAccessScope:ApplicationAccessScope example arn:aws:sso::123456789012:application/ssoins-123456789012/apl-123456789012,sso:account:access
```
t
applicationArn" ^Specifies the ARN of the application with the access scope with the targets to add or update.
z
authorizedTargetsB*" ]Specifies an array list of ARNs that represent the authorized targets for this access scope.
à
scope" {Specifies the name of the access scope to be associated with the specified targets.

The following arguments are optional:
"t
applicationArn" ^Specifies the ARN of the application with the access scope with the targets to add or update.
"z
authorizedTargetsB*" ]Specifies an array list of ARNs that represent the authorized targets for this access scope.
"à
scope" {Specifies the name of the access scope to be associated with the specified targets.

The following arguments are optional:
*Œ1
[
ssoadminApplicationAssignment8aws:ssoadmin/applicationAssignment:ApplicationAssignmentË,Resource for managing an AWS SSO Admin Application Assignment.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssoadmin.ApplicationAssignment("example", {
    applicationArn: exampleAwsSsoadminApplication.applicationArn,
    principalId: exampleAwsIdentitystoreUser.userId,
    principalType: "USER",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.ApplicationAssignment("example",
    application_arn=example_aws_ssoadmin_application["applicationArn"],
    principal_id=example_aws_identitystore_user["userId"],
    principal_type="USER")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SsoAdmin.ApplicationAssignment("example", new()
    {
        ApplicationArn = exampleAwsSsoadminApplication.ApplicationArn,
        PrincipalId = exampleAwsIdentitystoreUser.UserId,
        PrincipalType = "USER",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssoadmin.NewApplicationAssignment(ctx, "example", &ssoadmin.ApplicationAssignmentArgs{
			ApplicationArn: pulumi.Any(exampleAwsSsoadminApplication.ApplicationArn),
			PrincipalId:    pulumi.Any(exampleAwsIdentitystoreUser.UserId),
			PrincipalType:  pulumi.String("USER"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.ApplicationAssignment;
import com.pulumi.aws.ssoadmin.ApplicationAssignmentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ApplicationAssignment("example", ApplicationAssignmentArgs.builder()
            .applicationArn(exampleAwsSsoadminApplication.applicationArn())
            .principalId(exampleAwsIdentitystoreUser.userId())
            .principalType("USER")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssoadmin:ApplicationAssignment
    properties:
      applicationArn: ${exampleAwsSsoadminApplication.applicationArn}
      principalId: ${exampleAwsIdentitystoreUser.userId}
      principalType: USER
```
<!--End PulumiCodeChooser -->

### Group Type

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssoadmin.ApplicationAssignment("example", {
    applicationArn: exampleAwsSsoadminApplication.applicationArn,
    principalId: exampleAwsIdentitystoreGroup.groupId,
    principalType: "GROUP",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.ApplicationAssignment("example",
    application_arn=example_aws_ssoadmin_application["applicationArn"],
    principal_id=example_aws_identitystore_group["groupId"],
    principal_type="GROUP")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SsoAdmin.ApplicationAssignment("example", new()
    {
        ApplicationArn = exampleAwsSsoadminApplication.ApplicationArn,
        PrincipalId = exampleAwsIdentitystoreGroup.GroupId,
        PrincipalType = "GROUP",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssoadmin.NewApplicationAssignment(ctx, "example", &ssoadmin.ApplicationAssignmentArgs{
			ApplicationArn: pulumi.Any(exampleAwsSsoadminApplication.ApplicationArn),
			PrincipalId:    pulumi.Any(exampleAwsIdentitystoreGroup.GroupId),
			PrincipalType:  pulumi.String("GROUP"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.ApplicationAssignment;
import com.pulumi.aws.ssoadmin.ApplicationAssignmentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ApplicationAssignment("example", ApplicationAssignmentArgs.builder()
            .applicationArn(exampleAwsSsoadminApplication.applicationArn())
            .principalId(exampleAwsIdentitystoreGroup.groupId())
            .principalType("GROUP")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssoadmin:ApplicationAssignment
    properties:
      applicationArn: ${exampleAwsSsoadminApplication.applicationArn}
      principalId: ${exampleAwsIdentitystoreGroup.groupId}
      principalType: GROUP
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSO Admin Application Assignment using the `id`. For example:

```sh
$ pulumi import aws:ssoadmin/applicationAssignment:ApplicationAssignment example arn:aws:sso::123456789012:application/id-12345678,abcd1234,USER
```
.
applicationArn" ARN of the application.
`
principalId" MAn identifier for an object in IAM Identity Center, such as a user or group.
o
principalType" ZEntity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
".
applicationArn" ARN of the application.
"`
principalId" MAn identifier for an object in IAM Identity Center, such as a user or group.
"o
principalType" ZEntity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
*∂
Ç
ssoadmin"ApplicationAssignmentConfigurationRaws:ssoadmin/applicationAssignmentConfiguration:ApplicationAssignmentConfiguration˛Resource for managing an AWS SSO Admin Application Assignment Configuration.

By default, applications will require users to have an explicit assignment in order to access an application.
This resource can be used to adjust this default behavior if necessary.

> Deleting this resource will return the assignment configuration for the application to the default AWS behavior (ie. `assignment_required = true`).

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssoadmin.ApplicationAssignmentConfiguration("example", {
    applicationArn: exampleAwsSsoadminApplication.applicationArn,
    assignmentRequired: true,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.ApplicationAssignmentConfiguration("example",
    application_arn=example_aws_ssoadmin_application["applicationArn"],
    assignment_required=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SsoAdmin.ApplicationAssignmentConfiguration("example", new()
    {
        ApplicationArn = exampleAwsSsoadminApplication.ApplicationArn,
        AssignmentRequired = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssoadmin.NewApplicationAssignmentConfiguration(ctx, "example", &ssoadmin.ApplicationAssignmentConfigurationArgs{
			ApplicationArn:     pulumi.Any(exampleAwsSsoadminApplication.ApplicationArn),
			AssignmentRequired: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.ApplicationAssignmentConfiguration;
import com.pulumi.aws.ssoadmin.ApplicationAssignmentConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new ApplicationAssignmentConfiguration("example", ApplicationAssignmentConfigurationArgs.builder()
            .applicationArn(exampleAwsSsoadminApplication.applicationArn())
            .assignmentRequired(true)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssoadmin:ApplicationAssignmentConfiguration
    properties:
      applicationArn: ${exampleAwsSsoadminApplication.applicationArn}
      assignmentRequired: true
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSO Admin Application Assignment Configuration using the `id`. For example:

```sh
$ pulumi import aws:ssoadmin/applicationAssignmentConfiguration:ApplicationAssignmentConfiguration example arn:aws:sso::123456789012:application/id-12345678
```
.
applicationArn" ARN of the application.
•
assignmentRequired
 äIndicates whether users must have an explicit assignment to access the application. If `false`, all users have access to the application.
".
applicationArn" ARN of the application.
"•
assignmentRequired
 äIndicates whether users must have an explicit assignment to access the application. If `false`, all users have access to the application.
*¥P
y
ssoadminCustomerManagedPolicyAttachmentLaws:ssoadmin/customerManagedPolicyAttachment:CustomerManagedPolicyAttachment†GProvides a customer managed policy attachment for a Single Sign-On (SSO) Permission Set resource

> **NOTE:** Creating this resource will automatically [Provision the Permission Set](https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_ProvisionPermissionSet.html) to apply the corresponding updates to all assigned accounts.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const examplePermissionSet = new aws.ssoadmin.PermissionSet("example", {
    name: "Example",
    instanceArn: example.then(example => example.arns?.[0]),
});
const examplePolicy = new aws.iam.Policy("example", {
    name: "TestPolicy",
    description: "My test policy",
    policy: JSON.stringify({
        Version: "2012-10-17",
        Statement: [{
            Action: ["ec2:Describe*"],
            Effect: "Allow",
            Resource: "*",
        }],
    }),
});
const exampleCustomerManagedPolicyAttachment = new aws.ssoadmin.CustomerManagedPolicyAttachment("example", {
    instanceArn: examplePermissionSet.instanceArn,
    permissionSetArn: examplePermissionSet.arn,
    customerManagedPolicyReference: {
        name: examplePolicy.name,
        path: "/",
    },
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_permission_set = aws.ssoadmin.PermissionSet("example",
    name="Example",
    instance_arn=example.arns[0])
example_policy = aws.iam.Policy("example",
    name="TestPolicy",
    description="My test policy",
    policy=json.dumps({
        "Version": "2012-10-17",
        "Statement": [{
            "Action": ["ec2:Describe*"],
            "Effect": "Allow",
            "Resource": "*",
        }],
    }))
example_customer_managed_policy_attachment = aws.ssoadmin.CustomerManagedPolicyAttachment("example",
    instance_arn=example_permission_set.instance_arn,
    permission_set_arn=example_permission_set.arn,
    customer_managed_policy_reference={
        "name": example_policy.name,
        "path": "/",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var examplePermissionSet = new Aws.SsoAdmin.PermissionSet("example", new()
    {
        Name = "Example",
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
    });

    var examplePolicy = new Aws.Iam.Policy("example", new()
    {
        Name = "TestPolicy",
        Description = "My test policy",
        PolicyDocument = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Version"] = "2012-10-17",
            ["Statement"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["Action"] = new[]
                    {
                        "ec2:Describe*",
                    },
                    ["Effect"] = "Allow",
                    ["Resource"] = "*",
                },
            },
        }),
    });

    var exampleCustomerManagedPolicyAttachment = new Aws.SsoAdmin.CustomerManagedPolicyAttachment("example", new()
    {
        InstanceArn = examplePermissionSet.InstanceArn,
        PermissionSetArn = examplePermissionSet.Arn,
        CustomerManagedPolicyReference = new Aws.SsoAdmin.Inputs.CustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceArgs
        {
            Name = examplePolicy.Name,
            Path = "/",
        },
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		examplePermissionSet, err := ssoadmin.NewPermissionSet(ctx, "example", &ssoadmin.PermissionSetArgs{
			Name:        pulumi.String("Example"),
			InstanceArn: pulumi.String(example.Arns[0]),
		})
		if err != nil {
			return err
		}
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Version": "2012-10-17",
			"Statement": []map[string]interface{}{
				map[string]interface{}{
					"Action": []string{
						"ec2:Describe*",
					},
					"Effect":   "Allow",
					"Resource": "*",
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		examplePolicy, err := iam.NewPolicy(ctx, "example", &iam.PolicyArgs{
			Name:        pulumi.String("TestPolicy"),
			Description: pulumi.String("My test policy"),
			Policy:      pulumi.String(json0),
		})
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewCustomerManagedPolicyAttachment(ctx, "example", &ssoadmin.CustomerManagedPolicyAttachmentArgs{
			InstanceArn:      examplePermissionSet.InstanceArn,
			PermissionSetArn: examplePermissionSet.Arn,
			CustomerManagedPolicyReference: &ssoadmin.CustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceArgs{
				Name: examplePolicy.Name,
				Path: pulumi.String("/"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.PermissionSet;
import com.pulumi.aws.ssoadmin.PermissionSetArgs;
import com.pulumi.aws.iam.Policy;
import com.pulumi.aws.iam.PolicyArgs;
import com.pulumi.aws.ssoadmin.CustomerManagedPolicyAttachment;
import com.pulumi.aws.ssoadmin.CustomerManagedPolicyAttachmentArgs;
import com.pulumi.aws.ssoadmin.inputs.CustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        var examplePermissionSet = new PermissionSet("examplePermissionSet", PermissionSetArgs.builder()
            .name("Example")
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .build());

        var examplePolicy = new Policy("examplePolicy", PolicyArgs.builder()
            .name("TestPolicy")
            .description("My test policy")
            .policy(serializeJson(
                jsonObject(
                    jsonProperty("Version", "2012-10-17"),
                    jsonProperty("Statement", jsonArray(jsonObject(
                        jsonProperty("Action", jsonArray("ec2:Describe*")),
                        jsonProperty("Effect", "Allow"),
                        jsonProperty("Resource", "*")
                    )))
                )))
            .build());

        var exampleCustomerManagedPolicyAttachment = new CustomerManagedPolicyAttachment("exampleCustomerManagedPolicyAttachment", CustomerManagedPolicyAttachmentArgs.builder()
            .instanceArn(examplePermissionSet.instanceArn())
            .permissionSetArn(examplePermissionSet.arn())
            .customerManagedPolicyReference(CustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceArgs.builder()
                .name(examplePolicy.name())
                .path("/")
                .build())
            .build());

    }
}
```
```yaml
resources:
  examplePermissionSet:
    type: aws:ssoadmin:PermissionSet
    name: example
    properties:
      name: Example
      instanceArn: ${example.arns[0]}
  examplePolicy:
    type: aws:iam:Policy
    name: example
    properties:
      name: TestPolicy
      description: My test policy
      policy:
        fn::toJSON:
          Version: 2012-10-17
          Statement:
            - Action:
                - ec2:Describe*
              Effect: Allow
              Resource: '*'
  exampleCustomerManagedPolicyAttachment:
    type: aws:ssoadmin:CustomerManagedPolicyAttachment
    name: example
    properties:
      instanceArn: ${examplePermissionSet.instanceArn}
      permissionSetArn: ${examplePermissionSet.arn}
      customerManagedPolicyReference:
        name: ${examplePolicy.name}
        path: /
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSO Managed Policy Attachments using the `name`, `path`, `permission_set_arn`, and `instance_arn` separated by a comma (`,`). For example:

```sh
$ pulumi import aws:ssoadmin/customerManagedPolicyAttachment:CustomerManagedPolicyAttachment example TestPolicy,/,arn:aws:sso:::permissionSet/ssoins-2938j0x8920sbj72/ps-80383020jr9302rk,arn:aws:sso:::instance/ssoins-2938j0x8920sbj72
```
ƒ
customerManagedPolicyReference⁄:◊
‘
ssoadmin=CustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceàaws:ssoadmin/CustomerManagedPolicyAttachmentCustomerManagedPolicyReference:CustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceESpecifies the name and path of a customer managed policy. See below.
r
instanceArn" _The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
N
permissionSetArn" 6The Amazon Resource Name (ARN) of the Permission Set.
"ƒ
customerManagedPolicyReference⁄:◊
‘
ssoadmin=CustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceàaws:ssoadmin/CustomerManagedPolicyAttachmentCustomerManagedPolicyReference:CustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceESpecifies the name and path of a customer managed policy. See below.
"r
instanceArn" _The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
"N
permissionSetArn" 6The Amazon Resource Name (ARN) of the Permission Set.
*‡9
y
ssoadminInstanceAccessControlAttributesLaws:ssoadmin/instanceAccessControlAttributes:InstanceAccessControlAttributesÚ3Provides a Single Sign-On (SSO) ABAC Resource: https://docs.aws.amazon.com/singlesignon/latest/userguide/abac.html

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const exampleInstanceAccessControlAttributes = new aws.ssoadmin.InstanceAccessControlAttributes("example", {
    instanceArn: example.then(example => example.arns?.[0]),
    attributes: [
        {
            key: "name",
            values: [{
                sources: ["${path:name.givenName}"],
            }],
        },
        {
            key: "last",
            values: [{
                sources: ["${path:name.familyName}"],
            }],
        },
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_instance_access_control_attributes = aws.ssoadmin.InstanceAccessControlAttributes("example",
    instance_arn=example.arns[0],
    attributes=[
        {
            "key": "name",
            "values": [{
                "sources": ["${path:name.givenName}"],
            }],
        },
        {
            "key": "last",
            "values": [{
                "sources": ["${path:name.familyName}"],
            }],
        },
    ])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var exampleInstanceAccessControlAttributes = new Aws.SsoAdmin.InstanceAccessControlAttributes("example", new()
    {
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        Attributes = new[]
        {
            new Aws.SsoAdmin.Inputs.InstanceAccessControlAttributesAttributeArgs
            {
                Key = "name",
                Values = new[]
                {
                    new Aws.SsoAdmin.Inputs.InstanceAccessControlAttributesAttributeValueArgs
                    {
                        Sources = new[]
                        {
                            "${path:name.givenName}",
                        },
                    },
                },
            },
            new Aws.SsoAdmin.Inputs.InstanceAccessControlAttributesAttributeArgs
            {
                Key = "last",
                Values = new[]
                {
                    new Aws.SsoAdmin.Inputs.InstanceAccessControlAttributesAttributeValueArgs
                    {
                        Sources = new[]
                        {
                            "${path:name.familyName}",
                        },
                    },
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewInstanceAccessControlAttributes(ctx, "example", &ssoadmin.InstanceAccessControlAttributesArgs{
			InstanceArn: pulumi.String(example.Arns[0]),
			Attributes: ssoadmin.InstanceAccessControlAttributesAttributeArray{
				&ssoadmin.InstanceAccessControlAttributesAttributeArgs{
					Key: pulumi.String("name"),
					Values: ssoadmin.InstanceAccessControlAttributesAttributeValueArray{
						&ssoadmin.InstanceAccessControlAttributesAttributeValueArgs{
							Sources: pulumi.StringArray{
								pulumi.String("${path:name.givenName}"),
							},
						},
					},
				},
				&ssoadmin.InstanceAccessControlAttributesAttributeArgs{
					Key: pulumi.String("last"),
					Values: ssoadmin.InstanceAccessControlAttributesAttributeValueArray{
						&ssoadmin.InstanceAccessControlAttributesAttributeValueArgs{
							Sources: pulumi.StringArray{
								pulumi.String("${path:name.familyName}"),
							},
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.InstanceAccessControlAttributes;
import com.pulumi.aws.ssoadmin.InstanceAccessControlAttributesArgs;
import com.pulumi.aws.ssoadmin.inputs.InstanceAccessControlAttributesAttributeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        var exampleInstanceAccessControlAttributes = new InstanceAccessControlAttributes("exampleInstanceAccessControlAttributes", InstanceAccessControlAttributesArgs.builder()
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .attributes(            
                InstanceAccessControlAttributesAttributeArgs.builder()
                    .key("name")
                    .values(InstanceAccessControlAttributesAttributeValueArgs.builder()
                        .sources("${path:name.givenName}")
                        .build())
                    .build(),
                InstanceAccessControlAttributesAttributeArgs.builder()
                    .key("last")
                    .values(InstanceAccessControlAttributesAttributeValueArgs.builder()
                        .sources("${path:name.familyName}")
                        .build())
                    .build())
            .build());

    }
}
```
```yaml
resources:
  exampleInstanceAccessControlAttributes:
    type: aws:ssoadmin:InstanceAccessControlAttributes
    name: example
    properties:
      instanceArn: ${example.arns[0]}
      attributes:
        - key: name
          values:
            - sources:
                - $${path:name.givenName}
        - key: last
          values:
            - sources:
                - $${path:name.familyName}
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSO Account Assignments using the `instance_arn`. For example:

```sh
$ pulumi import aws:ssoadmin/instanceAccessControlAttributes:InstanceAccessControlAttributes example arn:aws:sso:::instance/ssoins-0123456789abcdef
```
€

attributesù*ö:ó
î
ssoadmin(InstanceAccessControlAttributesAttribute^aws:ssoadmin/InstanceAccessControlAttributesAttribute:InstanceAccessControlAttributesAttribute-See AccessControlAttribute for more details.
G
instanceArn" 4The Amazon Resource Name (ARN) of the SSO Instance.
"€

attributesù*ö:ó
î
ssoadmin(InstanceAccessControlAttributesAttribute^aws:ssoadmin/InstanceAccessControlAttributesAttribute:InstanceAccessControlAttributesAttribute-See AccessControlAttribute for more details.
"G
instanceArn" 4The Amazon Resource Name (ARN) of the SSO Instance.
"
status" "
statusReason" *˘{
a
ssoadminManagedPolicyAttachment<aws:ssoadmin/managedPolicyAttachment:ManagedPolicyAttachmentËuProvides an IAM managed policy for a Single Sign-On (SSO) Permission Set resource

> **NOTE:** Creating this resource will automatically [Provision the Permission Set](https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_ProvisionPermissionSet.html) to apply the corresponding updates to all assigned accounts.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const examplePermissionSet = new aws.ssoadmin.PermissionSet("example", {
    name: "Example",
    instanceArn: example.then(example => example.arns?.[0]),
});
const exampleManagedPolicyAttachment = new aws.ssoadmin.ManagedPolicyAttachment("example", {
    instanceArn: example.then(example => example.arns?.[0]),
    managedPolicyArn: "arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup",
    permissionSetArn: examplePermissionSet.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_permission_set = aws.ssoadmin.PermissionSet("example",
    name="Example",
    instance_arn=example.arns[0])
example_managed_policy_attachment = aws.ssoadmin.ManagedPolicyAttachment("example",
    instance_arn=example.arns[0],
    managed_policy_arn="arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup",
    permission_set_arn=example_permission_set.arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var examplePermissionSet = new Aws.SsoAdmin.PermissionSet("example", new()
    {
        Name = "Example",
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
    });

    var exampleManagedPolicyAttachment = new Aws.SsoAdmin.ManagedPolicyAttachment("example", new()
    {
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        ManagedPolicyArn = "arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup",
        PermissionSetArn = examplePermissionSet.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		examplePermissionSet, err := ssoadmin.NewPermissionSet(ctx, "example", &ssoadmin.PermissionSetArgs{
			Name:        pulumi.String("Example"),
			InstanceArn: pulumi.String(example.Arns[0]),
		})
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewManagedPolicyAttachment(ctx, "example", &ssoadmin.ManagedPolicyAttachmentArgs{
			InstanceArn:      pulumi.String(example.Arns[0]),
			ManagedPolicyArn: pulumi.String("arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup"),
			PermissionSetArn: examplePermissionSet.Arn,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.PermissionSet;
import com.pulumi.aws.ssoadmin.PermissionSetArgs;
import com.pulumi.aws.ssoadmin.ManagedPolicyAttachment;
import com.pulumi.aws.ssoadmin.ManagedPolicyAttachmentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        var examplePermissionSet = new PermissionSet("examplePermissionSet", PermissionSetArgs.builder()
            .name("Example")
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .build());

        var exampleManagedPolicyAttachment = new ManagedPolicyAttachment("exampleManagedPolicyAttachment", ManagedPolicyAttachmentArgs.builder()
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .managedPolicyArn("arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup")
            .permissionSetArn(examplePermissionSet.arn())
            .build());

    }
}
```
```yaml
resources:
  examplePermissionSet:
    type: aws:ssoadmin:PermissionSet
    name: example
    properties:
      name: Example
      instanceArn: ${example.arns[0]}
  exampleManagedPolicyAttachment:
    type: aws:ssoadmin:ManagedPolicyAttachment
    name: example
    properties:
      instanceArn: ${example.arns[0]}
      managedPolicyArn: arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup
      permissionSetArn: ${examplePermissionSet.arn}
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
```
<!--End PulumiCodeChooser -->

### With Account Assignment

> Because destruction of a managed policy attachment resource also re-provisions the associated permission set to all accounts, explicitly indicating the dependency with the account assignment resource via the `depends_on` meta argument is necessary to ensure proper deletion order when these resources are used together.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const examplePermissionSet = new aws.ssoadmin.PermissionSet("example", {
    name: "Example",
    instanceArn: example.then(example => example.arns?.[0]),
});
const exampleGroup = new aws.identitystore.Group("example", {
    identityStoreId: example.then(example => example.identityStoreIds?.[0]),
    displayName: "Admin",
    description: "Admin Group",
});
const accountAssignment = new aws.ssoadmin.AccountAssignment("account_assignment", {
    instanceArn: example.then(example => example.arns?.[0]),
    permissionSetArn: examplePermissionSet.arn,
    principalId: exampleGroup.groupId,
    principalType: "GROUP",
    targetId: "123456789012",
    targetType: "AWS_ACCOUNT",
});
const exampleManagedPolicyAttachment = new aws.ssoadmin.ManagedPolicyAttachment("example", {
    instanceArn: example.then(example => example.arns?.[0]),
    managedPolicyArn: "arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup",
    permissionSetArn: examplePermissionSet.arn,
}, {
    dependsOn: [exampleAwsSsoadminAccountAssignment],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_permission_set = aws.ssoadmin.PermissionSet("example",
    name="Example",
    instance_arn=example.arns[0])
example_group = aws.identitystore.Group("example",
    identity_store_id=example.identity_store_ids[0],
    display_name="Admin",
    description="Admin Group")
account_assignment = aws.ssoadmin.AccountAssignment("account_assignment",
    instance_arn=example.arns[0],
    permission_set_arn=example_permission_set.arn,
    principal_id=example_group.group_id,
    principal_type="GROUP",
    target_id="123456789012",
    target_type="AWS_ACCOUNT")
example_managed_policy_attachment = aws.ssoadmin.ManagedPolicyAttachment("example",
    instance_arn=example.arns[0],
    managed_policy_arn="arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup",
    permission_set_arn=example_permission_set.arn,
    opts = pulumi.ResourceOptions(depends_on=[example_aws_ssoadmin_account_assignment]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var examplePermissionSet = new Aws.SsoAdmin.PermissionSet("example", new()
    {
        Name = "Example",
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
    });

    var exampleGroup = new Aws.IdentityStore.Group("example", new()
    {
        IdentityStoreId = example.Apply(getInstancesResult => getInstancesResult.IdentityStoreIds[0]),
        DisplayName = "Admin",
        Description = "Admin Group",
    });

    var accountAssignment = new Aws.SsoAdmin.AccountAssignment("account_assignment", new()
    {
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        PermissionSetArn = examplePermissionSet.Arn,
        PrincipalId = exampleGroup.GroupId,
        PrincipalType = "GROUP",
        TargetId = "123456789012",
        TargetType = "AWS_ACCOUNT",
    });

    var exampleManagedPolicyAttachment = new Aws.SsoAdmin.ManagedPolicyAttachment("example", new()
    {
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        ManagedPolicyArn = "arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup",
        PermissionSetArn = examplePermissionSet.Arn,
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsSsoadminAccountAssignment,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/identitystore"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		examplePermissionSet, err := ssoadmin.NewPermissionSet(ctx, "example", &ssoadmin.PermissionSetArgs{
			Name:        pulumi.String("Example"),
			InstanceArn: pulumi.String(example.Arns[0]),
		})
		if err != nil {
			return err
		}
		exampleGroup, err := identitystore.NewGroup(ctx, "example", &identitystore.GroupArgs{
			IdentityStoreId: pulumi.String(example.IdentityStoreIds[0]),
			DisplayName:     pulumi.String("Admin"),
			Description:     pulumi.String("Admin Group"),
		})
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewAccountAssignment(ctx, "account_assignment", &ssoadmin.AccountAssignmentArgs{
			InstanceArn:      pulumi.String(example.Arns[0]),
			PermissionSetArn: examplePermissionSet.Arn,
			PrincipalId:      exampleGroup.GroupId,
			PrincipalType:    pulumi.String("GROUP"),
			TargetId:         pulumi.String("123456789012"),
			TargetType:       pulumi.String("AWS_ACCOUNT"),
		})
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewManagedPolicyAttachment(ctx, "example", &ssoadmin.ManagedPolicyAttachmentArgs{
			InstanceArn:      pulumi.String(example.Arns[0]),
			ManagedPolicyArn: pulumi.String("arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup"),
			PermissionSetArn: examplePermissionSet.Arn,
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsSsoadminAccountAssignment,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.PermissionSet;
import com.pulumi.aws.ssoadmin.PermissionSetArgs;
import com.pulumi.aws.identitystore.Group;
import com.pulumi.aws.identitystore.GroupArgs;
import com.pulumi.aws.ssoadmin.AccountAssignment;
import com.pulumi.aws.ssoadmin.AccountAssignmentArgs;
import com.pulumi.aws.ssoadmin.ManagedPolicyAttachment;
import com.pulumi.aws.ssoadmin.ManagedPolicyAttachmentArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        var examplePermissionSet = new PermissionSet("examplePermissionSet", PermissionSetArgs.builder()
            .name("Example")
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .build());

        var exampleGroup = new Group("exampleGroup", GroupArgs.builder()
            .identityStoreId(example.applyValue(getInstancesResult -> getInstancesResult.identityStoreIds()[0]))
            .displayName("Admin")
            .description("Admin Group")
            .build());

        var accountAssignment = new AccountAssignment("accountAssignment", AccountAssignmentArgs.builder()
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .permissionSetArn(examplePermissionSet.arn())
            .principalId(exampleGroup.groupId())
            .principalType("GROUP")
            .targetId("123456789012")
            .targetType("AWS_ACCOUNT")
            .build());

        var exampleManagedPolicyAttachment = new ManagedPolicyAttachment("exampleManagedPolicyAttachment", ManagedPolicyAttachmentArgs.builder()
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .managedPolicyArn("arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup")
            .permissionSetArn(examplePermissionSet.arn())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsSsoadminAccountAssignment)
                .build());

    }
}
```
```yaml
resources:
  examplePermissionSet:
    type: aws:ssoadmin:PermissionSet
    name: example
    properties:
      name: Example
      instanceArn: ${example.arns[0]}
  exampleGroup:
    type: aws:identitystore:Group
    name: example
    properties:
      identityStoreId: ${example.identityStoreIds[0]}
      displayName: Admin
      description: Admin Group
  accountAssignment:
    type: aws:ssoadmin:AccountAssignment
    name: account_assignment
    properties:
      instanceArn: ${example.arns[0]}
      permissionSetArn: ${examplePermissionSet.arn}
      principalId: ${exampleGroup.groupId}
      principalType: GROUP
      targetId: '123456789012'
      targetType: AWS_ACCOUNT
  exampleManagedPolicyAttachment:
    type: aws:ssoadmin:ManagedPolicyAttachment
    name: example
    properties:
      instanceArn: ${example.arns[0]}
      managedPolicyArn: arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup
      permissionSetArn: ${examplePermissionSet.arn}
    options:
      dependsOn:
        - ${exampleAwsSsoadminAccountAssignment}
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSO Managed Policy Attachments using the `managed_policy_arn`, `permission_set_arn`, and `instance_arn` separated by a comma (`,`). For example:

```sh
$ pulumi import aws:ssoadmin/managedPolicyAttachment:ManagedPolicyAttachment example arn:aws:iam::aws:policy/AlexaForBusinessDeviceSetup,arn:aws:sso:::permissionSet/ssoins-2938j0x8920sbj72/ps-80383020jr9302rk,arn:aws:sso:::instance/ssoins-2938j0x8920sbj72
```
r
instanceArn" _The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
p
managedPolicyArn" XThe IAM managed policy Amazon Resource Name (ARN) to be attached to the Permission Set.
N
permissionSetArn" 6The Amazon Resource Name (ARN) of the Permission Set.
"r
instanceArn" _The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
"p
managedPolicyArn" XThe IAM managed policy Amazon Resource Name (ARN) to be attached to the Permission Set.
"=
managedPolicyName" $The name of the IAM Managed Policy.
"N
permissionSetArn" 6The Amazon Resource Name (ARN) of the Permission Set.
*√/
C
ssoadminPermissionSet(aws:ssoadmin/permissionSet:PermissionSet¥!Provides a Single Sign-On (SSO) Permission Set resource

> **NOTE:** Updating this resource will automatically [Provision the Permission Set](https://docs.aws.amazon.com/singlesignon/latest/APIReference/API_ProvisionPermissionSet.html) to apply the corresponding updates to all assigned accounts.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const examplePermissionSet = new aws.ssoadmin.PermissionSet("example", {
    name: "Example",
    description: "An example",
    instanceArn: example.then(example => example.arns?.[0]),
    relayState: "https://s3.console.aws.amazon.com/s3/home?region=us-east-1#",
    sessionDuration: "PT2H",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_permission_set = aws.ssoadmin.PermissionSet("example",
    name="Example",
    description="An example",
    instance_arn=example.arns[0],
    relay_state="https://s3.console.aws.amazon.com/s3/home?region=us-east-1#",
    session_duration="PT2H")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var examplePermissionSet = new Aws.SsoAdmin.PermissionSet("example", new()
    {
        Name = "Example",
        Description = "An example",
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        RelayState = "https://s3.console.aws.amazon.com/s3/home?region=us-east-1#",
        SessionDuration = "PT2H",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewPermissionSet(ctx, "example", &ssoadmin.PermissionSetArgs{
			Name:            pulumi.String("Example"),
			Description:     pulumi.String("An example"),
			InstanceArn:     pulumi.String(example.Arns[0]),
			RelayState:      pulumi.String("https://s3.console.aws.amazon.com/s3/home?region=us-east-1#"),
			SessionDuration: pulumi.String("PT2H"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.PermissionSet;
import com.pulumi.aws.ssoadmin.PermissionSetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        var examplePermissionSet = new PermissionSet("examplePermissionSet", PermissionSetArgs.builder()
            .name("Example")
            .description("An example")
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .relayState("https://s3.console.aws.amazon.com/s3/home?region=us-east-1#")
            .sessionDuration("PT2H")
            .build());

    }
}
```
```yaml
resources:
  examplePermissionSet:
    type: aws:ssoadmin:PermissionSet
    name: example
    properties:
      name: Example
      description: An example
      instanceArn: ${example.arns[0]}
      relayState: https://s3.console.aws.amazon.com/s3/home?region=us-east-1#
      sessionDuration: PT2H
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSO Permission Sets using the `arn` and `instance_arn` separated by a comma (`,`). For example:

```sh
$ pulumi import aws:ssoadmin/permissionSet:PermissionSet example arn:aws:sso:::permissionSet/ssoins-2938j0x8920sbj72/ps-80383020jr9302rk,arn:aws:sso:::instance/ssoins-2938j0x8920sbj72
```
<
descriptionB" 'The description of the Permission Set.
r
instanceArn" _The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
.
nameB"  The name of the Permission Set.
Ñ

relayStateB" pThe relay state URL used to redirect users within the application during the federation authentication process.
Ñ
sessionDurationB" kThe length of time that the application user sessions are valid in the ISO-8601 standard. Default: `PT1H`.
»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"A
arn" 6The Amazon Resource Name (ARN) of the Permission Set.
"Å
createdDate" nThe date the Permission Set was created in [RFC3339 format](https://tools.ietf.org/html/rfc3339#section-5.8).
"<
descriptionB" 'The description of the Permission Set.
"r
instanceArn" _The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
",
name"  The name of the Permission Set.
"Ñ

relayStateB" pThe relay state URL used to redirect users within the application during the federation authentication process.
"Ñ
sessionDurationB" kThe length of time that the application user sessions are valid in the ISO-8601 standard. Default: `PT1H`.
"»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*å?
g
ssoadminPermissionSetInlinePolicy@aws:ssoadmin/permissionSetInlinePolicy:PermissionSetInlinePolicyÇ:## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const examplePermissionSet = new aws.ssoadmin.PermissionSet("example", {
    name: "Example",
    instanceArn: example.then(example => example.arns?.[0]),
});
const exampleGetPolicyDocument = aws.iam.getPolicyDocument({
    statements: [{
        sid: "1",
        actions: [
            "s3:ListAllMyBuckets",
            "s3:GetBucketLocation",
        ],
        resources: ["arn:aws:s3:::*"],
    }],
});
const examplePermissionSetInlinePolicy = new aws.ssoadmin.PermissionSetInlinePolicy("example", {
    inlinePolicy: exampleGetPolicyDocument.then(exampleGetPolicyDocument => exampleGetPolicyDocument.json),
    instanceArn: example.then(example => example.arns?.[0]),
    permissionSetArn: examplePermissionSet.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_permission_set = aws.ssoadmin.PermissionSet("example",
    name="Example",
    instance_arn=example.arns[0])
example_get_policy_document = aws.iam.get_policy_document(statements=[{
    "sid": "1",
    "actions": [
        "s3:ListAllMyBuckets",
        "s3:GetBucketLocation",
    ],
    "resources": ["arn:aws:s3:::*"],
}])
example_permission_set_inline_policy = aws.ssoadmin.PermissionSetInlinePolicy("example",
    inline_policy=example_get_policy_document.json,
    instance_arn=example.arns[0],
    permission_set_arn=example_permission_set.arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var examplePermissionSet = new Aws.SsoAdmin.PermissionSet("example", new()
    {
        Name = "Example",
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
    });

    var exampleGetPolicyDocument = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "1",
                Actions = new[]
                {
                    "s3:ListAllMyBuckets",
                    "s3:GetBucketLocation",
                },
                Resources = new[]
                {
                    "arn:aws:s3:::*",
                },
            },
        },
    });

    var examplePermissionSetInlinePolicy = new Aws.SsoAdmin.PermissionSetInlinePolicy("example", new()
    {
        InlinePolicy = exampleGetPolicyDocument.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        PermissionSetArn = examplePermissionSet.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		examplePermissionSet, err := ssoadmin.NewPermissionSet(ctx, "example", &ssoadmin.PermissionSetArgs{
			Name:        pulumi.String("Example"),
			InstanceArn: pulumi.String(example.Arns[0]),
		})
		if err != nil {
			return err
		}
		exampleGetPolicyDocument, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Sid: pulumi.StringRef("1"),
					Actions: []string{
						"s3:ListAllMyBuckets",
						"s3:GetBucketLocation",
					},
					Resources: []string{
						"arn:aws:s3:::*",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewPermissionSetInlinePolicy(ctx, "example", &ssoadmin.PermissionSetInlinePolicyArgs{
			InlinePolicy:     pulumi.String(exampleGetPolicyDocument.Json),
			InstanceArn:      pulumi.String(example.Arns[0]),
			PermissionSetArn: examplePermissionSet.Arn,
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.PermissionSet;
import com.pulumi.aws.ssoadmin.PermissionSetArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.ssoadmin.PermissionSetInlinePolicy;
import com.pulumi.aws.ssoadmin.PermissionSetInlinePolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        var examplePermissionSet = new PermissionSet("examplePermissionSet", PermissionSetArgs.builder()
            .name("Example")
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .build());

        final var exampleGetPolicyDocument = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("1")
                .actions(                
                    "s3:ListAllMyBuckets",
                    "s3:GetBucketLocation")
                .resources("arn:aws:s3:::*")
                .build())
            .build());

        var examplePermissionSetInlinePolicy = new PermissionSetInlinePolicy("examplePermissionSetInlinePolicy", PermissionSetInlinePolicyArgs.builder()
            .inlinePolicy(exampleGetPolicyDocument.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .permissionSetArn(examplePermissionSet.arn())
            .build());

    }
}
```
```yaml
resources:
  examplePermissionSet:
    type: aws:ssoadmin:PermissionSet
    name: example
    properties:
      name: Example
      instanceArn: ${example.arns[0]}
  examplePermissionSetInlinePolicy:
    type: aws:ssoadmin:PermissionSetInlinePolicy
    name: example
    properties:
      inlinePolicy: ${exampleGetPolicyDocument.json}
      instanceArn: ${example.arns[0]}
      permissionSetArn: ${examplePermissionSet.arn}
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
  exampleGetPolicyDocument:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: '1'
            actions:
              - s3:ListAllMyBuckets
              - s3:GetBucketLocation
            resources:
              - arn:aws:s3:::*
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSO Permission Set Inline Policies using the `permission_set_arn` and `instance_arn` separated by a comma (`,`). For example:

```sh
$ pulumi import aws:ssoadmin/permissionSetInlinePolicy:PermissionSetInlinePolicy example arn:aws:sso:::permissionSet/ssoins-2938j0x8920sbj72/ps-80383020jr9302rk,arn:aws:sso:::instance/ssoins-2938j0x8920sbj72
```
I
inlinePolicy" 5The IAM inline policy to attach to a Permission Set.
r
instanceArn" _The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
N
permissionSetArn" 6The Amazon Resource Name (ARN) of the Permission Set.
"I
inlinePolicy" 5The IAM inline policy to attach to a Permission Set.
"r
instanceArn" _The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
"N
permissionSetArn" 6The Amazon Resource Name (ARN) of the Permission Set.
*ép
s
ssoadminPermissionsBoundaryAttachmentHaws:ssoadmin/permissionsBoundaryAttachment:PermissionsBoundaryAttachmentòhAttaches a permissions boundary policy to a Single Sign-On (SSO) Permission Set resource.

> **NOTE:** A permission set can have at most one permissions boundary attached; using more than one `aws.ssoadmin.PermissionsBoundaryAttachment` references the same permission set will show a permanent difference.

## Example Usage

### Attaching a customer-managed policy

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const examplePermissionSet = new aws.ssoadmin.PermissionSet("example", {
    name: "Example",
    instanceArn: example.then(example => example.arns?.[0]),
});
const examplePolicy = new aws.iam.Policy("example", {
    name: "TestPolicy",
    description: "My test policy",
    policy: JSON.stringify({
        Version: "2012-10-17",
        Statement: [{
            Action: ["ec2:Describe*"],
            Effect: "Allow",
            Resource: "*",
        }],
    }),
});
const examplePermissionsBoundaryAttachment = new aws.ssoadmin.PermissionsBoundaryAttachment("example", {
    instanceArn: examplePermissionSet.instanceArn,
    permissionSetArn: examplePermissionSet.arn,
    permissionsBoundary: {
        customerManagedPolicyReference: {
            name: examplePolicy.name,
            path: "/",
        },
    },
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_permission_set = aws.ssoadmin.PermissionSet("example",
    name="Example",
    instance_arn=example.arns[0])
example_policy = aws.iam.Policy("example",
    name="TestPolicy",
    description="My test policy",
    policy=json.dumps({
        "Version": "2012-10-17",
        "Statement": [{
            "Action": ["ec2:Describe*"],
            "Effect": "Allow",
            "Resource": "*",
        }],
    }))
example_permissions_boundary_attachment = aws.ssoadmin.PermissionsBoundaryAttachment("example",
    instance_arn=example_permission_set.instance_arn,
    permission_set_arn=example_permission_set.arn,
    permissions_boundary={
        "customer_managed_policy_reference": {
            "name": example_policy.name,
            "path": "/",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var examplePermissionSet = new Aws.SsoAdmin.PermissionSet("example", new()
    {
        Name = "Example",
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
    });

    var examplePolicy = new Aws.Iam.Policy("example", new()
    {
        Name = "TestPolicy",
        Description = "My test policy",
        PolicyDocument = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Version"] = "2012-10-17",
            ["Statement"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["Action"] = new[]
                    {
                        "ec2:Describe*",
                    },
                    ["Effect"] = "Allow",
                    ["Resource"] = "*",
                },
            },
        }),
    });

    var examplePermissionsBoundaryAttachment = new Aws.SsoAdmin.PermissionsBoundaryAttachment("example", new()
    {
        InstanceArn = examplePermissionSet.InstanceArn,
        PermissionSetArn = examplePermissionSet.Arn,
        PermissionsBoundary = new Aws.SsoAdmin.Inputs.PermissionsBoundaryAttachmentPermissionsBoundaryArgs
        {
            CustomerManagedPolicyReference = new Aws.SsoAdmin.Inputs.PermissionsBoundaryAttachmentPermissionsBoundaryCustomerManagedPolicyReferenceArgs
            {
                Name = examplePolicy.Name,
                Path = "/",
            },
        },
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		examplePermissionSet, err := ssoadmin.NewPermissionSet(ctx, "example", &ssoadmin.PermissionSetArgs{
			Name:        pulumi.String("Example"),
			InstanceArn: pulumi.String(example.Arns[0]),
		})
		if err != nil {
			return err
		}
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Version": "2012-10-17",
			"Statement": []map[string]interface{}{
				map[string]interface{}{
					"Action": []string{
						"ec2:Describe*",
					},
					"Effect":   "Allow",
					"Resource": "*",
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		examplePolicy, err := iam.NewPolicy(ctx, "example", &iam.PolicyArgs{
			Name:        pulumi.String("TestPolicy"),
			Description: pulumi.String("My test policy"),
			Policy:      pulumi.String(json0),
		})
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewPermissionsBoundaryAttachment(ctx, "example", &ssoadmin.PermissionsBoundaryAttachmentArgs{
			InstanceArn:      examplePermissionSet.InstanceArn,
			PermissionSetArn: examplePermissionSet.Arn,
			PermissionsBoundary: &ssoadmin.PermissionsBoundaryAttachmentPermissionsBoundaryArgs{
				CustomerManagedPolicyReference: &ssoadmin.PermissionsBoundaryAttachmentPermissionsBoundaryCustomerManagedPolicyReferenceArgs{
					Name: examplePolicy.Name,
					Path: pulumi.String("/"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.PermissionSet;
import com.pulumi.aws.ssoadmin.PermissionSetArgs;
import com.pulumi.aws.iam.Policy;
import com.pulumi.aws.iam.PolicyArgs;
import com.pulumi.aws.ssoadmin.PermissionsBoundaryAttachment;
import com.pulumi.aws.ssoadmin.PermissionsBoundaryAttachmentArgs;
import com.pulumi.aws.ssoadmin.inputs.PermissionsBoundaryAttachmentPermissionsBoundaryArgs;
import com.pulumi.aws.ssoadmin.inputs.PermissionsBoundaryAttachmentPermissionsBoundaryCustomerManagedPolicyReferenceArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        var examplePermissionSet = new PermissionSet("examplePermissionSet", PermissionSetArgs.builder()
            .name("Example")
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .build());

        var examplePolicy = new Policy("examplePolicy", PolicyArgs.builder()
            .name("TestPolicy")
            .description("My test policy")
            .policy(serializeJson(
                jsonObject(
                    jsonProperty("Version", "2012-10-17"),
                    jsonProperty("Statement", jsonArray(jsonObject(
                        jsonProperty("Action", jsonArray("ec2:Describe*")),
                        jsonProperty("Effect", "Allow"),
                        jsonProperty("Resource", "*")
                    )))
                )))
            .build());

        var examplePermissionsBoundaryAttachment = new PermissionsBoundaryAttachment("examplePermissionsBoundaryAttachment", PermissionsBoundaryAttachmentArgs.builder()
            .instanceArn(examplePermissionSet.instanceArn())
            .permissionSetArn(examplePermissionSet.arn())
            .permissionsBoundary(PermissionsBoundaryAttachmentPermissionsBoundaryArgs.builder()
                .customerManagedPolicyReference(PermissionsBoundaryAttachmentPermissionsBoundaryCustomerManagedPolicyReferenceArgs.builder()
                    .name(examplePolicy.name())
                    .path("/")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  examplePermissionSet:
    type: aws:ssoadmin:PermissionSet
    name: example
    properties:
      name: Example
      instanceArn: ${example.arns[0]}
  examplePolicy:
    type: aws:iam:Policy
    name: example
    properties:
      name: TestPolicy
      description: My test policy
      policy:
        fn::toJSON:
          Version: 2012-10-17
          Statement:
            - Action:
                - ec2:Describe*
              Effect: Allow
              Resource: '*'
  examplePermissionsBoundaryAttachment:
    type: aws:ssoadmin:PermissionsBoundaryAttachment
    name: example
    properties:
      instanceArn: ${examplePermissionSet.instanceArn}
      permissionSetArn: ${examplePermissionSet.arn}
      permissionsBoundary:
        customerManagedPolicyReference:
          name: ${examplePolicy.name}
          path: /
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
```
<!--End PulumiCodeChooser -->

### Attaching an AWS-managed policy

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ssoadmin.PermissionsBoundaryAttachment("example", {
    instanceArn: exampleAwsSsoadminPermissionSet.instanceArn,
    permissionSetArn: exampleAwsSsoadminPermissionSet.arn,
    permissionsBoundary: {
        managedPolicyArn: "arn:aws:iam::aws:policy/ReadOnlyAccess",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.PermissionsBoundaryAttachment("example",
    instance_arn=example_aws_ssoadmin_permission_set["instanceArn"],
    permission_set_arn=example_aws_ssoadmin_permission_set["arn"],
    permissions_boundary={
        "managed_policy_arn": "arn:aws:iam::aws:policy/ReadOnlyAccess",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.SsoAdmin.PermissionsBoundaryAttachment("example", new()
    {
        InstanceArn = exampleAwsSsoadminPermissionSet.InstanceArn,
        PermissionSetArn = exampleAwsSsoadminPermissionSet.Arn,
        PermissionsBoundary = new Aws.SsoAdmin.Inputs.PermissionsBoundaryAttachmentPermissionsBoundaryArgs
        {
            ManagedPolicyArn = "arn:aws:iam::aws:policy/ReadOnlyAccess",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssoadmin.NewPermissionsBoundaryAttachment(ctx, "example", &ssoadmin.PermissionsBoundaryAttachmentArgs{
			InstanceArn:      pulumi.Any(exampleAwsSsoadminPermissionSet.InstanceArn),
			PermissionSetArn: pulumi.Any(exampleAwsSsoadminPermissionSet.Arn),
			PermissionsBoundary: &ssoadmin.PermissionsBoundaryAttachmentPermissionsBoundaryArgs{
				ManagedPolicyArn: pulumi.String("arn:aws:iam::aws:policy/ReadOnlyAccess"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.PermissionsBoundaryAttachment;
import com.pulumi.aws.ssoadmin.PermissionsBoundaryAttachmentArgs;
import com.pulumi.aws.ssoadmin.inputs.PermissionsBoundaryAttachmentPermissionsBoundaryArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new PermissionsBoundaryAttachment("example", PermissionsBoundaryAttachmentArgs.builder()
            .instanceArn(exampleAwsSsoadminPermissionSet.instanceArn())
            .permissionSetArn(exampleAwsSsoadminPermissionSet.arn())
            .permissionsBoundary(PermissionsBoundaryAttachmentPermissionsBoundaryArgs.builder()
                .managedPolicyArn("arn:aws:iam::aws:policy/ReadOnlyAccess")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ssoadmin:PermissionsBoundaryAttachment
    properties:
      instanceArn: ${exampleAwsSsoadminPermissionSet.instanceArn}
      permissionSetArn: ${exampleAwsSsoadminPermissionSet.arn}
      permissionsBoundary:
        managedPolicyArn: arn:aws:iam::aws:policy/ReadOnlyAccess
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSO Admin Permissions Boundary Attachments using the `permission_set_arn` and `instance_arn`, separated by a comma (`,`). For example:

```sh
$ pulumi import aws:ssoadmin/permissionsBoundaryAttachment:PermissionsBoundaryAttachment example arn:aws:sso:::permissionSet/ssoins-2938j0x8920sbj72/ps-80383020jr9302rk,arn:aws:sso:::instance/ssoins-2938j0x8920sbj72
```
r
instanceArn" _The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
N
permissionSetArn" 6The Amazon Resource Name (ARN) of the Permission Set.
¯
permissionsBoundary≤:Ø
¨
ssoadmin0PermissionsBoundaryAttachmentPermissionsBoundarynaws:ssoadmin/PermissionsBoundaryAttachmentPermissionsBoundary:PermissionsBoundaryAttachmentPermissionsBoundary,The permissions boundary policy. See below.
"r
instanceArn" _The Amazon Resource Name (ARN) of the SSO Instance under which the operation will be executed.
"N
permissionSetArn" 6The Amazon Resource Name (ARN) of the Permission Set.
"¯
permissionsBoundary≤:Ø
¨
ssoadmin0PermissionsBoundaryAttachmentPermissionsBoundarynaws:ssoadmin/PermissionsBoundaryAttachmentPermissionsBoundary:PermissionsBoundaryAttachmentPermissionsBoundary,The permissions boundary policy. See below.
*ûA
R
ssoadminTrustedTokenIssuer2aws:ssoadmin/trustedTokenIssuer:TrustedTokenIssuerë/Resource for managing an AWS SSO Admin Trusted Token Issuer.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const exampleTrustedTokenIssuer = new aws.ssoadmin.TrustedTokenIssuer("example", {
    name: "example",
    instanceArn: example.then(example => example.arns?.[0]),
    trustedTokenIssuerType: "OIDC_JWT",
    trustedTokenIssuerConfiguration: {
        oidcJwtConfiguration: {
            claimAttributePath: "email",
            identityStoreAttributePath: "emails.value",
            issuerUrl: "https://example.com",
            jwksRetrievalOption: "OPEN_ID_DISCOVERY",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_trusted_token_issuer = aws.ssoadmin.TrustedTokenIssuer("example",
    name="example",
    instance_arn=example.arns[0],
    trusted_token_issuer_type="OIDC_JWT",
    trusted_token_issuer_configuration={
        "oidc_jwt_configuration": {
            "claim_attribute_path": "email",
            "identity_store_attribute_path": "emails.value",
            "issuer_url": "https://example.com",
            "jwks_retrieval_option": "OPEN_ID_DISCOVERY",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var exampleTrustedTokenIssuer = new Aws.SsoAdmin.TrustedTokenIssuer("example", new()
    {
        Name = "example",
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        TrustedTokenIssuerType = "OIDC_JWT",
        TrustedTokenIssuerConfiguration = new Aws.SsoAdmin.Inputs.TrustedTokenIssuerTrustedTokenIssuerConfigurationArgs
        {
            OidcJwtConfiguration = new Aws.SsoAdmin.Inputs.TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfigurationArgs
            {
                ClaimAttributePath = "email",
                IdentityStoreAttributePath = "emails.value",
                IssuerUrl = "https://example.com",
                JwksRetrievalOption = "OPEN_ID_DISCOVERY",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		_, err = ssoadmin.NewTrustedTokenIssuer(ctx, "example", &ssoadmin.TrustedTokenIssuerArgs{
			Name:                   pulumi.String("example"),
			InstanceArn:            pulumi.String(example.Arns[0]),
			TrustedTokenIssuerType: pulumi.String("OIDC_JWT"),
			TrustedTokenIssuerConfiguration: &ssoadmin.TrustedTokenIssuerTrustedTokenIssuerConfigurationArgs{
				OidcJwtConfiguration: &ssoadmin.TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfigurationArgs{
					ClaimAttributePath:         pulumi.String("email"),
					IdentityStoreAttributePath: pulumi.String("emails.value"),
					IssuerUrl:                  pulumi.String("https://example.com"),
					JwksRetrievalOption:        pulumi.String("OPEN_ID_DISCOVERY"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.TrustedTokenIssuer;
import com.pulumi.aws.ssoadmin.TrustedTokenIssuerArgs;
import com.pulumi.aws.ssoadmin.inputs.TrustedTokenIssuerTrustedTokenIssuerConfigurationArgs;
import com.pulumi.aws.ssoadmin.inputs.TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        var exampleTrustedTokenIssuer = new TrustedTokenIssuer("exampleTrustedTokenIssuer", TrustedTokenIssuerArgs.builder()
            .name("example")
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .trustedTokenIssuerType("OIDC_JWT")
            .trustedTokenIssuerConfiguration(TrustedTokenIssuerTrustedTokenIssuerConfigurationArgs.builder()
                .oidcJwtConfiguration(TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfigurationArgs.builder()
                    .claimAttributePath("email")
                    .identityStoreAttributePath("emails.value")
                    .issuerUrl("https://example.com")
                    .jwksRetrievalOption("OPEN_ID_DISCOVERY")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  exampleTrustedTokenIssuer:
    type: aws:ssoadmin:TrustedTokenIssuer
    name: example
    properties:
      name: example
      instanceArn: ${example.arns[0]}
      trustedTokenIssuerType: OIDC_JWT
      trustedTokenIssuerConfiguration:
        oidcJwtConfiguration:
          claimAttributePath: email
          identityStoreAttributePath: emails.value
          issuerUrl: https://example.com
          jwksRetrievalOption: OPEN_ID_DISCOVERY
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SSO Admin Trusted Token Issuer using the `id`. For example:

```sh
$ pulumi import aws:ssoadmin/trustedTokenIssuer:TrustedTokenIssuer example arn:aws:sso::123456789012:trustedTokenIssuer/ssoins-lu1ye3gew4mbc7ju/tti-2657c556-9707-11ee-b9d1-0242ac120002
```
ù
clientTokenB" áA unique, case-sensitive ID that you provide to ensure the idempotency of the request. AWS generates a random value when not provided.
?
instanceArn" ,ARN of the instance of IAM Identity Center.
0
nameB" "Name of the trusted token issuer.
À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
à
trustedTokenIssuerConfiguration∏Bµ:≤
Ø
ssoadmin1TrustedTokenIssuerTrustedTokenIssuerConfigurationpaws:ssoadmin/TrustedTokenIssuerTrustedTokenIssuerConfiguration:TrustedTokenIssuerTrustedTokenIssuerConfiguration©A block that specifies settings that apply to the trusted token issuer, these change depending on the type you specify in `trusted_token_issuer_type`. Documented below.
ë
trustedTokenIssuerType" sSpecifies the type of the trusted token issuer. Valid values are `OIDC_JWT`

The following arguments are optional:
",
arn" !ARN of the trusted token issuer.
"ù
clientTokenB" áA unique, case-sensitive ID that you provide to ensure the idempotency of the request. AWS generates a random value when not provided.
"?
instanceArn" ,ARN of the instance of IAM Identity Center.
".
name" "Name of the trusted token issuer.
"À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"à
trustedTokenIssuerConfiguration∏Bµ:≤
Ø
ssoadmin1TrustedTokenIssuerTrustedTokenIssuerConfigurationpaws:ssoadmin/TrustedTokenIssuerTrustedTokenIssuerConfiguration:TrustedTokenIssuerTrustedTokenIssuerConfiguration©A block that specifies settings that apply to the trusted token issuer, these change depending on the type you specify in `trusted_token_issuer_type`. Documented below.
"ë
trustedTokenIssuerType" sSpecifies the type of the trusted token issuer. Valid values are `OIDC_JWT`

The following arguments are optional:
*Ä
7
storagegatewayCacheaws:storagegateway/cache:CacheîManages an AWS Storage Gateway cache.

> **NOTE:** The Storage Gateway API provides no method to remove a cache disk. Destroying this resource does not perform any Storage Gateway actions.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.Cache("example", {
    diskId: exampleAwsStoragegatewayLocalDisk.id,
    gatewayArn: exampleAwsStoragegatewayGateway.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.Cache("example",
    disk_id=example_aws_storagegateway_local_disk["id"],
    gateway_arn=example_aws_storagegateway_gateway["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.Cache("example", new()
    {
        DiskId = exampleAwsStoragegatewayLocalDisk.Id,
        GatewayArn = exampleAwsStoragegatewayGateway.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewCache(ctx, "example", &storagegateway.CacheArgs{
			DiskId:     pulumi.Any(exampleAwsStoragegatewayLocalDisk.Id),
			GatewayArn: pulumi.Any(exampleAwsStoragegatewayGateway.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.Cache;
import com.pulumi.aws.storagegateway.CacheArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Cache("example", CacheArgs.builder()
            .diskId(exampleAwsStoragegatewayLocalDisk.id())
            .gatewayArn(exampleAwsStoragegatewayGateway.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:Cache
    properties:
      diskId: ${exampleAwsStoragegatewayLocalDisk.id}
      gatewayArn: ${exampleAwsStoragegatewayGateway.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_storagegateway_cache` using the gateway Amazon Resource Name (ARN) and local disk identifier separated with a colon (`:`). For example:

```sh
$ pulumi import aws:storagegateway/cache:Cache example arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678:pci-0000:03:00.0-scsi-0:0:0:0
```
S
diskId" ELocal disk identifier. For example, `pci-0000:03:00.0-scsi-0:0:0:0`.
A

gatewayArn" /The Amazon Resource Name (ARN) of the gateway.
"S
diskId" ELocal disk identifier. For example, `pci-0000:03:00.0-scsi-0:0:0:0`.
"A

gatewayArn" /The Amazon Resource Name (ARN) of the gateway.
*Ío
[
storagegatewayCachesIscsiVolume6aws:storagegateway/cachesIscsiVolume:CachesIscsiVolume›RManages an AWS Storage Gateway cached iSCSI volume.

> **NOTE:** The gateway must have cache added (e.g., via the `aws.storagegateway.Cache` resource) before creating volumes otherwise the Storage Gateway API will return an error.

> **NOTE:** The gateway must have an upload buffer added (e.g., via the `aws.storagegateway.UploadBuffer` resource) before the volume is operational to clients, however the Storage Gateway API will allow volume creation without error in that case and return volume status as `UPLOAD BUFFER NOT CONFIGURED`.

## Example Usage

> **NOTE:** These examples are referencing the `aws.storagegateway.Cache` resource `gateway_arn` attribute to ensure this provider properly adds cache before creating the volume. If you are not using this method, you may need to declare an expicit dependency (e.g. via `depends_on = [aws_storagegateway_cache.example]`) to ensure proper ordering.

### Create Empty Cached iSCSI Volume

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.CachesIscsiVolume("example", {
    gatewayArn: exampleAwsStoragegatewayCache.gatewayArn,
    networkInterfaceId: exampleAwsInstance.privateIp,
    targetName: "example",
    volumeSizeInBytes: 5368709120,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.CachesIscsiVolume("example",
    gateway_arn=example_aws_storagegateway_cache["gatewayArn"],
    network_interface_id=example_aws_instance["privateIp"],
    target_name="example",
    volume_size_in_bytes=5368709120)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.CachesIscsiVolume("example", new()
    {
        GatewayArn = exampleAwsStoragegatewayCache.GatewayArn,
        NetworkInterfaceId = exampleAwsInstance.PrivateIp,
        TargetName = "example",
        VolumeSizeInBytes = 5368709120,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewCachesIscsiVolume(ctx, "example", &storagegateway.CachesIscsiVolumeArgs{
			GatewayArn:         pulumi.Any(exampleAwsStoragegatewayCache.GatewayArn),
			NetworkInterfaceId: pulumi.Any(exampleAwsInstance.PrivateIp),
			TargetName:         pulumi.String("example"),
			VolumeSizeInBytes:  pulumi.Int(5368709120),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.CachesIscsiVolume;
import com.pulumi.aws.storagegateway.CachesIscsiVolumeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new CachesIscsiVolume("example", CachesIscsiVolumeArgs.builder()
            .gatewayArn(exampleAwsStoragegatewayCache.gatewayArn())
            .networkInterfaceId(exampleAwsInstance.privateIp())
            .targetName("example")
            .volumeSizeInBytes(5368709120)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:CachesIscsiVolume
    properties:
      gatewayArn: ${exampleAwsStoragegatewayCache.gatewayArn}
      networkInterfaceId: ${exampleAwsInstance.privateIp}
      targetName: example
      volumeSizeInBytes: 5.36870912e+09 # 5 GB
```
<!--End PulumiCodeChooser -->

### Create Cached iSCSI Volume From Snapshot

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.CachesIscsiVolume("example", {
    gatewayArn: exampleAwsStoragegatewayCache.gatewayArn,
    networkInterfaceId: exampleAwsInstance.privateIp,
    snapshotId: exampleAwsEbsSnapshot.id,
    targetName: "example",
    volumeSizeInBytes: exampleAwsEbsSnapshot.volumeSize * 1024 * 1024 * 1024,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.CachesIscsiVolume("example",
    gateway_arn=example_aws_storagegateway_cache["gatewayArn"],
    network_interface_id=example_aws_instance["privateIp"],
    snapshot_id=example_aws_ebs_snapshot["id"],
    target_name="example",
    volume_size_in_bytes=example_aws_ebs_snapshot["volumeSize"] * 1024 * 1024 * 1024)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.CachesIscsiVolume("example", new()
    {
        GatewayArn = exampleAwsStoragegatewayCache.GatewayArn,
        NetworkInterfaceId = exampleAwsInstance.PrivateIp,
        SnapshotId = exampleAwsEbsSnapshot.Id,
        TargetName = "example",
        VolumeSizeInBytes = exampleAwsEbsSnapshot.VolumeSize * 1024 * 1024 * 1024,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewCachesIscsiVolume(ctx, "example", &storagegateway.CachesIscsiVolumeArgs{
			GatewayArn:         pulumi.Any(exampleAwsStoragegatewayCache.GatewayArn),
			NetworkInterfaceId: pulumi.Any(exampleAwsInstance.PrivateIp),
			SnapshotId:         pulumi.Any(exampleAwsEbsSnapshot.Id),
			TargetName:         pulumi.String("example"),
			VolumeSizeInBytes:  int(exampleAwsEbsSnapshot.VolumeSize * 1024 * 1024 * 1024),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.CachesIscsiVolume;
import com.pulumi.aws.storagegateway.CachesIscsiVolumeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new CachesIscsiVolume("example", CachesIscsiVolumeArgs.builder()
            .gatewayArn(exampleAwsStoragegatewayCache.gatewayArn())
            .networkInterfaceId(exampleAwsInstance.privateIp())
            .snapshotId(exampleAwsEbsSnapshot.id())
            .targetName("example")
            .volumeSizeInBytes(exampleAwsEbsSnapshot.volumeSize() * 1024 * 1024 * 1024)
            .build());

    }
}
```
<!--End PulumiCodeChooser -->

### Create Cached iSCSI Volume From Source Volume

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.CachesIscsiVolume("example", {
    gatewayArn: exampleAwsStoragegatewayCache.gatewayArn,
    networkInterfaceId: exampleAwsInstance.privateIp,
    sourceVolumeArn: existing.arn,
    targetName: "example",
    volumeSizeInBytes: existing.volumeSizeInBytes,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.CachesIscsiVolume("example",
    gateway_arn=example_aws_storagegateway_cache["gatewayArn"],
    network_interface_id=example_aws_instance["privateIp"],
    source_volume_arn=existing["arn"],
    target_name="example",
    volume_size_in_bytes=existing["volumeSizeInBytes"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.CachesIscsiVolume("example", new()
    {
        GatewayArn = exampleAwsStoragegatewayCache.GatewayArn,
        NetworkInterfaceId = exampleAwsInstance.PrivateIp,
        SourceVolumeArn = existing.Arn,
        TargetName = "example",
        VolumeSizeInBytes = existing.VolumeSizeInBytes,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewCachesIscsiVolume(ctx, "example", &storagegateway.CachesIscsiVolumeArgs{
			GatewayArn:         pulumi.Any(exampleAwsStoragegatewayCache.GatewayArn),
			NetworkInterfaceId: pulumi.Any(exampleAwsInstance.PrivateIp),
			SourceVolumeArn:    pulumi.Any(existing.Arn),
			TargetName:         pulumi.String("example"),
			VolumeSizeInBytes:  pulumi.Any(existing.VolumeSizeInBytes),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.CachesIscsiVolume;
import com.pulumi.aws.storagegateway.CachesIscsiVolumeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new CachesIscsiVolume("example", CachesIscsiVolumeArgs.builder()
            .gatewayArn(exampleAwsStoragegatewayCache.gatewayArn())
            .networkInterfaceId(exampleAwsInstance.privateIp())
            .sourceVolumeArn(existing.arn())
            .targetName("example")
            .volumeSizeInBytes(existing.volumeSizeInBytes())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:CachesIscsiVolume
    properties:
      gatewayArn: ${exampleAwsStoragegatewayCache.gatewayArn}
      networkInterfaceId: ${exampleAwsInstance.privateIp}
      sourceVolumeArn: ${existing.arn}
      targetName: example
      volumeSizeInBytes: ${existing.volumeSizeInBytes}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_storagegateway_cached_iscsi_volume` using the volume Amazon Resource Name (ARN). For example:

```sh
$ pulumi import aws:storagegateway/cachesIscsiVolume:CachesIscsiVolume example arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/volume/vol-12345678
```
A

gatewayArn" /The Amazon Resource Name (ARN) of the gateway.
ï
kmsEncryptedB
 Set to `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3.
ó
kmsKeyB" ÜThe Amazon Resource Name (ARN) of the AWS KMS key used for Amazon S3 server side encryption. Is required when `kms_encrypted` is set.
Ü
networkInterfaceId" lThe network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted.
n

snapshotIdB" ZThe snapshot ID of the snapshot to restore as the new cached volumeE.g., `snap-1122aabb`.
∞
sourceVolumeArnB" ñThe ARN for an existing volume. Specifying this ARN makes the new volume into an exact copy of the specified existing volume's latest recovery point. The `volume_size_in_bytes` value for this new volume must be equal to or larger than the size of the existing volume, in bytes.
»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
¿

targetName" ≠The name of the iSCSI target used by initiators to connect to the target and as a suffix for the target ARN. The target name must be unique across all volumes of a gateway.
:
volumeSizeInBytes !The size of the volume in bytes.
"è
arn" ÉVolume Amazon Resource Name (ARN), e.g., `arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/volume/vol-12345678`.
"H
chapEnabled
 5Whether mutual CHAP is enabled for the iSCSI target.
"A

gatewayArn" /The Amazon Resource Name (ARN) of the gateway.
"ï
kmsEncryptedB
 Set to `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3.
"ó
kmsKeyB" ÜThe Amazon Resource Name (ARN) of the AWS KMS key used for Amazon S3 server side encryption. Is required when `kms_encrypted` is set.
"&
	lunNumber Logical disk number.
"Ü
networkInterfaceId" lThe network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted.
"M
networkInterfacePort 1The port used to communicate with iSCSI targets.
"n

snapshotIdB" ZThe snapshot ID of the snapshot to restore as the new cached volumeE.g., `snap-1122aabb`.
"∞
sourceVolumeArnB" ñThe ARN for an existing volume. Specifying this ARN makes the new volume into an exact copy of the specified existing volume's latest recovery point. The `volume_size_in_bytes` value for this new volume must be equal to or larger than the size of the existing volume, in bytes.
"»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"™
	targetArn" òTarget Amazon Resource Name (ARN), e.g., `arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/target/iqn.1997-05.com.amazon:TargetName`.
"¿

targetName" ≠The name of the iSCSI target used by initiators to connect to the target and as a suffix for the target ARN. The target name must be unique across all volumes of a gateway.
"ï
	volumeArn" ÉVolume Amazon Resource Name (ARN), e.g., `arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/volume/vol-12345678`.
"1
volumeId" !Volume ID, e.g., `vol-12345678`.
":
volumeSizeInBytes !The size of the volume in bytes.
*©ú
g
storagegatewayFileSystemAssociation>aws:storagegateway/fileSystemAssociation:FileSystemAssociationÇäAssociate an Amazon FSx file system with the FSx File Gateway. After the association process is complete, the file shares on the Amazon FSx file system are available for access through the gateway. This operation only supports the FSx File Gateway type.

[FSx File Gateway requirements](https://docs.aws.amazon.com/filegateway/latest/filefsxw/Requirements.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.FileSystemAssociation("example", {
    gatewayArn: exampleAwsStoragegatewayGateway.arn,
    locationArn: exampleAwsFsxWindowsFileSystem.arn,
    username: "Admin",
    password: "avoid-plaintext-passwords",
    auditDestinationArn: exampleAwsS3Bucket.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.FileSystemAssociation("example",
    gateway_arn=example_aws_storagegateway_gateway["arn"],
    location_arn=example_aws_fsx_windows_file_system["arn"],
    username="Admin",
    password="avoid-plaintext-passwords",
    audit_destination_arn=example_aws_s3_bucket["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.FileSystemAssociation("example", new()
    {
        GatewayArn = exampleAwsStoragegatewayGateway.Arn,
        LocationArn = exampleAwsFsxWindowsFileSystem.Arn,
        Username = "Admin",
        Password = "avoid-plaintext-passwords",
        AuditDestinationArn = exampleAwsS3Bucket.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewFileSystemAssociation(ctx, "example", &storagegateway.FileSystemAssociationArgs{
			GatewayArn:          pulumi.Any(exampleAwsStoragegatewayGateway.Arn),
			LocationArn:         pulumi.Any(exampleAwsFsxWindowsFileSystem.Arn),
			Username:            pulumi.String("Admin"),
			Password:            pulumi.String("avoid-plaintext-passwords"),
			AuditDestinationArn: pulumi.Any(exampleAwsS3Bucket.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.FileSystemAssociation;
import com.pulumi.aws.storagegateway.FileSystemAssociationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new FileSystemAssociation("example", FileSystemAssociationArgs.builder()
            .gatewayArn(exampleAwsStoragegatewayGateway.arn())
            .locationArn(exampleAwsFsxWindowsFileSystem.arn())
            .username("Admin")
            .password("avoid-plaintext-passwords")
            .auditDestinationArn(exampleAwsS3Bucket.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:FileSystemAssociation
    properties:
      gatewayArn: ${exampleAwsStoragegatewayGateway.arn}
      locationArn: ${exampleAwsFsxWindowsFileSystem.arn}
      username: Admin
      password: avoid-plaintext-passwords
      auditDestinationArn: ${exampleAwsS3Bucket.arn}
```
<!--End PulumiCodeChooser -->

## Required Services Example

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const awsServiceStoragegatewayAmiFILES3Latest = aws.ssm.getParameter({
    name: "/aws/service/storagegateway/ami/FILE_S3/latest",
});
const test = new aws.ec2.Instance("test", {
    ami: awsServiceStoragegatewayAmiFILES3Latest.then(awsServiceStoragegatewayAmiFILES3Latest => awsServiceStoragegatewayAmiFILES3Latest.value),
    associatePublicIpAddress: true,
    instanceType: aws.ec2.InstanceType[available.instanceType],
    vpcSecurityGroupIds: [testAwsSecurityGroup.id],
    subnetId: testAwsSubnet[0].id,
}, {
    dependsOn: [
        testAwsRoute,
        testAwsVpcDhcpOptionsAssociation,
    ],
});
const testGateway = new aws.storagegateway.Gateway("test", {
    gatewayIpAddress: test.publicIp,
    gatewayName: "test-sgw",
    gatewayTimezone: "GMT",
    gatewayType: "FILE_FSX_SMB",
    smbActiveDirectorySettings: {
        domainName: testAwsDirectoryServiceDirectory.name,
        password: testAwsDirectoryServiceDirectory.password,
        username: "Admin",
    },
});
const testWindowsFileSystem = new aws.fsx.WindowsFileSystem("test", {
    activeDirectoryId: testAwsDirectoryServiceDirectory.id,
    securityGroupIds: [testAwsSecurityGroup.id],
    skipFinalBackup: true,
    storageCapacity: 32,
    subnetIds: [testAwsSubnet[0].id],
    throughputCapacity: 8,
});
const fsx = new aws.storagegateway.FileSystemAssociation("fsx", {
    gatewayArn: testGateway.arn,
    locationArn: testWindowsFileSystem.arn,
    username: "Admin",
    password: testAwsDirectoryServiceDirectory.password,
    cacheAttributes: {
        cacheStaleTimeoutInSeconds: 400,
    },
    auditDestinationArn: testAwsCloudwatchLogGroup.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

aws_service_storagegateway_ami_files3_latest = aws.ssm.get_parameter(name="/aws/service/storagegateway/ami/FILE_S3/latest")
test = aws.ec2.Instance("test",
    ami=aws_service_storagegateway_ami_files3_latest.value,
    associate_public_ip_address=True,
    instance_type=aws.ec2.InstanceType(available["instanceType"]),
    vpc_security_group_ids=[test_aws_security_group["id"]],
    subnet_id=test_aws_subnet[0]["id"],
    opts = pulumi.ResourceOptions(depends_on=[
            test_aws_route,
            test_aws_vpc_dhcp_options_association,
        ]))
test_gateway = aws.storagegateway.Gateway("test",
    gateway_ip_address=test.public_ip,
    gateway_name="test-sgw",
    gateway_timezone="GMT",
    gateway_type="FILE_FSX_SMB",
    smb_active_directory_settings={
        "domain_name": test_aws_directory_service_directory["name"],
        "password": test_aws_directory_service_directory["password"],
        "username": "Admin",
    })
test_windows_file_system = aws.fsx.WindowsFileSystem("test",
    active_directory_id=test_aws_directory_service_directory["id"],
    security_group_ids=[test_aws_security_group["id"]],
    skip_final_backup=True,
    storage_capacity=32,
    subnet_ids=[test_aws_subnet[0]["id"]],
    throughput_capacity=8)
fsx = aws.storagegateway.FileSystemAssociation("fsx",
    gateway_arn=test_gateway.arn,
    location_arn=test_windows_file_system.arn,
    username="Admin",
    password=test_aws_directory_service_directory["password"],
    cache_attributes={
        "cache_stale_timeout_in_seconds": 400,
    },
    audit_destination_arn=test_aws_cloudwatch_log_group["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var awsServiceStoragegatewayAmiFILES3Latest = Aws.Ssm.GetParameter.Invoke(new()
    {
        Name = "/aws/service/storagegateway/ami/FILE_S3/latest",
    });

    var test = new Aws.Ec2.Instance("test", new()
    {
        Ami = awsServiceStoragegatewayAmiFILES3Latest.Apply(getParameterResult => getParameterResult.Value),
        AssociatePublicIpAddress = true,
        InstanceType = System.Enum.Parse<Aws.Ec2.InstanceType>(available.InstanceType),
        VpcSecurityGroupIds = new[]
        {
            testAwsSecurityGroup.Id,
        },
        SubnetId = testAwsSubnet[0].Id,
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            testAwsRoute,
            testAwsVpcDhcpOptionsAssociation,
        },
    });

    var testGateway = new Aws.StorageGateway.Gateway("test", new()
    {
        GatewayIpAddress = test.PublicIp,
        GatewayName = "test-sgw",
        GatewayTimezone = "GMT",
        GatewayType = "FILE_FSX_SMB",
        SmbActiveDirectorySettings = new Aws.StorageGateway.Inputs.GatewaySmbActiveDirectorySettingsArgs
        {
            DomainName = testAwsDirectoryServiceDirectory.Name,
            Password = testAwsDirectoryServiceDirectory.Password,
            Username = "Admin",
        },
    });

    var testWindowsFileSystem = new Aws.Fsx.WindowsFileSystem("test", new()
    {
        ActiveDirectoryId = testAwsDirectoryServiceDirectory.Id,
        SecurityGroupIds = new[]
        {
            testAwsSecurityGroup.Id,
        },
        SkipFinalBackup = true,
        StorageCapacity = 32,
        SubnetIds = new[]
        {
            testAwsSubnet[0].Id,
        },
        ThroughputCapacity = 8,
    });

    var fsx = new Aws.StorageGateway.FileSystemAssociation("fsx", new()
    {
        GatewayArn = testGateway.Arn,
        LocationArn = testWindowsFileSystem.Arn,
        Username = "Admin",
        Password = testAwsDirectoryServiceDirectory.Password,
        CacheAttributes = new Aws.StorageGateway.Inputs.FileSystemAssociationCacheAttributesArgs
        {
            CacheStaleTimeoutInSeconds = 400,
        },
        AuditDestinationArn = testAwsCloudwatchLogGroup.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/fsx"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssm"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		awsServiceStoragegatewayAmiFILES3Latest, err := ssm.LookupParameter(ctx, &ssm.LookupParameterArgs{
			Name: "/aws/service/storagegateway/ami/FILE_S3/latest",
		}, nil)
		if err != nil {
			return err
		}
		test, err := ec2.NewInstance(ctx, "test", &ec2.InstanceArgs{
			Ami:                      pulumi.String(awsServiceStoragegatewayAmiFILES3Latest.Value),
			AssociatePublicIpAddress: pulumi.Bool(true),
			InstanceType:             ec2.InstanceType(available.InstanceType),
			VpcSecurityGroupIds: pulumi.StringArray{
				testAwsSecurityGroup.Id,
			},
			SubnetId: pulumi.Any(testAwsSubnet[0].Id),
		}, pulumi.DependsOn([]pulumi.Resource{
			testAwsRoute,
			testAwsVpcDhcpOptionsAssociation,
		}))
		if err != nil {
			return err
		}
		testGateway, err := storagegateway.NewGateway(ctx, "test", &storagegateway.GatewayArgs{
			GatewayIpAddress: test.PublicIp,
			GatewayName:      pulumi.String("test-sgw"),
			GatewayTimezone:  pulumi.String("GMT"),
			GatewayType:      pulumi.String("FILE_FSX_SMB"),
			SmbActiveDirectorySettings: &storagegateway.GatewaySmbActiveDirectorySettingsArgs{
				DomainName: pulumi.Any(testAwsDirectoryServiceDirectory.Name),
				Password:   pulumi.Any(testAwsDirectoryServiceDirectory.Password),
				Username:   pulumi.String("Admin"),
			},
		})
		if err != nil {
			return err
		}
		testWindowsFileSystem, err := fsx.NewWindowsFileSystem(ctx, "test", &fsx.WindowsFileSystemArgs{
			ActiveDirectoryId: pulumi.Any(testAwsDirectoryServiceDirectory.Id),
			SecurityGroupIds: pulumi.StringArray{
				testAwsSecurityGroup.Id,
			},
			SkipFinalBackup: pulumi.Bool(true),
			StorageCapacity: pulumi.Int(32),
			SubnetIds: pulumi.StringArray{
				testAwsSubnet[0].Id,
			},
			ThroughputCapacity: pulumi.Int(8),
		})
		if err != nil {
			return err
		}
		_, err = storagegateway.NewFileSystemAssociation(ctx, "fsx", &storagegateway.FileSystemAssociationArgs{
			GatewayArn:  testGateway.Arn,
			LocationArn: testWindowsFileSystem.Arn,
			Username:    pulumi.String("Admin"),
			Password:    pulumi.Any(testAwsDirectoryServiceDirectory.Password),
			CacheAttributes: &storagegateway.FileSystemAssociationCacheAttributesArgs{
				CacheStaleTimeoutInSeconds: pulumi.Int(400),
			},
			AuditDestinationArn: pulumi.Any(testAwsCloudwatchLogGroup.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssm.SsmFunctions;
import com.pulumi.aws.ssm.inputs.GetParameterArgs;
import com.pulumi.aws.ec2.Instance;
import com.pulumi.aws.ec2.InstanceArgs;
import com.pulumi.aws.storagegateway.Gateway;
import com.pulumi.aws.storagegateway.GatewayArgs;
import com.pulumi.aws.storagegateway.inputs.GatewaySmbActiveDirectorySettingsArgs;
import com.pulumi.aws.fsx.WindowsFileSystem;
import com.pulumi.aws.fsx.WindowsFileSystemArgs;
import com.pulumi.aws.storagegateway.FileSystemAssociation;
import com.pulumi.aws.storagegateway.FileSystemAssociationArgs;
import com.pulumi.aws.storagegateway.inputs.FileSystemAssociationCacheAttributesArgs;
import com.pulumi.resources.CustomResourceOptions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var awsServiceStoragegatewayAmiFILES3Latest = SsmFunctions.getParameter(GetParameterArgs.builder()
            .name("/aws/service/storagegateway/ami/FILE_S3/latest")
            .build());

        var test = new Instance("test", InstanceArgs.builder()
            .ami(awsServiceStoragegatewayAmiFILES3Latest.applyValue(getParameterResult -> getParameterResult.value()))
            .associatePublicIpAddress(true)
            .instanceType(available.instanceType())
            .vpcSecurityGroupIds(testAwsSecurityGroup.id())
            .subnetId(testAwsSubnet[0].id())
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    testAwsRoute,
                    testAwsVpcDhcpOptionsAssociation)
                .build());

        var testGateway = new Gateway("testGateway", GatewayArgs.builder()
            .gatewayIpAddress(test.publicIp())
            .gatewayName("test-sgw")
            .gatewayTimezone("GMT")
            .gatewayType("FILE_FSX_SMB")
            .smbActiveDirectorySettings(GatewaySmbActiveDirectorySettingsArgs.builder()
                .domainName(testAwsDirectoryServiceDirectory.name())
                .password(testAwsDirectoryServiceDirectory.password())
                .username("Admin")
                .build())
            .build());

        var testWindowsFileSystem = new WindowsFileSystem("testWindowsFileSystem", WindowsFileSystemArgs.builder()
            .activeDirectoryId(testAwsDirectoryServiceDirectory.id())
            .securityGroupIds(testAwsSecurityGroup.id())
            .skipFinalBackup(true)
            .storageCapacity(32)
            .subnetIds(testAwsSubnet[0].id())
            .throughputCapacity(8)
            .build());

        var fsx = new FileSystemAssociation("fsx", FileSystemAssociationArgs.builder()
            .gatewayArn(testGateway.arn())
            .locationArn(testWindowsFileSystem.arn())
            .username("Admin")
            .password(testAwsDirectoryServiceDirectory.password())
            .cacheAttributes(FileSystemAssociationCacheAttributesArgs.builder()
                .cacheStaleTimeoutInSeconds(400)
                .build())
            .auditDestinationArn(testAwsCloudwatchLogGroup.arn())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:ec2:Instance
    properties:
      ami: ${awsServiceStoragegatewayAmiFILES3Latest.value}
      associatePublicIpAddress: true
      instanceType: ${available.instanceType}
      vpcSecurityGroupIds:
        - ${testAwsSecurityGroup.id}
      subnetId: ${testAwsSubnet[0].id}
    options:
      dependsOn:
        - ${testAwsRoute}
        - ${testAwsVpcDhcpOptionsAssociation}
  testGateway:
    type: aws:storagegateway:Gateway
    name: test
    properties:
      gatewayIpAddress: ${test.publicIp}
      gatewayName: test-sgw
      gatewayTimezone: GMT
      gatewayType: FILE_FSX_SMB
      smbActiveDirectorySettings:
        domainName: ${testAwsDirectoryServiceDirectory.name}
        password: ${testAwsDirectoryServiceDirectory.password}
        username: Admin
  testWindowsFileSystem:
    type: aws:fsx:WindowsFileSystem
    name: test
    properties:
      activeDirectoryId: ${testAwsDirectoryServiceDirectory.id}
      securityGroupIds:
        - ${testAwsSecurityGroup.id}
      skipFinalBackup: true
      storageCapacity: 32
      subnetIds:
        - ${testAwsSubnet[0].id}
      throughputCapacity: 8
  fsx:
    type: aws:storagegateway:FileSystemAssociation
    properties:
      gatewayArn: ${testGateway.arn}
      locationArn: ${testWindowsFileSystem.arn}
      username: Admin
      password: ${testAwsDirectoryServiceDirectory.password}
      cacheAttributes:
        cacheStaleTimeoutInSeconds: 400
      auditDestinationArn: ${testAwsCloudwatchLogGroup.arn}
variables:
  awsServiceStoragegatewayAmiFILES3Latest:
    fn::invoke:
      function: aws:ssm:getParameter
      arguments:
        name: /aws/service/storagegateway/ami/FILE_S3/latest
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_storagegateway_file_system_association` using the FSx file system association Amazon Resource Name (ARN). For example:

```sh
$ pulumi import aws:storagegateway/fileSystemAssociation:FileSystemAssociation example arn:aws:storagegateway:us-east-1:123456789012:fs-association/fsa-0DA347732FDB40125
```
d
auditDestinationArnB" GThe Amazon Resource Name (ARN) of the storage used for the audit logs.
ı
cacheAttributesùBö:ó
î
storagegateway$FileSystemAssociationCacheAttributes\aws:storagegateway/FileSystemAssociationCacheAttributes:FileSystemAssociationCacheAttributesBRefresh cache information. see Cache Attributes for more details.
A

gatewayArn" /The Amazon Resource Name (ARN) of the gateway.
x
locationArn" eThe Amazon Resource Name (ARN) of the Amazon FSx file system to associate with the FSx File Gateway.
5
password" %The password of the user credential.
«
tagsB2" ∂Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
Õ
username" ºThe user name of the user credential that has permission to access the root share of the Amazon FSx file system. The user account must belong to the Amazon FSx delegated admin user group.
"T
arn" IAmazon Resource Name (ARN) of the newly created file system association.
"d
auditDestinationArnB" GThe Amazon Resource Name (ARN) of the storage used for the audit logs.
"ı
cacheAttributesùBö:ó
î
storagegateway$FileSystemAssociationCacheAttributes\aws:storagegateway/FileSystemAssociationCacheAttributes:FileSystemAssociationCacheAttributesBRefresh cache information. see Cache Attributes for more details.
"A

gatewayArn" /The Amazon Resource Name (ARN) of the gateway.
"x
locationArn" eThe Amazon Resource Name (ARN) of the Amazon FSx file system to associate with the FSx File Gateway.
"5
password" %The password of the user credential.
"«
tagsB2" ∂Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"Õ
username" ºThe user name of the user credential that has permission to access the root share of the Amazon FSx file system. The user account must belong to the Amazon FSx delegated admin user group.
*®Â
=
storagegatewayGateway"aws:storagegateway/gateway:Gateway”úManages an AWS Storage Gateway file, tape, or volume gateway in the provider region.

> **NOTE:** The Storage Gateway API requires the gateway to be connected to properly return information after activation. If you are receiving `The specified gateway is not connected` errors during resource creation (gateway activation), ensure your gateway instance meets the [Storage Gateway requirements](https://docs.aws.amazon.com/storagegateway/latest/userguide/Requirements.html).

## Example Usage

### Local Cache

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const testVolumeAttachment = new aws.ec2.VolumeAttachment("test", {
    deviceName: "/dev/xvdb",
    volumeId: testAwsEbsVolume.id,
    instanceId: testAwsInstance.id,
});
const test = aws.storagegateway.getLocalDisk({
    diskNode: testAwsVolumeAttachment.deviceName,
    gatewayArn: testAwsStoragegatewayGateway.arn,
});
const testCache = new aws.storagegateway.Cache("test", {
    diskId: test.then(test => test.diskId),
    gatewayArn: testAwsStoragegatewayGateway.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

test_volume_attachment = aws.ec2.VolumeAttachment("test",
    device_name="/dev/xvdb",
    volume_id=test_aws_ebs_volume["id"],
    instance_id=test_aws_instance["id"])
test = aws.storagegateway.get_local_disk(disk_node=test_aws_volume_attachment["deviceName"],
    gateway_arn=test_aws_storagegateway_gateway["arn"])
test_cache = aws.storagegateway.Cache("test",
    disk_id=test.disk_id,
    gateway_arn=test_aws_storagegateway_gateway["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var testVolumeAttachment = new Aws.Ec2.VolumeAttachment("test", new()
    {
        DeviceName = "/dev/xvdb",
        VolumeId = testAwsEbsVolume.Id,
        InstanceId = testAwsInstance.Id,
    });

    var test = Aws.StorageGateway.GetLocalDisk.Invoke(new()
    {
        DiskNode = testAwsVolumeAttachment.DeviceName,
        GatewayArn = testAwsStoragegatewayGateway.Arn,
    });

    var testCache = new Aws.StorageGateway.Cache("test", new()
    {
        DiskId = test.Apply(getLocalDiskResult => getLocalDiskResult.DiskId),
        GatewayArn = testAwsStoragegatewayGateway.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ec2.NewVolumeAttachment(ctx, "test", &ec2.VolumeAttachmentArgs{
			DeviceName: pulumi.String("/dev/xvdb"),
			VolumeId:   pulumi.Any(testAwsEbsVolume.Id),
			InstanceId: pulumi.Any(testAwsInstance.Id),
		})
		if err != nil {
			return err
		}
		test, err := storagegateway.GetLocalDisk(ctx, &storagegateway.GetLocalDiskArgs{
			DiskNode:   pulumi.StringRef(testAwsVolumeAttachment.DeviceName),
			GatewayArn: testAwsStoragegatewayGateway.Arn,
		}, nil)
		if err != nil {
			return err
		}
		_, err = storagegateway.NewCache(ctx, "test", &storagegateway.CacheArgs{
			DiskId:     pulumi.String(test.DiskId),
			GatewayArn: pulumi.Any(testAwsStoragegatewayGateway.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ec2.VolumeAttachment;
import com.pulumi.aws.ec2.VolumeAttachmentArgs;
import com.pulumi.aws.storagegateway.StoragegatewayFunctions;
import com.pulumi.aws.storagegateway.inputs.GetLocalDiskArgs;
import com.pulumi.aws.storagegateway.Cache;
import com.pulumi.aws.storagegateway.CacheArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var testVolumeAttachment = new VolumeAttachment("testVolumeAttachment", VolumeAttachmentArgs.builder()
            .deviceName("/dev/xvdb")
            .volumeId(testAwsEbsVolume.id())
            .instanceId(testAwsInstance.id())
            .build());

        final var test = StoragegatewayFunctions.getLocalDisk(GetLocalDiskArgs.builder()
            .diskNode(testAwsVolumeAttachment.deviceName())
            .gatewayArn(testAwsStoragegatewayGateway.arn())
            .build());

        var testCache = new Cache("testCache", CacheArgs.builder()
            .diskId(test.applyValue(getLocalDiskResult -> getLocalDiskResult.diskId()))
            .gatewayArn(testAwsStoragegatewayGateway.arn())
            .build());

    }
}
```
```yaml
resources:
  testVolumeAttachment:
    type: aws:ec2:VolumeAttachment
    name: test
    properties:
      deviceName: /dev/xvdb
      volumeId: ${testAwsEbsVolume.id}
      instanceId: ${testAwsInstance.id}
  testCache:
    type: aws:storagegateway:Cache
    name: test
    properties:
      diskId: ${test.diskId}
      gatewayArn: ${testAwsStoragegatewayGateway.arn}
variables:
  test:
    fn::invoke:
      function: aws:storagegateway:getLocalDisk
      arguments:
        diskNode: ${testAwsVolumeAttachment.deviceName}
        gatewayArn: ${testAwsStoragegatewayGateway.arn}
```
<!--End PulumiCodeChooser -->

### FSx File Gateway

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.Gateway("example", {
    gatewayIpAddress: "1.2.3.4",
    gatewayName: "example",
    gatewayTimezone: "GMT",
    gatewayType: "FILE_FSX_SMB",
    smbActiveDirectorySettings: {
        domainName: "corp.example.com",
        password: "avoid-plaintext-passwords",
        username: "Admin",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.Gateway("example",
    gateway_ip_address="1.2.3.4",
    gateway_name="example",
    gateway_timezone="GMT",
    gateway_type="FILE_FSX_SMB",
    smb_active_directory_settings={
        "domain_name": "corp.example.com",
        "password": "avoid-plaintext-passwords",
        "username": "Admin",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.Gateway("example", new()
    {
        GatewayIpAddress = "1.2.3.4",
        GatewayName = "example",
        GatewayTimezone = "GMT",
        GatewayType = "FILE_FSX_SMB",
        SmbActiveDirectorySettings = new Aws.StorageGateway.Inputs.GatewaySmbActiveDirectorySettingsArgs
        {
            DomainName = "corp.example.com",
            Password = "avoid-plaintext-passwords",
            Username = "Admin",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewGateway(ctx, "example", &storagegateway.GatewayArgs{
			GatewayIpAddress: pulumi.String("1.2.3.4"),
			GatewayName:      pulumi.String("example"),
			GatewayTimezone:  pulumi.String("GMT"),
			GatewayType:      pulumi.String("FILE_FSX_SMB"),
			SmbActiveDirectorySettings: &storagegateway.GatewaySmbActiveDirectorySettingsArgs{
				DomainName: pulumi.String("corp.example.com"),
				Password:   pulumi.String("avoid-plaintext-passwords"),
				Username:   pulumi.String("Admin"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.Gateway;
import com.pulumi.aws.storagegateway.GatewayArgs;
import com.pulumi.aws.storagegateway.inputs.GatewaySmbActiveDirectorySettingsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Gateway("example", GatewayArgs.builder()
            .gatewayIpAddress("1.2.3.4")
            .gatewayName("example")
            .gatewayTimezone("GMT")
            .gatewayType("FILE_FSX_SMB")
            .smbActiveDirectorySettings(GatewaySmbActiveDirectorySettingsArgs.builder()
                .domainName("corp.example.com")
                .password("avoid-plaintext-passwords")
                .username("Admin")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:Gateway
    properties:
      gatewayIpAddress: 1.2.3.4
      gatewayName: example
      gatewayTimezone: GMT
      gatewayType: FILE_FSX_SMB
      smbActiveDirectorySettings:
        domainName: corp.example.com
        password: avoid-plaintext-passwords
        username: Admin
```
<!--End PulumiCodeChooser -->

### S3 File Gateway

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.Gateway("example", {
    gatewayIpAddress: "1.2.3.4",
    gatewayName: "example",
    gatewayTimezone: "GMT",
    gatewayType: "FILE_S3",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.Gateway("example",
    gateway_ip_address="1.2.3.4",
    gateway_name="example",
    gateway_timezone="GMT",
    gateway_type="FILE_S3")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.Gateway("example", new()
    {
        GatewayIpAddress = "1.2.3.4",
        GatewayName = "example",
        GatewayTimezone = "GMT",
        GatewayType = "FILE_S3",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewGateway(ctx, "example", &storagegateway.GatewayArgs{
			GatewayIpAddress: pulumi.String("1.2.3.4"),
			GatewayName:      pulumi.String("example"),
			GatewayTimezone:  pulumi.String("GMT"),
			GatewayType:      pulumi.String("FILE_S3"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.Gateway;
import com.pulumi.aws.storagegateway.GatewayArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Gateway("example", GatewayArgs.builder()
            .gatewayIpAddress("1.2.3.4")
            .gatewayName("example")
            .gatewayTimezone("GMT")
            .gatewayType("FILE_S3")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:Gateway
    properties:
      gatewayIpAddress: 1.2.3.4
      gatewayName: example
      gatewayTimezone: GMT
      gatewayType: FILE_S3
```
<!--End PulumiCodeChooser -->

### Tape Gateway

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.Gateway("example", {
    gatewayIpAddress: "1.2.3.4",
    gatewayName: "example",
    gatewayTimezone: "GMT",
    gatewayType: "VTL",
    mediumChangerType: "AWS-Gateway-VTL",
    tapeDriveType: "IBM-ULT3580-TD5",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.Gateway("example",
    gateway_ip_address="1.2.3.4",
    gateway_name="example",
    gateway_timezone="GMT",
    gateway_type="VTL",
    medium_changer_type="AWS-Gateway-VTL",
    tape_drive_type="IBM-ULT3580-TD5")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.Gateway("example", new()
    {
        GatewayIpAddress = "1.2.3.4",
        GatewayName = "example",
        GatewayTimezone = "GMT",
        GatewayType = "VTL",
        MediumChangerType = "AWS-Gateway-VTL",
        TapeDriveType = "IBM-ULT3580-TD5",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewGateway(ctx, "example", &storagegateway.GatewayArgs{
			GatewayIpAddress:  pulumi.String("1.2.3.4"),
			GatewayName:       pulumi.String("example"),
			GatewayTimezone:   pulumi.String("GMT"),
			GatewayType:       pulumi.String("VTL"),
			MediumChangerType: pulumi.String("AWS-Gateway-VTL"),
			TapeDriveType:     pulumi.String("IBM-ULT3580-TD5"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.Gateway;
import com.pulumi.aws.storagegateway.GatewayArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Gateway("example", GatewayArgs.builder()
            .gatewayIpAddress("1.2.3.4")
            .gatewayName("example")
            .gatewayTimezone("GMT")
            .gatewayType("VTL")
            .mediumChangerType("AWS-Gateway-VTL")
            .tapeDriveType("IBM-ULT3580-TD5")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:Gateway
    properties:
      gatewayIpAddress: 1.2.3.4
      gatewayName: example
      gatewayTimezone: GMT
      gatewayType: VTL
      mediumChangerType: AWS-Gateway-VTL
      tapeDriveType: IBM-ULT3580-TD5
```
<!--End PulumiCodeChooser -->

### Volume Gateway (Cached)

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.Gateway("example", {
    gatewayIpAddress: "1.2.3.4",
    gatewayName: "example",
    gatewayTimezone: "GMT",
    gatewayType: "CACHED",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.Gateway("example",
    gateway_ip_address="1.2.3.4",
    gateway_name="example",
    gateway_timezone="GMT",
    gateway_type="CACHED")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.Gateway("example", new()
    {
        GatewayIpAddress = "1.2.3.4",
        GatewayName = "example",
        GatewayTimezone = "GMT",
        GatewayType = "CACHED",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewGateway(ctx, "example", &storagegateway.GatewayArgs{
			GatewayIpAddress: pulumi.String("1.2.3.4"),
			GatewayName:      pulumi.String("example"),
			GatewayTimezone:  pulumi.String("GMT"),
			GatewayType:      pulumi.String("CACHED"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.Gateway;
import com.pulumi.aws.storagegateway.GatewayArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Gateway("example", GatewayArgs.builder()
            .gatewayIpAddress("1.2.3.4")
            .gatewayName("example")
            .gatewayTimezone("GMT")
            .gatewayType("CACHED")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:Gateway
    properties:
      gatewayIpAddress: 1.2.3.4
      gatewayName: example
      gatewayTimezone: GMT
      gatewayType: CACHED
```
<!--End PulumiCodeChooser -->

### Volume Gateway (Stored)

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.Gateway("example", {
    gatewayIpAddress: "1.2.3.4",
    gatewayName: "example",
    gatewayTimezone: "GMT",
    gatewayType: "STORED",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.Gateway("example",
    gateway_ip_address="1.2.3.4",
    gateway_name="example",
    gateway_timezone="GMT",
    gateway_type="STORED")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.Gateway("example", new()
    {
        GatewayIpAddress = "1.2.3.4",
        GatewayName = "example",
        GatewayTimezone = "GMT",
        GatewayType = "STORED",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewGateway(ctx, "example", &storagegateway.GatewayArgs{
			GatewayIpAddress: pulumi.String("1.2.3.4"),
			GatewayName:      pulumi.String("example"),
			GatewayTimezone:  pulumi.String("GMT"),
			GatewayType:      pulumi.String("STORED"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.Gateway;
import com.pulumi.aws.storagegateway.GatewayArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Gateway("example", GatewayArgs.builder()
            .gatewayIpAddress("1.2.3.4")
            .gatewayName("example")
            .gatewayTimezone("GMT")
            .gatewayType("STORED")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:Gateway
    properties:
      gatewayIpAddress: 1.2.3.4
      gatewayName: example
      gatewayTimezone: GMT
      gatewayType: STORED
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_storagegateway_gateway` using the gateway Amazon Resource Name (ARN). For example:

```sh
$ pulumi import aws:storagegateway/gateway:Gateway example arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678
```
Certain resource arguments, like `gateway_ip_address` do not have a Storage Gateway API method for reading the information after creation, either omit the argument from the Pulumi program or use `ignore_changes` to hide the difference. For example:

å
activationKeyB" ÙGateway activation key during resource creation. Conflicts with `gateway_ip_address`. Additional information is available in the [Storage Gateway User Guide](https://docs.aws.amazon.com/storagegateway/latest/userguide/get-activation-key.html).
¥
$averageDownloadRateLimitInBitsPerSecB ÖThe average download bandwidth rate limit in bits per second. This is supported for the `CACHED`, `STORED`, and `VTL` gateway types.
∞
"averageUploadRateLimitInBitsPerSecB ÉThe average upload bandwidth rate limit in bits per second. This is supported for the `CACHED`, `STORED`, and `VTL` gateway types.
í
cloudwatchLogGroupArnB" sThe Amazon Resource Name (ARN) of the Amazon CloudWatch log group to use to monitor and log events in the gateway.
Ì
gatewayIpAddressB" “Gateway IP address to retrieve activation key during resource creation. Conflicts with `activation_key`. Gateway must be accessible on port 80 from where this provider is running. Additional information is available in the [Storage Gateway User Guide](https://docs.aws.amazon.com/storagegateway/latest/userguide/get-activation-key.html).
(
gatewayName" Name of the gateway.
û
gatewayTimezone" ÜTime zone for the gateway. The time zone is of the format "GMT", "GMT-hr:mm", or "GMT+hr:mm". For example, `GMT-4:00` indicates the time is 4 hours behind GMT. The time zone is used, for example, for scheduling snapshots and your gateway's maintenance schedule.
ç
gatewayTypeB" xType of the gateway. The default value is `STORED`. Valid values: `CACHED`, `FILE_FSX_SMB`, `FILE_S3`, `STORED`, `VTL`.
±
gatewayVpcEndpointB" îVPC endpoint address to be used when activating your gateway. This should be used when your instance is in a private subnet. Requires HTTP access from client computer running this provider. More info on what ports are required by your VPC Endpoint Security group in [Activating a Gateway in a Virtual Private Cloud](https://docs.aws.amazon.com/storagegateway/latest/userguide/gateway-private-link.html).
…
maintenanceStartTimeB}:{
y
storagegatewayGatewayMaintenanceStartTimeJaws:storagegateway/GatewayMaintenanceStartTime:GatewayMaintenanceStartTimeØThe gateway's weekly maintenance start time information, including day and time of the week. The maintenance time is the time in your gateway's time zone. More details below.
¬
mediumChangerTypeB" ¶Type of medium changer to use for tape gateway. This provider cannot detect drift of this argument. Valid values: `STK-L700`, `AWS-Gateway-VTL`, `IBM-03584L32-0402`.
Ω
smbActiveDirectorySettingsîBë:é
ã
storagegateway!GatewaySmbActiveDirectorySettingsVaws:storagegateway/GatewaySmbActiveDirectorySettings:GatewaySmbActiveDirectorySettingsáNested argument with Active Directory domain join information for Server Message Block (SMB) file shares. Only valid for `FILE_S3` and `FILE_FSX_SMB` gateway types. Must be set before creating `ActiveDirectory` authentication SMB file shares. More details below.
i
smbFileShareVisibilityB
 ISpecifies whether the shares on this gateway appear when listing shares.
ñ
smbGuestPasswordB" ˚Guest password for Server Message Block (SMB) file shares. Only valid for `FILE_S3` and `FILE_FSX_SMB` gateway types. Must be set before creating `GuestAccess` authentication SMB file shares. This provider can only detect drift of the existence of a guest password, not its actual value from the gateway. This provider can however update the password with changing the argument.
À
smbSecurityStrategyB" ≠Specifies the type of security strategy. Valid values are: `ClientSpecified`, `MandatorySigning`, and `MandatoryEncryption`. See [Setting a Security Level for Your Gateway](https://docs.aws.amazon.com/storagegateway/latest/userguide/managing-gateway-file.html#security-strategy) for more information.
«
tagsB2" ∂Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
ô
tapeDriveTypeB" ÅType of tape drive to use for tape gateway. This provider cannot detect drift of this argument. Valid values: `IBM-ULT3580-TD5`.
"ä
activationKey" ÙGateway activation key during resource creation. Conflicts with `gateway_ip_address`. Additional information is available in the [Storage Gateway User Guide](https://docs.aws.amazon.com/storagegateway/latest/userguide/get-activation-key.html).
"6
arn" +Amazon Resource Name (ARN) of the gateway.
"¥
$averageDownloadRateLimitInBitsPerSecB ÖThe average download bandwidth rate limit in bits per second. This is supported for the `CACHED`, `STORED`, and `VTL` gateway types.
"∞
"averageUploadRateLimitInBitsPerSecB ÉThe average upload bandwidth rate limit in bits per second. This is supported for the `CACHED`, `STORED`, and `VTL` gateway types.
"í
cloudwatchLogGroupArnB" sThe Amazon Resource Name (ARN) of the Amazon CloudWatch log group to use to monitor and log events in the gateway.
"\
ec2InstanceId" GThe ID of the Amazon EC2 instance that was used to launch the gateway.
";
endpointType" 'The type of endpoint for your gateway.
",
	gatewayId" Identifier of the gateway.
"Î
gatewayIpAddress" “Gateway IP address to retrieve activation key during resource creation. Conflicts with `activation_key`. Gateway must be accessible on port 80 from where this provider is running. Additional information is available in the [Storage Gateway User Guide](https://docs.aws.amazon.com/storagegateway/latest/userguide/get-activation-key.html).
"(
gatewayName" Name of the gateway.
"ê
gatewayNetworkInterfacesã*à:Ö
Ç
storagegatewayGatewayGatewayNetworkInterfacePaws:storagegateway/GatewayGatewayNetworkInterface:GatewayGatewayNetworkInterfacefAn array that contains descriptions of the gateway network interfaces. See Gateway Network Interface.
"û
gatewayTimezone" ÜTime zone for the gateway. The time zone is of the format "GMT", "GMT-hr:mm", or "GMT+hr:mm". For example, `GMT-4:00` indicates the time is 4 hours behind GMT. The time zone is used, for example, for scheduling snapshots and your gateway's maintenance schedule.
"ç
gatewayTypeB" xType of the gateway. The default value is `STORED`. Valid values: `CACHED`, `FILE_FSX_SMB`, `FILE_S3`, `STORED`, `VTL`.
"±
gatewayVpcEndpointB" îVPC endpoint address to be used when activating your gateway. This should be used when your instance is in a private subnet. Requires HTTP access from client computer running this provider. More info on what ports are required by your VPC Endpoint Security group in [Activating a Gateway in a Virtual Private Cloud](https://docs.aws.amazon.com/storagegateway/latest/userguide/gateway-private-link.html).
"L
hostEnvironment" 5The type of hypervisor environment used by the host.
"«
maintenanceStartTime}:{
y
storagegatewayGatewayMaintenanceStartTimeJaws:storagegateway/GatewayMaintenanceStartTime:GatewayMaintenanceStartTimeØThe gateway's weekly maintenance start time information, including day and time of the week. The maintenance time is the time in your gateway's time zone. More details below.
"¬
mediumChangerTypeB" ¶Type of medium changer to use for tape gateway. This provider cannot detect drift of this argument. Valid values: `STK-L700`, `AWS-Gateway-VTL`, `IBM-03584L32-0402`.
"Ω
smbActiveDirectorySettingsîBë:é
ã
storagegateway!GatewaySmbActiveDirectorySettingsVaws:storagegateway/GatewaySmbActiveDirectorySettings:GatewaySmbActiveDirectorySettingsáNested argument with Active Directory domain join information for Server Message Block (SMB) file shares. Only valid for `FILE_S3` and `FILE_FSX_SMB` gateway types. Must be set before creating `ActiveDirectory` authentication SMB file shares. More details below.
"i
smbFileShareVisibilityB
 ISpecifies whether the shares on this gateway appear when listing shares.
"ñ
smbGuestPasswordB" ˚Guest password for Server Message Block (SMB) file shares. Only valid for `FILE_S3` and `FILE_FSX_SMB` gateway types. Must be set before creating `GuestAccess` authentication SMB file shares. This provider can only detect drift of the existence of a guest password, not its actual value from the gateway. This provider can however update the password with changing the argument.
"…
smbSecurityStrategy" ≠Specifies the type of security strategy. Valid values are: `ClientSpecified`, `MandatorySigning`, and `MandatoryEncryption`. See [Setting a Security Level for Your Gateway](https://docs.aws.amazon.com/storagegateway/latest/userguide/managing-gateway-file.html#security-strategy) for more information.
"«
tagsB2" ∂Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"ô
tapeDriveTypeB" ÅType of tape drive to use for tape gateway. This provider cannot detect drift of this argument. Valid values: `IBM-ULT3580-TD5`.
*≠R
L
storagegatewayNfsFileShare,aws:storagegateway/nfsFileShare:NfsFileShare¸Manages an AWS Storage Gateway NFS File Share.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.NfsFileShare("example", {
    clientLists: ["0.0.0.0/0"],
    gatewayArn: exampleAwsStoragegatewayGateway.arn,
    locationArn: exampleAwsS3Bucket.arn,
    roleArn: exampleAwsIamRole.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.NfsFileShare("example",
    client_lists=["0.0.0.0/0"],
    gateway_arn=example_aws_storagegateway_gateway["arn"],
    location_arn=example_aws_s3_bucket["arn"],
    role_arn=example_aws_iam_role["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.NfsFileShare("example", new()
    {
        ClientLists = new[]
        {
            "0.0.0.0/0",
        },
        GatewayArn = exampleAwsStoragegatewayGateway.Arn,
        LocationArn = exampleAwsS3Bucket.Arn,
        RoleArn = exampleAwsIamRole.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewNfsFileShare(ctx, "example", &storagegateway.NfsFileShareArgs{
			ClientLists: pulumi.StringArray{
				pulumi.String("0.0.0.0/0"),
			},
			GatewayArn:  pulumi.Any(exampleAwsStoragegatewayGateway.Arn),
			LocationArn: pulumi.Any(exampleAwsS3Bucket.Arn),
			RoleArn:     pulumi.Any(exampleAwsIamRole.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.NfsFileShare;
import com.pulumi.aws.storagegateway.NfsFileShareArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new NfsFileShare("example", NfsFileShareArgs.builder()
            .clientLists("0.0.0.0/0")
            .gatewayArn(exampleAwsStoragegatewayGateway.arn())
            .locationArn(exampleAwsS3Bucket.arn())
            .roleArn(exampleAwsIamRole.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:NfsFileShare
    properties:
      clientLists:
        - 0.0.0.0/0
      gatewayArn: ${exampleAwsStoragegatewayGateway.arn}
      locationArn: ${exampleAwsS3Bucket.arn}
      roleArn: ${exampleAwsIamRole.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_storagegateway_nfs_file_share` using the NFS File Share Amazon Resource Name (ARN). For example:

```sh
$ pulumi import aws:storagegateway/nfsFileShare:NfsFileShare example arn:aws:storagegateway:us-east-1:123456789012:share/share-12345678
```
`
auditDestinationArnB" CThe Amazon Resource Name (ARN) of the storage used for audit logs.
|
bucketRegionB" fThe region of the S3 bucket used by the file share. Required when specifying `vpc_endpoint_dns_name`.
÷
cacheAttributesB}:{
y
storagegatewayNfsFileShareCacheAttributesJaws:storagegateway/NfsFileShareCacheAttributes:NfsFileShareCacheAttributesBRefresh cache information. see Cache Attributes for more details.
Ï
clientLists*" ÷The list of clients that are allowed to access the file gateway. The list must contain either valid IP addresses or valid CIDR blocks. Set to `["0.0.0.0/0"]` to not limit access. Minimum 1 item. Maximum 100 items.
≠
defaultStorageClassB" èThe default [storage class](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-DefaultStorageClass) for objects put into an Amazon S3 bucket by the file gateway. Defaults to `S3_STANDARD`.
n
fileShareNameB" WThe name of the file share. Must be set if an S3 prefix name is set in `location_arn`.
B

gatewayArn" 0Amazon Resource Name (ARN) of the file gateway.
ñ
guessMimeTypeEnabledB
 xBoolean value that enables guessing of the MIME type for uploaded objects based on file extensions. Defaults to `true`.
µ
kmsEncryptedB
 ûBoolean value if `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Defaults to `false`.
ü
	kmsKeyArnB" ãAmazon Resource Name (ARN) for KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is true.
M
locationArn" :The ARN of the backed storage used for storing file data.
¢
nfsFileShareDefaultsëBé:ã
à
storagegateway NfsFileShareNfsFileShareDefaultsTaws:storagegateway/NfsFileShareNfsFileShareDefaults:NfsFileShareNfsFileShareDefaultsvNested argument with file share default values. More information below. see NFS File Share Defaults for more details.
™
notificationPolicyB" çThe notification policy of the file share. For more information see the [AWS Documentation](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-NotificationPolicy). Default value is `{}`.
Y
	objectAclB" FAccess Control List permission for S3 objects. Defaults to `private`.
Ñ
readOnlyB
 rBoolean to indicate write status of file share. File share does not accept writes if `true`. Defaults to `false`.
ﬁ
requesterPaysB
 ∆Boolean who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to `true` if you want the requester to pay instead of the bucket owner. Defaults to `false`.
í
roleArn" ÇThe ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.
Ú
squashB" ·Maps a user to anonymous user. Defaults to `RootSquash`. Valid values: `RootSquash` (only root is mapped to anonymous user), `NoSquash` (no one is mapped to anonymous user), `AllSquash` (everyone is mapped to anonymous user)
»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
Q
vpcEndpointDnsNameB" 5The DNS name of the VPC endpoint for S3 PrivateLink.
"=
arn" 2Amazon Resource Name (ARN) of the NFS File Share.
"`
auditDestinationArnB" CThe Amazon Resource Name (ARN) of the storage used for audit logs.
"|
bucketRegionB" fThe region of the S3 bucket used by the file share. Required when specifying `vpc_endpoint_dns_name`.
"÷
cacheAttributesB}:{
y
storagegatewayNfsFileShareCacheAttributesJaws:storagegateway/NfsFileShareCacheAttributes:NfsFileShareCacheAttributesBRefresh cache information. see Cache Attributes for more details.
"Ï
clientLists*" ÷The list of clients that are allowed to access the file gateway. The list must contain either valid IP addresses or valid CIDR blocks. Set to `["0.0.0.0/0"]` to not limit access. Minimum 1 item. Maximum 100 items.
"≠
defaultStorageClassB" èThe default [storage class](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-DefaultStorageClass) for objects put into an Amazon S3 bucket by the file gateway. Defaults to `S3_STANDARD`.
"l
fileShareName" WThe name of the file share. Must be set if an S3 prefix name is set in `location_arn`.
"-
fileshareId" ID of the NFS File Share.
"B

gatewayArn" 0Amazon Resource Name (ARN) of the file gateway.
"ñ
guessMimeTypeEnabledB
 xBoolean value that enables guessing of the MIME type for uploaded objects based on file extensions. Defaults to `true`.
"µ
kmsEncryptedB
 ûBoolean value if `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Defaults to `false`.
"ü
	kmsKeyArnB" ãAmazon Resource Name (ARN) for KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is true.
"M
locationArn" :The ARN of the backed storage used for storing file data.
"¢
nfsFileShareDefaultsëBé:ã
à
storagegateway NfsFileShareNfsFileShareDefaultsTaws:storagegateway/NfsFileShareNfsFileShareDefaults:NfsFileShareNfsFileShareDefaultsvNested argument with file share default values. More information below. see NFS File Share Defaults for more details.
"™
notificationPolicyB" çThe notification policy of the file share. For more information see the [AWS Documentation](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-NotificationPolicy). Default value is `{}`.
"Y
	objectAclB" FAccess Control List permission for S3 objects. Defaults to `private`.
"P
path" DFile share path used by the NFS client to identify the mount point.
"Ñ
readOnlyB
 rBoolean to indicate write status of file share. File share does not accept writes if `true`. Defaults to `false`.
"ﬁ
requesterPaysB
 ∆Boolean who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to `true` if you want the requester to pay instead of the bucket owner. Defaults to `false`.
"í
roleArn" ÇThe ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.
"Ú
squashB" ·Maps a user to anonymous user. Defaults to `RootSquash`. Valid values: `RootSquash` (only root is mapped to anonymous user), `NoSquash` (no one is mapped to anonymous user), `AllSquash` (everyone is mapped to anonymous user)
"»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"Q
vpcEndpointDnsNameB" 5The DNS name of the VPC endpoint for S3 PrivateLink.
*ã}
L
storagegatewaySmbFileShare,aws:storagegateway/smbFileShare:SmbFileShare¯2Manages an AWS Storage Gateway SMB File Share.

## Example Usage

### Active Directory Authentication

> **NOTE:** The gateway must have already joined the Active Directory domain prior to SMB file share creationE.g., via "SMB Settings" in the AWS Storage Gateway console or `smb_active_directory_settings` in the `aws.storagegateway.Gateway` resource.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.SmbFileShare("example", {
    authentication: "ActiveDirectory",
    gatewayArn: exampleAwsStoragegatewayGateway.arn,
    locationArn: exampleAwsS3Bucket.arn,
    roleArn: exampleAwsIamRole.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.SmbFileShare("example",
    authentication="ActiveDirectory",
    gateway_arn=example_aws_storagegateway_gateway["arn"],
    location_arn=example_aws_s3_bucket["arn"],
    role_arn=example_aws_iam_role["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.SmbFileShare("example", new()
    {
        Authentication = "ActiveDirectory",
        GatewayArn = exampleAwsStoragegatewayGateway.Arn,
        LocationArn = exampleAwsS3Bucket.Arn,
        RoleArn = exampleAwsIamRole.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewSmbFileShare(ctx, "example", &storagegateway.SmbFileShareArgs{
			Authentication: pulumi.String("ActiveDirectory"),
			GatewayArn:     pulumi.Any(exampleAwsStoragegatewayGateway.Arn),
			LocationArn:    pulumi.Any(exampleAwsS3Bucket.Arn),
			RoleArn:        pulumi.Any(exampleAwsIamRole.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.SmbFileShare;
import com.pulumi.aws.storagegateway.SmbFileShareArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new SmbFileShare("example", SmbFileShareArgs.builder()
            .authentication("ActiveDirectory")
            .gatewayArn(exampleAwsStoragegatewayGateway.arn())
            .locationArn(exampleAwsS3Bucket.arn())
            .roleArn(exampleAwsIamRole.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:SmbFileShare
    properties:
      authentication: ActiveDirectory
      gatewayArn: ${exampleAwsStoragegatewayGateway.arn}
      locationArn: ${exampleAwsS3Bucket.arn}
      roleArn: ${exampleAwsIamRole.arn}
```
<!--End PulumiCodeChooser -->

### Guest Authentication

> **NOTE:** The gateway must have already had the SMB guest password set prior to SMB file share creationE.g., via "SMB Settings" in the AWS Storage Gateway console or `smb_guest_password` in the `aws.storagegateway.Gateway` resource.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.SmbFileShare("example", {
    authentication: "GuestAccess",
    gatewayArn: exampleAwsStoragegatewayGateway.arn,
    locationArn: exampleAwsS3Bucket.arn,
    roleArn: exampleAwsIamRole.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.SmbFileShare("example",
    authentication="GuestAccess",
    gateway_arn=example_aws_storagegateway_gateway["arn"],
    location_arn=example_aws_s3_bucket["arn"],
    role_arn=example_aws_iam_role["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.SmbFileShare("example", new()
    {
        Authentication = "GuestAccess",
        GatewayArn = exampleAwsStoragegatewayGateway.Arn,
        LocationArn = exampleAwsS3Bucket.Arn,
        RoleArn = exampleAwsIamRole.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewSmbFileShare(ctx, "example", &storagegateway.SmbFileShareArgs{
			Authentication: pulumi.String("GuestAccess"),
			GatewayArn:     pulumi.Any(exampleAwsStoragegatewayGateway.Arn),
			LocationArn:    pulumi.Any(exampleAwsS3Bucket.Arn),
			RoleArn:        pulumi.Any(exampleAwsIamRole.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.SmbFileShare;
import com.pulumi.aws.storagegateway.SmbFileShareArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new SmbFileShare("example", SmbFileShareArgs.builder()
            .authentication("GuestAccess")
            .gatewayArn(exampleAwsStoragegatewayGateway.arn())
            .locationArn(exampleAwsS3Bucket.arn())
            .roleArn(exampleAwsIamRole.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:SmbFileShare
    properties:
      authentication: GuestAccess
      gatewayArn: ${exampleAwsStoragegatewayGateway.arn}
      locationArn: ${exampleAwsS3Bucket.arn}
      roleArn: ${exampleAwsIamRole.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_storagegateway_smb_file_share` using the SMB File Share Amazon Resource Name (ARN). For example:

```sh
$ pulumi import aws:storagegateway/smbFileShare:SmbFileShare example arn:aws:storagegateway:us-east-1:123456789012:share/share-12345678
```
é
accessBasedEnumerationB
 nThe files and folders on this share will only be visible to users with read access. Default value is `false`.
©
adminUserListsB*" éA list of users in the Active Directory that have admin access to the file share. Only valid if `authentication` is set to `ActiveDirectory`.
q
auditDestinationArnB" TThe Amazon Resource Name (ARN) of the CloudWatch Log Group used for the audit logs.
´
authenticationB" íThe authentication method that users use to access the file share. Defaults to `ActiveDirectory`. Valid values: `ActiveDirectory`, `GuestAccess`.
|
bucketRegionB" fThe region of the S3 buck used by the file share. Required when specifying a `vpc_endpoint_dns_name`.
≈
cacheAttributesB}:{
y
storagegatewaySmbFileShareCacheAttributesJaws:storagegateway/SmbFileShareCacheAttributes:SmbFileShareCacheAttributes∞Refresh cache information. see `cache_attributes` Block for more details.

**Note:** If you have previously included a `cache_attributes` block in your configuration, removing it will not reset the refresh cache value and the previous value will remain. You must explicitly set a new value to change it.
˙
caseSensitivityB" ‡The case of an object name in an Amazon S3 bucket. For `ClientSpecified`, the client determines the case sensitivity. For `CaseSensitive`, the gateway determines the case sensitivity. The default value is `ClientSpecified`.
≠
defaultStorageClassB" èThe default [storage class](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-DefaultStorageClass) for objects put into an Amazon S3 bucket by the file gateway. Defaults to `S3_STANDARD`.
n
fileShareNameB" WThe name of the file share. Must be set if an S3 prefix name is set in `location_arn`.
B

gatewayArn" 0Amazon Resource Name (ARN) of the file gateway.
ñ
guessMimeTypeEnabledB
 xBoolean value that enables guessing of the MIME type for uploaded objects based on file extensions. Defaults to `true`.
∞
invalidUserListsB*" ìA list of users in the Active Directory that are not allowed to access the file share. Only valid if `authentication` is set to `ActiveDirectory`.
µ
kmsEncryptedB
 ûBoolean value if `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Defaults to `false`.
ü
	kmsKeyArnB" ãAmazon Resource Name (ARN) for KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is true.
M
locationArn" :The ARN of the backed storage used for storing file data.
™
notificationPolicyB" çThe notification policy of the file share. For more information see the [AWS Documentation](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-NotificationPolicy). Default value is `{}`.
Y
	objectAclB" FAccess Control List permission for S3 objects. Defaults to `private`.
d
oplocksEnabledB
 LBoolean to indicate Opportunistic lock (oplock) status. Defaults to `true`.
Ñ
readOnlyB
 rBoolean to indicate write status of file share. File share does not accept writes if `true`. Defaults to `false`.
ﬁ
requesterPaysB
 ∆Boolean who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to `true` if you want the requester to pay instead of the bucket owner. Defaults to `false`.
í
roleArn" ÇThe ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.
Å
smbAclEnabledB
 ÈSet this value to `true` to enable ACL (access control list) on the SMB fileshare. Set it to `false` to map file and directory permissions to the POSIX permissions. This setting applies only to `ActiveDirectory` authentication type.
»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
≤
validUserListsB*" óA list of users in the Active Directory that are allowed to access the file share. If you need to specify an Active directory group, add '@' before the name of the group. It will be set on Allowed group in AWS console. Only valid if `authentication` is set to `ActiveDirectory`.
R
vpcEndpointDnsNameB" 6The DNS name of the VPC endpoint for S3 private link.
"é
accessBasedEnumerationB
 nThe files and folders on this share will only be visible to users with read access. Default value is `false`.
"©
adminUserListsB*" éA list of users in the Active Directory that have admin access to the file share. Only valid if `authentication` is set to `ActiveDirectory`.
"=
arn" 2Amazon Resource Name (ARN) of the SMB File Share.
"q
auditDestinationArnB" TThe Amazon Resource Name (ARN) of the CloudWatch Log Group used for the audit logs.
"´
authenticationB" íThe authentication method that users use to access the file share. Defaults to `ActiveDirectory`. Valid values: `ActiveDirectory`, `GuestAccess`.
"|
bucketRegionB" fThe region of the S3 buck used by the file share. Required when specifying a `vpc_endpoint_dns_name`.
"≈
cacheAttributesB}:{
y
storagegatewaySmbFileShareCacheAttributesJaws:storagegateway/SmbFileShareCacheAttributes:SmbFileShareCacheAttributes∞Refresh cache information. see `cache_attributes` Block for more details.

**Note:** If you have previously included a `cache_attributes` block in your configuration, removing it will not reset the refresh cache value and the previous value will remain. You must explicitly set a new value to change it.
"˙
caseSensitivityB" ‡The case of an object name in an Amazon S3 bucket. For `ClientSpecified`, the client determines the case sensitivity. For `CaseSensitive`, the gateway determines the case sensitivity. The default value is `ClientSpecified`.
"≠
defaultStorageClassB" èThe default [storage class](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-DefaultStorageClass) for objects put into an Amazon S3 bucket by the file gateway. Defaults to `S3_STANDARD`.
"l
fileShareName" WThe name of the file share. Must be set if an S3 prefix name is set in `location_arn`.
"-
fileshareId" ID of the SMB File Share.
"B

gatewayArn" 0Amazon Resource Name (ARN) of the file gateway.
"ñ
guessMimeTypeEnabledB
 xBoolean value that enables guessing of the MIME type for uploaded objects based on file extensions. Defaults to `true`.
"∞
invalidUserListsB*" ìA list of users in the Active Directory that are not allowed to access the file share. Only valid if `authentication` is set to `ActiveDirectory`.
"µ
kmsEncryptedB
 ûBoolean value if `true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Defaults to `false`.
"ü
	kmsKeyArnB" ãAmazon Resource Name (ARN) for KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is true.
"M
locationArn" :The ARN of the backed storage used for storing file data.
"™
notificationPolicyB" çThe notification policy of the file share. For more information see the [AWS Documentation](https://docs.aws.amazon.com/storagegateway/latest/APIReference/API_CreateNFSFileShare.html#StorageGateway-CreateNFSFileShare-request-NotificationPolicy). Default value is `{}`.
"Y
	objectAclB" FAccess Control List permission for S3 objects. Defaults to `private`.
"b
oplocksEnabled
 LBoolean to indicate Opportunistic lock (oplock) status. Defaults to `true`.
"P
path" DFile share path used by the NFS client to identify the mount point.
"Ñ
readOnlyB
 rBoolean to indicate write status of file share. File share does not accept writes if `true`. Defaults to `false`.
"ﬁ
requesterPaysB
 ∆Boolean who pays the cost of the request and the data download from the Amazon S3 bucket. Set this value to `true` if you want the requester to pay instead of the bucket owner. Defaults to `false`.
"í
roleArn" ÇThe ARN of the AWS Identity and Access Management (IAM) role that a file gateway assumes when it accesses the underlying storage.
"Å
smbAclEnabledB
 ÈSet this value to `true` to enable ACL (access control list) on the SMB fileshare. Set it to `false` to map file and directory permissions to the POSIX permissions. This setting applies only to `ActiveDirectory` authentication type.
"»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"≤
validUserListsB*" óA list of users in the Active Directory that are allowed to access the file share. If you need to specify an Active directory group, add '@' before the name of the group. It will be set on Allowed group in AWS console. Only valid if `authentication` is set to `ActiveDirectory`.
"R
vpcEndpointDnsNameB" 6The DNS name of the VPC endpoint for S3 private link.
*èV
[
storagegatewayStoredIscsiVolume6aws:storagegateway/storedIscsiVolume:StoredIscsiVolume£8Manages an AWS Storage Gateway stored iSCSI volume.

> **NOTE:** The gateway must have a working storage added (e.g., via the `aws.storagegateway.WorkingStorage` resource) before the volume is operational to clients, however the Storage Gateway API will allow volume creation without error in that case and return volume status as `WORKING STORAGE NOT CONFIGURED`.

## Example Usage

### Create Empty Stored iSCSI Volume

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.StoredIscsiVolume("example", {
    gatewayArn: exampleAwsStoragegatewayCache.gatewayArn,
    networkInterfaceId: exampleAwsInstance.privateIp,
    targetName: "example",
    preserveExistingData: false,
    diskId: test.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.StoredIscsiVolume("example",
    gateway_arn=example_aws_storagegateway_cache["gatewayArn"],
    network_interface_id=example_aws_instance["privateIp"],
    target_name="example",
    preserve_existing_data=False,
    disk_id=test["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.StoredIscsiVolume("example", new()
    {
        GatewayArn = exampleAwsStoragegatewayCache.GatewayArn,
        NetworkInterfaceId = exampleAwsInstance.PrivateIp,
        TargetName = "example",
        PreserveExistingData = false,
        DiskId = test.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewStoredIscsiVolume(ctx, "example", &storagegateway.StoredIscsiVolumeArgs{
			GatewayArn:           pulumi.Any(exampleAwsStoragegatewayCache.GatewayArn),
			NetworkInterfaceId:   pulumi.Any(exampleAwsInstance.PrivateIp),
			TargetName:           pulumi.String("example"),
			PreserveExistingData: pulumi.Bool(false),
			DiskId:               pulumi.Any(test.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.StoredIscsiVolume;
import com.pulumi.aws.storagegateway.StoredIscsiVolumeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new StoredIscsiVolume("example", StoredIscsiVolumeArgs.builder()
            .gatewayArn(exampleAwsStoragegatewayCache.gatewayArn())
            .networkInterfaceId(exampleAwsInstance.privateIp())
            .targetName("example")
            .preserveExistingData(false)
            .diskId(test.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:StoredIscsiVolume
    properties:
      gatewayArn: ${exampleAwsStoragegatewayCache.gatewayArn}
      networkInterfaceId: ${exampleAwsInstance.privateIp}
      targetName: example
      preserveExistingData: false
      diskId: ${test.id}
```
<!--End PulumiCodeChooser -->

### Create Stored iSCSI Volume From Snapshot

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.StoredIscsiVolume("example", {
    gatewayArn: exampleAwsStoragegatewayCache.gatewayArn,
    networkInterfaceId: exampleAwsInstance.privateIp,
    snapshotId: exampleAwsEbsSnapshot.id,
    targetName: "example",
    preserveExistingData: false,
    diskId: test.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.StoredIscsiVolume("example",
    gateway_arn=example_aws_storagegateway_cache["gatewayArn"],
    network_interface_id=example_aws_instance["privateIp"],
    snapshot_id=example_aws_ebs_snapshot["id"],
    target_name="example",
    preserve_existing_data=False,
    disk_id=test["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.StoredIscsiVolume("example", new()
    {
        GatewayArn = exampleAwsStoragegatewayCache.GatewayArn,
        NetworkInterfaceId = exampleAwsInstance.PrivateIp,
        SnapshotId = exampleAwsEbsSnapshot.Id,
        TargetName = "example",
        PreserveExistingData = false,
        DiskId = test.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewStoredIscsiVolume(ctx, "example", &storagegateway.StoredIscsiVolumeArgs{
			GatewayArn:           pulumi.Any(exampleAwsStoragegatewayCache.GatewayArn),
			NetworkInterfaceId:   pulumi.Any(exampleAwsInstance.PrivateIp),
			SnapshotId:           pulumi.Any(exampleAwsEbsSnapshot.Id),
			TargetName:           pulumi.String("example"),
			PreserveExistingData: pulumi.Bool(false),
			DiskId:               pulumi.Any(test.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.StoredIscsiVolume;
import com.pulumi.aws.storagegateway.StoredIscsiVolumeArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new StoredIscsiVolume("example", StoredIscsiVolumeArgs.builder()
            .gatewayArn(exampleAwsStoragegatewayCache.gatewayArn())
            .networkInterfaceId(exampleAwsInstance.privateIp())
            .snapshotId(exampleAwsEbsSnapshot.id())
            .targetName("example")
            .preserveExistingData(false)
            .diskId(test.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:StoredIscsiVolume
    properties:
      gatewayArn: ${exampleAwsStoragegatewayCache.gatewayArn}
      networkInterfaceId: ${exampleAwsInstance.privateIp}
      snapshotId: ${exampleAwsEbsSnapshot.id}
      targetName: example
      preserveExistingData: false
      diskId: ${test.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_storagegateway_stored_iscsi_volume` using the volume Amazon Resource Name (ARN). For example:

```sh
$ pulumi import aws:storagegateway/storedIscsiVolume:StoredIscsiVolume example arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/volume/vol-12345678
```
f
diskId" XThe unique identifier for the gateway local disk that is configured as a stored volume.
A

gatewayArn" /The Amazon Resource Name (ARN) of the gateway.
ô
kmsEncryptedB
 Ç`true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Optional.
©
kmsKeyB" òThe Amazon Resource Name (ARN) of the AWS KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is `true`.
Ü
networkInterfaceId" lThe network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted.
±
preserveExistingData
 îSpecify this field as `true` if you want to preserve the data on the local disk. Otherwise, specifying this field as false creates an empty volume.
n

snapshotIdB" ZThe snapshot ID of the snapshot to restore as the new stored volumeE.g., `snap-1122aabb`.
Ã
tagsB2" ªKey-value mapping of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
¿

targetName" ≠The name of the iSCSI target used by initiators to connect to the target and as a suffix for the target ARN. The target name must be unique across all volumes of a gateway.
"è
arn" ÉVolume Amazon Resource Name (ARN), e.g., `arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/volume/vol-12345678`.
"H
chapEnabled
 5Whether mutual CHAP is enabled for the iSCSI target.
"f
diskId" XThe unique identifier for the gateway local disk that is configured as a stored volume.
"A

gatewayArn" /The Amazon Resource Name (ARN) of the gateway.
"ô
kmsEncryptedB
 Ç`true` to use Amazon S3 server side encryption with your own AWS KMS key, or `false` to use a key managed by Amazon S3. Optional.
"©
kmsKeyB" òThe Amazon Resource Name (ARN) of the AWS KMS key used for Amazon S3 server side encryption. This value can only be set when `kms_encrypted` is `true`.
"&
	lunNumber Logical disk number.
"Ü
networkInterfaceId" lThe network interface of the gateway on which to expose the iSCSI target. Only IPv4 addresses are accepted.
"M
networkInterfacePort 1The port used to communicate with iSCSI targets.
"±
preserveExistingData
 îSpecify this field as `true` if you want to preserve the data on the local disk. Otherwise, specifying this field as false creates an empty volume.
"n

snapshotIdB" ZThe snapshot ID of the snapshot to restore as the new stored volumeE.g., `snap-1122aabb`.
"Ã
tagsB2" ªKey-value mapping of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"™
	targetArn" òTarget Amazon Resource Name (ARN), e.g., `arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678/target/iqn.1997-05.com.amazon:TargetName`.
"¿

targetName" ≠The name of the iSCSI target used by initiators to connect to the target and as a suffix for the target ARN. The target name must be unique across all volumes of a gateway.
"†
volumeAttachmentStatus" ÅA value that indicates whether a storage volume is attached to, detached from, or is in the process of detaching from a gateway.
"1
volumeId" !Volume ID, e.g., `vol-12345678`.
"M
volumeSizeInBytes 4The size of the data stored on the volume in bytes.
"?
volumeStatus" +indicates the state of the storage volume.
"4

volumeType" "indicates the type of the volume.
*å'
@
storagegatewayTapePool$aws:storagegateway/tapePool:TapePoolöManages an AWS Storage Gateway Tape Pool.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.TapePool("example", {
    poolName: "example",
    storageClass: "GLACIER",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.TapePool("example",
    pool_name="example",
    storage_class="GLACIER")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.TapePool("example", new()
    {
        PoolName = "example",
        StorageClass = "GLACIER",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewTapePool(ctx, "example", &storagegateway.TapePoolArgs{
			PoolName:     pulumi.String("example"),
			StorageClass: pulumi.String("GLACIER"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.TapePool;
import com.pulumi.aws.storagegateway.TapePoolArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new TapePool("example", TapePoolArgs.builder()
            .poolName("example")
            .storageClass("GLACIER")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:TapePool
    properties:
      poolName: example
      storageClass: GLACIER
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_storagegateway_tape_pool` using the volume Amazon Resource Name (ARN). For example:

```sh
$ pulumi import aws:storagegateway/tapePool:TapePool example arn:aws:storagegateway:us-east-1:123456789012:tapepool/pool-12345678
```
6
poolName" &The name of the new custom tape pool.
•
retentionLockTimeInDaysB ÉTape retention lock time is set in days. Tape retention lock can be enabled for up to 100 years (36,500 days). Default value is 0.
Ω
retentionLockTypeB" °Tape retention lock can be configured in two modes. When configured in governance mode, AWS accounts with specific IAM permissions are authorized to remove the tape retention lock from archived virtual tapes. When configured in compliance mode, the tape retention lock cannot be removed by any user, including the root AWS account. Possible values are `COMPLIANCE`, `GOVERNANCE`, and `NONE`. Default value is `NONE`.
ê
storageClass" ˚The storage class that is associated with the new custom pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class that corresponds to the pool. Possible values are `DEEP_ARCHIVE` or `GLACIER`.
»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"¢
arn" ñVolume Amazon Resource Name (ARN), e.g., `aws_storagegateway_tape_pool.example arn:aws:storagegateway:us-east-1:123456789012:tapepool/pool-12345678`.
"6
poolName" &The name of the new custom tape pool.
"•
retentionLockTimeInDaysB ÉTape retention lock time is set in days. Tape retention lock can be enabled for up to 100 years (36,500 days). Default value is 0.
"Ω
retentionLockTypeB" °Tape retention lock can be configured in two modes. When configured in governance mode, AWS accounts with specific IAM permissions are authorized to remove the tape retention lock from archived virtual tapes. When configured in compliance mode, the tape retention lock cannot be removed by any user, including the root AWS account. Possible values are `COMPLIANCE`, `GOVERNANCE`, and `NONE`. Default value is `NONE`.
"ê
storageClass" ˚The storage class that is associated with the new custom pool. When you use your backup application to eject the tape, the tape is archived directly into the storage class that corresponds to the pool. Possible values are `DEEP_ARCHIVE` or `GLACIER`.
"»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*≤B
L
storagegatewayUploadBuffer,aws:storagegateway/uploadBuffer:UploadBuffer≠>Manages an AWS Storage Gateway upload buffer.

> **NOTE:** The Storage Gateway API provides no method to remove an upload buffer disk. Destroying this resource does not perform any Storage Gateway actions.

## Example Usage

### Cached and VTL Gateway Type

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.storagegateway.getLocalDisk({
    diskNode: testAwsVolumeAttachment.deviceName,
    gatewayArn: testAwsStoragegatewayGateway.arn,
});
const testUploadBuffer = new aws.storagegateway.UploadBuffer("test", {
    diskPath: test.then(test => test.diskPath),
    gatewayArn: testAwsStoragegatewayGateway.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.storagegateway.get_local_disk(disk_node=test_aws_volume_attachment["deviceName"],
    gateway_arn=test_aws_storagegateway_gateway["arn"])
test_upload_buffer = aws.storagegateway.UploadBuffer("test",
    disk_path=test.disk_path,
    gateway_arn=test_aws_storagegateway_gateway["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.StorageGateway.GetLocalDisk.Invoke(new()
    {
        DiskNode = testAwsVolumeAttachment.DeviceName,
        GatewayArn = testAwsStoragegatewayGateway.Arn,
    });

    var testUploadBuffer = new Aws.StorageGateway.UploadBuffer("test", new()
    {
        DiskPath = test.Apply(getLocalDiskResult => getLocalDiskResult.DiskPath),
        GatewayArn = testAwsStoragegatewayGateway.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		test, err := storagegateway.GetLocalDisk(ctx, &storagegateway.GetLocalDiskArgs{
			DiskNode:   pulumi.StringRef(testAwsVolumeAttachment.DeviceName),
			GatewayArn: testAwsStoragegatewayGateway.Arn,
		}, nil)
		if err != nil {
			return err
		}
		_, err = storagegateway.NewUploadBuffer(ctx, "test", &storagegateway.UploadBufferArgs{
			DiskPath:   pulumi.String(test.DiskPath),
			GatewayArn: pulumi.Any(testAwsStoragegatewayGateway.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.StoragegatewayFunctions;
import com.pulumi.aws.storagegateway.inputs.GetLocalDiskArgs;
import com.pulumi.aws.storagegateway.UploadBuffer;
import com.pulumi.aws.storagegateway.UploadBufferArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = StoragegatewayFunctions.getLocalDisk(GetLocalDiskArgs.builder()
            .diskNode(testAwsVolumeAttachment.deviceName())
            .gatewayArn(testAwsStoragegatewayGateway.arn())
            .build());

        var testUploadBuffer = new UploadBuffer("testUploadBuffer", UploadBufferArgs.builder()
            .diskPath(test.applyValue(getLocalDiskResult -> getLocalDiskResult.diskPath()))
            .gatewayArn(testAwsStoragegatewayGateway.arn())
            .build());

    }
}
```
```yaml
resources:
  testUploadBuffer:
    type: aws:storagegateway:UploadBuffer
    name: test
    properties:
      diskPath: ${test.diskPath}
      gatewayArn: ${testAwsStoragegatewayGateway.arn}
variables:
  test:
    fn::invoke:
      function: aws:storagegateway:getLocalDisk
      arguments:
        diskNode: ${testAwsVolumeAttachment.deviceName}
        gatewayArn: ${testAwsStoragegatewayGateway.arn}
```
<!--End PulumiCodeChooser -->

### Stored Gateway Type

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.storagegateway.getLocalDisk({
    diskNode: testAwsVolumeAttachment.deviceName,
    gatewayArn: testAwsStoragegatewayGateway.arn,
});
const example = new aws.storagegateway.UploadBuffer("example", {
    diskId: exampleAwsStoragegatewayLocalDisk.id,
    gatewayArn: exampleAwsStoragegatewayGateway.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.storagegateway.get_local_disk(disk_node=test_aws_volume_attachment["deviceName"],
    gateway_arn=test_aws_storagegateway_gateway["arn"])
example = aws.storagegateway.UploadBuffer("example",
    disk_id=example_aws_storagegateway_local_disk["id"],
    gateway_arn=example_aws_storagegateway_gateway["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.StorageGateway.GetLocalDisk.Invoke(new()
    {
        DiskNode = testAwsVolumeAttachment.DeviceName,
        GatewayArn = testAwsStoragegatewayGateway.Arn,
    });

    var example = new Aws.StorageGateway.UploadBuffer("example", new()
    {
        DiskId = exampleAwsStoragegatewayLocalDisk.Id,
        GatewayArn = exampleAwsStoragegatewayGateway.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.GetLocalDisk(ctx, &storagegateway.GetLocalDiskArgs{
			DiskNode:   pulumi.StringRef(testAwsVolumeAttachment.DeviceName),
			GatewayArn: testAwsStoragegatewayGateway.Arn,
		}, nil)
		if err != nil {
			return err
		}
		_, err = storagegateway.NewUploadBuffer(ctx, "example", &storagegateway.UploadBufferArgs{
			DiskId:     pulumi.Any(exampleAwsStoragegatewayLocalDisk.Id),
			GatewayArn: pulumi.Any(exampleAwsStoragegatewayGateway.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.StoragegatewayFunctions;
import com.pulumi.aws.storagegateway.inputs.GetLocalDiskArgs;
import com.pulumi.aws.storagegateway.UploadBuffer;
import com.pulumi.aws.storagegateway.UploadBufferArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = StoragegatewayFunctions.getLocalDisk(GetLocalDiskArgs.builder()
            .diskNode(testAwsVolumeAttachment.deviceName())
            .gatewayArn(testAwsStoragegatewayGateway.arn())
            .build());

        var example = new UploadBuffer("example", UploadBufferArgs.builder()
            .diskId(exampleAwsStoragegatewayLocalDisk.id())
            .gatewayArn(exampleAwsStoragegatewayGateway.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:UploadBuffer
    properties:
      diskId: ${exampleAwsStoragegatewayLocalDisk.id}
      gatewayArn: ${exampleAwsStoragegatewayGateway.arn}
variables:
  test:
    fn::invoke:
      function: aws:storagegateway:getLocalDisk
      arguments:
        diskNode: ${testAwsVolumeAttachment.deviceName}
        gatewayArn: ${testAwsStoragegatewayGateway.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_storagegateway_upload_buffer` using the gateway Amazon Resource Name (ARN) and local disk identifier separated with a colon (`:`). For example:

```sh
$ pulumi import aws:storagegateway/uploadBuffer:UploadBuffer example arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678:pci-0000:03:00.0-scsi-0:0:0:0
```
U
diskIdB" ELocal disk identifier. For example, `pci-0000:03:00.0-scsi-0:0:0:0`.
@
diskPathB" .Local disk path. For example, `/dev/nvme1n1`.
A

gatewayArn" /The Amazon Resource Name (ARN) of the gateway.
"S
diskId" ELocal disk identifier. For example, `pci-0000:03:00.0-scsi-0:0:0:0`.
">
diskPath" .Local disk path. For example, `/dev/nvme1n1`.
"A

gatewayArn" /The Amazon Resource Name (ARN) of the gateway.
*•
R
storagegatewayWorkingStorage0aws:storagegateway/workingStorage:WorkingStorageûManages an AWS Storage Gateway working storage.

> **NOTE:** The Storage Gateway API provides no method to remove a working storage disk. Destroying this resource does not perform any Storage Gateway actions.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.storagegateway.WorkingStorage("example", {
    diskId: exampleAwsStoragegatewayLocalDisk.id,
    gatewayArn: exampleAwsStoragegatewayGateway.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.storagegateway.WorkingStorage("example",
    disk_id=example_aws_storagegateway_local_disk["id"],
    gateway_arn=example_aws_storagegateway_gateway["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.StorageGateway.WorkingStorage("example", new()
    {
        DiskId = exampleAwsStoragegatewayLocalDisk.Id,
        GatewayArn = exampleAwsStoragegatewayGateway.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.NewWorkingStorage(ctx, "example", &storagegateway.WorkingStorageArgs{
			DiskId:     pulumi.Any(exampleAwsStoragegatewayLocalDisk.Id),
			GatewayArn: pulumi.Any(exampleAwsStoragegatewayGateway.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.WorkingStorage;
import com.pulumi.aws.storagegateway.WorkingStorageArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new WorkingStorage("example", WorkingStorageArgs.builder()
            .diskId(exampleAwsStoragegatewayLocalDisk.id())
            .gatewayArn(exampleAwsStoragegatewayGateway.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:storagegateway:WorkingStorage
    properties:
      diskId: ${exampleAwsStoragegatewayLocalDisk.id}
      gatewayArn: ${exampleAwsStoragegatewayGateway.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_storagegateway_working_storage` using the gateway Amazon Resource Name (ARN) and local disk identifier separated with a colon (`:`). For example:

```sh
$ pulumi import aws:storagegateway/workingStorage:WorkingStorage example arn:aws:storagegateway:us-east-1:123456789012:gateway/sgw-12345678:pci-0000:03:00.0-scsi-0:0:0:0
```
S
diskId" ELocal disk identifier. For example, `pci-0000:03:00.0-scsi-0:0:0:0`.
A

gatewayArn" /The Amazon Resource Name (ARN) of the gateway.
"S
diskId" ELocal disk identifier. For example, `pci-0000:03:00.0-scsi-0:0:0:0`.
"A

gatewayArn" /The Amazon Resource Name (ARN) of the gateway.
*É
$
swfDomainaws:swf/domain:DomainÙProvides an SWF Domain resource.

## Example Usage

To register a basic SWF domain:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const foo = new aws.swf.Domain("foo", {
    name: "foo",
    description: "SWF Domain",
    workflowExecutionRetentionPeriodInDays: "30",
});
```
```python
import pulumi
import pulumi_aws as aws

foo = aws.swf.Domain("foo",
    name="foo",
    description="SWF Domain",
    workflow_execution_retention_period_in_days="30")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var foo = new Aws.Swf.Domain("foo", new()
    {
        Name = "foo",
        Description = "SWF Domain",
        WorkflowExecutionRetentionPeriodInDays = "30",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/swf"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := swf.NewDomain(ctx, "foo", &swf.DomainArgs{
			Name:                                   pulumi.String("foo"),
			Description:                            pulumi.String("SWF Domain"),
			WorkflowExecutionRetentionPeriodInDays: pulumi.String("30"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.swf.Domain;
import com.pulumi.aws.swf.DomainArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var foo = new Domain("foo", DomainArgs.builder()
            .name("foo")
            .description("SWF Domain")
            .workflowExecutionRetentionPeriodInDays(30)
            .build());

    }
}
```
```yaml
resources:
  foo:
    type: aws:swf:Domain
    properties:
      name: foo
      description: SWF Domain
      workflowExecutionRetentionPeriodInDays: 30
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import SWF Domains using the `name`. For example:

```sh
$ pulumi import aws:swf/domain:Domain foo test-domain
```
-
descriptionB" The domain description.
c
nameB" UThe name of the domain. If omitted, this provider will assign a random, unique name.
f

namePrefixB" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
—
&workflowExecutionRetentionPeriodInDays" ¢Length of time that SWF will continue to retain information about the workflow execution after the workflow execution is complete, must be between 0 and 90 days.
"&
arn" Amazon Resource Name (ARN)
"-
descriptionB" The domain description.
"a
name" UThe name of the domain. If omitted, this provider will assign a random, unique name.
"d

namePrefix" RCreates a unique name beginning with the specified prefix. Conflicts with `name`.
"»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"—
&workflowExecutionRetentionPeriodInDays" ¢Length of time that SWF will continue to retain information about the workflow execution after the workflow execution is complete, must be between 0 and 90 days.
*¶Y
2

syntheticsCanaryaws:synthetics/canary:CanaryÚ!Provides a Synthetics Canary resource.

> **NOTE:** When you create a canary, AWS creates supporting implicit resources. See the Amazon CloudWatch Synthetics documentation on [DeleteCanary](https://docs.aws.amazon.com/AmazonSynthetics/latest/APIReference/API_DeleteCanary.html) for a full list. Neither AWS nor this provider deletes these implicit resources automatically when the canary is deleted. Before deleting a canary, ensure you have all the information about the canary that you need to delete the implicit resources using the AWS Console, or AWS CLI.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const some = new aws.synthetics.Canary("some", {
    name: "some-canary",
    artifactS3Location: "s3://some-bucket/",
    executionRoleArn: "some-role",
    handler: "exports.handler",
    zipFile: "test-fixtures/lambdatest.zip",
    runtimeVersion: "syn-1.0",
    schedule: {
        expression: "rate(0 minute)",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

some = aws.synthetics.Canary("some",
    name="some-canary",
    artifact_s3_location="s3://some-bucket/",
    execution_role_arn="some-role",
    handler="exports.handler",
    zip_file="test-fixtures/lambdatest.zip",
    runtime_version="syn-1.0",
    schedule={
        "expression": "rate(0 minute)",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var some = new Aws.Synthetics.Canary("some", new()
    {
        Name = "some-canary",
        ArtifactS3Location = "s3://some-bucket/",
        ExecutionRoleArn = "some-role",
        Handler = "exports.handler",
        ZipFile = "test-fixtures/lambdatest.zip",
        RuntimeVersion = "syn-1.0",
        Schedule = new Aws.Synthetics.Inputs.CanaryScheduleArgs
        {
            Expression = "rate(0 minute)",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/synthetics"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := synthetics.NewCanary(ctx, "some", &synthetics.CanaryArgs{
			Name:               pulumi.String("some-canary"),
			ArtifactS3Location: pulumi.String("s3://some-bucket/"),
			ExecutionRoleArn:   pulumi.String("some-role"),
			Handler:            pulumi.String("exports.handler"),
			ZipFile:            pulumi.String("test-fixtures/lambdatest.zip"),
			RuntimeVersion:     pulumi.String("syn-1.0"),
			Schedule: &synthetics.CanaryScheduleArgs{
				Expression: pulumi.String("rate(0 minute)"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.synthetics.Canary;
import com.pulumi.aws.synthetics.CanaryArgs;
import com.pulumi.aws.synthetics.inputs.CanaryScheduleArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var some = new Canary("some", CanaryArgs.builder()
            .name("some-canary")
            .artifactS3Location("s3://some-bucket/")
            .executionRoleArn("some-role")
            .handler("exports.handler")
            .zipFile("test-fixtures/lambdatest.zip")
            .runtimeVersion("syn-1.0")
            .schedule(CanaryScheduleArgs.builder()
                .expression("rate(0 minute)")
                .build())
            .build());

    }
}
```
```yaml
resources:
  some:
    type: aws:synthetics:Canary
    properties:
      name: some-canary
      artifactS3Location: s3://some-bucket/
      executionRoleArn: some-role
      handler: exports.handler
      zipFile: test-fixtures/lambdatest.zip
      runtimeVersion: syn-1.0
      schedule:
        expression: rate(0 minute)
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Synthetics Canaries using the `name`. For example:

```sh
$ pulumi import aws:synthetics/canary:Canary some some-canary
```
é
artifactConfigbB`:^
\

syntheticsCanaryArtifactConfig8aws:synthetics/CanaryArtifactConfig:CanaryArtifactConfigóconfiguration for canary artifacts, including the encryption-at-rest settings for artifacts that the canary uploads to Amazon S3. See Artifact Config.
u
artifactS3Location" [Location in Amazon S3 where Synthetics stores artifacts from the test runs of this canary.
Ñ
deleteLambdaB
 nSpecifies whether to also delete the Lambda functions and layers used by this canary. The default is `false`.
ı
executionRoleArn" ‹ARN of the IAM role to be used to run the canary. see [AWS Docs](https://docs.aws.amazon.com/AmazonSynthetics/latest/APIReference/API_CreateCanary.html#API_CreateCanary_RequestSyntax) for permissions needs for IAM Role.
ª
failureRetentionPeriodB öNumber of days to retain data about failed runs of this canary. If you omit this field, the default of 31 days is used. The valid range is 1 to 455 days.
Ä
handler" qEntry point to use for the source code when running the canary. This value must end with the string `.handler` .
ê
nameB" ÅName for this canary. Has a maximum length of 21 characters. Valid characters are lowercase alphanumeric, hyphen, or underscore.
¢
	runConfigSBQ:O
M

syntheticsCanaryRunConfig.aws:synthetics/CanaryRunConfig:CanaryRunConfig@Configuration block for individual canary runs. Detailed below.
õ
runtimeVersion" ÑRuntime version to use for the canary. Versions change often so consult the [Amazon CloudWatch documentation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html) for the latest valid versions. Values include `syn-python-selenium-1.0`, `syn-nodejs-puppeteer-3.0`, `syn-nodejs-2.2`, `syn-nodejs-2.1`, `syn-nodejs-2.0`, and `syn-1.0`.
ò
s3BucketB" ÖFull bucket name which is used if your canary script is located in S3. The bucket must already exist. **Conflicts with `zip_file`.**
E
s3KeyB" 6S3 key of your script. **Conflicts with `zip_file`.**
P
	s3VersionB" =S3 version ID of your script. **Conflicts with `zip_file`.**
˜
scheduleN:L
J

syntheticsCanarySchedule,aws:synthetics/CanarySchedule:CanaryScheduleöConfiguration block providing how often the canary is to run and when these test runs are to stop. Detailed below.

The following arguments are optional:
8
startCanaryB
 #Whether to run or stop the canary.
ø
successRetentionPeriodB ûNumber of days to retain data about successful runs of this canary. If you omit this field, the default of 31 days is used. The valid range is 1 to 455 days.
«
tagsB2" ∂Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
á
	vpcConfigSBQ:O
M

syntheticsCanaryVpcConfig.aws:synthetics/CanaryVpcConfig:CanaryVpcConfig%Configuration block. Detailed below.
È
zipFileB" ◊ZIP file that contains the script, if you input your canary script directly into the canary instead of referring to an S3 location. It can be up to 225KB. **Conflicts with `s3_bucket`, `s3_key`, and `s3_version`.**
"5
arn" *Amazon Resource Name (ARN) of the Canary.
"é
artifactConfigbB`:^
\

syntheticsCanaryArtifactConfig8aws:synthetics/CanaryArtifactConfig:CanaryArtifactConfigóconfiguration for canary artifacts, including the encryption-at-rest settings for artifacts that the canary uploads to Amazon S3. See Artifact Config.
"u
artifactS3Location" [Location in Amazon S3 where Synthetics stores artifacts from the test runs of this canary.
"Ñ
deleteLambdaB
 nSpecifies whether to also delete the Lambda functions and layers used by this canary. The default is `false`.
"R
	engineArn" AARN of the Lambda function that is used as your canary's engine.
"ı
executionRoleArn" ‹ARN of the IAM role to be used to run the canary. see [AWS Docs](https://docs.aws.amazon.com/AmazonSynthetics/latest/APIReference/API_CreateCanary.html#API_CreateCanary_RequestSyntax) for permissions needs for IAM Role.
"ª
failureRetentionPeriodB öNumber of days to retain data about failed runs of this canary. If you omit this field, the default of 31 days is used. The valid range is 1 to 455 days.
"Ä
handler" qEntry point to use for the source code when running the canary. This value must end with the string `.handler` .
"é
name" ÅName for this canary. Has a maximum length of 21 characters. Valid characters are lowercase alphanumeric, hyphen, or underscore.
"†
	runConfigQ:O
M

syntheticsCanaryRunConfig.aws:synthetics/CanaryRunConfig:CanaryRunConfig@Configuration block for individual canary runs. Detailed below.
"õ
runtimeVersion" ÑRuntime version to use for the canary. Versions change often so consult the [Amazon CloudWatch documentation](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html) for the latest valid versions. Values include `syn-python-selenium-1.0`, `syn-nodejs-puppeteer-3.0`, `syn-nodejs-2.2`, `syn-nodejs-2.1`, `syn-nodejs-2.0`, and `syn-1.0`.
"ò
s3BucketB" ÖFull bucket name which is used if your canary script is located in S3. The bucket must already exist. **Conflicts with `zip_file`.**
"E
s3KeyB" 6S3 key of your script. **Conflicts with `zip_file`.**
"P
	s3VersionB" =S3 version ID of your script. **Conflicts with `zip_file`.**
"˜
scheduleN:L
J

syntheticsCanarySchedule,aws:synthetics/CanarySchedule:CanaryScheduleöConfiguration block providing how often the canary is to run and when these test runs are to stop. Detailed below.

The following arguments are optional:
"a
sourceLocationArn" HARN of the Lambda layer where Synthetics stores the canary script code.
"8
startCanaryB
 #Whether to run or stop the canary.
"
status" Canary status.
"ø
successRetentionPeriodB ûNumber of days to retain data about successful runs of this canary. If you omit this field, the default of 31 days is used. The valid range is 1 to 455 days.
"«
tagsB2" ∂Key-value map of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"’
	timelinesP*N:L
J

syntheticsCanaryTimeline,aws:synthetics/CanaryTimeline:CanaryTimelinevStructure that contains information about when the canary was created, modified, and most recently run. see Timeline.
"á
	vpcConfigSBQ:O
M

syntheticsCanaryVpcConfig.aws:synthetics/CanaryVpcConfig:CanaryVpcConfig%Configuration block. Detailed below.
"È
zipFileB" ◊ZIP file that contains the script, if you input your canary script directly into the canary instead of referring to an S3 location. It can be up to 225KB. **Conflicts with `s3_bucket`, `s3_key`, and `s3_version`.**
*ø
/

syntheticsGroupaws:synthetics/group:GroupêProvides a Synthetics Group resource.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.synthetics.Group("example", {name: "example"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.synthetics.Group("example", name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Synthetics.Group("example", new()
    {
        Name = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/synthetics"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := synthetics.NewGroup(ctx, "example", &synthetics.GroupArgs{
			Name: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.synthetics.Group;
import com.pulumi.aws.synthetics.GroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Group("example", GroupArgs.builder()
            .name("example")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:synthetics:Group
    properties:
      name: example
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CloudWatch Synthetics Group using the `name`. For example:

```sh
$ pulumi import aws:synthetics/group:Group example example
```
H
nameB" :Name of the group.

The following arguments are optional:
À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
arn" ARN of the Group.
" 
groupId" ID of the Group.
"F
name" :Name of the group.

The following arguments are optional:
"À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*Ê
P

syntheticsGroupAssociation0aws:synthetics/groupAssociation:GroupAssociation˜Provides a Synthetics Group Association resource.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.synthetics.GroupAssociation("example", {
    groupName: exampleAwsSyntheticsGroup.name,
    canaryArn: exampleAwsSyntheticsCanary.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.synthetics.GroupAssociation("example",
    group_name=example_aws_synthetics_group["name"],
    canary_arn=example_aws_synthetics_canary["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Synthetics.GroupAssociation("example", new()
    {
        GroupName = exampleAwsSyntheticsGroup.Name,
        CanaryArn = exampleAwsSyntheticsCanary.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/synthetics"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := synthetics.NewGroupAssociation(ctx, "example", &synthetics.GroupAssociationArgs{
			GroupName: pulumi.Any(exampleAwsSyntheticsGroup.Name),
			CanaryArn: pulumi.Any(exampleAwsSyntheticsCanary.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.synthetics.GroupAssociation;
import com.pulumi.aws.synthetics.GroupAssociationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new GroupAssociation("example", GroupAssociationArgs.builder()
            .groupName(exampleAwsSyntheticsGroup.name())
            .canaryArn(exampleAwsSyntheticsCanary.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:synthetics:GroupAssociation
    properties:
      groupName: ${exampleAwsSyntheticsGroup.name}
      canaryArn: ${exampleAwsSyntheticsCanary.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CloudWatch Synthetics Group Association using the `canary_arn,group_name`. For example:

```sh
$ pulumi import aws:synthetics/groupAssociation:GroupAssociation example arn:aws:synthetics:us-west-2:123456789012:canary:tf-acc-test-abcd1234,examplename
```
$
	canaryArn" ARN of the canary.
L
	groupName" ;Name of the group that the canary will be associated with.
"$
	canaryArn" ARN of the canary.
"
groupArn" " 
groupId" ID of the Group.
"L
	groupName" ;Name of the group that the canary will be associated with.
*Êª
N
timestreaminfluxdb
DbInstance,aws:timestreaminfluxdb/dbInstance:DbInstanceÉËResource for managing an Amazon Timestream for InfluxDB database instance.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.timestreaminfluxdb.DbInstance("example", {
    allocatedStorage: 20,
    bucket: "example-bucket-name",
    dbInstanceType: "db.influx.medium",
    username: "admin",
    password: "example-password",
    organization: "organization",
    vpcSubnetIds: [exampleid],
    vpcSecurityGroupIds: [exampleAwsSecurityGroup.id],
    name: "example-db-instance",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.timestreaminfluxdb.DbInstance("example",
    allocated_storage=20,
    bucket="example-bucket-name",
    db_instance_type="db.influx.medium",
    username="admin",
    password="example-password",
    organization="organization",
    vpc_subnet_ids=[exampleid],
    vpc_security_group_ids=[example_aws_security_group["id"]],
    name="example-db-instance")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.TimestreamInfluxDB.DbInstance("example", new()
    {
        AllocatedStorage = 20,
        Bucket = "example-bucket-name",
        DbInstanceType = "db.influx.medium",
        Username = "admin",
        Password = "example-password",
        Organization = "organization",
        VpcSubnetIds = new[]
        {
            exampleid,
        },
        VpcSecurityGroupIds = new[]
        {
            exampleAwsSecurityGroup.Id,
        },
        Name = "example-db-instance",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/timestreaminfluxdb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := timestreaminfluxdb.NewDbInstance(ctx, "example", &timestreaminfluxdb.DbInstanceArgs{
			AllocatedStorage: pulumi.Int(20),
			Bucket:           pulumi.String("example-bucket-name"),
			DbInstanceType:   pulumi.String("db.influx.medium"),
			Username:         pulumi.String("admin"),
			Password:         pulumi.String("example-password"),
			Organization:     pulumi.String("organization"),
			VpcSubnetIds: pulumi.StringArray{
				exampleid,
			},
			VpcSecurityGroupIds: pulumi.StringArray{
				exampleAwsSecurityGroup.Id,
			},
			Name: pulumi.String("example-db-instance"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.timestreaminfluxdb.DbInstance;
import com.pulumi.aws.timestreaminfluxdb.DbInstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new DbInstance("example", DbInstanceArgs.builder()
            .allocatedStorage(20)
            .bucket("example-bucket-name")
            .dbInstanceType("db.influx.medium")
            .username("admin")
            .password("example-password")
            .organization("organization")
            .vpcSubnetIds(exampleid)
            .vpcSecurityGroupIds(exampleAwsSecurityGroup.id())
            .name("example-db-instance")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:timestreaminfluxdb:DbInstance
    properties:
      allocatedStorage: 20
      bucket: example-bucket-name
      dbInstanceType: db.influx.medium
      username: admin
      password: example-password
      organization: organization
      vpcSubnetIds:
        - ${exampleid}
      vpcSecurityGroupIds:
        - ${exampleAwsSecurityGroup.id}
      name: example-db-instance
```
<!--End PulumiCodeChooser -->

### Usage with Prerequisite Resources

All Timestream for InfluxDB instances require a VPC, subnet, and security group. The following example shows how these prerequisite resources can be created and used with `aws.timestreaminfluxdb.DbInstance`.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.ec2.Vpc("example", {cidrBlock: "10.0.0.0/16"});
const exampleSubnet = new aws.ec2.Subnet("example", {
    vpcId: example.id,
    cidrBlock: "10.0.1.0/24",
});
const exampleSecurityGroup = new aws.ec2.SecurityGroup("example", {
    name: "example",
    vpcId: example.id,
});
const exampleDbInstance = new aws.timestreaminfluxdb.DbInstance("example", {
    allocatedStorage: 20,
    bucket: "example-bucket-name",
    dbInstanceType: "db.influx.medium",
    username: "admin",
    password: "example-password",
    organization: "organization",
    vpcSubnetIds: [exampleSubnet.id],
    vpcSecurityGroupIds: [exampleSecurityGroup.id],
    name: "example-db-instance",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ec2.Vpc("example", cidr_block="10.0.0.0/16")
example_subnet = aws.ec2.Subnet("example",
    vpc_id=example.id,
    cidr_block="10.0.1.0/24")
example_security_group = aws.ec2.SecurityGroup("example",
    name="example",
    vpc_id=example.id)
example_db_instance = aws.timestreaminfluxdb.DbInstance("example",
    allocated_storage=20,
    bucket="example-bucket-name",
    db_instance_type="db.influx.medium",
    username="admin",
    password="example-password",
    organization="organization",
    vpc_subnet_ids=[example_subnet.id],
    vpc_security_group_ids=[example_security_group.id],
    name="example-db-instance")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Ec2.Vpc("example", new()
    {
        CidrBlock = "10.0.0.0/16",
    });

    var exampleSubnet = new Aws.Ec2.Subnet("example", new()
    {
        VpcId = example.Id,
        CidrBlock = "10.0.1.0/24",
    });

    var exampleSecurityGroup = new Aws.Ec2.SecurityGroup("example", new()
    {
        Name = "example",
        VpcId = example.Id,
    });

    var exampleDbInstance = new Aws.TimestreamInfluxDB.DbInstance("example", new()
    {
        AllocatedStorage = 20,
        Bucket = "example-bucket-name",
        DbInstanceType = "db.influx.medium",
        Username = "admin",
        Password = "example-password",
        Organization = "organization",
        VpcSubnetIds = new[]
        {
            exampleSubnet.Id,
        },
        VpcSecurityGroupIds = new[]
        {
            exampleSecurityGroup.Id,
        },
        Name = "example-db-instance",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/timestreaminfluxdb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ec2.NewVpc(ctx, "example", &ec2.VpcArgs{
			CidrBlock: pulumi.String("10.0.0.0/16"),
		})
		if err != nil {
			return err
		}
		exampleSubnet, err := ec2.NewSubnet(ctx, "example", &ec2.SubnetArgs{
			VpcId:     example.ID(),
			CidrBlock: pulumi.String("10.0.1.0/24"),
		})
		if err != nil {
			return err
		}
		exampleSecurityGroup, err := ec2.NewSecurityGroup(ctx, "example", &ec2.SecurityGroupArgs{
			Name:  pulumi.String("example"),
			VpcId: example.ID(),
		})
		if err != nil {
			return err
		}
		_, err = timestreaminfluxdb.NewDbInstance(ctx, "example", &timestreaminfluxdb.DbInstanceArgs{
			AllocatedStorage: pulumi.Int(20),
			Bucket:           pulumi.String("example-bucket-name"),
			DbInstanceType:   pulumi.String("db.influx.medium"),
			Username:         pulumi.String("admin"),
			Password:         pulumi.String("example-password"),
			Organization:     pulumi.String("organization"),
			VpcSubnetIds: pulumi.StringArray{
				exampleSubnet.ID(),
			},
			VpcSecurityGroupIds: pulumi.StringArray{
				exampleSecurityGroup.ID(),
			},
			Name: pulumi.String("example-db-instance"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ec2.Vpc;
import com.pulumi.aws.ec2.VpcArgs;
import com.pulumi.aws.ec2.Subnet;
import com.pulumi.aws.ec2.SubnetArgs;
import com.pulumi.aws.ec2.SecurityGroup;
import com.pulumi.aws.ec2.SecurityGroupArgs;
import com.pulumi.aws.timestreaminfluxdb.DbInstance;
import com.pulumi.aws.timestreaminfluxdb.DbInstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Vpc("example", VpcArgs.builder()
            .cidrBlock("10.0.0.0/16")
            .build());

        var exampleSubnet = new Subnet("exampleSubnet", SubnetArgs.builder()
            .vpcId(example.id())
            .cidrBlock("10.0.1.0/24")
            .build());

        var exampleSecurityGroup = new SecurityGroup("exampleSecurityGroup", SecurityGroupArgs.builder()
            .name("example")
            .vpcId(example.id())
            .build());

        var exampleDbInstance = new DbInstance("exampleDbInstance", DbInstanceArgs.builder()
            .allocatedStorage(20)
            .bucket("example-bucket-name")
            .dbInstanceType("db.influx.medium")
            .username("admin")
            .password("example-password")
            .organization("organization")
            .vpcSubnetIds(exampleSubnet.id())
            .vpcSecurityGroupIds(exampleSecurityGroup.id())
            .name("example-db-instance")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:ec2:Vpc
    properties:
      cidrBlock: 10.0.0.0/16
  exampleSubnet:
    type: aws:ec2:Subnet
    name: example
    properties:
      vpcId: ${example.id}
      cidrBlock: 10.0.1.0/24
  exampleSecurityGroup:
    type: aws:ec2:SecurityGroup
    name: example
    properties:
      name: example
      vpcId: ${example.id}
  exampleDbInstance:
    type: aws:timestreaminfluxdb:DbInstance
    name: example
    properties:
      allocatedStorage: 20
      bucket: example-bucket-name
      dbInstanceType: db.influx.medium
      username: admin
      password: example-password
      organization: organization
      vpcSubnetIds:
        - ${exampleSubnet.id}
      vpcSecurityGroupIds:
        - ${exampleSecurityGroup.id}
      name: example-db-instance
```
<!--End PulumiCodeChooser -->

### Usage with S3 Log Delivery Enabled

You can use an S3 bucket to store logs generated by your Timestream for InfluxDB instance. The following example shows what resources and arguments are required to configure an S3 bucket for logging, including the IAM policy that needs to be set in order to allow Timestream for InfluxDB to place logs in your S3 bucket. The configuration of the required VPC, security group, and subnet have been left out of the example for brevity.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleBucketV2 = new aws.s3.BucketV2("example", {bucket: "example-s3-bucket"});
const example = aws.iam.getPolicyDocumentOutput({
    statements: [{
        actions: ["s3:PutObject"],
        principals: [{
            type: "Service",
            identifiers: ["timestream-influxdb.amazonaws.com"],
        }],
        resources: [pulumi.interpolate`${exampleBucketV2.arn}/*`],
    }],
});
const exampleBucketPolicy = new aws.s3.BucketPolicy("example", {
    bucket: exampleBucketV2.id,
    policy: example.apply(example => example.json),
});
const exampleDbInstance = new aws.timestreaminfluxdb.DbInstance("example", {
    allocatedStorage: 20,
    bucket: "example-bucket-name",
    dbInstanceType: "db.influx.medium",
    username: "admin",
    password: "example-password",
    organization: "organization",
    vpcSubnetIds: [exampleAwsSubnet.id],
    vpcSecurityGroupIds: [exampleAwsSecurityGroup.id],
    name: "example-db-instance",
    logDeliveryConfiguration: {
        s3Configuration: {
            bucketName: exampleBucketV2.name,
            enabled: true,
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example_bucket_v2 = aws.s3.BucketV2("example", bucket="example-s3-bucket")
example = aws.iam.get_policy_document_output(statements=[{
    "actions": ["s3:PutObject"],
    "principals": [{
        "type": "Service",
        "identifiers": ["timestream-influxdb.amazonaws.com"],
    }],
    "resources": [example_bucket_v2.arn.apply(lambda arn: f"{arn}/*")],
}])
example_bucket_policy = aws.s3.BucketPolicy("example",
    bucket=example_bucket_v2.id,
    policy=example.json)
example_db_instance = aws.timestreaminfluxdb.DbInstance("example",
    allocated_storage=20,
    bucket="example-bucket-name",
    db_instance_type="db.influx.medium",
    username="admin",
    password="example-password",
    organization="organization",
    vpc_subnet_ids=[example_aws_subnet["id"]],
    vpc_security_group_ids=[example_aws_security_group["id"]],
    name="example-db-instance",
    log_delivery_configuration={
        "s3_configuration": {
            "bucket_name": example_bucket_v2.name,
            "enabled": True,
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleBucketV2 = new Aws.S3.BucketV2("example", new()
    {
        Bucket = "example-s3-bucket",
    });

    var example = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "s3:PutObject",
                },
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "timestream-influxdb.amazonaws.com",
                        },
                    },
                },
                Resources = new[]
                {
                    $"{exampleBucketV2.Arn}/*",
                },
            },
        },
    });

    var exampleBucketPolicy = new Aws.S3.BucketPolicy("example", new()
    {
        Bucket = exampleBucketV2.Id,
        Policy = example.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var exampleDbInstance = new Aws.TimestreamInfluxDB.DbInstance("example", new()
    {
        AllocatedStorage = 20,
        Bucket = "example-bucket-name",
        DbInstanceType = "db.influx.medium",
        Username = "admin",
        Password = "example-password",
        Organization = "organization",
        VpcSubnetIds = new[]
        {
            exampleAwsSubnet.Id,
        },
        VpcSecurityGroupIds = new[]
        {
            exampleAwsSecurityGroup.Id,
        },
        Name = "example-db-instance",
        LogDeliveryConfiguration = new Aws.TimestreamInfluxDB.Inputs.DbInstanceLogDeliveryConfigurationArgs
        {
            S3Configuration = new Aws.TimestreamInfluxDB.Inputs.DbInstanceLogDeliveryConfigurationS3ConfigurationArgs
            {
                BucketName = exampleBucketV2.Name,
                Enabled = true,
            },
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/timestreaminfluxdb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleBucketV2, err := s3.NewBucketV2(ctx, "example", &s3.BucketV2Args{
			Bucket: pulumi.String("example-s3-bucket"),
		})
		if err != nil {
			return err
		}
		example := iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
			Statements: iam.GetPolicyDocumentStatementArray{
				&iam.GetPolicyDocumentStatementArgs{
					Actions: pulumi.StringArray{
						pulumi.String("s3:PutObject"),
					},
					Principals: iam.GetPolicyDocumentStatementPrincipalArray{
						&iam.GetPolicyDocumentStatementPrincipalArgs{
							Type: pulumi.String("Service"),
							Identifiers: pulumi.StringArray{
								pulumi.String("timestream-influxdb.amazonaws.com"),
							},
						},
					},
					Resources: pulumi.StringArray{
						exampleBucketV2.Arn.ApplyT(func(arn string) (string, error) {
							return fmt.Sprintf("%v/*", arn), nil
						}).(pulumi.StringOutput),
					},
				},
			},
		}, nil)
		_, err = s3.NewBucketPolicy(ctx, "example", &s3.BucketPolicyArgs{
			Bucket: exampleBucketV2.ID(),
			Policy: pulumi.String(example.ApplyT(func(example iam.GetPolicyDocumentResult) (*string, error) {
				return &example.Json, nil
			}).(pulumi.StringPtrOutput)),
		})
		if err != nil {
			return err
		}
		_, err = timestreaminfluxdb.NewDbInstance(ctx, "example", &timestreaminfluxdb.DbInstanceArgs{
			AllocatedStorage: pulumi.Int(20),
			Bucket:           pulumi.String("example-bucket-name"),
			DbInstanceType:   pulumi.String("db.influx.medium"),
			Username:         pulumi.String("admin"),
			Password:         pulumi.String("example-password"),
			Organization:     pulumi.String("organization"),
			VpcSubnetIds: pulumi.StringArray{
				exampleAwsSubnet.Id,
			},
			VpcSecurityGroupIds: pulumi.StringArray{
				exampleAwsSecurityGroup.Id,
			},
			Name: pulumi.String("example-db-instance"),
			LogDeliveryConfiguration: &timestreaminfluxdb.DbInstanceLogDeliveryConfigurationArgs{
				S3Configuration: &timestreaminfluxdb.DbInstanceLogDeliveryConfigurationS3ConfigurationArgs{
					BucketName: exampleBucketV2.Name,
					Enabled:    pulumi.Bool(true),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.s3.BucketPolicy;
import com.pulumi.aws.s3.BucketPolicyArgs;
import com.pulumi.aws.timestreaminfluxdb.DbInstance;
import com.pulumi.aws.timestreaminfluxdb.DbInstanceArgs;
import com.pulumi.aws.timestreaminfluxdb.inputs.DbInstanceLogDeliveryConfigurationArgs;
import com.pulumi.aws.timestreaminfluxdb.inputs.DbInstanceLogDeliveryConfigurationS3ConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var exampleBucketV2 = new BucketV2("exampleBucketV2", BucketV2Args.builder()
            .bucket("example-s3-bucket")
            .build());

        final var example = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions("s3:PutObject")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("timestream-influxdb.amazonaws.com")
                    .build())
                .resources(exampleBucketV2.arn().applyValue(arn -> String.format("%s/*", arn)))
                .build())
            .build());

        var exampleBucketPolicy = new BucketPolicy("exampleBucketPolicy", BucketPolicyArgs.builder()
            .bucket(exampleBucketV2.id())
            .policy(example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(example -> example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

        var exampleDbInstance = new DbInstance("exampleDbInstance", DbInstanceArgs.builder()
            .allocatedStorage(20)
            .bucket("example-bucket-name")
            .dbInstanceType("db.influx.medium")
            .username("admin")
            .password("example-password")
            .organization("organization")
            .vpcSubnetIds(exampleAwsSubnet.id())
            .vpcSecurityGroupIds(exampleAwsSecurityGroup.id())
            .name("example-db-instance")
            .logDeliveryConfiguration(DbInstanceLogDeliveryConfigurationArgs.builder()
                .s3Configuration(DbInstanceLogDeliveryConfigurationS3ConfigurationArgs.builder()
                    .bucketName(exampleBucketV2.name())
                    .enabled(true)
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  exampleBucketV2:
    type: aws:s3:BucketV2
    name: example
    properties:
      bucket: example-s3-bucket
  exampleBucketPolicy:
    type: aws:s3:BucketPolicy
    name: example
    properties:
      bucket: ${exampleBucketV2.id}
      policy: ${example.json}
  exampleDbInstance:
    type: aws:timestreaminfluxdb:DbInstance
    name: example
    properties:
      allocatedStorage: 20
      bucket: example-bucket-name
      dbInstanceType: db.influx.medium
      username: admin
      password: example-password
      organization: organization
      vpcSubnetIds:
        - ${exampleAwsSubnet.id}
      vpcSecurityGroupIds:
        - ${exampleAwsSecurityGroup.id}
      name: example-db-instance
      logDeliveryConfiguration:
        s3Configuration:
          bucketName: ${exampleBucketV2.name}
          enabled: true
variables:
  example:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - actions:
              - s3:PutObject
            principals:
              - type: Service
                identifiers:
                  - timestream-influxdb.amazonaws.com
            resources:
              - ${exampleBucketV2.arn}/*
```
<!--End PulumiCodeChooser -->

### Usage with MultiAZ Deployment

To use multi-region availability, at least two subnets must be created in different availability zones and used with your Timestream for InfluxDB instance.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example1 = new aws.ec2.Subnet("example_1", {
    vpcId: exampleAwsVpc.id,
    cidrBlock: "10.0.1.0/24",
    availabilityZone: "us-west-2a",
});
const example2 = new aws.ec2.Subnet("example_2", {
    vpcId: exampleAwsVpc.id,
    cidrBlock: "10.0.2.0/24",
    availabilityZone: "us-west-2b",
});
const example = new aws.timestreaminfluxdb.DbInstance("example", {
    allocatedStorage: 20,
    bucket: "example-bucket-name",
    dbInstanceType: "db.influx.medium",
    deploymentType: "WITH_MULTIAZ_STANDBY",
    username: "admin",
    password: "example-password",
    organization: "organization",
    vpcSubnetIds: [
        example1.id,
        example2.id,
    ],
    vpcSecurityGroupIds: [exampleAwsSecurityGroup.id],
    name: "example-db-instance",
});
```
```python
import pulumi
import pulumi_aws as aws

example1 = aws.ec2.Subnet("example_1",
    vpc_id=example_aws_vpc["id"],
    cidr_block="10.0.1.0/24",
    availability_zone="us-west-2a")
example2 = aws.ec2.Subnet("example_2",
    vpc_id=example_aws_vpc["id"],
    cidr_block="10.0.2.0/24",
    availability_zone="us-west-2b")
example = aws.timestreaminfluxdb.DbInstance("example",
    allocated_storage=20,
    bucket="example-bucket-name",
    db_instance_type="db.influx.medium",
    deployment_type="WITH_MULTIAZ_STANDBY",
    username="admin",
    password="example-password",
    organization="organization",
    vpc_subnet_ids=[
        example1.id,
        example2.id,
    ],
    vpc_security_group_ids=[example_aws_security_group["id"]],
    name="example-db-instance")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example1 = new Aws.Ec2.Subnet("example_1", new()
    {
        VpcId = exampleAwsVpc.Id,
        CidrBlock = "10.0.1.0/24",
        AvailabilityZone = "us-west-2a",
    });

    var example2 = new Aws.Ec2.Subnet("example_2", new()
    {
        VpcId = exampleAwsVpc.Id,
        CidrBlock = "10.0.2.0/24",
        AvailabilityZone = "us-west-2b",
    });

    var example = new Aws.TimestreamInfluxDB.DbInstance("example", new()
    {
        AllocatedStorage = 20,
        Bucket = "example-bucket-name",
        DbInstanceType = "db.influx.medium",
        DeploymentType = "WITH_MULTIAZ_STANDBY",
        Username = "admin",
        Password = "example-password",
        Organization = "organization",
        VpcSubnetIds = new[]
        {
            example1.Id,
            example2.Id,
        },
        VpcSecurityGroupIds = new[]
        {
            exampleAwsSecurityGroup.Id,
        },
        Name = "example-db-instance",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/timestreaminfluxdb"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example1, err := ec2.NewSubnet(ctx, "example_1", &ec2.SubnetArgs{
			VpcId:            pulumi.Any(exampleAwsVpc.Id),
			CidrBlock:        pulumi.String("10.0.1.0/24"),
			AvailabilityZone: pulumi.String("us-west-2a"),
		})
		if err != nil {
			return err
		}
		example2, err := ec2.NewSubnet(ctx, "example_2", &ec2.SubnetArgs{
			VpcId:            pulumi.Any(exampleAwsVpc.Id),
			CidrBlock:        pulumi.String("10.0.2.0/24"),
			AvailabilityZone: pulumi.String("us-west-2b"),
		})
		if err != nil {
			return err
		}
		_, err = timestreaminfluxdb.NewDbInstance(ctx, "example", &timestreaminfluxdb.DbInstanceArgs{
			AllocatedStorage: pulumi.Int(20),
			Bucket:           pulumi.String("example-bucket-name"),
			DbInstanceType:   pulumi.String("db.influx.medium"),
			DeploymentType:   pulumi.String("WITH_MULTIAZ_STANDBY"),
			Username:         pulumi.String("admin"),
			Password:         pulumi.String("example-password"),
			Organization:     pulumi.String("organization"),
			VpcSubnetIds: pulumi.StringArray{
				example1.ID(),
				example2.ID(),
			},
			VpcSecurityGroupIds: pulumi.StringArray{
				exampleAwsSecurityGroup.Id,
			},
			Name: pulumi.String("example-db-instance"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ec2.Subnet;
import com.pulumi.aws.ec2.SubnetArgs;
import com.pulumi.aws.timestreaminfluxdb.DbInstance;
import com.pulumi.aws.timestreaminfluxdb.DbInstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example1 = new Subnet("example1", SubnetArgs.builder()
            .vpcId(exampleAwsVpc.id())
            .cidrBlock("10.0.1.0/24")
            .availabilityZone("us-west-2a")
            .build());

        var example2 = new Subnet("example2", SubnetArgs.builder()
            .vpcId(exampleAwsVpc.id())
            .cidrBlock("10.0.2.0/24")
            .availabilityZone("us-west-2b")
            .build());

        var example = new DbInstance("example", DbInstanceArgs.builder()
            .allocatedStorage(20)
            .bucket("example-bucket-name")
            .dbInstanceType("db.influx.medium")
            .deploymentType("WITH_MULTIAZ_STANDBY")
            .username("admin")
            .password("example-password")
            .organization("organization")
            .vpcSubnetIds(            
                example1.id(),
                example2.id())
            .vpcSecurityGroupIds(exampleAwsSecurityGroup.id())
            .name("example-db-instance")
            .build());

    }
}
```
```yaml
resources:
  example1:
    type: aws:ec2:Subnet
    name: example_1
    properties:
      vpcId: ${exampleAwsVpc.id}
      cidrBlock: 10.0.1.0/24
      availabilityZone: us-west-2a
  example2:
    type: aws:ec2:Subnet
    name: example_2
    properties:
      vpcId: ${exampleAwsVpc.id}
      cidrBlock: 10.0.2.0/24
      availabilityZone: us-west-2b
  example:
    type: aws:timestreaminfluxdb:DbInstance
    properties:
      allocatedStorage: 20
      bucket: example-bucket-name
      dbInstanceType: db.influx.medium
      deploymentType: WITH_MULTIAZ_STANDBY
      username: admin
      password: example-password
      organization: organization
      vpcSubnetIds:
        - ${example1.id}
        - ${example2.id}
      vpcSecurityGroupIds:
        - ${exampleAwsSecurityGroup.id}
      name: example-db-instance
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Timestream for InfluxDB Db Instance using its identifier. For example:

```sh
$ pulumi import aws:timestreaminfluxdb/dbInstance:DbInstance example 12345abcde
```
s
allocatedStorage [Amount of storage in GiB (gibibytes). The minimum value is 20, the maximum value is 16384.
ü
bucket" êName of the initial InfluxDB bucket. All InfluxDB data is stored in a bucket. A bucket combines the concept of a database and a retention period (the duration of time that each data point persists). A bucket belongs to an organization. Along with `organization`, `username`, and `password`, this argument will be stored in the secret referred to by the `influx_auth_parameters_secret_arn` attribute.
°
dbInstanceType" äTimestream for InfluxDB DB instance type to run InfluxDB on. Valid options are: `"db.influx.medium"`, `"db.influx.large"`, `"db.influx.xlarge"`, `"db.influx.2xlarge"`, `"db.influx.4xlarge"`, `"db.influx.8xlarge"`, `"db.influx.12xlarge"`, and `"db.influx.16xlarge"`.
è
dbParameterGroupIdentifierB" ÍID of the DB parameter group assigned to your DB instance. If added to an existing Timestream for InfluxDB instance or given a new value, will cause an in-place update to the instance. However, if an instance already has a value for `db_parameter_group_identifier`, removing `db_parameter_group_identifier` will cause the instance to be destroyed and recreated.
á
dbStorageTypeB" ÔTimestream for InfluxDB DB storage type to read and write InfluxDB data. You can choose between 3 different types of provisioned Influx IOPS included storage according to your workloads requirements: Influx IO Included 3000 IOPS, Influx IO Included 12000 IOPS, Influx IO Included 16000 IOPS. Valid options are: `"InfluxIOIncludedT1"`, `"InfluxIOIncludedT2"`, and `"InfluxIOIncludedT1"`. If you use `"InfluxIOIncludedT2" or "InfluxIOIncludedT3", the minimum value for `allocated_storage` is 400.
”
deploymentTypeB" ∫Specifies whether the DB instance will be deployed as a standalone instance or with a Multi-AZ standby for high availability. Valid options are: `"SINGLE_AZ"`, `"WITH_MULTIAZ_STANDBY"`.
á
logDeliveryConfigurationüBú:ô
ñ
timestreaminfluxdb"DbInstanceLogDeliveryConfiguration\aws:timestreaminfluxdb/DbInstanceLogDeliveryConfiguration:DbInstanceLogDeliveryConfigurationIConfiguration for sending InfluxDB engine logs to a specified S3 bucket.
Ú
nameB" „Name that uniquely identifies the DB instance when interacting with the Amazon Timestream for InfluxDB API and CLI commands. This name will also be a prefix included in the endpoint. DB instance names must be unique per customer and per region. The argument must start with a letter, cannot contain consecutive hyphens (`-`) and cannot end with a hyphen.
∫
organization" •Name of the initial organization for the initial admin user in InfluxDB. An InfluxDB organization is a workspace for a group of users. Along with `bucket`, `username`, and `password`, this argument will be stored in the secret referred to by the `influx_auth_parameters_secret_arn` attribute.
Ö
password" ÙPassword of the initial admin user created in InfluxDB. This password will allow you to access the InfluxDB UI to perform various administrative tasks and also use the InfluxDB CLI to create an operator token. Along with `bucket`, `username`, and `organization`, this argument will be stored in the secret referred to by the `influx_auth_parameters_secret_arn` attribute.
å
publiclyAccessibleB
 ÔConfigures the DB instance with a public IP to facilitate access. Other resources, such as a VPC, a subnet, an internet gateway, and a route table with routes, are also required to enabled public access, in addition to this argument. See "Usage with Public Internet Access Enabled" for an example configuration with all required resources for public internet access.
Õ
tagsB2" ºMap of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
x
timeoutslBj:h
f
timestreaminfluxdbDbInstanceTimeouts<aws:timestreaminfluxdb/DbInstanceTimeouts:DbInstanceTimeoutsﬁ
username" ÕUsername of the initial admin user created in InfluxDB. Must start with a letter and can't end with a hyphen or contain two consecutive hyphens. This username will allow you to access the InfluxDB UI to perform various administrative tasks and also use the InfluxDB CLI to create an operator token. Along with `bucket`, `organization`, and `password`, this argument will be stored in the secret referred to by the `influx_auth_parameters_secret_arn` attribute.
_
vpcSecurityGroupIds*" BList of VPC security group IDs to associate with the DB instance.
‰
vpcSubnetIds*" ÕList of VPC subnet IDs to associate with the DB instance. Provide at least two VPC subnet IDs in different availability zones when deploying with a Multi-AZ standby.

The following arguments are optional:
"s
allocatedStorage [Amount of storage in GiB (gibibytes). The minimum value is 20, the maximum value is 16384.
"8
arn" -ARN of the Timestream for InfluxDB Instance.
"L
availabilityZone" 4Availability Zone in which the DB instance resides.
"ü
bucket" êName of the initial InfluxDB bucket. All InfluxDB data is stored in a bucket. A bucket combines the concept of a database and a retention period (the duration of time that each data point persists). A bucket belongs to an organization. Along with `organization`, `username`, and `password`, this argument will be stored in the secret referred to by the `influx_auth_parameters_secret_arn` attribute.
"°
dbInstanceType" äTimestream for InfluxDB DB instance type to run InfluxDB on. Valid options are: `"db.influx.medium"`, `"db.influx.large"`, `"db.influx.xlarge"`, `"db.influx.2xlarge"`, `"db.influx.4xlarge"`, `"db.influx.8xlarge"`, `"db.influx.12xlarge"`, and `"db.influx.16xlarge"`.
"è
dbParameterGroupIdentifierB" ÍID of the DB parameter group assigned to your DB instance. If added to an existing Timestream for InfluxDB instance or given a new value, will cause an in-place update to the instance. However, if an instance already has a value for `db_parameter_group_identifier`, removing `db_parameter_group_identifier` will cause the instance to be destroyed and recreated.
"Ö
dbStorageType" ÔTimestream for InfluxDB DB storage type to read and write InfluxDB data. You can choose between 3 different types of provisioned Influx IOPS included storage according to your workloads requirements: Influx IO Included 3000 IOPS, Influx IO Included 12000 IOPS, Influx IO Included 16000 IOPS. Valid options are: `"InfluxIOIncludedT1"`, `"InfluxIOIncludedT2"`, and `"InfluxIOIncludedT1"`. If you use `"InfluxIOIncludedT2" or "InfluxIOIncludedT3", the minimum value for `allocated_storage` is 400.
"—
deploymentType" ∫Specifies whether the DB instance will be deployed as a standalone instance or with a Multi-AZ standby for high availability. Valid options are: `"SINGLE_AZ"`, `"WITH_MULTIAZ_STANDBY"`.
"Y
endpoint" IEndpoint used to connect to InfluxDB. The default InfluxDB port is 8086.
"≤
influxAuthParametersSecretArn" åARN of the AWS Secrets Manager secret containing the initial InfluxDB authorization parameters. The secret value is a JSON formatted key-value pair holding InfluxDB authorization values: organization, bucket, username, and password. This secret will be read by the `aws.timestreaminfluxdb.DbInstance` resource in order to support importing: deleting the secret or secret values can cause errors.
"á
logDeliveryConfigurationüBú:ô
ñ
timestreaminfluxdb"DbInstanceLogDeliveryConfiguration\aws:timestreaminfluxdb/DbInstanceLogDeliveryConfiguration:DbInstanceLogDeliveryConfigurationIConfiguration for sending InfluxDB engine logs to a specified S3 bucket.
"
name" „Name that uniquely identifies the DB instance when interacting with the Amazon Timestream for InfluxDB API and CLI commands. This name will also be a prefix included in the endpoint. DB instance names must be unique per customer and per region. The argument must start with a letter, cannot contain consecutive hyphens (`-`) and cannot end with a hyphen.
"∫
organization" •Name of the initial organization for the initial admin user in InfluxDB. An InfluxDB organization is a workspace for a group of users. Along with `bucket`, `username`, and `password`, this argument will be stored in the secret referred to by the `influx_auth_parameters_secret_arn` attribute.
"Ö
password" ÙPassword of the initial admin user created in InfluxDB. This password will allow you to access the InfluxDB UI to perform various administrative tasks and also use the InfluxDB CLI to create an operator token. Along with `bucket`, `username`, and `organization`, this argument will be stored in the secret referred to by the `influx_auth_parameters_secret_arn` attribute.
"ä
publiclyAccessible
 ÔConfigures the DB instance with a public IP to facilitate access. Other resources, such as a VPC, a subnet, an internet gateway, and a route table with routes, are also required to enabled public access, in addition to this argument. See "Usage with Public Internet Access Enabled" for an example configuration with all required resources for public internet access.
"å
secondaryAvailabilityZone" kAvailability Zone in which the standby instance is located when deploying with a MultiAZ standby instance.
"Õ
tagsB2" ºMap of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"x
timeoutslBj:h
f
timestreaminfluxdbDbInstanceTimeouts<aws:timestreaminfluxdb/DbInstanceTimeouts:DbInstanceTimeouts"ﬁ
username" ÕUsername of the initial admin user created in InfluxDB. Must start with a letter and can't end with a hyphen or contain two consecutive hyphens. This username will allow you to access the InfluxDB UI to perform various administrative tasks and also use the InfluxDB CLI to create an operator token. Along with `bucket`, `organization`, and `password`, this argument will be stored in the secret referred to by the `influx_auth_parameters_secret_arn` attribute.
"_
vpcSecurityGroupIds*" BList of VPC security group IDs to associate with the DB instance.
"‰
vpcSubnetIds*" ÕList of VPC subnet IDs to associate with the DB instance. Provide at least two VPC subnet IDs in different availability zones when deploying with a Multi-AZ standby.

The following arguments are optional:
*†1
B
timestreamwriteDatabase%aws:timestreamwrite/database:DatabaseÁ#Provides a Timestream database resource.

## Example Usage

### Basic usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.timestreamwrite.Database("example", {databaseName: "database-example"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.timestreamwrite.Database("example", database_name="database-example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.TimestreamWrite.Database("example", new()
    {
        DatabaseName = "database-example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/timestreamwrite"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := timestreamwrite.NewDatabase(ctx, "example", &timestreamwrite.DatabaseArgs{
			DatabaseName: pulumi.String("database-example"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.timestreamwrite.Database;
import com.pulumi.aws.timestreamwrite.DatabaseArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Database("example", DatabaseArgs.builder()
            .databaseName("database-example")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:timestreamwrite:Database
    properties:
      databaseName: database-example
```
<!--End PulumiCodeChooser -->

### Full usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.timestreamwrite.Database("example", {
    databaseName: "database-example",
    kmsKeyId: exampleAwsKmsKey.arn,
    tags: {
        Name: "value",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.timestreamwrite.Database("example",
    database_name="database-example",
    kms_key_id=example_aws_kms_key["arn"],
    tags={
        "Name": "value",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.TimestreamWrite.Database("example", new()
    {
        DatabaseName = "database-example",
        KmsKeyId = exampleAwsKmsKey.Arn,
        Tags = 
        {
            { "Name", "value" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/timestreamwrite"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := timestreamwrite.NewDatabase(ctx, "example", &timestreamwrite.DatabaseArgs{
			DatabaseName: pulumi.String("database-example"),
			KmsKeyId:     pulumi.Any(exampleAwsKmsKey.Arn),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("value"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.timestreamwrite.Database;
import com.pulumi.aws.timestreamwrite.DatabaseArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Database("example", DatabaseArgs.builder()
            .databaseName("database-example")
            .kmsKeyId(exampleAwsKmsKey.arn())
            .tags(Map.of("Name", "value"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:timestreamwrite:Database
    properties:
      databaseName: database-example
      kmsKeyId: ${exampleAwsKmsKey.arn}
      tags:
        Name: value
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Timestream databases using the `database_name`. For example:

```sh
$ pulumi import aws:timestreamwrite/database:Database example example
```
d
databaseName" PThe name of the Timestream database. Minimum length of 3. Maximum length of 64.

kmsKeyIdB" ›The ARN (not Alias ARN) of the KMS key to be used to encrypt the data stored in the database. If the KMS key is not specified, the database will be encrypted with a Timestream managed KMS key located in your account. Refer to [AWS managed KMS keys](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk) for more info.
œ
tagsB2" æMap of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
";
arn" 0The ARN that uniquely identifies this database.
"d
databaseName" PThe name of the Timestream database. Minimum length of 3. Maximum length of 64.
"Ó
kmsKeyId" ›The ARN (not Alias ARN) of the KMS key to be used to encrypt the data stored in the database. If the KMS key is not specified, the database will be encrypted with a Timestream managed KMS key located in your account. Refer to [AWS managed KMS keys](https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#aws-managed-cmk) for more info.
"S

tableCount AThe total number of tables found within the Timestream database.
"œ
tagsB2" æMap of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*¥c
9
timestreamwriteTableaws:timestreamwrite/table:Table”NProvides a Timestream table resource.

## Example Usage

### Basic usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.timestreamwrite.Table("example", {
    databaseName: exampleAwsTimestreamwriteDatabase.databaseName,
    tableName: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.timestreamwrite.Table("example",
    database_name=example_aws_timestreamwrite_database["databaseName"],
    table_name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.TimestreamWrite.Table("example", new()
    {
        DatabaseName = exampleAwsTimestreamwriteDatabase.DatabaseName,
        TableName = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/timestreamwrite"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := timestreamwrite.NewTable(ctx, "example", &timestreamwrite.TableArgs{
			DatabaseName: pulumi.Any(exampleAwsTimestreamwriteDatabase.DatabaseName),
			TableName:    pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.timestreamwrite.Table;
import com.pulumi.aws.timestreamwrite.TableArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Table("example", TableArgs.builder()
            .databaseName(exampleAwsTimestreamwriteDatabase.databaseName())
            .tableName("example")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:timestreamwrite:Table
    properties:
      databaseName: ${exampleAwsTimestreamwriteDatabase.databaseName}
      tableName: example
```
<!--End PulumiCodeChooser -->

### Full usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.timestreamwrite.Table("example", {
    databaseName: exampleAwsTimestreamwriteDatabase.databaseName,
    tableName: "example",
    retentionProperties: {
        magneticStoreRetentionPeriodInDays: 30,
        memoryStoreRetentionPeriodInHours: 8,
    },
    tags: {
        Name: "example-timestream-table",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.timestreamwrite.Table("example",
    database_name=example_aws_timestreamwrite_database["databaseName"],
    table_name="example",
    retention_properties={
        "magnetic_store_retention_period_in_days": 30,
        "memory_store_retention_period_in_hours": 8,
    },
    tags={
        "Name": "example-timestream-table",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.TimestreamWrite.Table("example", new()
    {
        DatabaseName = exampleAwsTimestreamwriteDatabase.DatabaseName,
        TableName = "example",
        RetentionProperties = new Aws.TimestreamWrite.Inputs.TableRetentionPropertiesArgs
        {
            MagneticStoreRetentionPeriodInDays = 30,
            MemoryStoreRetentionPeriodInHours = 8,
        },
        Tags = 
        {
            { "Name", "example-timestream-table" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/timestreamwrite"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := timestreamwrite.NewTable(ctx, "example", &timestreamwrite.TableArgs{
			DatabaseName: pulumi.Any(exampleAwsTimestreamwriteDatabase.DatabaseName),
			TableName:    pulumi.String("example"),
			RetentionProperties: &timestreamwrite.TableRetentionPropertiesArgs{
				MagneticStoreRetentionPeriodInDays: pulumi.Int(30),
				MemoryStoreRetentionPeriodInHours:  pulumi.Int(8),
			},
			Tags: pulumi.StringMap{
				"Name": pulumi.String("example-timestream-table"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.timestreamwrite.Table;
import com.pulumi.aws.timestreamwrite.TableArgs;
import com.pulumi.aws.timestreamwrite.inputs.TableRetentionPropertiesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Table("example", TableArgs.builder()
            .databaseName(exampleAwsTimestreamwriteDatabase.databaseName())
            .tableName("example")
            .retentionProperties(TableRetentionPropertiesArgs.builder()
                .magneticStoreRetentionPeriodInDays(30)
                .memoryStoreRetentionPeriodInHours(8)
                .build())
            .tags(Map.of("Name", "example-timestream-table"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:timestreamwrite:Table
    properties:
      databaseName: ${exampleAwsTimestreamwriteDatabase.databaseName}
      tableName: example
      retentionProperties:
        magneticStoreRetentionPeriodInDays: 30
        memoryStoreRetentionPeriodInHours: 8
      tags:
        Name: example-timestream-table
```
<!--End PulumiCodeChooser -->

### Customer-defined Partition Key

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.timestreamwrite.Table("example", {
    databaseName: exampleAwsTimestreamwriteDatabase.databaseName,
    tableName: "example",
    schema: {
        compositePartitionKey: {
            enforcementInRecord: "REQUIRED",
            name: "attr1",
            type: "DIMENSION",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.timestreamwrite.Table("example",
    database_name=example_aws_timestreamwrite_database["databaseName"],
    table_name="example",
    schema={
        "composite_partition_key": {
            "enforcement_in_record": "REQUIRED",
            "name": "attr1",
            "type": "DIMENSION",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.TimestreamWrite.Table("example", new()
    {
        DatabaseName = exampleAwsTimestreamwriteDatabase.DatabaseName,
        TableName = "example",
        Schema = new Aws.TimestreamWrite.Inputs.TableSchemaArgs
        {
            CompositePartitionKey = new Aws.TimestreamWrite.Inputs.TableSchemaCompositePartitionKeyArgs
            {
                EnforcementInRecord = "REQUIRED",
                Name = "attr1",
                Type = "DIMENSION",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/timestreamwrite"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := timestreamwrite.NewTable(ctx, "example", &timestreamwrite.TableArgs{
			DatabaseName: pulumi.Any(exampleAwsTimestreamwriteDatabase.DatabaseName),
			TableName:    pulumi.String("example"),
			Schema: &timestreamwrite.TableSchemaArgs{
				CompositePartitionKey: &timestreamwrite.TableSchemaCompositePartitionKeyArgs{
					EnforcementInRecord: pulumi.String("REQUIRED"),
					Name:                pulumi.String("attr1"),
					Type:                pulumi.String("DIMENSION"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.timestreamwrite.Table;
import com.pulumi.aws.timestreamwrite.TableArgs;
import com.pulumi.aws.timestreamwrite.inputs.TableSchemaArgs;
import com.pulumi.aws.timestreamwrite.inputs.TableSchemaCompositePartitionKeyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Table("example", TableArgs.builder()
            .databaseName(exampleAwsTimestreamwriteDatabase.databaseName())
            .tableName("example")
            .schema(TableSchemaArgs.builder()
                .compositePartitionKey(TableSchemaCompositePartitionKeyArgs.builder()
                    .enforcementInRecord("REQUIRED")
                    .name("attr1")
                    .type("DIMENSION")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:timestreamwrite:Table
    properties:
      databaseName: ${exampleAwsTimestreamwriteDatabase.databaseName}
      tableName: example
      schema:
        compositePartitionKey:
          enforcementInRecord: REQUIRED
          name: attr1
          type: DIMENSION
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Timestream tables using the `table_name` and `database_name` separate by a colon (`:`). For example:

```sh
$ pulumi import aws:timestreamwrite/table:Table example ExampleTable:ExampleDatabase
```
9
databaseName" %The name of the Timestream database.
√
magneticStoreWritePropertiesñBì:ê
ç
timestreamwrite!TableMagneticStoreWritePropertiesWaws:timestreamwrite/TableMagneticStoreWriteProperties:TableMagneticStoreWritePropertiesâContains properties to set on the table when enabling magnetic store writes. See Magnetic Store Write Properties below for more details.
ã
retentionPropertiesxBv:t
r
timestreamwriteTableRetentionPropertiesEaws:timestreamwrite/TableRetentionProperties:TableRetentionProperties˘The retention duration for the memory store and magnetic store. See Retention Properties below for more details. If not provided, `magnetic_store_retention_period_in_days` default to 73000 and `memory_store_retention_period_in_hours` defaults to 6.
ô
schemaQBO:M
K
timestreamwriteTableSchema+aws:timestreamwrite/TableSchema:TableSchema<The schema of the table. See Schema below for more details.
3
	tableName" "The name of the Timestream table.
œ
tagsB2" æMap of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"8
arn" -The ARN that uniquely identifies this table.
"9
databaseName" %The name of the Timestream database.
"¿
magneticStoreWritePropertiesì:ê
ç
timestreamwrite!TableMagneticStoreWritePropertiesWaws:timestreamwrite/TableMagneticStoreWriteProperties:TableMagneticStoreWritePropertiesâContains properties to set on the table when enabling magnetic store writes. See Magnetic Store Write Properties below for more details.
"â
retentionPropertiesv:t
r
timestreamwriteTableRetentionPropertiesEaws:timestreamwrite/TableRetentionProperties:TableRetentionProperties˘The retention duration for the memory store and magnetic store. See Retention Properties below for more details. If not provided, `magnetic_store_retention_period_in_days` default to 73000 and `memory_store_retention_period_in_hours` defaults to 6.
"ó
schemaO:M
K
timestreamwriteTableSchema+aws:timestreamwrite/TableSchema:TableSchema<The schema of the table. See Schema below for more details.
"3
	tableName" "The name of the Timestream table.
"œ
tagsB2" æMap of tags to assign to this resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*Îs
G

transcribeLanguageModel*aws:transcribe/languageModel:LanguageModelüjResource for managing an AWS Transcribe LanguageModel.

> This resource can take a significant amount of time to provision. See Language Model [FAQ](https://aws.amazon.com/transcribe/faqs/) for more details.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.iam.getPolicyDocument({
    statements: [{
        actions: ["sts:AssumeRole"],
        principals: [{
            type: "Service",
            identifiers: ["transcribe.amazonaws.com"],
        }],
    }],
});
const exampleRole = new aws.iam.Role("example", {
    name: "example",
    assumeRolePolicy: example.then(example => example.json),
});
const testPolicy = new aws.iam.RolePolicy("test_policy", {
    name: "example",
    role: exampleRole.id,
    policy: JSON.stringify({
        Version: "2012-10-17",
        Statement: [{
            Action: [
                "s3:GetObject",
                "s3:ListBucket",
            ],
            Effect: "Allow",
            Resource: ["*"],
        }],
    }),
});
const exampleBucketV2 = new aws.s3.BucketV2("example", {
    bucket: "example-transcribe",
    forceDestroy: true,
});
const object = new aws.s3.BucketObjectv2("object", {
    bucket: exampleBucketV2.id,
    key: "transcribe/test1.txt",
    source: new pulumi.asset.FileAsset("test1.txt"),
});
const exampleLanguageModel = new aws.transcribe.LanguageModel("example", {
    modelName: "example",
    baseModelName: "NarrowBand",
    inputDataConfig: {
        dataAccessRoleArn: exampleRole.arn,
        s3Uri: pulumi.interpolate`s3://${exampleBucketV2.id}/transcribe/`,
    },
    languageCode: "en-US",
    tags: {
        ENVIRONMENT: "development",
    },
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.iam.get_policy_document(statements=[{
    "actions": ["sts:AssumeRole"],
    "principals": [{
        "type": "Service",
        "identifiers": ["transcribe.amazonaws.com"],
    }],
}])
example_role = aws.iam.Role("example",
    name="example",
    assume_role_policy=example.json)
test_policy = aws.iam.RolePolicy("test_policy",
    name="example",
    role=example_role.id,
    policy=json.dumps({
        "Version": "2012-10-17",
        "Statement": [{
            "Action": [
                "s3:GetObject",
                "s3:ListBucket",
            ],
            "Effect": "Allow",
            "Resource": ["*"],
        }],
    }))
example_bucket_v2 = aws.s3.BucketV2("example",
    bucket="example-transcribe",
    force_destroy=True)
object = aws.s3.BucketObjectv2("object",
    bucket=example_bucket_v2.id,
    key="transcribe/test1.txt",
    source=pulumi.FileAsset("test1.txt"))
example_language_model = aws.transcribe.LanguageModel("example",
    model_name="example",
    base_model_name="NarrowBand",
    input_data_config={
        "data_access_role_arn": example_role.arn,
        "s3_uri": example_bucket_v2.id.apply(lambda id: f"s3://{id}/transcribe/"),
    },
    language_code="en-US",
    tags={
        "ENVIRONMENT": "development",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "transcribe.amazonaws.com",
                        },
                    },
                },
            },
        },
    });

    var exampleRole = new Aws.Iam.Role("example", new()
    {
        Name = "example",
        AssumeRolePolicy = example.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var testPolicy = new Aws.Iam.RolePolicy("test_policy", new()
    {
        Name = "example",
        Role = exampleRole.Id,
        Policy = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Version"] = "2012-10-17",
            ["Statement"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["Action"] = new[]
                    {
                        "s3:GetObject",
                        "s3:ListBucket",
                    },
                    ["Effect"] = "Allow",
                    ["Resource"] = new[]
                    {
                        "*",
                    },
                },
            },
        }),
    });

    var exampleBucketV2 = new Aws.S3.BucketV2("example", new()
    {
        Bucket = "example-transcribe",
        ForceDestroy = true,
    });

    var @object = new Aws.S3.BucketObjectv2("object", new()
    {
        Bucket = exampleBucketV2.Id,
        Key = "transcribe/test1.txt",
        Source = new FileAsset("test1.txt"),
    });

    var exampleLanguageModel = new Aws.Transcribe.LanguageModel("example", new()
    {
        ModelName = "example",
        BaseModelName = "NarrowBand",
        InputDataConfig = new Aws.Transcribe.Inputs.LanguageModelInputDataConfigArgs
        {
            DataAccessRoleArn = exampleRole.Arn,
            S3Uri = exampleBucketV2.Id.Apply(id => $"s3://{id}/transcribe/"),
        },
        LanguageCode = "en-US",
        Tags = 
        {
            { "ENVIRONMENT", "development" },
        },
    });

});
```
```go
package main

import (
	"encoding/json"
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transcribe"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Actions: []string{
						"sts:AssumeRole",
					},
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"transcribe.amazonaws.com",
							},
						},
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		exampleRole, err := iam.NewRole(ctx, "example", &iam.RoleArgs{
			Name:             pulumi.String("example"),
			AssumeRolePolicy: pulumi.String(example.Json),
		})
		if err != nil {
			return err
		}
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Version": "2012-10-17",
			"Statement": []map[string]interface{}{
				map[string]interface{}{
					"Action": []string{
						"s3:GetObject",
						"s3:ListBucket",
					},
					"Effect": "Allow",
					"Resource": []string{
						"*",
					},
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = iam.NewRolePolicy(ctx, "test_policy", &iam.RolePolicyArgs{
			Name:   pulumi.String("example"),
			Role:   exampleRole.ID(),
			Policy: pulumi.String(json0),
		})
		if err != nil {
			return err
		}
		exampleBucketV2, err := s3.NewBucketV2(ctx, "example", &s3.BucketV2Args{
			Bucket:       pulumi.String("example-transcribe"),
			ForceDestroy: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		_, err = s3.NewBucketObjectv2(ctx, "object", &s3.BucketObjectv2Args{
			Bucket: exampleBucketV2.ID(),
			Key:    pulumi.String("transcribe/test1.txt"),
			Source: pulumi.NewFileAsset("test1.txt"),
		})
		if err != nil {
			return err
		}
		_, err = transcribe.NewLanguageModel(ctx, "example", &transcribe.LanguageModelArgs{
			ModelName:     pulumi.String("example"),
			BaseModelName: pulumi.String("NarrowBand"),
			InputDataConfig: &transcribe.LanguageModelInputDataConfigArgs{
				DataAccessRoleArn: exampleRole.Arn,
				S3Uri: exampleBucketV2.ID().ApplyT(func(id string) (string, error) {
					return fmt.Sprintf("s3://%v/transcribe/", id), nil
				}).(pulumi.StringOutput),
			},
			LanguageCode: pulumi.String("en-US"),
			Tags: pulumi.StringMap{
				"ENVIRONMENT": pulumi.String("development"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.s3.BucketObjectv2;
import com.pulumi.aws.s3.BucketObjectv2Args;
import com.pulumi.aws.transcribe.LanguageModel;
import com.pulumi.aws.transcribe.LanguageModelArgs;
import com.pulumi.aws.transcribe.inputs.LanguageModelInputDataConfigArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import com.pulumi.asset.FileAsset;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions("sts:AssumeRole")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("transcribe.amazonaws.com")
                    .build())
                .build())
            .build());

        var exampleRole = new Role("exampleRole", RoleArgs.builder()
            .name("example")
            .assumeRolePolicy(example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var testPolicy = new RolePolicy("testPolicy", RolePolicyArgs.builder()
            .name("example")
            .role(exampleRole.id())
            .policy(serializeJson(
                jsonObject(
                    jsonProperty("Version", "2012-10-17"),
                    jsonProperty("Statement", jsonArray(jsonObject(
                        jsonProperty("Action", jsonArray(
                            "s3:GetObject", 
                            "s3:ListBucket"
                        )),
                        jsonProperty("Effect", "Allow"),
                        jsonProperty("Resource", jsonArray("*"))
                    )))
                )))
            .build());

        var exampleBucketV2 = new BucketV2("exampleBucketV2", BucketV2Args.builder()
            .bucket("example-transcribe")
            .forceDestroy(true)
            .build());

        var object = new BucketObjectv2("object", BucketObjectv2Args.builder()
            .bucket(exampleBucketV2.id())
            .key("transcribe/test1.txt")
            .source(new FileAsset("test1.txt"))
            .build());

        var exampleLanguageModel = new LanguageModel("exampleLanguageModel", LanguageModelArgs.builder()
            .modelName("example")
            .baseModelName("NarrowBand")
            .inputDataConfig(LanguageModelInputDataConfigArgs.builder()
                .dataAccessRoleArn(exampleRole.arn())
                .s3Uri(exampleBucketV2.id().applyValue(id -> String.format("s3://%s/transcribe/", id)))
                .build())
            .languageCode("en-US")
            .tags(Map.of("ENVIRONMENT", "development"))
            .build());

    }
}
```
```yaml
resources:
  exampleRole:
    type: aws:iam:Role
    name: example
    properties:
      name: example
      assumeRolePolicy: ${example.json}
  testPolicy:
    type: aws:iam:RolePolicy
    name: test_policy
    properties:
      name: example
      role: ${exampleRole.id}
      policy:
        fn::toJSON:
          Version: 2012-10-17
          Statement:
            - Action:
                - s3:GetObject
                - s3:ListBucket
              Effect: Allow
              Resource:
                - '*'
  exampleBucketV2:
    type: aws:s3:BucketV2
    name: example
    properties:
      bucket: example-transcribe
      forceDestroy: true
  object:
    type: aws:s3:BucketObjectv2
    properties:
      bucket: ${exampleBucketV2.id}
      key: transcribe/test1.txt
      source:
        fn::FileAsset: test1.txt
  exampleLanguageModel:
    type: aws:transcribe:LanguageModel
    name: example
    properties:
      modelName: example
      baseModelName: NarrowBand
      inputDataConfig:
        dataAccessRoleArn: ${exampleRole.arn}
        s3Uri: s3://${exampleBucketV2.id}/transcribe/
      languageCode: en-US
      tags:
        ENVIRONMENT: development
variables:
  example:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - actions:
              - sts:AssumeRole
            principals:
              - type: Service
                identifiers:
                  - transcribe.amazonaws.com
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Transcribe LanguageModel using the `model_name`. For example:

```sh
$ pulumi import aws:transcribe/languageModel:LanguageModel example example-name
```
3
baseModelName" Name of reference base model.
‚
inputDataConfigx:v
t

transcribeLanguageModelInputDataConfigHaws:transcribe/LanguageModelInputDataConfig:LanguageModelInputDataConfigUThe input data config for the LanguageModel. See Input Data Config for more details.
‘
languageCode" øThe language code you selected for your language model. Refer to the [supported languages](https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html) page for accepted codes.
!
	modelName" The model name.

tagsB2" "%
arn" ARN of the LanguageModel.
"3
baseModelName" Name of reference base model.
"‚
inputDataConfigx:v
t

transcribeLanguageModelInputDataConfigHaws:transcribe/LanguageModelInputDataConfig:LanguageModelInputDataConfigUThe input data config for the LanguageModel. See Input Data Config for more details.
"‘
languageCode" øThe language code you selected for your language model. Refer to the [supported languages](https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html) page for accepted codes.
"!
	modelName" The model name.
"
tagsB2" "
tagsAll2" *¿?
S

transcribeMedicalVocabulary2aws:transcribe/medicalVocabulary:MedicalVocabulary«4Resource for managing an AWS Transcribe MedicalVocabulary.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.s3.BucketV2("example", {
    bucket: "example-medical-vocab-123",
    forceDestroy: true,
});
const object = new aws.s3.BucketObjectv2("object", {
    bucket: example.id,
    key: "transcribe/test1.txt",
    source: new pulumi.asset.FileAsset("test.txt"),
});
const exampleMedicalVocabulary = new aws.transcribe.MedicalVocabulary("example", {
    vocabularyName: "example",
    languageCode: "en-US",
    vocabularyFileUri: pulumi.interpolate`s3://${example.id}/${object.key}`,
    tags: {
        tag1: "value1",
        tag2: "value3",
    },
}, {
    dependsOn: [object],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.s3.BucketV2("example",
    bucket="example-medical-vocab-123",
    force_destroy=True)
object = aws.s3.BucketObjectv2("object",
    bucket=example.id,
    key="transcribe/test1.txt",
    source=pulumi.FileAsset("test.txt"))
example_medical_vocabulary = aws.transcribe.MedicalVocabulary("example",
    vocabulary_name="example",
    language_code="en-US",
    vocabulary_file_uri=pulumi.Output.all(
        id=example.id,
        key=object.key
).apply(lambda resolved_outputs: f"s3://{resolved_outputs['id']}/{resolved_outputs['key']}")
,
    tags={
        "tag1": "value1",
        "tag2": "value3",
    },
    opts = pulumi.ResourceOptions(depends_on=[object]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.S3.BucketV2("example", new()
    {
        Bucket = "example-medical-vocab-123",
        ForceDestroy = true,
    });

    var @object = new Aws.S3.BucketObjectv2("object", new()
    {
        Bucket = example.Id,
        Key = "transcribe/test1.txt",
        Source = new FileAsset("test.txt"),
    });

    var exampleMedicalVocabulary = new Aws.Transcribe.MedicalVocabulary("example", new()
    {
        VocabularyName = "example",
        LanguageCode = "en-US",
        VocabularyFileUri = Output.Tuple(example.Id, @object.Key).Apply(values =>
        {
            var id = values.Item1;
            var key = values.Item2;
            return $"s3://{id}/{key}";
        }),
        Tags = 
        {
            { "tag1", "value1" },
            { "tag2", "value3" },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            @object,
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transcribe"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := s3.NewBucketV2(ctx, "example", &s3.BucketV2Args{
			Bucket:       pulumi.String("example-medical-vocab-123"),
			ForceDestroy: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		object, err := s3.NewBucketObjectv2(ctx, "object", &s3.BucketObjectv2Args{
			Bucket: example.ID(),
			Key:    pulumi.String("transcribe/test1.txt"),
			Source: pulumi.NewFileAsset("test.txt"),
		})
		if err != nil {
			return err
		}
		_, err = transcribe.NewMedicalVocabulary(ctx, "example", &transcribe.MedicalVocabularyArgs{
			VocabularyName: pulumi.String("example"),
			LanguageCode:   pulumi.String("en-US"),
			VocabularyFileUri: pulumi.All(example.ID(), object.Key).ApplyT(func(_args []interface{}) (string, error) {
				id := _args[0].(string)
				key := _args[1].(string)
				return fmt.Sprintf("s3://%v/%v", id, key), nil
			}).(pulumi.StringOutput),
			Tags: pulumi.StringMap{
				"tag1": pulumi.String("value1"),
				"tag2": pulumi.String("value3"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			object,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.s3.BucketObjectv2;
import com.pulumi.aws.s3.BucketObjectv2Args;
import com.pulumi.aws.transcribe.MedicalVocabulary;
import com.pulumi.aws.transcribe.MedicalVocabularyArgs;
import com.pulumi.resources.CustomResourceOptions;
import com.pulumi.asset.FileAsset;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new BucketV2("example", BucketV2Args.builder()
            .bucket("example-medical-vocab-123")
            .forceDestroy(true)
            .build());

        var object = new BucketObjectv2("object", BucketObjectv2Args.builder()
            .bucket(example.id())
            .key("transcribe/test1.txt")
            .source(new FileAsset("test.txt"))
            .build());

        var exampleMedicalVocabulary = new MedicalVocabulary("exampleMedicalVocabulary", MedicalVocabularyArgs.builder()
            .vocabularyName("example")
            .languageCode("en-US")
            .vocabularyFileUri(Output.tuple(example.id(), object.key()).applyValue(values -> {
                var id = values.t1;
                var key = values.t2;
                return String.format("s3://%s/%s", id,key);
            }))
            .tags(Map.ofEntries(
                Map.entry("tag1", "value1"),
                Map.entry("tag2", "value3")
            ))
            .build(), CustomResourceOptions.builder()
                .dependsOn(object)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:s3:BucketV2
    properties:
      bucket: example-medical-vocab-123
      forceDestroy: true
  object:
    type: aws:s3:BucketObjectv2
    properties:
      bucket: ${example.id}
      key: transcribe/test1.txt
      source:
        fn::FileAsset: test.txt
  exampleMedicalVocabulary:
    type: aws:transcribe:MedicalVocabulary
    name: example
    properties:
      vocabularyName: example
      languageCode: en-US
      vocabularyFileUri: s3://${example.id}/${object.key}
      tags:
        tag1: value1
        tag2: value3
    options:
      dependsOn:
        - ${object}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Transcribe MedicalVocabulary using the `vocabulary_name`. For example:

```sh
$ pulumi import aws:transcribe/medicalVocabulary:MedicalVocabulary example example-name
```
£
languageCode" éThe language code you selected for your medical vocabulary. US English (en-US) is the only language supported with Amazon Transcribe Medical.
Ÿ
tagsB2" »A map of tags to assign to the MedicalVocabulary. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
u
vocabularyFileUri" \The Amazon S3 location (URI) of the text file that contains your custom medical vocabulary.
a
vocabularyName" KThe name of the Medical Vocabulary.

The following arguments are optional:
")
arn" ARN of the MedicalVocabulary.
"+
downloadUri" Generated download URI.
"£
languageCode" éThe language code you selected for your medical vocabulary. US English (en-US) is the only language supported with Amazon Transcribe Medical.
"Ÿ
tagsB2" »A map of tags to assign to the MedicalVocabulary. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "u
vocabularyFileUri" \The Amazon S3 location (URI) of the text file that contains your custom medical vocabulary.
"a
vocabularyName" KThe name of the Medical Vocabulary.

The following arguments are optional:
*á>
>

transcribe
Vocabulary$aws:transcribe/vocabulary:Vocabularyä3Resource for managing an AWS Transcribe Vocabulary.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.s3.BucketV2("example", {
    bucket: "example-vocab-123",
    forceDestroy: true,
});
const object = new aws.s3.BucketObjectv2("object", {
    bucket: example.id,
    key: "transcribe/test1.txt",
    source: new pulumi.asset.FileAsset("test.txt"),
});
const exampleVocabulary = new aws.transcribe.Vocabulary("example", {
    vocabularyName: "example",
    languageCode: "en-US",
    vocabularyFileUri: pulumi.interpolate`s3://${example.id}/${object.key}`,
    tags: {
        tag1: "value1",
        tag2: "value3",
    },
}, {
    dependsOn: [object],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.s3.BucketV2("example",
    bucket="example-vocab-123",
    force_destroy=True)
object = aws.s3.BucketObjectv2("object",
    bucket=example.id,
    key="transcribe/test1.txt",
    source=pulumi.FileAsset("test.txt"))
example_vocabulary = aws.transcribe.Vocabulary("example",
    vocabulary_name="example",
    language_code="en-US",
    vocabulary_file_uri=pulumi.Output.all(
        id=example.id,
        key=object.key
).apply(lambda resolved_outputs: f"s3://{resolved_outputs['id']}/{resolved_outputs['key']}")
,
    tags={
        "tag1": "value1",
        "tag2": "value3",
    },
    opts = pulumi.ResourceOptions(depends_on=[object]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.S3.BucketV2("example", new()
    {
        Bucket = "example-vocab-123",
        ForceDestroy = true,
    });

    var @object = new Aws.S3.BucketObjectv2("object", new()
    {
        Bucket = example.Id,
        Key = "transcribe/test1.txt",
        Source = new FileAsset("test.txt"),
    });

    var exampleVocabulary = new Aws.Transcribe.Vocabulary("example", new()
    {
        VocabularyName = "example",
        LanguageCode = "en-US",
        VocabularyFileUri = Output.Tuple(example.Id, @object.Key).Apply(values =>
        {
            var id = values.Item1;
            var key = values.Item2;
            return $"s3://{id}/{key}";
        }),
        Tags = 
        {
            { "tag1", "value1" },
            { "tag2", "value3" },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            @object,
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transcribe"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := s3.NewBucketV2(ctx, "example", &s3.BucketV2Args{
			Bucket:       pulumi.String("example-vocab-123"),
			ForceDestroy: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		object, err := s3.NewBucketObjectv2(ctx, "object", &s3.BucketObjectv2Args{
			Bucket: example.ID(),
			Key:    pulumi.String("transcribe/test1.txt"),
			Source: pulumi.NewFileAsset("test.txt"),
		})
		if err != nil {
			return err
		}
		_, err = transcribe.NewVocabulary(ctx, "example", &transcribe.VocabularyArgs{
			VocabularyName: pulumi.String("example"),
			LanguageCode:   pulumi.String("en-US"),
			VocabularyFileUri: pulumi.All(example.ID(), object.Key).ApplyT(func(_args []interface{}) (string, error) {
				id := _args[0].(string)
				key := _args[1].(string)
				return fmt.Sprintf("s3://%v/%v", id, key), nil
			}).(pulumi.StringOutput),
			Tags: pulumi.StringMap{
				"tag1": pulumi.String("value1"),
				"tag2": pulumi.String("value3"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			object,
		}))
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.s3.BucketObjectv2;
import com.pulumi.aws.s3.BucketObjectv2Args;
import com.pulumi.aws.transcribe.Vocabulary;
import com.pulumi.aws.transcribe.VocabularyArgs;
import com.pulumi.resources.CustomResourceOptions;
import com.pulumi.asset.FileAsset;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new BucketV2("example", BucketV2Args.builder()
            .bucket("example-vocab-123")
            .forceDestroy(true)
            .build());

        var object = new BucketObjectv2("object", BucketObjectv2Args.builder()
            .bucket(example.id())
            .key("transcribe/test1.txt")
            .source(new FileAsset("test.txt"))
            .build());

        var exampleVocabulary = new Vocabulary("exampleVocabulary", VocabularyArgs.builder()
            .vocabularyName("example")
            .languageCode("en-US")
            .vocabularyFileUri(Output.tuple(example.id(), object.key()).applyValue(values -> {
                var id = values.t1;
                var key = values.t2;
                return String.format("s3://%s/%s", id,key);
            }))
            .tags(Map.ofEntries(
                Map.entry("tag1", "value1"),
                Map.entry("tag2", "value3")
            ))
            .build(), CustomResourceOptions.builder()
                .dependsOn(object)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:s3:BucketV2
    properties:
      bucket: example-vocab-123
      forceDestroy: true
  object:
    type: aws:s3:BucketObjectv2
    properties:
      bucket: ${example.id}
      key: transcribe/test1.txt
      source:
        fn::FileAsset: test.txt
  exampleVocabulary:
    type: aws:transcribe:Vocabulary
    name: example
    properties:
      vocabularyName: example
      languageCode: en-US
      vocabularyFileUri: s3://${example.id}/${object.key}
      tags:
        tag1: value1
        tag2: value3
    options:
      dependsOn:
        - ${object}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Transcribe Vocabulary using the `vocabulary_name`. For example:

```sh
$ pulumi import aws:transcribe/vocabulary:Vocabulary example example-name
```
H
languageCode" 4The language code you selected for your vocabulary.
f
phrasesB*" SA list of terms to include in the vocabulary. Conflicts with `vocabulary_file_uri`
“
tagsB2" ¡A map of tags to assign to the Vocabulary. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
à
vocabularyFileUriB" mThe Amazon S3 location (URI) of the text file that contains your custom vocabulary. Conflicts wth `phrases`.
Y
vocabularyName" CThe name of the Vocabulary.

The following arguments are optional:
""
arn" ARN of the Vocabulary.
"+
downloadUri" Generated download URI.
"H
languageCode" 4The language code you selected for your vocabulary.
"f
phrasesB*" SA list of terms to include in the vocabulary. Conflicts with `vocabulary_file_uri`
"“
tagsB2" ¡A map of tags to assign to the Vocabulary. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "Ü
vocabularyFileUri" mThe Amazon S3 location (URI) of the text file that contains your custom vocabulary. Conflicts wth `phrases`.
"Y
vocabularyName" CThe name of the Vocabulary.

The following arguments are optional:
*»(
P

transcribeVocabularyFilter0aws:transcribe/vocabularyFilter:VocabularyFilterßResource for managing an AWS Transcribe VocabularyFilter.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transcribe.VocabularyFilter("example", {
    vocabularyFilterName: "example",
    languageCode: "en-US",
    words: [
        "cars",
        "bucket",
    ],
    tags: {
        tag1: "value1",
        tag2: "value3",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transcribe.VocabularyFilter("example",
    vocabulary_filter_name="example",
    language_code="en-US",
    words=[
        "cars",
        "bucket",
    ],
    tags={
        "tag1": "value1",
        "tag2": "value3",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transcribe.VocabularyFilter("example", new()
    {
        VocabularyFilterName = "example",
        LanguageCode = "en-US",
        Words = new[]
        {
            "cars",
            "bucket",
        },
        Tags = 
        {
            { "tag1", "value1" },
            { "tag2", "value3" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transcribe"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transcribe.NewVocabularyFilter(ctx, "example", &transcribe.VocabularyFilterArgs{
			VocabularyFilterName: pulumi.String("example"),
			LanguageCode:         pulumi.String("en-US"),
			Words: pulumi.StringArray{
				pulumi.String("cars"),
				pulumi.String("bucket"),
			},
			Tags: pulumi.StringMap{
				"tag1": pulumi.String("value1"),
				"tag2": pulumi.String("value3"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transcribe.VocabularyFilter;
import com.pulumi.aws.transcribe.VocabularyFilterArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new VocabularyFilter("example", VocabularyFilterArgs.builder()
            .vocabularyFilterName("example")
            .languageCode("en-US")
            .words(            
                "cars",
                "bucket")
            .tags(Map.ofEntries(
                Map.entry("tag1", "value1"),
                Map.entry("tag2", "value3")
            ))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transcribe:VocabularyFilter
    properties:
      vocabularyFilterName: example
      languageCode: en-US
      words:
        - cars
        - bucket
      tags:
        tag1: value1
        tag2: value3
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Transcribe VocabularyFilter using the `vocabulary_filter_name`. For example:

```sh
$ pulumi import aws:transcribe/vocabularyFilter:VocabularyFilter example example-name
```
◊
languageCode" ¬The language code you selected for your vocabulary filter. Refer to the [supported languages](https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html) page for accepted codes.
ÿ
tagsB2" «A map of tags to assign to the VocabularyFilter. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
ú
vocabularyFilterFileUriB" {The Amazon S3 location (URI) of the text file that contains your custom VocabularyFilter. Conflicts with `words` argument.
e
vocabularyFilterName" IThe name of the VocabularyFilter.

The following arguments are optional:
u
wordsB*" dA list of terms to include in the vocabulary. Conflicts with `vocabulary_filter_file_uri` argument.
"(
arn" ARN of the VocabularyFilter.
"+
downloadUri" Generated download URI.
"◊
languageCode" ¬The language code you selected for your vocabulary filter. Refer to the [supported languages](https://docs.aws.amazon.com/transcribe/latest/dg/supported-languages.html) page for accepted codes.
"ÿ
tagsB2" «A map of tags to assign to the VocabularyFilter. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "ú
vocabularyFilterFileUriB" {The Amazon S3 location (URI) of the text file that contains your custom VocabularyFilter. Conflicts with `words` argument.
"e
vocabularyFilterName" IThe name of the VocabularyFilter.

The following arguments are optional:
"u
wordsB*" dA list of terms to include in the vocabulary. Conflicts with `vocabulary_filter_file_uri` argument.
*ÖV
.
transferAccessaws:transfer/access:AccessÜ7Provides a AWS Transfer Access resource.

> **NOTE:** We suggest using explicit JSON encoding or `aws.iam.getPolicyDocument` when assigning a value to `policy`. They seamlessly translate configuration to JSON, enabling you to maintain consistency within your configuration without the need for context switches. Also, you can sidestep potential complications arising from formatting discrepancies, whitespace inconsistencies, and other nuances inherent to JSON.

## Example Usage

### Basic S3

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transfer.Access("example", {
    externalId: "S-1-1-12-1234567890-123456789-1234567890-1234",
    serverId: exampleAwsTransferServer.id,
    role: exampleAwsIamRole.arn,
    homeDirectory: `/${exampleAwsS3Bucket.id}/`,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.Access("example",
    external_id="S-1-1-12-1234567890-123456789-1234567890-1234",
    server_id=example_aws_transfer_server["id"],
    role=example_aws_iam_role["arn"],
    home_directory=f"/{example_aws_s3_bucket['id']}/")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transfer.Access("example", new()
    {
        ExternalId = "S-1-1-12-1234567890-123456789-1234567890-1234",
        ServerId = exampleAwsTransferServer.Id,
        Role = exampleAwsIamRole.Arn,
        HomeDirectory = $"/{exampleAwsS3Bucket.Id}/",
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.NewAccess(ctx, "example", &transfer.AccessArgs{
			ExternalId:    pulumi.String("S-1-1-12-1234567890-123456789-1234567890-1234"),
			ServerId:      pulumi.Any(exampleAwsTransferServer.Id),
			Role:          pulumi.Any(exampleAwsIamRole.Arn),
			HomeDirectory: pulumi.Sprintf("/%v/", exampleAwsS3Bucket.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Access;
import com.pulumi.aws.transfer.AccessArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Access("example", AccessArgs.builder()
            .externalId("S-1-1-12-1234567890-123456789-1234567890-1234")
            .serverId(exampleAwsTransferServer.id())
            .role(exampleAwsIamRole.arn())
            .homeDirectory(String.format("/%s/", exampleAwsS3Bucket.id()))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transfer:Access
    properties:
      externalId: S-1-1-12-1234567890-123456789-1234567890-1234
      serverId: ${exampleAwsTransferServer.id}
      role: ${exampleAwsIamRole.arn}
      homeDirectory: /${exampleAwsS3Bucket.id}/
```
<!--End PulumiCodeChooser -->

### Basic EFS

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.transfer.Access("test", {
    externalId: "S-1-1-12-1234567890-123456789-1234567890-1234",
    serverId: testAwsTransferServer.id,
    role: testAwsIamRole.arn,
    homeDirectory: `/${testAwsEfsFileSystem.id}/`,
    posixProfile: {
        gid: 1000,
        uid: 1000,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.transfer.Access("test",
    external_id="S-1-1-12-1234567890-123456789-1234567890-1234",
    server_id=test_aws_transfer_server["id"],
    role=test_aws_iam_role["arn"],
    home_directory=f"/{test_aws_efs_file_system['id']}/",
    posix_profile={
        "gid": 1000,
        "uid": 1000,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Transfer.Access("test", new()
    {
        ExternalId = "S-1-1-12-1234567890-123456789-1234567890-1234",
        ServerId = testAwsTransferServer.Id,
        Role = testAwsIamRole.Arn,
        HomeDirectory = $"/{testAwsEfsFileSystem.Id}/",
        PosixProfile = new Aws.Transfer.Inputs.AccessPosixProfileArgs
        {
            Gid = 1000,
            Uid = 1000,
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.NewAccess(ctx, "test", &transfer.AccessArgs{
			ExternalId:    pulumi.String("S-1-1-12-1234567890-123456789-1234567890-1234"),
			ServerId:      pulumi.Any(testAwsTransferServer.Id),
			Role:          pulumi.Any(testAwsIamRole.Arn),
			HomeDirectory: pulumi.Sprintf("/%v/", testAwsEfsFileSystem.Id),
			PosixProfile: &transfer.AccessPosixProfileArgs{
				Gid: pulumi.Int(1000),
				Uid: pulumi.Int(1000),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Access;
import com.pulumi.aws.transfer.AccessArgs;
import com.pulumi.aws.transfer.inputs.AccessPosixProfileArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new Access("test", AccessArgs.builder()
            .externalId("S-1-1-12-1234567890-123456789-1234567890-1234")
            .serverId(testAwsTransferServer.id())
            .role(testAwsIamRole.arn())
            .homeDirectory(String.format("/%s/", testAwsEfsFileSystem.id()))
            .posixProfile(AccessPosixProfileArgs.builder()
                .gid(1000)
                .uid(1000)
                .build())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:transfer:Access
    properties:
      externalId: S-1-1-12-1234567890-123456789-1234567890-1234
      serverId: ${testAwsTransferServer.id}
      role: ${testAwsIamRole.arn}
      homeDirectory: /${testAwsEfsFileSystem.id}/
      posixProfile:
        gid: 1000
        uid: 1000
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Transfer Accesses using the `server_id` and `external_id`. For example:

```sh
$ pulumi import aws:transfer/access:Access example s-12345678/S-1-1-12-1234567890-123456789-1234567890-1234
```
è

externalId" }The SID of a group in the directory connected to the Transfer Server (e.g., `S-1-1-12-1234567890-123456789-1234567890-1234`)
Ì
homeDirectoryB" ’The landing directory (folder) for a user when they log in to the server using their SFTP client.  It should begin with a `/`.  The first item in the path is the name of the home bucket (accessible as `${Transfer:HomeBucket}` in the policy) and the rest is the home directory (accessible as `${Transfer:HomeDirectory}` in the policy). For example, `/example-bucket-1234/username` would set the home bucket to `example-bucket-1234` and the home directory to `username`.
∂
homeDirectoryMappingsrBp*n:l
j
transferAccessHomeDirectoryMappingBaws:transfer/AccessHomeDirectoryMapping:AccessHomeDirectoryMapping®Logical directory mappings that specify what S3 paths and keys should be visible to your user and how you want to make them visible. See Home Directory Mappings below.
î
homeDirectoryTypeB" yThe type of landing directory (folder) you mapped for your users' home directory. Valid values are `PATH` and `LOGICAL`.
¥
policyB" £An IAM JSON policy document that scopes down user access to portions of their Amazon S3 bucket. IAM variables you can use inside this policy include `${Transfer:UserName}`, `${Transfer:HomeDirectory}`, and `${Transfer:HomeBucket}`. These are evaluated on-the-fly when navigating the bucket.
¿
posixProfileXBV:T
R
transferAccessPosixProfile2aws:transfer/AccessPosixProfile:AccessPosixProfile’Specifies the full POSIX identity, including user ID (Uid), group ID (Gid), and any secondary groups IDs (SecondaryGids), that controls your users' access to your Amazon EFS file systems. See Posix Profile below.
ã
roleB" }Amazon Resource Name (ARN) of an IAM role that allows the service to controls your user‚Äôs access to your Amazon S3 bucket.
J
serverId" :The Server ID of the Transfer Server (e.g., `s-12345678`)
"è

externalId" }The SID of a group in the directory connected to the Transfer Server (e.g., `S-1-1-12-1234567890-123456789-1234567890-1234`)
"Ì
homeDirectoryB" ’The landing directory (folder) for a user when they log in to the server using their SFTP client.  It should begin with a `/`.  The first item in the path is the name of the home bucket (accessible as `${Transfer:HomeBucket}` in the policy) and the rest is the home directory (accessible as `${Transfer:HomeDirectory}` in the policy). For example, `/example-bucket-1234/username` would set the home bucket to `example-bucket-1234` and the home directory to `username`.
"∂
homeDirectoryMappingsrBp*n:l
j
transferAccessHomeDirectoryMappingBaws:transfer/AccessHomeDirectoryMapping:AccessHomeDirectoryMapping®Logical directory mappings that specify what S3 paths and keys should be visible to your user and how you want to make them visible. See Home Directory Mappings below.
"î
homeDirectoryTypeB" yThe type of landing directory (folder) you mapped for your users' home directory. Valid values are `PATH` and `LOGICAL`.
"¥
policyB" £An IAM JSON policy document that scopes down user access to portions of their Amazon S3 bucket. IAM variables you can use inside this policy include `${Transfer:UserName}`, `${Transfer:HomeDirectory}`, and `${Transfer:HomeBucket}`. These are evaluated on-the-fly when navigating the bucket.
"¿
posixProfileXBV:T
R
transferAccessPosixProfile2aws:transfer/AccessPosixProfile:AccessPosixProfile’Specifies the full POSIX identity, including user ID (Uid), group ID (Gid), and any secondary groups IDs (SecondaryGids), that controls your users' access to your Amazon EFS file systems. See Posix Profile below.
"ã
roleB" }Amazon Resource Name (ARN) of an IAM role that allows the service to controls your user‚Äôs access to your Amazon S3 bucket.
"J
serverId" :The Server ID of the Transfer Server (e.g., `s-12345678`)
*ü)
7
transfer	Agreement aws:transfer/agreement:AgreementèProvides a AWS Transfer AS2 Agreement resource.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transfer.Agreement("example", {
    accessRole: test.arn,
    baseDirectory: "/DOC-EXAMPLE-BUCKET/home/mydirectory",
    description: "example",
    localProfileId: local.profileId,
    partnerProfileId: partner.profileId,
    serverId: testAwsTransferServer.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.Agreement("example",
    access_role=test["arn"],
    base_directory="/DOC-EXAMPLE-BUCKET/home/mydirectory",
    description="example",
    local_profile_id=local["profileId"],
    partner_profile_id=partner["profileId"],
    server_id=test_aws_transfer_server["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transfer.Agreement("example", new()
    {
        AccessRole = test.Arn,
        BaseDirectory = "/DOC-EXAMPLE-BUCKET/home/mydirectory",
        Description = "example",
        LocalProfileId = local.ProfileId,
        PartnerProfileId = partner.ProfileId,
        ServerId = testAwsTransferServer.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.NewAgreement(ctx, "example", &transfer.AgreementArgs{
			AccessRole:       pulumi.Any(test.Arn),
			BaseDirectory:    pulumi.String("/DOC-EXAMPLE-BUCKET/home/mydirectory"),
			Description:      pulumi.String("example"),
			LocalProfileId:   pulumi.Any(local.ProfileId),
			PartnerProfileId: pulumi.Any(partner.ProfileId),
			ServerId:         pulumi.Any(testAwsTransferServer.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Agreement;
import com.pulumi.aws.transfer.AgreementArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Agreement("example", AgreementArgs.builder()
            .accessRole(test.arn())
            .baseDirectory("/DOC-EXAMPLE-BUCKET/home/mydirectory")
            .description("example")
            .localProfileId(local.profileId())
            .partnerProfileId(partner.profileId())
            .serverId(testAwsTransferServer.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transfer:Agreement
    properties:
      accessRole: ${test.arn}
      baseDirectory: /DOC-EXAMPLE-BUCKET/home/mydirectory
      description: example
      localProfileId: ${local.profileId}
      partnerProfileId: ${partner.profileId}
      serverId: ${testAwsTransferServer.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Transfer AS2 Agreement using the `server_id/agreement_id`. For example:

```sh
$ pulumi import aws:transfer/agreement:Agreement example s-4221a88afd5f4362a/a-4221a88afd5f4362a
```
û

accessRole" ãThe IAM Role which provides read and write access to the parent directory of the file location mentioned in the StartFileTransfer request.
`
baseDirectory" KThe landing directory for the files transferred by using the AS2 protocol.
@
descriptionB" +The Optional description of the transdfer.
G
localProfileId" 1The unique identifier for the AS2 local profile.
K
partnerProfileId" 3The unique identifier for the AS2 partner profile.
v
serverId" fThe unique server identifier for the server instance. This is the specific server the agreement uses.
–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"û

accessRole" ãThe IAM Role which provides read and write access to the parent directory of the file location mentioned in the StartFileTransfer request.
"@
agreementId" -The unique identifier for the AS2 agreement.
"%
arn" The ARN of the agreement.
"`
baseDirectory" KThe landing directory for the files transferred by using the AS2 protocol.
"@
descriptionB" +The Optional description of the transdfer.
"G
localProfileId" 1The unique identifier for the AS2 local profile.
"K
partnerProfileId" 3The unique identifier for the AS2 partner profile.
"v
serverId" fThe unique server identifier for the server instance. This is the specific server the agreement uses.
"
status" "–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" *ÿ
=
transferCertificate$aws:transfer/certificate:CertificateåProvides a AWS Transfer AS2 Certificate resource.

## Example Usage

## Import

Using `pulumi import`, import Transfer AS2 Certificate using the `certificate_id`. For example:

```sh
$ pulumi import aws:transfer/certificate:Certificate example c-4221a88afd5f4362a
```
I
certificate" 6The valid certificate file required for the transfer.
~
certificateChainB" dThe optional list of certificate that make up the chain for the certificate that is being imported.
N
descriptionB" 9A short description that helps identify the certificate.
T

privateKeyB" @The private key associated with the certificate being imported.
–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
~
usage" qSpecifies if a certificate is being used for signing or encryption. The valid values are SIGNING and ENCRYPTION.
">

activeDate" ,An date when the certificate becomes active
"&
arn" The ARN of the certificate
"I
certificate" 6The valid certificate file required for the transfer.
"~
certificateChainB" dThe optional list of certificate that make up the chain for the certificate that is being imported.
"C
certificateId" .The unique identifier for the AS2 certificate
"N
descriptionB" 9A short description that helps identify the certificate.
"B
inactiveDate" .An date when the certificate becomes inactive
"T

privateKeyB" @The private key associated with the certificate being imported.
"–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "~
usage" qSpecifies if a certificate is being used for signing or encryption. The valid values are SIGNING and ENCRYPTION.
*‡O
7
transfer	Connector aws:transfer/connector:Connectorò=Provides a AWS Transfer AS2 Connector resource.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transfer.Connector("example", {
    accessRole: test.arn,
    as2Config: {
        compression: "DISABLED",
        encryptionAlgorithm: "AWS128_CBC",
        messageSubject: "For Connector",
        localProfileId: local.profileId,
        mdnResponse: "NONE",
        mdnSigningAlgorithm: "NONE",
        partnerProfileId: partner.profileId,
        signingAlgorithm: "NONE",
    },
    url: "http://www.test.com",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.Connector("example",
    access_role=test["arn"],
    as2_config={
        "compression": "DISABLED",
        "encryption_algorithm": "AWS128_CBC",
        "message_subject": "For Connector",
        "local_profile_id": local["profileId"],
        "mdn_response": "NONE",
        "mdn_signing_algorithm": "NONE",
        "partner_profile_id": partner["profileId"],
        "signing_algorithm": "NONE",
    },
    url="http://www.test.com")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transfer.Connector("example", new()
    {
        AccessRole = test.Arn,
        As2Config = new Aws.Transfer.Inputs.ConnectorAs2ConfigArgs
        {
            Compression = "DISABLED",
            EncryptionAlgorithm = "AWS128_CBC",
            MessageSubject = "For Connector",
            LocalProfileId = local.ProfileId,
            MdnResponse = "NONE",
            MdnSigningAlgorithm = "NONE",
            PartnerProfileId = partner.ProfileId,
            SigningAlgorithm = "NONE",
        },
        Url = "http://www.test.com",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.NewConnector(ctx, "example", &transfer.ConnectorArgs{
			AccessRole: pulumi.Any(test.Arn),
			As2Config: &transfer.ConnectorAs2ConfigArgs{
				Compression:         pulumi.String("DISABLED"),
				EncryptionAlgorithm: pulumi.String("AWS128_CBC"),
				MessageSubject:      pulumi.String("For Connector"),
				LocalProfileId:      pulumi.Any(local.ProfileId),
				MdnResponse:         pulumi.String("NONE"),
				MdnSigningAlgorithm: pulumi.String("NONE"),
				PartnerProfileId:    pulumi.Any(partner.ProfileId),
				SigningAlgorithm:    pulumi.String("NONE"),
			},
			Url: pulumi.String("http://www.test.com"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Connector;
import com.pulumi.aws.transfer.ConnectorArgs;
import com.pulumi.aws.transfer.inputs.ConnectorAs2ConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Connector("example", ConnectorArgs.builder()
            .accessRole(test.arn())
            .as2Config(ConnectorAs2ConfigArgs.builder()
                .compression("DISABLED")
                .encryptionAlgorithm("AWS128_CBC")
                .messageSubject("For Connector")
                .localProfileId(local.profileId())
                .mdnResponse("NONE")
                .mdnSigningAlgorithm("NONE")
                .partnerProfileId(partner.profileId())
                .signingAlgorithm("NONE")
                .build())
            .url("http://www.test.com")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transfer:Connector
    properties:
      accessRole: ${test.arn}
      as2Config:
        compression: DISABLED
        encryptionAlgorithm: AWS128_CBC
        messageSubject: For Connector
        localProfileId: ${local.profileId}
        mdnResponse: NONE
        mdnSigningAlgorithm: NONE
        partnerProfileId: ${partner.profileId}
        signingAlgorithm: NONE
      url: http://www.test.com
```
<!--End PulumiCodeChooser -->

### SFTP Connector

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transfer.Connector("example", {
    accessRole: test.arn,
    sftpConfig: {
        trustedHostKeys: ["ssh-rsa AAAAB3NYourKeysHere"],
        userSecretId: exampleAwsSecretsmanagerSecret.id,
    },
    url: "sftp://test.com",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.Connector("example",
    access_role=test["arn"],
    sftp_config={
        "trusted_host_keys": ["ssh-rsa AAAAB3NYourKeysHere"],
        "user_secret_id": example_aws_secretsmanager_secret["id"],
    },
    url="sftp://test.com")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transfer.Connector("example", new()
    {
        AccessRole = test.Arn,
        SftpConfig = new Aws.Transfer.Inputs.ConnectorSftpConfigArgs
        {
            TrustedHostKeys = new[]
            {
                "ssh-rsa AAAAB3NYourKeysHere",
            },
            UserSecretId = exampleAwsSecretsmanagerSecret.Id,
        },
        Url = "sftp://test.com",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.NewConnector(ctx, "example", &transfer.ConnectorArgs{
			AccessRole: pulumi.Any(test.Arn),
			SftpConfig: &transfer.ConnectorSftpConfigArgs{
				TrustedHostKeys: pulumi.StringArray{
					pulumi.String("ssh-rsa AAAAB3NYourKeysHere"),
				},
				UserSecretId: pulumi.Any(exampleAwsSecretsmanagerSecret.Id),
			},
			Url: pulumi.String("sftp://test.com"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Connector;
import com.pulumi.aws.transfer.ConnectorArgs;
import com.pulumi.aws.transfer.inputs.ConnectorSftpConfigArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Connector("example", ConnectorArgs.builder()
            .accessRole(test.arn())
            .sftpConfig(ConnectorSftpConfigArgs.builder()
                .trustedHostKeys("ssh-rsa AAAAB3NYourKeysHere")
                .userSecretId(exampleAwsSecretsmanagerSecret.id())
                .build())
            .url("sftp://test.com")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transfer:Connector
    properties:
      accessRole: ${test.arn}
      sftpConfig:
        trustedHostKeys:
          - ssh-rsa AAAAB3NYourKeysHere
        userSecretId: ${exampleAwsSecretsmanagerSecret.id}
      url: sftp://test.com
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Transfer AS2 Connector using the `connector_id`. For example:

```sh
$ pulumi import aws:transfer/connector:Connector example c-4221a88afd5f4362a
```
û

accessRole" ãThe IAM Role which provides read and write access to the parent directory of the file location mentioned in the StartFileTransfer request.
◊
	as2ConfigXBV:T
R
transferConnectorAs2Config2aws:transfer/ConnectorAs2Config:ConnectorAs2ConfigpEither SFTP or AS2 is configured.The parameters to configure for the connector object. Fields documented below.
É
loggingRoleB" nThe IAM Role which is required for allowing the connector to turn on CloudWatch logging for Amazon S3 events.
K
securityPolicyNameB" /Name of the security policy for the connector.
€

sftpConfig[BY:W
U
transferConnectorSftpConfig4aws:transfer/ConnectorSftpConfig:ConnectorSftpConfigpEither SFTP or AS2 is configured.The parameters to configure for the connector object. Fields documented below.
–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
B
url" 7The URL of the partners AS2 endpoint or SFTP endpoint.
"û

accessRole" ãThe IAM Role which provides read and write access to the parent directory of the file location mentioned in the StartFileTransfer request.
"%
arn" The ARN of the connector.
"◊
	as2ConfigXBV:T
R
transferConnectorAs2Config2aws:transfer/ConnectorAs2Config:ConnectorAs2ConfigpEither SFTP or AS2 is configured.The parameters to configure for the connector object. Fields documented below.
"N
connectorId" ;The unique identifier for the AS2 profile or SFTP Profile.
"É
loggingRoleB" nThe IAM Role which is required for allowing the connector to turn on CloudWatch logging for Amazon S3 events.
"I
securityPolicyName" /Name of the security policy for the connector.
"€

sftpConfig[BY:W
U
transferConnectorSftpConfig4aws:transfer/ConnectorSftpConfig:ConnectorSftpConfigpEither SFTP or AS2 is configured.The parameters to configure for the connector object. Fields documented below.
"–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "B
url" 7The URL of the partners AS2 endpoint or SFTP endpoint.
*‹
1
transferProfileaws:transfer/profile:ProfileäProvides a AWS Transfer AS2 Profile resource.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```yaml
resources:
  example:
    type: aws:transfer:Profile
    properties:
      as2Id: example
      certificateIds:
        - ${exampleAwsTransferCertificate.certificateId}
      usage: LOCAL
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Transfer AS2 Profile using the `profile_id`. For example:

```sh
$ pulumi import aws:transfer/profile:Profile example p-4221a88afd5f4362a
```
õ
as2Id" çThe As2Id is the AS2 name as defined in the RFC 4130. For inbound ttransfers this is the AS2 From Header for the AS2 messages sent from the partner. For Outbound messages this is the AS2 To Header for the AS2 messages sent to the partner. his ID cannot include spaces.
_
certificateIdsB*" EThe list of certificate Ids from the imported certificate operation.
@
profileType" -The profile type should be LOCAL or PARTNER.
–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"#
arn" The ARN of the profile.
"õ
as2Id" çThe As2Id is the AS2 name as defined in the RFC 4130. For inbound ttransfers this is the AS2 From Header for the AS2 messages sent from the partner. For Outbound messages this is the AS2 To Header for the AS2 messages sent to the partner. his ID cannot include spaces.
"_
certificateIdsB*" EThe list of certificate Ids from the imported certificate operation.
"<
	profileId" +The unique identifier for the AS2 profile.
"@
profileType" -The profile type should be LOCAL or PARTNER.
"–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" *Ω®
.
transferServeraws:transfer/server:Server˜≈Provides a AWS Transfer Server resource.

> **NOTE on AWS IAM permissions:** If the `endpoint_type` is set to `VPC`, the `ec2:DescribeVpcEndpoints` and `ec2:ModifyVpcEndpoint` [actions](https://docs.aws.amazon.com/service-authorization/latest/reference/list_amazonec2.html#amazonec2-actions-as-permissions) are used.

> **NOTE:** Use the `aws.transfer.Tag` resource to manage the system tags used for [custom hostnames](https://docs.aws.amazon.com/transfer/latest/userguide/requirements-dns.html#tag-custom-hostname-cdk).

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transfer.Server("example", {tags: {
    Name: "Example",
}});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.Server("example", tags={
    "Name": "Example",
})
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transfer.Server("example", new()
    {
        Tags = 
        {
            { "Name", "Example" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.NewServer(ctx, "example", &transfer.ServerArgs{
			Tags: pulumi.StringMap{
				"Name": pulumi.String("Example"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Server;
import com.pulumi.aws.transfer.ServerArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Server("example", ServerArgs.builder()
            .tags(Map.of("Name", "Example"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transfer:Server
    properties:
      tags:
        Name: Example
```
<!--End PulumiCodeChooser -->

### Security Policy Name

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transfer.Server("example", {securityPolicyName: "TransferSecurityPolicy-2020-06"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.Server("example", security_policy_name="TransferSecurityPolicy-2020-06")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transfer.Server("example", new()
    {
        SecurityPolicyName = "TransferSecurityPolicy-2020-06",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.NewServer(ctx, "example", &transfer.ServerArgs{
			SecurityPolicyName: pulumi.String("TransferSecurityPolicy-2020-06"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Server;
import com.pulumi.aws.transfer.ServerArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Server("example", ServerArgs.builder()
            .securityPolicyName("TransferSecurityPolicy-2020-06")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transfer:Server
    properties:
      securityPolicyName: TransferSecurityPolicy-2020-06
```
<!--End PulumiCodeChooser -->

### VPC Endpoint

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transfer.Server("example", {
    endpointType: "VPC",
    endpointDetails: {
        addressAllocationIds: [exampleAwsEip.id],
        subnetIds: [exampleAwsSubnet.id],
        vpcId: exampleAwsVpc.id,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.Server("example",
    endpoint_type="VPC",
    endpoint_details={
        "address_allocation_ids": [example_aws_eip["id"]],
        "subnet_ids": [example_aws_subnet["id"]],
        "vpc_id": example_aws_vpc["id"],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transfer.Server("example", new()
    {
        EndpointType = "VPC",
        EndpointDetails = new Aws.Transfer.Inputs.ServerEndpointDetailsArgs
        {
            AddressAllocationIds = new[]
            {
                exampleAwsEip.Id,
            },
            SubnetIds = new[]
            {
                exampleAwsSubnet.Id,
            },
            VpcId = exampleAwsVpc.Id,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.NewServer(ctx, "example", &transfer.ServerArgs{
			EndpointType: pulumi.String("VPC"),
			EndpointDetails: &transfer.ServerEndpointDetailsArgs{
				AddressAllocationIds: pulumi.StringArray{
					exampleAwsEip.Id,
				},
				SubnetIds: pulumi.StringArray{
					exampleAwsSubnet.Id,
				},
				VpcId: pulumi.Any(exampleAwsVpc.Id),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Server;
import com.pulumi.aws.transfer.ServerArgs;
import com.pulumi.aws.transfer.inputs.ServerEndpointDetailsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Server("example", ServerArgs.builder()
            .endpointType("VPC")
            .endpointDetails(ServerEndpointDetailsArgs.builder()
                .addressAllocationIds(exampleAwsEip.id())
                .subnetIds(exampleAwsSubnet.id())
                .vpcId(exampleAwsVpc.id())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transfer:Server
    properties:
      endpointType: VPC
      endpointDetails:
        addressAllocationIds:
          - ${exampleAwsEip.id}
        subnetIds:
          - ${exampleAwsSubnet.id}
        vpcId: ${exampleAwsVpc.id}
```
<!--End PulumiCodeChooser -->

### AWS Directory authentication

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transfer.Server("example", {
    identityProviderType: "AWS_DIRECTORY_SERVICE",
    directoryId: exampleAwsDirectoryServiceDirectory.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.Server("example",
    identity_provider_type="AWS_DIRECTORY_SERVICE",
    directory_id=example_aws_directory_service_directory["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transfer.Server("example", new()
    {
        IdentityProviderType = "AWS_DIRECTORY_SERVICE",
        DirectoryId = exampleAwsDirectoryServiceDirectory.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.NewServer(ctx, "example", &transfer.ServerArgs{
			IdentityProviderType: pulumi.String("AWS_DIRECTORY_SERVICE"),
			DirectoryId:          pulumi.Any(exampleAwsDirectoryServiceDirectory.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Server;
import com.pulumi.aws.transfer.ServerArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Server("example", ServerArgs.builder()
            .identityProviderType("AWS_DIRECTORY_SERVICE")
            .directoryId(exampleAwsDirectoryServiceDirectory.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transfer:Server
    properties:
      identityProviderType: AWS_DIRECTORY_SERVICE
      directoryId: ${exampleAwsDirectoryServiceDirectory.id}
```
<!--End PulumiCodeChooser -->

### AWS Lambda authentication

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transfer.Server("example", {
    identityProviderType: "AWS_LAMBDA",
    "function": exampleAwsLambdaIdentityProvider.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.Server("example",
    identity_provider_type="AWS_LAMBDA",
    function=example_aws_lambda_identity_provider["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transfer.Server("example", new()
    {
        IdentityProviderType = "AWS_LAMBDA",
        Function = exampleAwsLambdaIdentityProvider.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.NewServer(ctx, "example", &transfer.ServerArgs{
			IdentityProviderType: pulumi.String("AWS_LAMBDA"),
			Function:             pulumi.Any(exampleAwsLambdaIdentityProvider.Arn),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Server;
import com.pulumi.aws.transfer.ServerArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Server("example", ServerArgs.builder()
            .identityProviderType("AWS_LAMBDA")
            .function(exampleAwsLambdaIdentityProvider.arn())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transfer:Server
    properties:
      identityProviderType: AWS_LAMBDA
      function: ${exampleAwsLambdaIdentityProvider.arn}
```
<!--End PulumiCodeChooser -->

### Protocols

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transfer.Server("example", {
    endpointType: "VPC",
    endpointDetails: {
        subnetIds: [exampleAwsSubnet.id],
        vpcId: exampleAwsVpc.id,
    },
    protocols: [
        "FTP",
        "FTPS",
    ],
    certificate: exampleAwsAcmCertificate.arn,
    identityProviderType: "API_GATEWAY",
    url: `${exampleAwsApiGatewayDeployment.invokeUrl}${exampleAwsApiGatewayResource.path}`,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.Server("example",
    endpoint_type="VPC",
    endpoint_details={
        "subnet_ids": [example_aws_subnet["id"]],
        "vpc_id": example_aws_vpc["id"],
    },
    protocols=[
        "FTP",
        "FTPS",
    ],
    certificate=example_aws_acm_certificate["arn"],
    identity_provider_type="API_GATEWAY",
    url=f"{example_aws_api_gateway_deployment['invokeUrl']}{example_aws_api_gateway_resource['path']}")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transfer.Server("example", new()
    {
        EndpointType = "VPC",
        EndpointDetails = new Aws.Transfer.Inputs.ServerEndpointDetailsArgs
        {
            SubnetIds = new[]
            {
                exampleAwsSubnet.Id,
            },
            VpcId = exampleAwsVpc.Id,
        },
        Protocols = new[]
        {
            "FTP",
            "FTPS",
        },
        Certificate = exampleAwsAcmCertificate.Arn,
        IdentityProviderType = "API_GATEWAY",
        Url = $"{exampleAwsApiGatewayDeployment.InvokeUrl}{exampleAwsApiGatewayResource.Path}",
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.NewServer(ctx, "example", &transfer.ServerArgs{
			EndpointType: pulumi.String("VPC"),
			EndpointDetails: &transfer.ServerEndpointDetailsArgs{
				SubnetIds: pulumi.StringArray{
					exampleAwsSubnet.Id,
				},
				VpcId: pulumi.Any(exampleAwsVpc.Id),
			},
			Protocols: pulumi.StringArray{
				pulumi.String("FTP"),
				pulumi.String("FTPS"),
			},
			Certificate:          pulumi.Any(exampleAwsAcmCertificate.Arn),
			IdentityProviderType: pulumi.String("API_GATEWAY"),
			Url:                  pulumi.Sprintf("%v%v", exampleAwsApiGatewayDeployment.InvokeUrl, exampleAwsApiGatewayResource.Path),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Server;
import com.pulumi.aws.transfer.ServerArgs;
import com.pulumi.aws.transfer.inputs.ServerEndpointDetailsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Server("example", ServerArgs.builder()
            .endpointType("VPC")
            .endpointDetails(ServerEndpointDetailsArgs.builder()
                .subnetIds(exampleAwsSubnet.id())
                .vpcId(exampleAwsVpc.id())
                .build())
            .protocols(            
                "FTP",
                "FTPS")
            .certificate(exampleAwsAcmCertificate.arn())
            .identityProviderType("API_GATEWAY")
            .url(String.format("%s%s", exampleAwsApiGatewayDeployment.invokeUrl(),exampleAwsApiGatewayResource.path()))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transfer:Server
    properties:
      endpointType: VPC
      endpointDetails:
        subnetIds:
          - ${exampleAwsSubnet.id}
        vpcId: ${exampleAwsVpc.id}
      protocols:
        - FTP
        - FTPS
      certificate: ${exampleAwsAcmCertificate.arn}
      identityProviderType: API_GATEWAY
      url: ${exampleAwsApiGatewayDeployment.invokeUrl}${exampleAwsApiGatewayResource.path}
```
<!--End PulumiCodeChooser -->

### Using Structured Logging Destinations

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const transfer = new aws.cloudwatch.LogGroup("transfer", {namePrefix: "transfer_test_"});
const transferAssumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["transfer.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const iamForTransfer = new aws.iam.Role("iam_for_transfer", {
    namePrefix: "iam_for_transfer_",
    assumeRolePolicy: transferAssumeRole.then(transferAssumeRole => transferAssumeRole.json),
    managedPolicyArns: ["arn:aws:iam::aws:policy/service-role/AWSTransferLoggingAccess"],
});
const transferServer = new aws.transfer.Server("transfer", {
    endpointType: "PUBLIC",
    loggingRole: iamForTransfer.arn,
    protocols: ["SFTP"],
    structuredLogDestinations: [pulumi.interpolate`${transfer.arn}:*`],
});
```
```python
import pulumi
import pulumi_aws as aws

transfer = aws.cloudwatch.LogGroup("transfer", name_prefix="transfer_test_")
transfer_assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["transfer.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
iam_for_transfer = aws.iam.Role("iam_for_transfer",
    name_prefix="iam_for_transfer_",
    assume_role_policy=transfer_assume_role.json,
    managed_policy_arns=["arn:aws:iam::aws:policy/service-role/AWSTransferLoggingAccess"])
transfer_server = aws.transfer.Server("transfer",
    endpoint_type="PUBLIC",
    logging_role=iam_for_transfer.arn,
    protocols=["SFTP"],
    structured_log_destinations=[transfer.arn.apply(lambda arn: f"{arn}:*")])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var transfer = new Aws.CloudWatch.LogGroup("transfer", new()
    {
        NamePrefix = "transfer_test_",
    });

    var transferAssumeRole = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "transfer.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
            },
        },
    });

    var iamForTransfer = new Aws.Iam.Role("iam_for_transfer", new()
    {
        NamePrefix = "iam_for_transfer_",
        AssumeRolePolicy = transferAssumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
        ManagedPolicyArns = new[]
        {
            "arn:aws:iam::aws:policy/service-role/AWSTransferLoggingAccess",
        },
    });

    var transferServer = new Aws.Transfer.Server("transfer", new()
    {
        EndpointType = "PUBLIC",
        LoggingRole = iamForTransfer.Arn,
        Protocols = new[]
        {
            "SFTP",
        },
        StructuredLogDestinations = new[]
        {
            transfer.Arn.Apply(arn => $"{arn}:*"),
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudwatch"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		transfer, err := cloudwatch.NewLogGroup(ctx, "transfer", &cloudwatch.LogGroupArgs{
			NamePrefix: pulumi.String("transfer_test_"),
		})
		if err != nil {
			return err
		}
		transferAssumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"transfer.amazonaws.com",
							},
						},
					},
					Actions: []string{
						"sts:AssumeRole",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		iamForTransfer, err := iam.NewRole(ctx, "iam_for_transfer", &iam.RoleArgs{
			NamePrefix:       pulumi.String("iam_for_transfer_"),
			AssumeRolePolicy: pulumi.String(transferAssumeRole.Json),
			ManagedPolicyArns: pulumi.StringArray{
				pulumi.String("arn:aws:iam::aws:policy/service-role/AWSTransferLoggingAccess"),
			},
		})
		if err != nil {
			return err
		}
		_, err = transfer.NewServer(ctx, "transfer", &transfer.ServerArgs{
			EndpointType: pulumi.String("PUBLIC"),
			LoggingRole:  iamForTransfer.Arn,
			Protocols: pulumi.StringArray{
				pulumi.String("SFTP"),
			},
			StructuredLogDestinations: pulumi.StringArray{
				transfer.Arn.ApplyT(func(arn string) (string, error) {
					return fmt.Sprintf("%v:*", arn), nil
				}).(pulumi.StringOutput),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cloudwatch.LogGroup;
import com.pulumi.aws.cloudwatch.LogGroupArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.transfer.Server;
import com.pulumi.aws.transfer.ServerArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var transfer = new LogGroup("transfer", LogGroupArgs.builder()
            .namePrefix("transfer_test_")
            .build());

        final var transferAssumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("transfer.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var iamForTransfer = new Role("iamForTransfer", RoleArgs.builder()
            .namePrefix("iam_for_transfer_")
            .assumeRolePolicy(transferAssumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .managedPolicyArns("arn:aws:iam::aws:policy/service-role/AWSTransferLoggingAccess")
            .build());

        var transferServer = new Server("transferServer", ServerArgs.builder()
            .endpointType("PUBLIC")
            .loggingRole(iamForTransfer.arn())
            .protocols("SFTP")
            .structuredLogDestinations(transfer.arn().applyValue(arn -> String.format("%s:*", arn)))
            .build());

    }
}
```
```yaml
resources:
  transfer:
    type: aws:cloudwatch:LogGroup
    properties:
      namePrefix: transfer_test_
  iamForTransfer:
    type: aws:iam:Role
    name: iam_for_transfer
    properties:
      namePrefix: iam_for_transfer_
      assumeRolePolicy: ${transferAssumeRole.json}
      managedPolicyArns:
        - arn:aws:iam::aws:policy/service-role/AWSTransferLoggingAccess
  transferServer:
    type: aws:transfer:Server
    name: transfer
    properties:
      endpointType: PUBLIC
      loggingRole: ${iamForTransfer.arn}
      protocols:
        - SFTP
      structuredLogDestinations:
        - ${transfer.arn}:*
variables:
  transferAssumeRole:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: Service
                identifiers:
                  - transfer.amazonaws.com
            actions:
              - sts:AssumeRole
```
<!--End PulumiCodeChooser -->

## Import

In Terraform v1.5.0 and later, use an `import` Block to import Transfer Servers using the server `id`. For example:

Using `pulumi import`, import Transfer Servers using the server `id`. For example:

```sh
$ pulumi import aws:transfer/server:Server example s-12345678
```
Certain resource arguments, such as `host_key`, cannot be read via the API and imported into the provider. This provider will display a difference for these arguments the first run after import if declared in the provider configuration for an imported resource.

ö
certificateB" ÑThe Amazon Resource Name (ARN) of the AWS Certificate Manager (ACM) certificate. This is required when `protocols` is set to `FTPS`
ú
directoryIdB" ÜThe directory service ID of the directory service you want to connect to with an `identity_provider_type` of `AWS_DIRECTORY_SERVICE`.
è
domainB" The domain of the storage system that is used for file transfers. Valid values are: `S3` and `EFS`. The default value is `S3`.
ä
endpointDetailsaB_:]
[
transferServerEndpointDetails8aws:transfer/ServerEndpointDetails:ServerEndpointDetailsìThe virtual private cloud (VPC) endpoint settings that you want to configure for your SFTP server. See `endpoint_details` Block below for details.
§
endpointTypeB" çThe type of endpoint that you want your SFTP server connect to. If you connect to a `VPC` (or `VPC_ENDPOINT`), your SFTP server isn't accessible over the public internet. If you want to connect your SFTP server via public internet, set `PUBLIC`.  Defaults to `PUBLIC`.
ö
forceDestroyB
 ÉA boolean that indicates all users associated with the server should be deleted so that the Server can be destroyed without error. The default value is `false`. This option only applies to servers configured with a `SERVICE_MANAGED` `identity_provider_type`.
R
functionB" @The ARN for a lambda function to use for the Identity provider.
è
hostKeyB" ˝RSA, ECDSA, or ED25519 private key (e.g., as generated by the `ssh-keygen -t rsa -b 2048 -N "" -m PEM -f my-new-server-key`, `ssh-keygen -t ecdsa -b 256 -N "" -m PEM -f my-new-server-key` or `ssh-keygen -t ed25519 -N "" -f my-new-server-key` commands).
Î
identityProviderTypeB" ÃThe mode of authentication enabled for this service. The default value is `SERVICE_MANAGED`, which allows you to store and access SFTP user credentials within the service. `API_GATEWAY` indicates that user authentication requires a call to an API Gateway endpoint URL provided by you to integrate an identity provider of your choice. Using `AWS_DIRECTORY_SERVICE` will allow for authentication against AWS Managed Active Directory or Microsoft Active Directory in your on-premises environment, or in AWS using AD Connectors. Use the `AWS_LAMBDA` value to directly use a Lambda function as your identity provider. If you choose this value, you must specify the ARN for the lambda function in the `function` argument.
ù
invocationRoleB" ÑAmazon Resource Name (ARN) of the IAM role used to authenticate the user account with an `identity_provider_type` of `API_GATEWAY`.
¬
loggingRoleB" ¨Amazon Resource Name (ARN) of an IAM role that allows the service to write your SFTP users‚Äô activity to your Amazon CloudWatch logs for monitoring and auditing purposes.
„
postAuthenticationLoginBannerB" ªSpecify a string to display when users connect to a server. This string is displayed after the user authenticates. The SFTP protocol does not support post-authentication display banners.
ö
preAuthenticationLoginBannerB" tSpecify a string to display when users connect to a server. This string is displayed before the user authenticates.
·
protocolDetailsaB_:]
[
transferServerProtocolDetails8aws:transfer/ServerProtocolDetails:ServerProtocolDetailskThe protocol settings that are configured for your server. See `protocol_details` Block below for details.
˙
	protocolsB*" ‰Specifies the file transfer protocol or protocols over which your file transfer protocol client can connect to your server's endpoint. This defaults to `SFTP` . The available protocols are:
* `AS2`: File transfer over Applicability Statement 2
* `SFTP`: File transfer over SSH
* `FTPS`: File transfer with TLS encryption
* `FTP`: Unencrypted file transfer
ú
s3StorageOptionsdBb:`
^
transferServerS3StorageOptions:aws:transfer/ServerS3StorageOptions:ServerS3StorageOptions°Specifies whether or not performance for your Amazon S3 directories is optimized. This is disabled by default. See `s3_storage_options` Block below for details.
ï
securityPolicyNameB" ¯Specifies the name of the security policy that is attached to the server. Default value is: `TransferSecurityPolicy-2018-11`. The available values are:
* `TransferSecurityPolicy-2018-11`
* `TransferSecurityPolicy-2020-06`
* `TransferSecurityPolicy-2022-03`
* `TransferSecurityPolicy-2023-05`
* `TransferSecurityPolicy-2024-01`
* `TransferSecurityPolicy-FIPS-2020-06`
* `TransferSecurityPolicy-FIPS-2023-05`
* `TransferSecurityPolicy-FIPS-2024-01`
* `TransferSecurityPolicy-FIPS-2024-05`
* `TransferSecurityPolicy-PQ-SSH-Experimental-2023-04`
* `TransferSecurityPolicy-PQ-SSH-FIPS-Experimental-2023-04`
* `TransferSecurityPolicy-Restricted-2018-11`
* `TransferSecurityPolicy-Restricted-2020-06`
* `TransferSecurityPolicy-Restricted-2024-06`

See [Security policies for AWS Transfer Family servers](https://docs.aws.amazon.com/transfer/latest/userguide/security-policies.html) for details.
ı
sftpAuthenticationMethodsB" —For SFTP-enabled servers, and for custom identity providers only. Valid values are `PASSWORD`, `PUBLIC_KEY`, `PUBLIC_KEY_OR_PASSWORD` and `PUBLIC_KEY_AND_PASSWORD`. Default value is: `PUBLIC_KEY_OR_PASSWORD`.
É
structuredLogDestinationsB*" ›A set of ARNs of destinations that will receive structured logs from the transfer server such as CloudWatch Log Group ARNs. If provided this enables the transfer server to emit structured logs to the specified locations.
–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
w
urlB" jURL of the service endpoint used to authenticate users with an `identity_provider_type` of `API_GATEWAY`.
∆
workflowDetailsaB_:]
[
transferServerWorkflowDetails8aws:transfer/ServerWorkflowDetails:ServerWorkflowDetailsPSpecifies the workflow details. See `workflow_details` Block below for details.
"9
arn" .Amazon Resource Name (ARN) of Transfer Server
"ö
certificateB" ÑThe Amazon Resource Name (ARN) of the AWS Certificate Manager (ACM) certificate. This is required when `protocols` is set to `FTPS`
"ú
directoryIdB" ÜThe directory service ID of the directory service you want to connect to with an `identity_provider_type` of `AWS_DIRECTORY_SERVICE`.
"è
domainB" The domain of the storage system that is used for file transfers. Valid values are: `S3` and `EFS`. The default value is `S3`.
"n
endpoint" ^The endpoint of the Transfer Server (e.g., `s-12345678.server.transfer.REGION.amazonaws.com`)
"ä
endpointDetailsaB_:]
[
transferServerEndpointDetails8aws:transfer/ServerEndpointDetails:ServerEndpointDetailsìThe virtual private cloud (VPC) endpoint settings that you want to configure for your SFTP server. See `endpoint_details` Block below for details.
"§
endpointTypeB" çThe type of endpoint that you want your SFTP server connect to. If you connect to a `VPC` (or `VPC_ENDPOINT`), your SFTP server isn't accessible over the public internet. If you want to connect your SFTP server via public internet, set `PUBLIC`.  Defaults to `PUBLIC`.
"ö
forceDestroyB
 ÉA boolean that indicates all users associated with the server should be deleted so that the Server can be destroyed without error. The default value is `false`. This option only applies to servers configured with a `SERVICE_MANAGED` `identity_provider_type`.
"R
functionB" @The ARN for a lambda function to use for the Identity provider.
"è
hostKeyB" ˝RSA, ECDSA, or ED25519 private key (e.g., as generated by the `ssh-keygen -t rsa -b 2048 -N "" -m PEM -f my-new-server-key`, `ssh-keygen -t ecdsa -b 256 -N "" -m PEM -f my-new-server-key` or `ssh-keygen -t ed25519 -N "" -f my-new-server-key` commands).
"‘
hostKeyFingerprint" πThis value contains the message-digest algorithm (MD5) hash of the server's host key. This value is equivalent to the output of the `ssh-keygen -l -E md5 -f my-new-server-key` command.
"Î
identityProviderTypeB" ÃThe mode of authentication enabled for this service. The default value is `SERVICE_MANAGED`, which allows you to store and access SFTP user credentials within the service. `API_GATEWAY` indicates that user authentication requires a call to an API Gateway endpoint URL provided by you to integrate an identity provider of your choice. Using `AWS_DIRECTORY_SERVICE` will allow for authentication against AWS Managed Active Directory or Microsoft Active Directory in your on-premises environment, or in AWS using AD Connectors. Use the `AWS_LAMBDA` value to directly use a Lambda function as your identity provider. If you choose this value, you must specify the ARN for the lambda function in the `function` argument.
"ù
invocationRoleB" ÑAmazon Resource Name (ARN) of the IAM role used to authenticate the user account with an `identity_provider_type` of `API_GATEWAY`.
"¬
loggingRoleB" ¨Amazon Resource Name (ARN) of an IAM role that allows the service to write your SFTP users‚Äô activity to your Amazon CloudWatch logs for monitoring and auditing purposes.
"„
postAuthenticationLoginBannerB" ªSpecify a string to display when users connect to a server. This string is displayed after the user authenticates. The SFTP protocol does not support post-authentication display banners.
"ö
preAuthenticationLoginBannerB" tSpecify a string to display when users connect to a server. This string is displayed before the user authenticates.
"ﬂ
protocolDetails_:]
[
transferServerProtocolDetails8aws:transfer/ServerProtocolDetails:ServerProtocolDetailskThe protocol settings that are configured for your server. See `protocol_details` Block below for details.
"¯
	protocols*" ‰Specifies the file transfer protocol or protocols over which your file transfer protocol client can connect to your server's endpoint. This defaults to `SFTP` . The available protocols are:
* `AS2`: File transfer over Applicability Statement 2
* `SFTP`: File transfer over SSH
* `FTPS`: File transfer with TLS encryption
* `FTP`: Unencrypted file transfer
"ö
s3StorageOptionsb:`
^
transferServerS3StorageOptions:aws:transfer/ServerS3StorageOptions:ServerS3StorageOptions°Specifies whether or not performance for your Amazon S3 directories is optimized. This is disabled by default. See `s3_storage_options` Block below for details.
"ï
securityPolicyNameB" ¯Specifies the name of the security policy that is attached to the server. Default value is: `TransferSecurityPolicy-2018-11`. The available values are:
* `TransferSecurityPolicy-2018-11`
* `TransferSecurityPolicy-2020-06`
* `TransferSecurityPolicy-2022-03`
* `TransferSecurityPolicy-2023-05`
* `TransferSecurityPolicy-2024-01`
* `TransferSecurityPolicy-FIPS-2020-06`
* `TransferSecurityPolicy-FIPS-2023-05`
* `TransferSecurityPolicy-FIPS-2024-01`
* `TransferSecurityPolicy-FIPS-2024-05`
* `TransferSecurityPolicy-PQ-SSH-Experimental-2023-04`
* `TransferSecurityPolicy-PQ-SSH-FIPS-Experimental-2023-04`
* `TransferSecurityPolicy-Restricted-2018-11`
* `TransferSecurityPolicy-Restricted-2020-06`
* `TransferSecurityPolicy-Restricted-2024-06`

See [Security policies for AWS Transfer Family servers](https://docs.aws.amazon.com/transfer/latest/userguide/security-policies.html) for details.
"Û
sftpAuthenticationMethods" —For SFTP-enabled servers, and for custom identity providers only. Valid values are `PASSWORD`, `PUBLIC_KEY`, `PUBLIC_KEY_OR_PASSWORD` and `PUBLIC_KEY_AND_PASSWORD`. Default value is: `PUBLIC_KEY_OR_PASSWORD`.
"É
structuredLogDestinationsB*" ›A set of ARNs of destinations that will receive structured logs from the transfer server such as CloudWatch Log Group ARNs. If provided this enables the transfer server to emit structured logs to the specified locations.
"–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"w
urlB" jURL of the service endpoint used to authenticate users with an `identity_provider_type` of `API_GATEWAY`.
"∆
workflowDetailsaB_:]
[
transferServerWorkflowDetails8aws:transfer/ServerWorkflowDetails:ServerWorkflowDetailsPSpecifies the workflow details. See `workflow_details` Block below for details.
*πs
.
transferSshKeyaws:transfer/sshKey:SshKeyºoProvides a AWS Transfer User SSH Key resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";
import * as tls from "@pulumi/tls";

const examplePrivateKey = new tls.PrivateKey("example", {
    algorithm: "RSA",
    rsaBits: 4096,
});
const exampleServer = new aws.transfer.Server("example", {
    identityProviderType: "SERVICE_MANAGED",
    tags: {
        NAME: "tf-acc-test-transfer-server",
    },
});
const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["transfer.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const exampleRole = new aws.iam.Role("example", {
    name: "tf-test-transfer-user-iam-role",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const exampleUser = new aws.transfer.User("example", {
    serverId: exampleServer.id,
    userName: "tftestuser",
    role: exampleRole.arn,
    tags: {
        NAME: "tftestuser",
    },
});
const exampleSshKey = new aws.transfer.SshKey("example", {
    serverId: exampleServer.id,
    userName: exampleUser.userName,
    body: std.trimspaceOutput({
        input: examplePrivateKey.publicKeyOpenssh,
    }).apply(invoke => invoke.result),
});
const example = aws.iam.getPolicyDocument({
    statements: [{
        sid: "AllowFullAccesstoS3",
        effect: "Allow",
        actions: ["s3:*"],
        resources: ["*"],
    }],
});
const exampleRolePolicy = new aws.iam.RolePolicy("example", {
    name: "tf-test-transfer-user-iam-policy",
    role: exampleRole.id,
    policy: example.then(example => example.json),
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std
import pulumi_tls as tls

example_private_key = tls.PrivateKey("example",
    algorithm="RSA",
    rsa_bits=4096)
example_server = aws.transfer.Server("example",
    identity_provider_type="SERVICE_MANAGED",
    tags={
        "NAME": "tf-acc-test-transfer-server",
    })
assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["transfer.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
example_role = aws.iam.Role("example",
    name="tf-test-transfer-user-iam-role",
    assume_role_policy=assume_role.json)
example_user = aws.transfer.User("example",
    server_id=example_server.id,
    user_name="tftestuser",
    role=example_role.arn,
    tags={
        "NAME": "tftestuser",
    })
example_ssh_key = aws.transfer.SshKey("example",
    server_id=example_server.id,
    user_name=example_user.user_name,
    body=std.trimspace_output(input=example_private_key.public_key_openssh).apply(lambda invoke: invoke.result))
example = aws.iam.get_policy_document(statements=[{
    "sid": "AllowFullAccesstoS3",
    "effect": "Allow",
    "actions": ["s3:*"],
    "resources": ["*"],
}])
example_role_policy = aws.iam.RolePolicy("example",
    name="tf-test-transfer-user-iam-policy",
    role=example_role.id,
    policy=example.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;
using Tls = Pulumi.Tls;

return await Deployment.RunAsync(() => 
{
    var examplePrivateKey = new Tls.PrivateKey("example", new()
    {
        Algorithm = "RSA",
        RsaBits = 4096,
    });

    var exampleServer = new Aws.Transfer.Server("example", new()
    {
        IdentityProviderType = "SERVICE_MANAGED",
        Tags = 
        {
            { "NAME", "tf-acc-test-transfer-server" },
        },
    });

    var assumeRole = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "transfer.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
            },
        },
    });

    var exampleRole = new Aws.Iam.Role("example", new()
    {
        Name = "tf-test-transfer-user-iam-role",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var exampleUser = new Aws.Transfer.User("example", new()
    {
        ServerId = exampleServer.Id,
        UserName = "tftestuser",
        Role = exampleRole.Arn,
        Tags = 
        {
            { "NAME", "tftestuser" },
        },
    });

    var exampleSshKey = new Aws.Transfer.SshKey("example", new()
    {
        ServerId = exampleServer.Id,
        UserName = exampleUser.UserName,
        Body = Std.Trimspace.Invoke(new()
        {
            Input = examplePrivateKey.PublicKeyOpenssh,
        }).Apply(invoke => invoke.Result),
    });

    var example = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "AllowFullAccesstoS3",
                Effect = "Allow",
                Actions = new[]
                {
                    "s3:*",
                },
                Resources = new[]
                {
                    "*",
                },
            },
        },
    });

    var exampleRolePolicy = new Aws.Iam.RolePolicy("example", new()
    {
        Name = "tf-test-transfer-user-iam-policy",
        Role = exampleRole.Id,
        Policy = example.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi-tls/sdk/v4/go/tls"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		examplePrivateKey, err := tls.NewPrivateKey(ctx, "example", &tls.PrivateKeyArgs{
			Algorithm: pulumi.String("RSA"),
			RsaBits:   pulumi.Int(4096),
		})
		if err != nil {
			return err
		}
		exampleServer, err := transfer.NewServer(ctx, "example", &transfer.ServerArgs{
			IdentityProviderType: pulumi.String("SERVICE_MANAGED"),
			Tags: pulumi.StringMap{
				"NAME": pulumi.String("tf-acc-test-transfer-server"),
			},
		})
		if err != nil {
			return err
		}
		assumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"transfer.amazonaws.com",
							},
						},
					},
					Actions: []string{
						"sts:AssumeRole",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		exampleRole, err := iam.NewRole(ctx, "example", &iam.RoleArgs{
			Name:             pulumi.String("tf-test-transfer-user-iam-role"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		exampleUser, err := transfer.NewUser(ctx, "example", &transfer.UserArgs{
			ServerId: exampleServer.ID(),
			UserName: pulumi.String("tftestuser"),
			Role:     exampleRole.Arn,
			Tags: pulumi.StringMap{
				"NAME": pulumi.String("tftestuser"),
			},
		})
		if err != nil {
			return err
		}
		_, err = transfer.NewSshKey(ctx, "example", &transfer.SshKeyArgs{
			ServerId: exampleServer.ID(),
			UserName: exampleUser.UserName,
			Body: pulumi.String(std.TrimspaceOutput(ctx, std.TrimspaceOutputArgs{
				Input: examplePrivateKey.PublicKeyOpenssh,
			}, nil).ApplyT(func(invoke std.TrimspaceResult) (*string, error) {
				return invoke.Result, nil
			}).(pulumi.StringPtrOutput)),
		})
		if err != nil {
			return err
		}
		example, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Sid:    pulumi.StringRef("AllowFullAccesstoS3"),
					Effect: pulumi.StringRef("Allow"),
					Actions: []string{
						"s3:*",
					},
					Resources: []string{
						"*",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = iam.NewRolePolicy(ctx, "example", &iam.RolePolicyArgs{
			Name:   pulumi.String("tf-test-transfer-user-iam-policy"),
			Role:   exampleRole.ID(),
			Policy: pulumi.String(example.Json),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.tls.PrivateKey;
import com.pulumi.tls.PrivateKeyArgs;
import com.pulumi.aws.transfer.Server;
import com.pulumi.aws.transfer.ServerArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.transfer.User;
import com.pulumi.aws.transfer.UserArgs;
import com.pulumi.aws.transfer.SshKey;
import com.pulumi.aws.transfer.SshKeyArgs;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var examplePrivateKey = new PrivateKey("examplePrivateKey", PrivateKeyArgs.builder()
            .algorithm("RSA")
            .rsaBits(4096)
            .build());

        var exampleServer = new Server("exampleServer", ServerArgs.builder()
            .identityProviderType("SERVICE_MANAGED")
            .tags(Map.of("NAME", "tf-acc-test-transfer-server"))
            .build());

        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("transfer.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var exampleRole = new Role("exampleRole", RoleArgs.builder()
            .name("tf-test-transfer-user-iam-role")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var exampleUser = new User("exampleUser", UserArgs.builder()
            .serverId(exampleServer.id())
            .userName("tftestuser")
            .role(exampleRole.arn())
            .tags(Map.of("NAME", "tftestuser"))
            .build());

        var exampleSshKey = new SshKey("exampleSshKey", SshKeyArgs.builder()
            .serverId(exampleServer.id())
            .userName(exampleUser.userName())
            .body(StdFunctions.trimspace().applyValue(invoke -> invoke.result()))
            .build());

        final var example = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("AllowFullAccesstoS3")
                .effect("Allow")
                .actions("s3:*")
                .resources("*")
                .build())
            .build());

        var exampleRolePolicy = new RolePolicy("exampleRolePolicy", RolePolicyArgs.builder()
            .name("tf-test-transfer-user-iam-policy")
            .role(exampleRole.id())
            .policy(example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

    }
}
```
```yaml
resources:
  examplePrivateKey:
    type: tls:PrivateKey
    name: example
    properties:
      algorithm: RSA
      rsaBits: 4096
  exampleSshKey:
    type: aws:transfer:SshKey
    name: example
    properties:
      serverId: ${exampleServer.id}
      userName: ${exampleUser.userName}
      body:
        fn::invoke:
          function: std:trimspace
          arguments:
            input: ${examplePrivateKey.publicKeyOpenssh}
          return: result
  exampleServer:
    type: aws:transfer:Server
    name: example
    properties:
      identityProviderType: SERVICE_MANAGED
      tags:
        NAME: tf-acc-test-transfer-server
  exampleUser:
    type: aws:transfer:User
    name: example
    properties:
      serverId: ${exampleServer.id}
      userName: tftestuser
      role: ${exampleRole.arn}
      tags:
        NAME: tftestuser
  exampleRole:
    type: aws:iam:Role
    name: example
    properties:
      name: tf-test-transfer-user-iam-role
      assumeRolePolicy: ${assumeRole.json}
  exampleRolePolicy:
    type: aws:iam:RolePolicy
    name: example
    properties:
      name: tf-test-transfer-user-iam-policy
      role: ${exampleRole.id}
      policy: ${example.json}
variables:
  assumeRole:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: Service
                identifiers:
                  - transfer.amazonaws.com
            actions:
              - sts:AssumeRole
  example:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: AllowFullAccesstoS3
            effect: Allow
            actions:
              - s3:*
            resources:
              - '*'
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Transfer SSH Public Key using the `server_id` and `user_name` and `ssh_public_key_id` separated by `/`. For example:

```sh
$ pulumi import aws:transfer/sshKey:SshKey bar s-12345678/test-username/key-12345
```
7
body" +The public key portion of an SSH key pair.
J
serverId" :The Server ID of the Transfer Server (e.g., `s-12345678`)
V
userName" FThe name of the user account that is assigned to one or more servers.
"7
body" +The public key portion of an SSH key pair.
"J
serverId" :The Server ID of the Transfer Server (e.g., `s-12345678`)
"
sshKeyId" "V
userName" FThe name of the user account that is assigned to one or more servers.
*Ò*
%
transferTagaws:transfer/tag:Tagµ(Manages an individual Transfer Family resource tag. This resource should only be used in cases where Transfer Family resources are created outside the provider (e.g., Servers without AWS Management Console) or the tag key has the `aws:` prefix.

> **NOTE:** This tagging resource should not be combined with the resource for managing the parent resource. For example, using `aws.transfer.Server` and `aws.transfer.Tag` to manage tags of the same server will cause a perpetual difference where the `aws.transfer.Server` resource will try to remove the tag being added by the `aws.transfer.Tag` resource.

> **NOTE:** This tagging resource does not use the provider `ignore_tags` configuration.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transfer.Server("example", {identityProviderType: "SERVICE_MANAGED"});
const zoneId = new aws.transfer.Tag("zone_id", {
    resourceArn: example.arn,
    key: "aws:transfer:route53HostedZoneId",
    value: "/hostedzone/MyHostedZoneId",
});
const hostname = new aws.transfer.Tag("hostname", {
    resourceArn: example.arn,
    key: "aws:transfer:customHostname",
    value: "example.com",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.Server("example", identity_provider_type="SERVICE_MANAGED")
zone_id = aws.transfer.Tag("zone_id",
    resource_arn=example.arn,
    key="aws:transfer:route53HostedZoneId",
    value="/hostedzone/MyHostedZoneId")
hostname = aws.transfer.Tag("hostname",
    resource_arn=example.arn,
    key="aws:transfer:customHostname",
    value="example.com")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transfer.Server("example", new()
    {
        IdentityProviderType = "SERVICE_MANAGED",
    });

    var zoneId = new Aws.Transfer.Tag("zone_id", new()
    {
        ResourceArn = example.Arn,
        Key = "aws:transfer:route53HostedZoneId",
        Value = "/hostedzone/MyHostedZoneId",
    });

    var hostname = new Aws.Transfer.Tag("hostname", new()
    {
        ResourceArn = example.Arn,
        Key = "aws:transfer:customHostname",
        Value = "example.com",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := transfer.NewServer(ctx, "example", &transfer.ServerArgs{
			IdentityProviderType: pulumi.String("SERVICE_MANAGED"),
		})
		if err != nil {
			return err
		}
		_, err = transfer.NewTag(ctx, "zone_id", &transfer.TagArgs{
			ResourceArn: example.Arn,
			Key:         pulumi.String("aws:transfer:route53HostedZoneId"),
			Value:       pulumi.String("/hostedzone/MyHostedZoneId"),
		})
		if err != nil {
			return err
		}
		_, err = transfer.NewTag(ctx, "hostname", &transfer.TagArgs{
			ResourceArn: example.Arn,
			Key:         pulumi.String("aws:transfer:customHostname"),
			Value:       pulumi.String("example.com"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Server;
import com.pulumi.aws.transfer.ServerArgs;
import com.pulumi.aws.transfer.Tag;
import com.pulumi.aws.transfer.TagArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Server("example", ServerArgs.builder()
            .identityProviderType("SERVICE_MANAGED")
            .build());

        var zoneId = new Tag("zoneId", TagArgs.builder()
            .resourceArn(example.arn())
            .key("aws:transfer:route53HostedZoneId")
            .value("/hostedzone/MyHostedZoneId")
            .build());

        var hostname = new Tag("hostname", TagArgs.builder()
            .resourceArn(example.arn())
            .key("aws:transfer:customHostname")
            .value("example.com")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transfer:Server
    properties:
      identityProviderType: SERVICE_MANAGED
  zoneId:
    type: aws:transfer:Tag
    name: zone_id
    properties:
      resourceArn: ${example.arn}
      key: aws:transfer:route53HostedZoneId
      value: /hostedzone/MyHostedZoneId
  hostname:
    type: aws:transfer:Tag
    properties:
      resourceArn: ${example.arn}
      key: aws:transfer:customHostname
      value: example.com
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_transfer_tag` using the Transfer Family resource identifier and key, separated by a comma (`,`). For example:

```sh
$ pulumi import aws:transfer/tag:Tag example arn:aws:transfer:us-east-1:123456789012:server/s-1234567890abcdef0,Name
```

key" 
Tag name.
V
resourceArn" CAmazon Resource Name (ARN) of the Transfer Family resource to tag.

value" Tag value.
"
key" 
Tag name.
"V
resourceArn" CAmazon Resource Name (ARN) of the Transfer Family resource to tag.
"
value" Tag value.
*À~
(
transferUseraws:transfer/user:Userπ\## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const fooServer = new aws.transfer.Server("foo", {
    identityProviderType: "SERVICE_MANAGED",
    tags: {
        NAME: "tf-acc-test-transfer-server",
    },
});
const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["transfer.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const fooRole = new aws.iam.Role("foo", {
    name: "tf-test-transfer-user-iam-role",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const foo = aws.iam.getPolicyDocument({
    statements: [{
        sid: "AllowFullAccesstoS3",
        effect: "Allow",
        actions: ["s3:*"],
        resources: ["*"],
    }],
});
const fooRolePolicy = new aws.iam.RolePolicy("foo", {
    name: "tf-test-transfer-user-iam-policy",
    role: fooRole.id,
    policy: foo.then(foo => foo.json),
});
const fooUser = new aws.transfer.User("foo", {
    serverId: fooServer.id,
    userName: "tftestuser",
    role: fooRole.arn,
    homeDirectoryType: "LOGICAL",
    homeDirectoryMappings: [{
        entry: "/test.pdf",
        target: "/bucket3/test-path/tftestuser.pdf",
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

foo_server = aws.transfer.Server("foo",
    identity_provider_type="SERVICE_MANAGED",
    tags={
        "NAME": "tf-acc-test-transfer-server",
    })
assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["transfer.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
foo_role = aws.iam.Role("foo",
    name="tf-test-transfer-user-iam-role",
    assume_role_policy=assume_role.json)
foo = aws.iam.get_policy_document(statements=[{
    "sid": "AllowFullAccesstoS3",
    "effect": "Allow",
    "actions": ["s3:*"],
    "resources": ["*"],
}])
foo_role_policy = aws.iam.RolePolicy("foo",
    name="tf-test-transfer-user-iam-policy",
    role=foo_role.id,
    policy=foo.json)
foo_user = aws.transfer.User("foo",
    server_id=foo_server.id,
    user_name="tftestuser",
    role=foo_role.arn,
    home_directory_type="LOGICAL",
    home_directory_mappings=[{
        "entry": "/test.pdf",
        "target": "/bucket3/test-path/tftestuser.pdf",
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var fooServer = new Aws.Transfer.Server("foo", new()
    {
        IdentityProviderType = "SERVICE_MANAGED",
        Tags = 
        {
            { "NAME", "tf-acc-test-transfer-server" },
        },
    });

    var assumeRole = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "Service",
                        Identifiers = new[]
                        {
                            "transfer.amazonaws.com",
                        },
                    },
                },
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
            },
        },
    });

    var fooRole = new Aws.Iam.Role("foo", new()
    {
        Name = "tf-test-transfer-user-iam-role",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var foo = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "AllowFullAccesstoS3",
                Effect = "Allow",
                Actions = new[]
                {
                    "s3:*",
                },
                Resources = new[]
                {
                    "*",
                },
            },
        },
    });

    var fooRolePolicy = new Aws.Iam.RolePolicy("foo", new()
    {
        Name = "tf-test-transfer-user-iam-policy",
        Role = fooRole.Id,
        Policy = foo.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var fooUser = new Aws.Transfer.User("foo", new()
    {
        ServerId = fooServer.Id,
        UserName = "tftestuser",
        Role = fooRole.Arn,
        HomeDirectoryType = "LOGICAL",
        HomeDirectoryMappings = new[]
        {
            new Aws.Transfer.Inputs.UserHomeDirectoryMappingArgs
            {
                Entry = "/test.pdf",
                Target = "/bucket3/test-path/tftestuser.pdf",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		fooServer, err := transfer.NewServer(ctx, "foo", &transfer.ServerArgs{
			IdentityProviderType: pulumi.String("SERVICE_MANAGED"),
			Tags: pulumi.StringMap{
				"NAME": pulumi.String("tf-acc-test-transfer-server"),
			},
		})
		if err != nil {
			return err
		}
		assumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"transfer.amazonaws.com",
							},
						},
					},
					Actions: []string{
						"sts:AssumeRole",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		fooRole, err := iam.NewRole(ctx, "foo", &iam.RoleArgs{
			Name:             pulumi.String("tf-test-transfer-user-iam-role"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		foo, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Sid:    pulumi.StringRef("AllowFullAccesstoS3"),
					Effect: pulumi.StringRef("Allow"),
					Actions: []string{
						"s3:*",
					},
					Resources: []string{
						"*",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = iam.NewRolePolicy(ctx, "foo", &iam.RolePolicyArgs{
			Name:   pulumi.String("tf-test-transfer-user-iam-policy"),
			Role:   fooRole.ID(),
			Policy: pulumi.String(foo.Json),
		})
		if err != nil {
			return err
		}
		_, err = transfer.NewUser(ctx, "foo", &transfer.UserArgs{
			ServerId:          fooServer.ID(),
			UserName:          pulumi.String("tftestuser"),
			Role:              fooRole.Arn,
			HomeDirectoryType: pulumi.String("LOGICAL"),
			HomeDirectoryMappings: transfer.UserHomeDirectoryMappingArray{
				&transfer.UserHomeDirectoryMappingArgs{
					Entry:  pulumi.String("/test.pdf"),
					Target: pulumi.String("/bucket3/test-path/tftestuser.pdf"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Server;
import com.pulumi.aws.transfer.ServerArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
import com.pulumi.aws.transfer.User;
import com.pulumi.aws.transfer.UserArgs;
import com.pulumi.aws.transfer.inputs.UserHomeDirectoryMappingArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var fooServer = new Server("fooServer", ServerArgs.builder()
            .identityProviderType("SERVICE_MANAGED")
            .tags(Map.of("NAME", "tf-acc-test-transfer-server"))
            .build());

        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("transfer.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var fooRole = new Role("fooRole", RoleArgs.builder()
            .name("tf-test-transfer-user-iam-role")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        final var foo = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("AllowFullAccesstoS3")
                .effect("Allow")
                .actions("s3:*")
                .resources("*")
                .build())
            .build());

        var fooRolePolicy = new RolePolicy("fooRolePolicy", RolePolicyArgs.builder()
            .name("tf-test-transfer-user-iam-policy")
            .role(fooRole.id())
            .policy(foo.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var fooUser = new User("fooUser", UserArgs.builder()
            .serverId(fooServer.id())
            .userName("tftestuser")
            .role(fooRole.arn())
            .homeDirectoryType("LOGICAL")
            .homeDirectoryMappings(UserHomeDirectoryMappingArgs.builder()
                .entry("/test.pdf")
                .target("/bucket3/test-path/tftestuser.pdf")
                .build())
            .build());

    }
}
```
```yaml
resources:
  fooServer:
    type: aws:transfer:Server
    name: foo
    properties:
      identityProviderType: SERVICE_MANAGED
      tags:
        NAME: tf-acc-test-transfer-server
  fooRole:
    type: aws:iam:Role
    name: foo
    properties:
      name: tf-test-transfer-user-iam-role
      assumeRolePolicy: ${assumeRole.json}
  fooRolePolicy:
    type: aws:iam:RolePolicy
    name: foo
    properties:
      name: tf-test-transfer-user-iam-policy
      role: ${fooRole.id}
      policy: ${foo.json}
  fooUser:
    type: aws:transfer:User
    name: foo
    properties:
      serverId: ${fooServer.id}
      userName: tftestuser
      role: ${fooRole.arn}
      homeDirectoryType: LOGICAL
      homeDirectoryMappings:
        - entry: /test.pdf
          target: /bucket3/test-path/tftestuser.pdf
variables:
  assumeRole:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: Service
                identifiers:
                  - transfer.amazonaws.com
            actions:
              - sts:AssumeRole
  foo:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: AllowFullAccesstoS3
            effect: Allow
            actions:
              - s3:*
            resources:
              - '*'
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Transfer Users using the `server_id` and `user_name` separated by `/`. For example:

```sh
$ pulumi import aws:transfer/user:User bar s-12345678/test-username
```
Ì
homeDirectoryB" ’The landing directory (folder) for a user when they log in to the server using their SFTP client.  It should begin with a `/`.  The first item in the path is the name of the home bucket (accessible as `${Transfer:HomeBucket}` in the policy) and the rest is the home directory (accessible as `${Transfer:HomeDirectory}` in the policy). For example, `/example-bucket-1234/username` would set the home bucket to `example-bucket-1234` and the home directory to `username`.
∞
homeDirectoryMappingslBj*h:f
d
transferUserHomeDirectoryMapping>aws:transfer/UserHomeDirectoryMapping:UserHomeDirectoryMapping®Logical directory mappings that specify what S3 paths and keys should be visible to your user and how you want to make them visible. See Home Directory Mappings below.
î
homeDirectoryTypeB" yThe type of landing directory (folder) you mapped for your users' home directory. Valid values are `PATH` and `LOGICAL`.
¥
policyB" £An IAM JSON policy document that scopes down user access to portions of their Amazon S3 bucket. IAM variables you can use inside this policy include `${Transfer:UserName}`, `${Transfer:HomeDirectory}`, and `${Transfer:HomeBucket}`. These are evaluated on-the-fly when navigating the bucket.
∫
posixProfileRBP:N
L
transferUserPosixProfile.aws:transfer/UserPosixProfile:UserPosixProfile’Specifies the full POSIX identity, including user ID (Uid), group ID (Gid), and any secondary groups IDs (SecondaryGids), that controls your users' access to your Amazon EFS file systems. See Posix Profile below.
à
role" |Amazon Resource Name (ARN) of an IAM role that allows the service to control your user‚Äôs access to your Amazon S3 bucket.
J
serverId" :The Server ID of the Transfer Server (e.g., `s-12345678`)
»
tagsB2" ∑A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block, tags with matching keys will overwrite those defined at the provider-level.
>
userName" .The name used for log in to your SFTP server.
"7
arn" ,Amazon Resource Name (ARN) of Transfer User
"Ì
homeDirectoryB" ’The landing directory (folder) for a user when they log in to the server using their SFTP client.  It should begin with a `/`.  The first item in the path is the name of the home bucket (accessible as `${Transfer:HomeBucket}` in the policy) and the rest is the home directory (accessible as `${Transfer:HomeDirectory}` in the policy). For example, `/example-bucket-1234/username` would set the home bucket to `example-bucket-1234` and the home directory to `username`.
"∞
homeDirectoryMappingslBj*h:f
d
transferUserHomeDirectoryMapping>aws:transfer/UserHomeDirectoryMapping:UserHomeDirectoryMapping®Logical directory mappings that specify what S3 paths and keys should be visible to your user and how you want to make them visible. See Home Directory Mappings below.
"î
homeDirectoryTypeB" yThe type of landing directory (folder) you mapped for your users' home directory. Valid values are `PATH` and `LOGICAL`.
"¥
policyB" £An IAM JSON policy document that scopes down user access to portions of their Amazon S3 bucket. IAM variables you can use inside this policy include `${Transfer:UserName}`, `${Transfer:HomeDirectory}`, and `${Transfer:HomeBucket}`. These are evaluated on-the-fly when navigating the bucket.
"∫
posixProfileRBP:N
L
transferUserPosixProfile.aws:transfer/UserPosixProfile:UserPosixProfile’Specifies the full POSIX identity, including user ID (Uid), group ID (Gid), and any secondary groups IDs (SecondaryGids), that controls your users' access to your Amazon EFS file systems. See Posix Profile below.
"à
role" |Amazon Resource Name (ARN) of an IAM role that allows the service to control your user‚Äôs access to your Amazon S3 bucket.
"J
serverId" :The Server ID of the Transfer Server (e.g., `s-12345678`)
"»
tagsB2" ∑A map of tags to assign to the resource. If configured with a provider `default_tags` configuration block, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
">
userName" .The name used for log in to your SFTP server.
*≥X
4
transferWorkflowaws:transfer/workflow:Workflow√KProvides a AWS Transfer Workflow resource.

## Example Usage

### Basic single step example

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transfer.Workflow("example", {steps: [{
    deleteStepDetails: {
        name: "example",
        sourceFileLocation: "${original.file}",
    },
    type: "DELETE",
}]});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.Workflow("example", steps=[{
    "delete_step_details": {
        "name": "example",
        "source_file_location": "${original.file}",
    },
    "type": "DELETE",
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transfer.Workflow("example", new()
    {
        Steps = new[]
        {
            new Aws.Transfer.Inputs.WorkflowStepArgs
            {
                DeleteStepDetails = new Aws.Transfer.Inputs.WorkflowStepDeleteStepDetailsArgs
                {
                    Name = "example",
                    SourceFileLocation = "${original.file}",
                },
                Type = "DELETE",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.NewWorkflow(ctx, "example", &transfer.WorkflowArgs{
			Steps: transfer.WorkflowStepArray{
				&transfer.WorkflowStepArgs{
					DeleteStepDetails: &transfer.WorkflowStepDeleteStepDetailsArgs{
						Name:               pulumi.String("example"),
						SourceFileLocation: pulumi.String("${original.file}"),
					},
					Type: pulumi.String("DELETE"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Workflow;
import com.pulumi.aws.transfer.WorkflowArgs;
import com.pulumi.aws.transfer.inputs.WorkflowStepArgs;
import com.pulumi.aws.transfer.inputs.WorkflowStepDeleteStepDetailsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Workflow("example", WorkflowArgs.builder()
            .steps(WorkflowStepArgs.builder()
                .deleteStepDetails(WorkflowStepDeleteStepDetailsArgs.builder()
                    .name("example")
                    .sourceFileLocation("${original.file}")
                    .build())
                .type("DELETE")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transfer:Workflow
    properties:
      steps:
        - deleteStepDetails:
            name: example
            sourceFileLocation: $${original.file}
          type: DELETE
```
<!--End PulumiCodeChooser -->

### Multistep example

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.transfer.Workflow("example", {steps: [
    {
        customStepDetails: {
            name: "example",
            sourceFileLocation: "${original.file}",
            target: exampleAwsLambdaFunction.arn,
            timeoutSeconds: 60,
        },
        type: "CUSTOM",
    },
    {
        tagStepDetails: {
            name: "example",
            sourceFileLocation: "${original.file}",
            tags: [{
                key: "Name",
                value: "Hello World",
            }],
        },
        type: "TAG",
    },
]});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.Workflow("example", steps=[
    {
        "custom_step_details": {
            "name": "example",
            "source_file_location": "${original.file}",
            "target": example_aws_lambda_function["arn"],
            "timeout_seconds": 60,
        },
        "type": "CUSTOM",
    },
    {
        "tag_step_details": {
            "name": "example",
            "source_file_location": "${original.file}",
            "tags": [{
                "key": "Name",
                "value": "Hello World",
            }],
        },
        "type": "TAG",
    },
])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Transfer.Workflow("example", new()
    {
        Steps = new[]
        {
            new Aws.Transfer.Inputs.WorkflowStepArgs
            {
                CustomStepDetails = new Aws.Transfer.Inputs.WorkflowStepCustomStepDetailsArgs
                {
                    Name = "example",
                    SourceFileLocation = "${original.file}",
                    Target = exampleAwsLambdaFunction.Arn,
                    TimeoutSeconds = 60,
                },
                Type = "CUSTOM",
            },
            new Aws.Transfer.Inputs.WorkflowStepArgs
            {
                TagStepDetails = new Aws.Transfer.Inputs.WorkflowStepTagStepDetailsArgs
                {
                    Name = "example",
                    SourceFileLocation = "${original.file}",
                    Tags = new[]
                    {
                        new Aws.Transfer.Inputs.WorkflowStepTagStepDetailsTagArgs
                        {
                            Key = "Name",
                            Value = "Hello World",
                        },
                    },
                },
                Type = "TAG",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.NewWorkflow(ctx, "example", &transfer.WorkflowArgs{
			Steps: transfer.WorkflowStepArray{
				&transfer.WorkflowStepArgs{
					CustomStepDetails: &transfer.WorkflowStepCustomStepDetailsArgs{
						Name:               pulumi.String("example"),
						SourceFileLocation: pulumi.String("${original.file}"),
						Target:             pulumi.Any(exampleAwsLambdaFunction.Arn),
						TimeoutSeconds:     pulumi.Int(60),
					},
					Type: pulumi.String("CUSTOM"),
				},
				&transfer.WorkflowStepArgs{
					TagStepDetails: &transfer.WorkflowStepTagStepDetailsArgs{
						Name:               pulumi.String("example"),
						SourceFileLocation: pulumi.String("${original.file}"),
						Tags: transfer.WorkflowStepTagStepDetailsTagArray{
							&transfer.WorkflowStepTagStepDetailsTagArgs{
								Key:   pulumi.String("Name"),
								Value: pulumi.String("Hello World"),
							},
						},
					},
					Type: pulumi.String("TAG"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.Workflow;
import com.pulumi.aws.transfer.WorkflowArgs;
import com.pulumi.aws.transfer.inputs.WorkflowStepArgs;
import com.pulumi.aws.transfer.inputs.WorkflowStepCustomStepDetailsArgs;
import com.pulumi.aws.transfer.inputs.WorkflowStepTagStepDetailsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Workflow("example", WorkflowArgs.builder()
            .steps(            
                WorkflowStepArgs.builder()
                    .customStepDetails(WorkflowStepCustomStepDetailsArgs.builder()
                        .name("example")
                        .sourceFileLocation("${original.file}")
                        .target(exampleAwsLambdaFunction.arn())
                        .timeoutSeconds(60)
                        .build())
                    .type("CUSTOM")
                    .build(),
                WorkflowStepArgs.builder()
                    .tagStepDetails(WorkflowStepTagStepDetailsArgs.builder()
                        .name("example")
                        .sourceFileLocation("${original.file}")
                        .tags(WorkflowStepTagStepDetailsTagArgs.builder()
                            .key("Name")
                            .value("Hello World")
                            .build())
                        .build())
                    .type("TAG")
                    .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:transfer:Workflow
    properties:
      steps:
        - customStepDetails:
            name: example
            sourceFileLocation: $${original.file}
            target: ${exampleAwsLambdaFunction.arn}
            timeoutSeconds: 60
          type: CUSTOM
        - tagStepDetails:
            name: example
            sourceFileLocation: $${original.file}
            tags:
              - key: Name
                value: Hello World
          type: TAG
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Transfer Workflows using the `worflow_id`. For example:

```sh
$ pulumi import aws:transfer/workflow:Workflow example example
```
=
descriptionB" (A textual description for the workflow.
˚
onExceptionStepsiBg*e:c
a
transferWorkflowOnExceptionStep<aws:transfer/WorkflowOnExceptionStep:WorkflowOnExceptionStep|Specifies the steps (actions) to take if errors are encountered during execution of the workflow. See Workflow Steps below.
≥
stepsF*D:B
@
transferWorkflowStep&aws:transfer/WorkflowStep:WorkflowStepbSpecifies the details for the steps that are in the specified workflow. See Workflow Steps below.
–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
arn" The Workflow ARN.
"=
descriptionB" (A textual description for the workflow.
"˚
onExceptionStepsiBg*e:c
a
transferWorkflowOnExceptionStep<aws:transfer/WorkflowOnExceptionStep:WorkflowOnExceptionStep|Specifies the steps (actions) to take if errors are encountered during execution of the workflow. See Workflow Steps below.
"≥
stepsF*D:B
@
transferWorkflowStep&aws:transfer/WorkflowStep:WorkflowStepbSpecifies the details for the steps that are in the specified workflow. See Workflow Steps below.
"–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*î_
@
verifiedaccessEndpoint$aws:verifiedaccess/endpoint:EndpointÊ=Resource for managing an AWS EC2 (Elastic Compute Cloud) Verified Access Endpoint.

## Example Usage

### ALB Example

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedaccess.Endpoint("example", {
    applicationDomain: "example.com",
    attachmentType: "vpc",
    description: "example",
    domainCertificateArn: exampleAwsAcmCertificate.arn,
    endpointDomainPrefix: "example",
    endpointType: "load-balancer",
    loadBalancerOptions: {
        loadBalancerArn: exampleAwsLb.arn,
        port: 443,
        protocol: "https",
        subnetIds: .map(subnet => (subnet.id)),
    },
    securityGroupIds: [exampleAwsSecurityGroup.id],
    verifiedAccessGroupId: exampleAwsVerifiedaccessGroup.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedaccess.Endpoint("example",
    application_domain="example.com",
    attachment_type="vpc",
    description="example",
    domain_certificate_arn=example_aws_acm_certificate["arn"],
    endpoint_domain_prefix="example",
    endpoint_type="load-balancer",
    load_balancer_options={
        "load_balancer_arn": example_aws_lb["arn"],
        "port": 443,
        "protocol": "https",
        "subnet_ids": [subnet["id"] for subnet in public],
    },
    security_group_ids=[example_aws_security_group["id"]],
    verified_access_group_id=example_aws_verifiedaccess_group["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedAccess.Endpoint("example", new()
    {
        ApplicationDomain = "example.com",
        AttachmentType = "vpc",
        Description = "example",
        DomainCertificateArn = exampleAwsAcmCertificate.Arn,
        EndpointDomainPrefix = "example",
        EndpointType = "load-balancer",
        LoadBalancerOptions = new Aws.VerifiedAccess.Inputs.EndpointLoadBalancerOptionsArgs
        {
            LoadBalancerArn = exampleAwsLb.Arn,
            Port = 443,
            Protocol = "https",
            SubnetIds = .Select(subnet => 
            {
                return subnet.Id;
            }).ToList(),
        },
        SecurityGroupIds = new[]
        {
            exampleAwsSecurityGroup.Id,
        },
        VerifiedAccessGroupId = exampleAwsVerifiedaccessGroup.Id,
    });

});
```
<!--End PulumiCodeChooser -->

### Network Interface Example

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedaccess.Endpoint("example", {
    applicationDomain: "example.com",
    attachmentType: "vpc",
    description: "example",
    domainCertificateArn: exampleAwsAcmCertificate.arn,
    endpointDomainPrefix: "example",
    endpointType: "network-interface",
    networkInterfaceOptions: {
        networkInterfaceId: exampleAwsNetworkInterface.id,
        port: 443,
        protocol: "https",
    },
    securityGroupIds: [exampleAwsSecurityGroup.id],
    verifiedAccessGroupId: exampleAwsVerifiedaccessGroup.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedaccess.Endpoint("example",
    application_domain="example.com",
    attachment_type="vpc",
    description="example",
    domain_certificate_arn=example_aws_acm_certificate["arn"],
    endpoint_domain_prefix="example",
    endpoint_type="network-interface",
    network_interface_options={
        "network_interface_id": example_aws_network_interface["id"],
        "port": 443,
        "protocol": "https",
    },
    security_group_ids=[example_aws_security_group["id"]],
    verified_access_group_id=example_aws_verifiedaccess_group["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedAccess.Endpoint("example", new()
    {
        ApplicationDomain = "example.com",
        AttachmentType = "vpc",
        Description = "example",
        DomainCertificateArn = exampleAwsAcmCertificate.Arn,
        EndpointDomainPrefix = "example",
        EndpointType = "network-interface",
        NetworkInterfaceOptions = new Aws.VerifiedAccess.Inputs.EndpointNetworkInterfaceOptionsArgs
        {
            NetworkInterfaceId = exampleAwsNetworkInterface.Id,
            Port = 443,
            Protocol = "https",
        },
        SecurityGroupIds = new[]
        {
            exampleAwsSecurityGroup.Id,
        },
        VerifiedAccessGroupId = exampleAwsVerifiedaccessGroup.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedaccess.NewEndpoint(ctx, "example", &verifiedaccess.EndpointArgs{
			ApplicationDomain:    pulumi.String("example.com"),
			AttachmentType:       pulumi.String("vpc"),
			Description:          pulumi.String("example"),
			DomainCertificateArn: pulumi.Any(exampleAwsAcmCertificate.Arn),
			EndpointDomainPrefix: pulumi.String("example"),
			EndpointType:         pulumi.String("network-interface"),
			NetworkInterfaceOptions: &verifiedaccess.EndpointNetworkInterfaceOptionsArgs{
				NetworkInterfaceId: pulumi.Any(exampleAwsNetworkInterface.Id),
				Port:               pulumi.Int(443),
				Protocol:           pulumi.String("https"),
			},
			SecurityGroupIds: pulumi.StringArray{
				exampleAwsSecurityGroup.Id,
			},
			VerifiedAccessGroupId: pulumi.Any(exampleAwsVerifiedaccessGroup.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedaccess.Endpoint;
import com.pulumi.aws.verifiedaccess.EndpointArgs;
import com.pulumi.aws.verifiedaccess.inputs.EndpointNetworkInterfaceOptionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Endpoint("example", EndpointArgs.builder()
            .applicationDomain("example.com")
            .attachmentType("vpc")
            .description("example")
            .domainCertificateArn(exampleAwsAcmCertificate.arn())
            .endpointDomainPrefix("example")
            .endpointType("network-interface")
            .networkInterfaceOptions(EndpointNetworkInterfaceOptionsArgs.builder()
                .networkInterfaceId(exampleAwsNetworkInterface.id())
                .port(443)
                .protocol("https")
                .build())
            .securityGroupIds(exampleAwsSecurityGroup.id())
            .verifiedAccessGroupId(exampleAwsVerifiedaccessGroup.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedaccess:Endpoint
    properties:
      applicationDomain: example.com
      attachmentType: vpc
      description: example
      domainCertificateArn: ${exampleAwsAcmCertificate.arn}
      endpointDomainPrefix: example
      endpointType: network-interface
      networkInterfaceOptions:
        networkInterfaceId: ${exampleAwsNetworkInterface.id}
        port: 443
        protocol: https
      securityGroupIds:
        - ${exampleAwsSecurityGroup.id}
      verifiedAccessGroupId: ${exampleAwsVerifiedaccessGroup.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Verified Access Instances using the  `id`. For example:

```sh
$ pulumi import aws:verifiedaccess/endpoint:Endpoint example vae-8012925589
```
K
applicationDomain" 2The DNS name for users to reach your application.
R
attachmentType" <The type of attachment. Currently, only `vpc` is supported.
E
descriptionB" 0A description for the Verified Access endpoint.
Á
domainCertificateArn"  The ARN of the public TLS/SSL certificate in AWS Certificate Manager to associate with the endpoint. The CN in the certificate must match the DNS name your end users will use to reach your application.
v
endpointDomainPrefix" ZA custom identifier that is prepended to the DNS name that is generated for the endpoint.
Ñ
endpointType" pThe type of Verified Access endpoint to create. Currently `load-balancer` or `network-interface` are supported.
˜
loadBalancerOptionsB}:{
y
verifiedaccessEndpointLoadBalancerOptionsJaws:verifiedaccess/EndpointLoadBalancerOptions:EndpointLoadBalancerOptions_The load balancer details. This parameter is required if the endpoint type is `load-balancer`.
ì
networkInterfaceOptionséBã:à
Ö
verifiedaccessEndpointNetworkInterfaceOptionsRaws:verifiedaccess/EndpointNetworkInterfaceOptions:EndpointNetworkInterfaceOptionsgThe network interface details. This parameter is required if the endpoint type is `network-interface`.
S
policyDocumentB" ;The policy document that is associated with this resource.
p
securityGroupIdsB*" TList of the the security groups IDs to associate with the Verified Access endpoint.
ª
sseSpecificationvBt:r
p
verifiedaccessEndpointSseSpecificationDaws:verifiedaccess/EndpointSseSpecification:EndpointSseSpecification/The options in use for server side encryption.
ÿ
tagsB2" «Key-value tags for the Verified Access Endpoint. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
à
verifiedAccessGroupId" kThe ID of the Verified Access group to associate the endpoint with.

The following arguments are optional:
"K
applicationDomain" 2The DNS name for users to reach your application.
"R
attachmentType" <The type of attachment. Currently, only `vpc` is supported.
"E
descriptionB" 0A description for the Verified Access endpoint.
"Y
deviceValidationDomain" ;Returned if endpoint has a device trust provider attached.
"Á
domainCertificateArn"  The ARN of the public TLS/SSL certificate in AWS Certificate Manager to associate with the endpoint. The CN in the certificate must match the DNS name your end users will use to reach your application.
"E
endpointDomain" /A DNS name that is generated for the endpoint.
"v
endpointDomainPrefix" ZA custom identifier that is prepended to the DNS name that is generated for the endpoint.
"Ñ
endpointType" pThe type of Verified Access endpoint to create. Currently `load-balancer` or `network-interface` are supported.
"˜
loadBalancerOptionsB}:{
y
verifiedaccessEndpointLoadBalancerOptionsJaws:verifiedaccess/EndpointLoadBalancerOptions:EndpointLoadBalancerOptions_The load balancer details. This parameter is required if the endpoint type is `load-balancer`.
"ì
networkInterfaceOptionséBã:à
Ö
verifiedaccessEndpointNetworkInterfaceOptionsRaws:verifiedaccess/EndpointNetworkInterfaceOptions:EndpointNetworkInterfaceOptionsgThe network interface details. This parameter is required if the endpoint type is `network-interface`.
"S
policyDocumentB" ;The policy document that is associated with this resource.
"p
securityGroupIdsB*" TList of the the security groups IDs to associate with the Verified Access endpoint.
"π
sseSpecificationt:r
p
verifiedaccessEndpointSseSpecificationDaws:verifiedaccess/EndpointSseSpecification:EndpointSseSpecification/The options in use for server side encryption.
"ÿ
tagsB2" «Key-value tags for the Verified Access Endpoint. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "à
verifiedAccessGroupId" kThe ID of the Verified Access group to associate the endpoint with.

The following arguments are optional:
"
verifiedAccessInstanceId" *œ<
7
verifiedaccessGroupaws:verifiedaccess/group:GroupÜ.Resource for managing a Verified Access Group.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedaccess.Group("example", {verifiedaccessInstanceId: exampleAwsVerifiedaccessInstance.id});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedaccess.Group("example", verifiedaccess_instance_id=example_aws_verifiedaccess_instance["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedAccess.Group("example", new()
    {
        VerifiedaccessInstanceId = exampleAwsVerifiedaccessInstance.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedaccess.NewGroup(ctx, "example", &verifiedaccess.GroupArgs{
			VerifiedaccessInstanceId: pulumi.Any(exampleAwsVerifiedaccessInstance.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedaccess.Group;
import com.pulumi.aws.verifiedaccess.GroupArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Group("example", GroupArgs.builder()
            .verifiedaccessInstanceId(exampleAwsVerifiedaccessInstance.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedaccess:Group
    properties:
      verifiedaccessInstanceId: ${exampleAwsVerifiedaccessInstance.id}
```
<!--End PulumiCodeChooser -->

### Usage with KMS Key

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const testKey = new aws.kms.Key("test_key", {description: "KMS key for Verified Access Group test"});
const test = new aws.verifiedaccess.Group("test", {
    verifiedaccessInstanceId: testAwsVerifiedaccessInstanceTrustProviderAttachment.verifiedaccessInstanceId,
    sseConfiguration: {
        kmsKeyArn: testKey.arn,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test_key = aws.kms.Key("test_key", description="KMS key for Verified Access Group test")
test = aws.verifiedaccess.Group("test",
    verifiedaccess_instance_id=test_aws_verifiedaccess_instance_trust_provider_attachment["verifiedaccessInstanceId"],
    sse_configuration={
        "kms_key_arn": test_key.arn,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var testKey = new Aws.Kms.Key("test_key", new()
    {
        Description = "KMS key for Verified Access Group test",
    });

    var test = new Aws.VerifiedAccess.Group("test", new()
    {
        VerifiedaccessInstanceId = testAwsVerifiedaccessInstanceTrustProviderAttachment.VerifiedaccessInstanceId,
        SseConfiguration = new Aws.VerifiedAccess.Inputs.GroupSseConfigurationArgs
        {
            KmsKeyArn = testKey.Arn,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/kms"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		testKey, err := kms.NewKey(ctx, "test_key", &kms.KeyArgs{
			Description: pulumi.String("KMS key for Verified Access Group test"),
		})
		if err != nil {
			return err
		}
		_, err = verifiedaccess.NewGroup(ctx, "test", &verifiedaccess.GroupArgs{
			VerifiedaccessInstanceId: pulumi.Any(testAwsVerifiedaccessInstanceTrustProviderAttachment.VerifiedaccessInstanceId),
			SseConfiguration: &verifiedaccess.GroupSseConfigurationArgs{
				KmsKeyArn: testKey.Arn,
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.kms.Key;
import com.pulumi.aws.kms.KeyArgs;
import com.pulumi.aws.verifiedaccess.Group;
import com.pulumi.aws.verifiedaccess.GroupArgs;
import com.pulumi.aws.verifiedaccess.inputs.GroupSseConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var testKey = new Key("testKey", KeyArgs.builder()
            .description("KMS key for Verified Access Group test")
            .build());

        var test = new Group("test", GroupArgs.builder()
            .verifiedaccessInstanceId(testAwsVerifiedaccessInstanceTrustProviderAttachment.verifiedaccessInstanceId())
            .sseConfiguration(GroupSseConfigurationArgs.builder()
                .kmsKeyArn(testKey.arn())
                .build())
            .build());

    }
}
```
```yaml
resources:
  testKey:
    type: aws:kms:Key
    name: test_key
    properties:
      description: KMS key for Verified Access Group test
  test:
    type: aws:verifiedaccess:Group
    properties:
      verifiedaccessInstanceId: ${testAwsVerifiedaccessInstanceTrustProviderAttachment.verifiedaccessInstanceId}
      sseConfiguration:
        kmsKeyArn: ${testKey.arn}
```
<!--End PulumiCodeChooser -->
?
descriptionB" *Description of the verified access group.
S
policyDocumentB" ;The policy document that is associated with this resource.
√
sseConfigurationmBk:i
g
verifiedaccessGroupSseConfiguration>aws:verifiedaccess/GroupSseConfiguration:GroupSseConfiguration@Configuration block to use KMS keys for server-side encryption.
À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
ç
verifiedaccessInstanceId" mThe id of the verified access instance this group is associated with.

The following arguments are optional:
"A
creationTime" -Timestamp when the access group was created.
"A
deletionTime" -Timestamp when the access group was deleted.
"=
description" *Description of the verified access group.
"I
lastUpdatedTime" 2Timestamp when the access group was last updated.
"6
owner" )AWS account number owning this resource.
"S
policyDocumentB" ;The policy document that is associated with this resource.
"¡
sseConfigurationk:i
g
verifiedaccessGroupSseConfiguration>aws:verifiedaccess/GroupSseConfiguration:GroupSseConfiguration@Configuration block to use KMS keys for server-side encryption.
"À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "@
verifiedaccessGroupArn" "ARN of this verified acess group.
"?
verifiedaccessGroupId" "ID of this verified access group.
"ç
verifiedaccessInstanceId" mThe id of the verified access instance this group is associated with.

The following arguments are optional:
*Ñ,
@
verifiedaccessInstance$aws:verifiedaccess/instance:Instance› Resource for managing a Verified Access Instance.

## Example Usage

### Basic

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedaccess.Instance("example", {
    description: "example",
    tags: {
        Name: "example",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedaccess.Instance("example",
    description="example",
    tags={
        "Name": "example",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedAccess.Instance("example", new()
    {
        Description = "example",
        Tags = 
        {
            { "Name", "example" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedaccess.NewInstance(ctx, "example", &verifiedaccess.InstanceArgs{
			Description: pulumi.String("example"),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("example"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedaccess.Instance;
import com.pulumi.aws.verifiedaccess.InstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Instance("example", InstanceArgs.builder()
            .description("example")
            .tags(Map.of("Name", "example"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedaccess:Instance
    properties:
      description: example
      tags:
        Name: example
```
<!--End PulumiCodeChooser -->

### With `fips_enabled`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedaccess.Instance("example", {fipsEnabled: true});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedaccess.Instance("example", fips_enabled=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedAccess.Instance("example", new()
    {
        FipsEnabled = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedaccess.NewInstance(ctx, "example", &verifiedaccess.InstanceArgs{
			FipsEnabled: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedaccess.Instance;
import com.pulumi.aws.verifiedaccess.InstanceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Instance("example", InstanceArgs.builder()
            .fipsEnabled(true)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedaccess:Instance
    properties:
      fipsEnabled: true
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Verified Access Instances using the  `id`. For example:

```sh
$ pulumi import aws:verifiedaccess/instance:Instance example vai-1234567890abcdef0
```
I
descriptionB" 4A description for the AWS Verified Access Instance.
à
fipsEnabledB
 sEnable or disable support for Federal Information Processing Standards (FIPS) on the AWS Verified Access Instance.
À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"L
creationTime" 8The time that the Verified Access Instance was created.
"I
descriptionB" 4A description for the AWS Verified Access Instance.
"à
fipsEnabledB
 sEnable or disable support for Federal Information Processing Standards (FIPS) on the AWS Verified Access Instance.
"T
lastUpdatedTime" =The time that the Verified Access Instance was last updated.
"À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "‚
verifiedAccessTrustProvidersö*ó:î
ë
verifiedaccess#InstanceVerifiedAccessTrustProviderZaws:verifiedaccess/InstanceVerifiedAccessTrustProvider:InstanceVerifiedAccessTrustProvider§One or more blocks of providing information about the AWS Verified Access Trust Providers. See verified_access_trust_providers below for details.One or more blocks
*‚√
|
verifiedaccessInstanceLoggingConfigurationLaws:verifiedaccess/instanceLoggingConfiguration:InstanceLoggingConfiguration§ΩResource for managing a Verified Access Logging Configuration.

## Example Usage

### With CloudWatch Logging

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedaccess.InstanceLoggingConfiguration("example", {
    accessLogs: {
        cloudwatchLogs: {
            enabled: true,
            logGroup: exampleAwsCloudwatchLogGroup.id,
        },
    },
    verifiedaccessInstanceId: exampleAwsVerifiedaccessInstance.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedaccess.InstanceLoggingConfiguration("example",
    access_logs={
        "cloudwatch_logs": {
            "enabled": True,
            "log_group": example_aws_cloudwatch_log_group["id"],
        },
    },
    verifiedaccess_instance_id=example_aws_verifiedaccess_instance["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedAccess.InstanceLoggingConfiguration("example", new()
    {
        AccessLogs = new Aws.VerifiedAccess.Inputs.InstanceLoggingConfigurationAccessLogsArgs
        {
            CloudwatchLogs = new Aws.VerifiedAccess.Inputs.InstanceLoggingConfigurationAccessLogsCloudwatchLogsArgs
            {
                Enabled = true,
                LogGroup = exampleAwsCloudwatchLogGroup.Id,
            },
        },
        VerifiedaccessInstanceId = exampleAwsVerifiedaccessInstance.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedaccess.NewInstanceLoggingConfiguration(ctx, "example", &verifiedaccess.InstanceLoggingConfigurationArgs{
			AccessLogs: &verifiedaccess.InstanceLoggingConfigurationAccessLogsArgs{
				CloudwatchLogs: &verifiedaccess.InstanceLoggingConfigurationAccessLogsCloudwatchLogsArgs{
					Enabled:  pulumi.Bool(true),
					LogGroup: pulumi.Any(exampleAwsCloudwatchLogGroup.Id),
				},
			},
			VerifiedaccessInstanceId: pulumi.Any(exampleAwsVerifiedaccessInstance.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedaccess.InstanceLoggingConfiguration;
import com.pulumi.aws.verifiedaccess.InstanceLoggingConfigurationArgs;
import com.pulumi.aws.verifiedaccess.inputs.InstanceLoggingConfigurationAccessLogsArgs;
import com.pulumi.aws.verifiedaccess.inputs.InstanceLoggingConfigurationAccessLogsCloudwatchLogsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new InstanceLoggingConfiguration("example", InstanceLoggingConfigurationArgs.builder()
            .accessLogs(InstanceLoggingConfigurationAccessLogsArgs.builder()
                .cloudwatchLogs(InstanceLoggingConfigurationAccessLogsCloudwatchLogsArgs.builder()
                    .enabled(true)
                    .logGroup(exampleAwsCloudwatchLogGroup.id())
                    .build())
                .build())
            .verifiedaccessInstanceId(exampleAwsVerifiedaccessInstance.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedaccess:InstanceLoggingConfiguration
    properties:
      accessLogs:
        cloudwatchLogs:
          enabled: true
          logGroup: ${exampleAwsCloudwatchLogGroup.id}
      verifiedaccessInstanceId: ${exampleAwsVerifiedaccessInstance.id}
```
<!--End PulumiCodeChooser -->

### With Kinesis Data Firehose Logging

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedaccess.InstanceLoggingConfiguration("example", {
    accessLogs: {
        kinesisDataFirehose: {
            deliveryStream: exampleAwsKinesisFirehoseDeliveryStream.name,
            enabled: true,
        },
    },
    verifiedaccessInstanceId: exampleAwsVerifiedaccessInstance.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedaccess.InstanceLoggingConfiguration("example",
    access_logs={
        "kinesis_data_firehose": {
            "delivery_stream": example_aws_kinesis_firehose_delivery_stream["name"],
            "enabled": True,
        },
    },
    verifiedaccess_instance_id=example_aws_verifiedaccess_instance["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedAccess.InstanceLoggingConfiguration("example", new()
    {
        AccessLogs = new Aws.VerifiedAccess.Inputs.InstanceLoggingConfigurationAccessLogsArgs
        {
            KinesisDataFirehose = new Aws.VerifiedAccess.Inputs.InstanceLoggingConfigurationAccessLogsKinesisDataFirehoseArgs
            {
                DeliveryStream = exampleAwsKinesisFirehoseDeliveryStream.Name,
                Enabled = true,
            },
        },
        VerifiedaccessInstanceId = exampleAwsVerifiedaccessInstance.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedaccess.NewInstanceLoggingConfiguration(ctx, "example", &verifiedaccess.InstanceLoggingConfigurationArgs{
			AccessLogs: &verifiedaccess.InstanceLoggingConfigurationAccessLogsArgs{
				KinesisDataFirehose: &verifiedaccess.InstanceLoggingConfigurationAccessLogsKinesisDataFirehoseArgs{
					DeliveryStream: pulumi.Any(exampleAwsKinesisFirehoseDeliveryStream.Name),
					Enabled:        pulumi.Bool(true),
				},
			},
			VerifiedaccessInstanceId: pulumi.Any(exampleAwsVerifiedaccessInstance.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedaccess.InstanceLoggingConfiguration;
import com.pulumi.aws.verifiedaccess.InstanceLoggingConfigurationArgs;
import com.pulumi.aws.verifiedaccess.inputs.InstanceLoggingConfigurationAccessLogsArgs;
import com.pulumi.aws.verifiedaccess.inputs.InstanceLoggingConfigurationAccessLogsKinesisDataFirehoseArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new InstanceLoggingConfiguration("example", InstanceLoggingConfigurationArgs.builder()
            .accessLogs(InstanceLoggingConfigurationAccessLogsArgs.builder()
                .kinesisDataFirehose(InstanceLoggingConfigurationAccessLogsKinesisDataFirehoseArgs.builder()
                    .deliveryStream(exampleAwsKinesisFirehoseDeliveryStream.name())
                    .enabled(true)
                    .build())
                .build())
            .verifiedaccessInstanceId(exampleAwsVerifiedaccessInstance.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedaccess:InstanceLoggingConfiguration
    properties:
      accessLogs:
        kinesisDataFirehose:
          deliveryStream: ${exampleAwsKinesisFirehoseDeliveryStream.name}
          enabled: true
      verifiedaccessInstanceId: ${exampleAwsVerifiedaccessInstance.id}
```
<!--End PulumiCodeChooser -->

### With S3 logging

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedaccess.InstanceLoggingConfiguration("example", {
    accessLogs: {
        s3: {
            bucketName: exampleAwsS3Bucket.id,
            enabled: true,
            prefix: "example",
        },
    },
    verifiedaccessInstanceId: exampleAwsVerifiedaccessInstance.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedaccess.InstanceLoggingConfiguration("example",
    access_logs={
        "s3": {
            "bucket_name": example_aws_s3_bucket["id"],
            "enabled": True,
            "prefix": "example",
        },
    },
    verifiedaccess_instance_id=example_aws_verifiedaccess_instance["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedAccess.InstanceLoggingConfiguration("example", new()
    {
        AccessLogs = new Aws.VerifiedAccess.Inputs.InstanceLoggingConfigurationAccessLogsArgs
        {
            S3 = new Aws.VerifiedAccess.Inputs.InstanceLoggingConfigurationAccessLogsS3Args
            {
                BucketName = exampleAwsS3Bucket.Id,
                Enabled = true,
                Prefix = "example",
            },
        },
        VerifiedaccessInstanceId = exampleAwsVerifiedaccessInstance.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedaccess.NewInstanceLoggingConfiguration(ctx, "example", &verifiedaccess.InstanceLoggingConfigurationArgs{
			AccessLogs: &verifiedaccess.InstanceLoggingConfigurationAccessLogsArgs{
				S3: &verifiedaccess.InstanceLoggingConfigurationAccessLogsS3Args{
					BucketName: pulumi.Any(exampleAwsS3Bucket.Id),
					Enabled:    pulumi.Bool(true),
					Prefix:     pulumi.String("example"),
				},
			},
			VerifiedaccessInstanceId: pulumi.Any(exampleAwsVerifiedaccessInstance.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedaccess.InstanceLoggingConfiguration;
import com.pulumi.aws.verifiedaccess.InstanceLoggingConfigurationArgs;
import com.pulumi.aws.verifiedaccess.inputs.InstanceLoggingConfigurationAccessLogsArgs;
import com.pulumi.aws.verifiedaccess.inputs.InstanceLoggingConfigurationAccessLogsS3Args;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new InstanceLoggingConfiguration("example", InstanceLoggingConfigurationArgs.builder()
            .accessLogs(InstanceLoggingConfigurationAccessLogsArgs.builder()
                .s3(InstanceLoggingConfigurationAccessLogsS3Args.builder()
                    .bucketName(exampleAwsS3Bucket.id())
                    .enabled(true)
                    .prefix("example")
                    .build())
                .build())
            .verifiedaccessInstanceId(exampleAwsVerifiedaccessInstance.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedaccess:InstanceLoggingConfiguration
    properties:
      accessLogs:
        s3:
          bucketName: ${exampleAwsS3Bucket.id}
          enabled: true
          prefix: example
      verifiedaccessInstanceId: ${exampleAwsVerifiedaccessInstance.id}
```
<!--End PulumiCodeChooser -->

### With all three logging options

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedaccess.InstanceLoggingConfiguration("example", {
    accessLogs: {
        cloudwatchLogs: {
            enabled: true,
            logGroup: exampleAwsCloudwatchLogGroup.id,
        },
        kinesisDataFirehose: {
            deliveryStream: exampleAwsKinesisFirehoseDeliveryStream.name,
            enabled: true,
        },
        s3: {
            bucketName: exampleAwsS3Bucket.id,
            enabled: true,
        },
    },
    verifiedaccessInstanceId: exampleAwsVerifiedaccessInstance.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedaccess.InstanceLoggingConfiguration("example",
    access_logs={
        "cloudwatch_logs": {
            "enabled": True,
            "log_group": example_aws_cloudwatch_log_group["id"],
        },
        "kinesis_data_firehose": {
            "delivery_stream": example_aws_kinesis_firehose_delivery_stream["name"],
            "enabled": True,
        },
        "s3": {
            "bucket_name": example_aws_s3_bucket["id"],
            "enabled": True,
        },
    },
    verifiedaccess_instance_id=example_aws_verifiedaccess_instance["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedAccess.InstanceLoggingConfiguration("example", new()
    {
        AccessLogs = new Aws.VerifiedAccess.Inputs.InstanceLoggingConfigurationAccessLogsArgs
        {
            CloudwatchLogs = new Aws.VerifiedAccess.Inputs.InstanceLoggingConfigurationAccessLogsCloudwatchLogsArgs
            {
                Enabled = true,
                LogGroup = exampleAwsCloudwatchLogGroup.Id,
            },
            KinesisDataFirehose = new Aws.VerifiedAccess.Inputs.InstanceLoggingConfigurationAccessLogsKinesisDataFirehoseArgs
            {
                DeliveryStream = exampleAwsKinesisFirehoseDeliveryStream.Name,
                Enabled = true,
            },
            S3 = new Aws.VerifiedAccess.Inputs.InstanceLoggingConfigurationAccessLogsS3Args
            {
                BucketName = exampleAwsS3Bucket.Id,
                Enabled = true,
            },
        },
        VerifiedaccessInstanceId = exampleAwsVerifiedaccessInstance.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedaccess.NewInstanceLoggingConfiguration(ctx, "example", &verifiedaccess.InstanceLoggingConfigurationArgs{
			AccessLogs: &verifiedaccess.InstanceLoggingConfigurationAccessLogsArgs{
				CloudwatchLogs: &verifiedaccess.InstanceLoggingConfigurationAccessLogsCloudwatchLogsArgs{
					Enabled:  pulumi.Bool(true),
					LogGroup: pulumi.Any(exampleAwsCloudwatchLogGroup.Id),
				},
				KinesisDataFirehose: &verifiedaccess.InstanceLoggingConfigurationAccessLogsKinesisDataFirehoseArgs{
					DeliveryStream: pulumi.Any(exampleAwsKinesisFirehoseDeliveryStream.Name),
					Enabled:        pulumi.Bool(true),
				},
				S3: &verifiedaccess.InstanceLoggingConfigurationAccessLogsS3Args{
					BucketName: pulumi.Any(exampleAwsS3Bucket.Id),
					Enabled:    pulumi.Bool(true),
				},
			},
			VerifiedaccessInstanceId: pulumi.Any(exampleAwsVerifiedaccessInstance.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedaccess.InstanceLoggingConfiguration;
import com.pulumi.aws.verifiedaccess.InstanceLoggingConfigurationArgs;
import com.pulumi.aws.verifiedaccess.inputs.InstanceLoggingConfigurationAccessLogsArgs;
import com.pulumi.aws.verifiedaccess.inputs.InstanceLoggingConfigurationAccessLogsCloudwatchLogsArgs;
import com.pulumi.aws.verifiedaccess.inputs.InstanceLoggingConfigurationAccessLogsKinesisDataFirehoseArgs;
import com.pulumi.aws.verifiedaccess.inputs.InstanceLoggingConfigurationAccessLogsS3Args;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new InstanceLoggingConfiguration("example", InstanceLoggingConfigurationArgs.builder()
            .accessLogs(InstanceLoggingConfigurationAccessLogsArgs.builder()
                .cloudwatchLogs(InstanceLoggingConfigurationAccessLogsCloudwatchLogsArgs.builder()
                    .enabled(true)
                    .logGroup(exampleAwsCloudwatchLogGroup.id())
                    .build())
                .kinesisDataFirehose(InstanceLoggingConfigurationAccessLogsKinesisDataFirehoseArgs.builder()
                    .deliveryStream(exampleAwsKinesisFirehoseDeliveryStream.name())
                    .enabled(true)
                    .build())
                .s3(InstanceLoggingConfigurationAccessLogsS3Args.builder()
                    .bucketName(exampleAwsS3Bucket.id())
                    .enabled(true)
                    .build())
                .build())
            .verifiedaccessInstanceId(exampleAwsVerifiedaccessInstance.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedaccess:InstanceLoggingConfiguration
    properties:
      accessLogs:
        cloudwatchLogs:
          enabled: true
          logGroup: ${exampleAwsCloudwatchLogGroup.id}
        kinesisDataFirehose:
          deliveryStream: ${exampleAwsKinesisFirehoseDeliveryStream.name}
          enabled: true
        s3:
          bucketName: ${exampleAwsS3Bucket.id}
          enabled: true
      verifiedaccessInstanceId: ${exampleAwsVerifiedaccessInstance.id}
```
<!--End PulumiCodeChooser -->

### With `include_trust_context`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedaccess.InstanceLoggingConfiguration("example", {
    accessLogs: {
        includeTrustContext: true,
    },
    verifiedaccessInstanceId: exampleAwsVerifiedaccessInstance.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedaccess.InstanceLoggingConfiguration("example",
    access_logs={
        "include_trust_context": True,
    },
    verifiedaccess_instance_id=example_aws_verifiedaccess_instance["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedAccess.InstanceLoggingConfiguration("example", new()
    {
        AccessLogs = new Aws.VerifiedAccess.Inputs.InstanceLoggingConfigurationAccessLogsArgs
        {
            IncludeTrustContext = true,
        },
        VerifiedaccessInstanceId = exampleAwsVerifiedaccessInstance.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedaccess.NewInstanceLoggingConfiguration(ctx, "example", &verifiedaccess.InstanceLoggingConfigurationArgs{
			AccessLogs: &verifiedaccess.InstanceLoggingConfigurationAccessLogsArgs{
				IncludeTrustContext: pulumi.Bool(true),
			},
			VerifiedaccessInstanceId: pulumi.Any(exampleAwsVerifiedaccessInstance.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedaccess.InstanceLoggingConfiguration;
import com.pulumi.aws.verifiedaccess.InstanceLoggingConfigurationArgs;
import com.pulumi.aws.verifiedaccess.inputs.InstanceLoggingConfigurationAccessLogsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new InstanceLoggingConfiguration("example", InstanceLoggingConfigurationArgs.builder()
            .accessLogs(InstanceLoggingConfigurationAccessLogsArgs.builder()
                .includeTrustContext(true)
                .build())
            .verifiedaccessInstanceId(exampleAwsVerifiedaccessInstance.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedaccess:InstanceLoggingConfiguration
    properties:
      accessLogs:
        includeTrustContext: true
      verifiedaccessInstanceId: ${exampleAwsVerifiedaccessInstance.id}
```
<!--End PulumiCodeChooser -->

### With `log_version`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedaccess.InstanceLoggingConfiguration("example", {
    accessLogs: {
        logVersion: "ocsf-1.0.0-rc.2",
    },
    verifiedaccessInstanceId: exampleAwsVerifiedaccessInstance.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedaccess.InstanceLoggingConfiguration("example",
    access_logs={
        "log_version": "ocsf-1.0.0-rc.2",
    },
    verifiedaccess_instance_id=example_aws_verifiedaccess_instance["id"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedAccess.InstanceLoggingConfiguration("example", new()
    {
        AccessLogs = new Aws.VerifiedAccess.Inputs.InstanceLoggingConfigurationAccessLogsArgs
        {
            LogVersion = "ocsf-1.0.0-rc.2",
        },
        VerifiedaccessInstanceId = exampleAwsVerifiedaccessInstance.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedaccess.NewInstanceLoggingConfiguration(ctx, "example", &verifiedaccess.InstanceLoggingConfigurationArgs{
			AccessLogs: &verifiedaccess.InstanceLoggingConfigurationAccessLogsArgs{
				LogVersion: pulumi.String("ocsf-1.0.0-rc.2"),
			},
			VerifiedaccessInstanceId: pulumi.Any(exampleAwsVerifiedaccessInstance.Id),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedaccess.InstanceLoggingConfiguration;
import com.pulumi.aws.verifiedaccess.InstanceLoggingConfigurationArgs;
import com.pulumi.aws.verifiedaccess.inputs.InstanceLoggingConfigurationAccessLogsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new InstanceLoggingConfiguration("example", InstanceLoggingConfigurationArgs.builder()
            .accessLogs(InstanceLoggingConfigurationAccessLogsArgs.builder()
                .logVersion("ocsf-1.0.0-rc.2")
                .build())
            .verifiedaccessInstanceId(exampleAwsVerifiedaccessInstance.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedaccess:InstanceLoggingConfiguration
    properties:
      accessLogs:
        logVersion: ocsf-1.0.0-rc.2
      verifiedaccessInstanceId: ${exampleAwsVerifiedaccessInstance.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Verified Access Logging Configuration using the Verified Access Instance `id`. For example:

```sh
$ pulumi import aws:verifiedaccess/instanceLoggingConfiguration:InstanceLoggingConfiguration example vai-1234567890abcdef0
```
ë

accessLogs†:ù
ö
verifiedaccess&InstanceLoggingConfigurationAccessLogs`aws:verifiedaccess/InstanceLoggingConfigurationAccessLogs:InstanceLoggingConfigurationAccessLogs`A block that specifies the configuration options for Verified Access instances. Detailed below.
H
verifiedaccessInstanceId" (The ID of the Verified Access instance.
"ë

accessLogs†:ù
ö
verifiedaccess&InstanceLoggingConfigurationAccessLogs`aws:verifiedaccess/InstanceLoggingConfigurationAccessLogs:InstanceLoggingConfigurationAccessLogs`A block that specifies the configuration options for Verified Access instances. Detailed below.
"H
verifiedaccessInstanceId" (The ID of the Verified Access instance.
*”0
Ö
verifiedaccessInstanceTrustProviderAttachmentRaws:verifiedaccess/instanceTrustProviderAttachment:InstanceTrustProviderAttachment ,Resource for managing a Verified Access Instance Trust Provider Attachment.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedaccess.Instance("example", {});
const exampleTrustProvider = new aws.verifiedaccess.TrustProvider("example", {
    deviceTrustProviderType: "jamf",
    policyReferenceName: "example",
    trustProviderType: "device",
    deviceOptions: {
        tenantId: "example",
    },
});
const exampleInstanceTrustProviderAttachment = new aws.verifiedaccess.InstanceTrustProviderAttachment("example", {
    verifiedaccessInstanceId: example.id,
    verifiedaccessTrustProviderId: exampleTrustProvider.id,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedaccess.Instance("example")
example_trust_provider = aws.verifiedaccess.TrustProvider("example",
    device_trust_provider_type="jamf",
    policy_reference_name="example",
    trust_provider_type="device",
    device_options={
        "tenant_id": "example",
    })
example_instance_trust_provider_attachment = aws.verifiedaccess.InstanceTrustProviderAttachment("example",
    verifiedaccess_instance_id=example.id,
    verifiedaccess_trust_provider_id=example_trust_provider.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedAccess.Instance("example");

    var exampleTrustProvider = new Aws.VerifiedAccess.TrustProvider("example", new()
    {
        DeviceTrustProviderType = "jamf",
        PolicyReferenceName = "example",
        TrustProviderType = "device",
        DeviceOptions = new Aws.VerifiedAccess.Inputs.TrustProviderDeviceOptionsArgs
        {
            TenantId = "example",
        },
    });

    var exampleInstanceTrustProviderAttachment = new Aws.VerifiedAccess.InstanceTrustProviderAttachment("example", new()
    {
        VerifiedaccessInstanceId = example.Id,
        VerifiedaccessTrustProviderId = exampleTrustProvider.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := verifiedaccess.NewInstance(ctx, "example", nil)
		if err != nil {
			return err
		}
		exampleTrustProvider, err := verifiedaccess.NewTrustProvider(ctx, "example", &verifiedaccess.TrustProviderArgs{
			DeviceTrustProviderType: pulumi.String("jamf"),
			PolicyReferenceName:     pulumi.String("example"),
			TrustProviderType:       pulumi.String("device"),
			DeviceOptions: &verifiedaccess.TrustProviderDeviceOptionsArgs{
				TenantId: pulumi.String("example"),
			},
		})
		if err != nil {
			return err
		}
		_, err = verifiedaccess.NewInstanceTrustProviderAttachment(ctx, "example", &verifiedaccess.InstanceTrustProviderAttachmentArgs{
			VerifiedaccessInstanceId:      example.ID(),
			VerifiedaccessTrustProviderId: exampleTrustProvider.ID(),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedaccess.Instance;
import com.pulumi.aws.verifiedaccess.TrustProvider;
import com.pulumi.aws.verifiedaccess.TrustProviderArgs;
import com.pulumi.aws.verifiedaccess.inputs.TrustProviderDeviceOptionsArgs;
import com.pulumi.aws.verifiedaccess.InstanceTrustProviderAttachment;
import com.pulumi.aws.verifiedaccess.InstanceTrustProviderAttachmentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Instance("example");

        var exampleTrustProvider = new TrustProvider("exampleTrustProvider", TrustProviderArgs.builder()
            .deviceTrustProviderType("jamf")
            .policyReferenceName("example")
            .trustProviderType("device")
            .deviceOptions(TrustProviderDeviceOptionsArgs.builder()
                .tenantId("example")
                .build())
            .build());

        var exampleInstanceTrustProviderAttachment = new InstanceTrustProviderAttachment("exampleInstanceTrustProviderAttachment", InstanceTrustProviderAttachmentArgs.builder()
            .verifiedaccessInstanceId(example.id())
            .verifiedaccessTrustProviderId(exampleTrustProvider.id())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedaccess:Instance
  exampleTrustProvider:
    type: aws:verifiedaccess:TrustProvider
    name: example
    properties:
      deviceTrustProviderType: jamf
      policyReferenceName: example
      trustProviderType: device
      deviceOptions:
        tenantId: example
  exampleInstanceTrustProviderAttachment:
    type: aws:verifiedaccess:InstanceTrustProviderAttachment
    name: example
    properties:
      verifiedaccessInstanceId: ${example.id}
      verifiedaccessTrustProviderId: ${exampleTrustProvider.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Verified Access Instance Trust Provider Attachments using the `verifiedaccess_instance_id` and `verifiedaccess_trust_provider_id` separated by a forward slash (`/`). For example:

```sh
$ pulumi import aws:verifiedaccess/instanceTrustProviderAttachment:InstanceTrustProviderAttachment example vai-1234567890abcdef0/vatp-8012925589
```
h
verifiedaccessInstanceId" HThe ID of the Verified Access instance to attach the Trust Provider to.
S
verifiedaccessTrustProviderId" .The ID of the Verified Access trust provider.
"h
verifiedaccessInstanceId" HThe ID of the Verified Access instance to attach the Trust Provider to.
"S
verifiedaccessTrustProviderId" .The ID of the Verified Access trust provider.
*Ú&
O
verifiedaccessTrustProvider.aws:verifiedaccess/trustProvider:TrustProvider°Resource for managing a Verified Access Trust Provider.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedaccess.TrustProvider("example", {
    policyReferenceName: "example",
    trustProviderType: "user",
    userTrustProviderType: "iam-identity-center",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedaccess.TrustProvider("example",
    policy_reference_name="example",
    trust_provider_type="user",
    user_trust_provider_type="iam-identity-center")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedAccess.TrustProvider("example", new()
    {
        PolicyReferenceName = "example",
        TrustProviderType = "user",
        UserTrustProviderType = "iam-identity-center",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedaccess"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedaccess.NewTrustProvider(ctx, "example", &verifiedaccess.TrustProviderArgs{
			PolicyReferenceName:   pulumi.String("example"),
			TrustProviderType:     pulumi.String("user"),
			UserTrustProviderType: pulumi.String("iam-identity-center"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedaccess.TrustProvider;
import com.pulumi.aws.verifiedaccess.TrustProviderArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new TrustProvider("example", TrustProviderArgs.builder()
            .policyReferenceName("example")
            .trustProviderType("user")
            .userTrustProviderType("iam-identity-center")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedaccess:TrustProvider
    properties:
      policyReferenceName: example
      trustProviderType: user
      userTrustProviderType: iam-identity-center
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Transfer Workflows using the  `id`. For example:

```sh
$ pulumi import aws:verifiedaccess/trustProvider:TrustProvider example vatp-8012925589
```
O
descriptionB" :A description for the AWS Verified Access trust provider.
Õ
deviceOptions|Bz:x
v
verifiedaccessTrustProviderDeviceOptionsHaws:verifiedaccess/TrustProviderDeviceOptions:TrustProviderDeviceOptions>A block of options for device identity based trust providers.
J
deviceTrustProviderTypeB" )The type of device-based trust provider.
ÿ
oidcOptionsvBt:r
p
verifiedaccessTrustProviderOidcOptionsDaws:verifiedaccess/TrustProviderOidcOptions:TrustProviderOidcOptionsQThe OpenID Connect details for an oidc-type, user-identity based trust provider.
U
policyReferenceName" :The identifier to be used when working with policy rules.
À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.

trustProviderType" fThe type of trust provider can be either user or device-based.

The following arguments are optional:
F
userTrustProviderTypeB" 'The type of user-based trust provider.
"O
descriptionB" :A description for the AWS Verified Access trust provider.
"Õ
deviceOptions|Bz:x
v
verifiedaccessTrustProviderDeviceOptionsHaws:verifiedaccess/TrustProviderDeviceOptions:TrustProviderDeviceOptions>A block of options for device identity based trust providers.
"J
deviceTrustProviderTypeB" )The type of device-based trust provider.
"ÿ
oidcOptionsvBt:r
p
verifiedaccessTrustProviderOidcOptionsDaws:verifiedaccess/TrustProviderOidcOptions:TrustProviderOidcOptionsQThe OpenID Connect details for an oidc-type, user-identity based trust provider.
"U
policyReferenceName" :The identifier to be used when working with policy rules.
"À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
tagsAll2" "
trustProviderType" fThe type of trust provider can be either user or device-based.

The following arguments are optional:
"F
userTrustProviderTypeB" 'The type of user-based trust provider.
*Ëå
\
verifiedpermissionsIdentitySource5aws:verifiedpermissions/identitySource:IdentitySourceîÉResource for managing an AWS Verified Permissions Identity Source.

## Example Usage

### Cognito User Pool Configuration Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedpermissions.PolicyStore("example", {validationSettings: {
    mode: "STRICT",
}});
const exampleUserPool = new aws.cognito.UserPool("example", {name: "example"});
const exampleUserPoolClient = new aws.cognito.UserPoolClient("example", {
    name: "example",
    userPoolId: exampleUserPool.id,
    explicitAuthFlows: ["ADMIN_NO_SRP_AUTH"],
});
const exampleIdentitySource = new aws.verifiedpermissions.IdentitySource("example", {
    policyStoreId: example.id,
    configuration: {
        cognitoUserPoolConfiguration: {
            userPoolArn: exampleUserPool.arn,
            clientIds: [exampleUserPoolClient.id],
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedpermissions.PolicyStore("example", validation_settings={
    "mode": "STRICT",
})
example_user_pool = aws.cognito.UserPool("example", name="example")
example_user_pool_client = aws.cognito.UserPoolClient("example",
    name="example",
    user_pool_id=example_user_pool.id,
    explicit_auth_flows=["ADMIN_NO_SRP_AUTH"])
example_identity_source = aws.verifiedpermissions.IdentitySource("example",
    policy_store_id=example.id,
    configuration={
        "cognito_user_pool_configuration": {
            "user_pool_arn": example_user_pool.arn,
            "client_ids": [example_user_pool_client.id],
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedPermissions.PolicyStore("example", new()
    {
        ValidationSettings = new Aws.VerifiedPermissions.Inputs.PolicyStoreValidationSettingsArgs
        {
            Mode = "STRICT",
        },
    });

    var exampleUserPool = new Aws.Cognito.UserPool("example", new()
    {
        Name = "example",
    });

    var exampleUserPoolClient = new Aws.Cognito.UserPoolClient("example", new()
    {
        Name = "example",
        UserPoolId = exampleUserPool.Id,
        ExplicitAuthFlows = new[]
        {
            "ADMIN_NO_SRP_AUTH",
        },
    });

    var exampleIdentitySource = new Aws.VerifiedPermissions.IdentitySource("example", new()
    {
        PolicyStoreId = example.Id,
        Configuration = new Aws.VerifiedPermissions.Inputs.IdentitySourceConfigurationArgs
        {
            CognitoUserPoolConfiguration = new Aws.VerifiedPermissions.Inputs.IdentitySourceConfigurationCognitoUserPoolConfigurationArgs
            {
                UserPoolArn = exampleUserPool.Arn,
                ClientIds = new[]
                {
                    exampleUserPoolClient.Id,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cognito"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedpermissions"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := verifiedpermissions.NewPolicyStore(ctx, "example", &verifiedpermissions.PolicyStoreArgs{
			ValidationSettings: &verifiedpermissions.PolicyStoreValidationSettingsArgs{
				Mode: pulumi.String("STRICT"),
			},
		})
		if err != nil {
			return err
		}
		exampleUserPool, err := cognito.NewUserPool(ctx, "example", &cognito.UserPoolArgs{
			Name: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		exampleUserPoolClient, err := cognito.NewUserPoolClient(ctx, "example", &cognito.UserPoolClientArgs{
			Name:       pulumi.String("example"),
			UserPoolId: exampleUserPool.ID(),
			ExplicitAuthFlows: pulumi.StringArray{
				pulumi.String("ADMIN_NO_SRP_AUTH"),
			},
		})
		if err != nil {
			return err
		}
		_, err = verifiedpermissions.NewIdentitySource(ctx, "example", &verifiedpermissions.IdentitySourceArgs{
			PolicyStoreId: example.ID(),
			Configuration: &verifiedpermissions.IdentitySourceConfigurationArgs{
				CognitoUserPoolConfiguration: &verifiedpermissions.IdentitySourceConfigurationCognitoUserPoolConfigurationArgs{
					UserPoolArn: exampleUserPool.Arn,
					ClientIds: pulumi.StringArray{
						exampleUserPoolClient.ID(),
					},
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedpermissions.PolicyStore;
import com.pulumi.aws.verifiedpermissions.PolicyStoreArgs;
import com.pulumi.aws.verifiedpermissions.inputs.PolicyStoreValidationSettingsArgs;
import com.pulumi.aws.cognito.UserPool;
import com.pulumi.aws.cognito.UserPoolArgs;
import com.pulumi.aws.cognito.UserPoolClient;
import com.pulumi.aws.cognito.UserPoolClientArgs;
import com.pulumi.aws.verifiedpermissions.IdentitySource;
import com.pulumi.aws.verifiedpermissions.IdentitySourceArgs;
import com.pulumi.aws.verifiedpermissions.inputs.IdentitySourceConfigurationArgs;
import com.pulumi.aws.verifiedpermissions.inputs.IdentitySourceConfigurationCognitoUserPoolConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new PolicyStore("example", PolicyStoreArgs.builder()
            .validationSettings(PolicyStoreValidationSettingsArgs.builder()
                .mode("STRICT")
                .build())
            .build());

        var exampleUserPool = new UserPool("exampleUserPool", UserPoolArgs.builder()
            .name("example")
            .build());

        var exampleUserPoolClient = new UserPoolClient("exampleUserPoolClient", UserPoolClientArgs.builder()
            .name("example")
            .userPoolId(exampleUserPool.id())
            .explicitAuthFlows("ADMIN_NO_SRP_AUTH")
            .build());

        var exampleIdentitySource = new IdentitySource("exampleIdentitySource", IdentitySourceArgs.builder()
            .policyStoreId(example.id())
            .configuration(IdentitySourceConfigurationArgs.builder()
                .cognitoUserPoolConfiguration(IdentitySourceConfigurationCognitoUserPoolConfigurationArgs.builder()
                    .userPoolArn(exampleUserPool.arn())
                    .clientIds(exampleUserPoolClient.id())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedpermissions:PolicyStore
    properties:
      validationSettings:
        mode: STRICT
  exampleUserPool:
    type: aws:cognito:UserPool
    name: example
    properties:
      name: example
  exampleUserPoolClient:
    type: aws:cognito:UserPoolClient
    name: example
    properties:
      name: example
      userPoolId: ${exampleUserPool.id}
      explicitAuthFlows:
        - ADMIN_NO_SRP_AUTH
  exampleIdentitySource:
    type: aws:verifiedpermissions:IdentitySource
    name: example
    properties:
      policyStoreId: ${example.id}
      configuration:
        cognitoUserPoolConfiguration:
          userPoolArn: ${exampleUserPool.arn}
          clientIds:
            - ${exampleUserPoolClient.id}
```
<!--End PulumiCodeChooser -->

### OpenID Connect Configuration Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedpermissions.PolicyStore("example", {validationSettings: {
    mode: "STRICT",
}});
const exampleIdentitySource = new aws.verifiedpermissions.IdentitySource("example", {
    policyStoreId: example.id,
    configuration: {
        openIdConnectConfiguration: {
            issuer: "https://auth.example.com",
            tokenSelection: {
                accessTokenOnly: {
                    audiences: ["https://myapp.example.com"],
                    principalIdClaim: "sub",
                },
            },
            entityIdPrefix: "MyOIDCProvider",
            groupConfiguration: {
                groupClaim: "groups",
                groupEntityType: "MyCorp::UserGroup",
            },
        },
    },
    principalEntityType: "MyCorp::User",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedpermissions.PolicyStore("example", validation_settings={
    "mode": "STRICT",
})
example_identity_source = aws.verifiedpermissions.IdentitySource("example",
    policy_store_id=example.id,
    configuration={
        "open_id_connect_configuration": {
            "issuer": "https://auth.example.com",
            "token_selection": {
                "access_token_only": {
                    "audiences": ["https://myapp.example.com"],
                    "principal_id_claim": "sub",
                },
            },
            "entity_id_prefix": "MyOIDCProvider",
            "group_configuration": {
                "group_claim": "groups",
                "group_entity_type": "MyCorp::UserGroup",
            },
        },
    },
    principal_entity_type="MyCorp::User")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedPermissions.PolicyStore("example", new()
    {
        ValidationSettings = new Aws.VerifiedPermissions.Inputs.PolicyStoreValidationSettingsArgs
        {
            Mode = "STRICT",
        },
    });

    var exampleIdentitySource = new Aws.VerifiedPermissions.IdentitySource("example", new()
    {
        PolicyStoreId = example.Id,
        Configuration = new Aws.VerifiedPermissions.Inputs.IdentitySourceConfigurationArgs
        {
            OpenIdConnectConfiguration = new Aws.VerifiedPermissions.Inputs.IdentitySourceConfigurationOpenIdConnectConfigurationArgs
            {
                Issuer = "https://auth.example.com",
                TokenSelection = new Aws.VerifiedPermissions.Inputs.IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionArgs
                {
                    AccessTokenOnly = new Aws.VerifiedPermissions.Inputs.IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnlyArgs
                    {
                        Audiences = new[]
                        {
                            "https://myapp.example.com",
                        },
                        PrincipalIdClaim = "sub",
                    },
                },
                EntityIdPrefix = "MyOIDCProvider",
                GroupConfiguration = new Aws.VerifiedPermissions.Inputs.IdentitySourceConfigurationOpenIdConnectConfigurationGroupConfigurationArgs
                {
                    GroupClaim = "groups",
                    GroupEntityType = "MyCorp::UserGroup",
                },
            },
        },
        PrincipalEntityType = "MyCorp::User",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedpermissions"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := verifiedpermissions.NewPolicyStore(ctx, "example", &verifiedpermissions.PolicyStoreArgs{
			ValidationSettings: &verifiedpermissions.PolicyStoreValidationSettingsArgs{
				Mode: pulumi.String("STRICT"),
			},
		})
		if err != nil {
			return err
		}
		_, err = verifiedpermissions.NewIdentitySource(ctx, "example", &verifiedpermissions.IdentitySourceArgs{
			PolicyStoreId: example.ID(),
			Configuration: &verifiedpermissions.IdentitySourceConfigurationArgs{
				OpenIdConnectConfiguration: &verifiedpermissions.IdentitySourceConfigurationOpenIdConnectConfigurationArgs{
					Issuer: pulumi.String("https://auth.example.com"),
					TokenSelection: &verifiedpermissions.IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionArgs{
						AccessTokenOnly: &verifiedpermissions.IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnlyArgs{
							Audiences: pulumi.StringArray{
								pulumi.String("https://myapp.example.com"),
							},
							PrincipalIdClaim: pulumi.String("sub"),
						},
					},
					EntityIdPrefix: pulumi.String("MyOIDCProvider"),
					GroupConfiguration: &verifiedpermissions.IdentitySourceConfigurationOpenIdConnectConfigurationGroupConfigurationArgs{
						GroupClaim:      pulumi.String("groups"),
						GroupEntityType: pulumi.String("MyCorp::UserGroup"),
					},
				},
			},
			PrincipalEntityType: pulumi.String("MyCorp::User"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedpermissions.PolicyStore;
import com.pulumi.aws.verifiedpermissions.PolicyStoreArgs;
import com.pulumi.aws.verifiedpermissions.inputs.PolicyStoreValidationSettingsArgs;
import com.pulumi.aws.verifiedpermissions.IdentitySource;
import com.pulumi.aws.verifiedpermissions.IdentitySourceArgs;
import com.pulumi.aws.verifiedpermissions.inputs.IdentitySourceConfigurationArgs;
import com.pulumi.aws.verifiedpermissions.inputs.IdentitySourceConfigurationOpenIdConnectConfigurationArgs;
import com.pulumi.aws.verifiedpermissions.inputs.IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionArgs;
import com.pulumi.aws.verifiedpermissions.inputs.IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnlyArgs;
import com.pulumi.aws.verifiedpermissions.inputs.IdentitySourceConfigurationOpenIdConnectConfigurationGroupConfigurationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new PolicyStore("example", PolicyStoreArgs.builder()
            .validationSettings(PolicyStoreValidationSettingsArgs.builder()
                .mode("STRICT")
                .build())
            .build());

        var exampleIdentitySource = new IdentitySource("exampleIdentitySource", IdentitySourceArgs.builder()
            .policyStoreId(example.id())
            .configuration(IdentitySourceConfigurationArgs.builder()
                .openIdConnectConfiguration(IdentitySourceConfigurationOpenIdConnectConfigurationArgs.builder()
                    .issuer("https://auth.example.com")
                    .tokenSelection(IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionArgs.builder()
                        .accessTokenOnly(IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnlyArgs.builder()
                            .audiences("https://myapp.example.com")
                            .principalIdClaim("sub")
                            .build())
                        .build())
                    .entityIdPrefix("MyOIDCProvider")
                    .groupConfiguration(IdentitySourceConfigurationOpenIdConnectConfigurationGroupConfigurationArgs.builder()
                        .groupClaim("groups")
                        .groupEntityType("MyCorp::UserGroup")
                        .build())
                    .build())
                .build())
            .principalEntityType("MyCorp::User")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedpermissions:PolicyStore
    properties:
      validationSettings:
        mode: STRICT
  exampleIdentitySource:
    type: aws:verifiedpermissions:IdentitySource
    name: example
    properties:
      policyStoreId: ${example.id}
      configuration:
        openIdConnectConfiguration:
          issuer: https://auth.example.com
          tokenSelection:
            accessTokenOnly:
              audiences:
                - https://myapp.example.com
              principalIdClaim: sub
          entityIdPrefix: MyOIDCProvider
          groupConfiguration:
            groupClaim: groups
            groupEntityType: MyCorp::UserGroup
      principalEntityType: MyCorp::User
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Verified Permissions Identity Source using the `policy_store_id:identity_source_id`. For example:

```sh
$ pulumi import aws:verifiedpermissions/identitySource:IdentitySource example policy-store-id-12345678:identity-source-id-12345678
```
Ø
configurationåBâ:Ü
É
verifiedpermissionsIdentitySourceConfigurationOaws:verifiedpermissions/IdentitySourceConfiguration:IdentitySourceConfigurationéSpecifies the details required to communicate with the identity provider (IdP) associated with this identity source. See Configuration below.
k
policyStoreId" VSpecifies the ID of the policy store in which you want to store this identity source.
ò
principalEntityTypeB" {Specifies the namespace and data type of the principals generated for identities authenticated by the new identity source.
"Ø
configurationåBâ:Ü
É
verifiedpermissionsIdentitySourceConfigurationOaws:verifiedpermissions/IdentitySourceConfiguration:IdentitySourceConfigurationéSpecifies the details required to communicate with the identity provider (IdP) associated with this identity source. See Configuration below.
"k
policyStoreId" VSpecifies the ID of the policy store in which you want to store this identity source.
"ñ
principalEntityType" {Specifies the namespace and data type of the principals generated for identities authenticated by the new identity source.
*É#
D
verifiedpermissionsPolicy%aws:verifiedpermissions/policy:Policy˜Resource for managing an AWS Verified Permissions Policy.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.verifiedpermissions.Policy("test", {
    policyStoreId: testAwsVerifiedpermissionsPolicyStore.id,
    definition: {
        static: {
            statement: "permit (principal, action == Action::\"view\", resource in Album:: \"test_album\");",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.verifiedpermissions.Policy("test",
    policy_store_id=test_aws_verifiedpermissions_policy_store["id"],
    definition={
        "static": {
            "statement": "permit (principal, action == Action::\"view\", resource in Album:: \"test_album\");",
        },
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.VerifiedPermissions.Policy("test", new()
    {
        PolicyStoreId = testAwsVerifiedpermissionsPolicyStore.Id,
        Definition = new Aws.VerifiedPermissions.Inputs.PolicyDefinitionArgs
        {
            Static = new Aws.VerifiedPermissions.Inputs.PolicyDefinitionStaticArgs
            {
                Statement = "permit (principal, action == Action::\"view\", resource in Album:: \"test_album\");",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedpermissions"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedpermissions.NewPolicy(ctx, "test", &verifiedpermissions.PolicyArgs{
			PolicyStoreId: pulumi.Any(testAwsVerifiedpermissionsPolicyStore.Id),
			Definition: &verifiedpermissions.PolicyDefinitionArgs{
				Static: &verifiedpermissions.PolicyDefinitionStaticArgs{
					Statement: pulumi.String("permit (principal, action == Action::\"view\", resource in Album:: \"test_album\");"),
				},
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedpermissions.Policy;
import com.pulumi.aws.verifiedpermissions.PolicyArgs;
import com.pulumi.aws.verifiedpermissions.inputs.PolicyDefinitionArgs;
import com.pulumi.aws.verifiedpermissions.inputs.PolicyDefinitionStaticArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var test = new Policy("test", PolicyArgs.builder()
            .policyStoreId(testAwsVerifiedpermissionsPolicyStore.id())
            .definition(PolicyDefinitionArgs.builder()
                .static_(PolicyDefinitionStaticArgs.builder()
                    .statement("permit (principal, action == Action::\"view\", resource in Album:: \"test_album\");")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:verifiedpermissions:Policy
    properties:
      policyStoreId: ${testAwsVerifiedpermissionsPolicyStore.id}
      definition:
        static:
          statement: 'permit (principal, action == Action::"view", resource in Album:: "test_album");'
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Verified Permissions Policy using the `policy_id,policy_store_id`. For example:

```sh
$ pulumi import aws:verifiedpermissions/policy:Policy example policy-id-12345678,policy-store-id-12345678
```
¨

definitionhBf:d
b
verifiedpermissionsPolicyDefinition9aws:verifiedpermissions/PolicyDefinition:PolicyDefinition4The definition of the policy. See Definition below.
>
policyStoreId" )The Policy Store ID of the policy store.
"4
createdDate" !The date the policy was created.
"¨

definitionhBf:d
b
verifiedpermissionsPolicyDefinition9aws:verifiedpermissions/PolicyDefinition:PolicyDefinition4The definition of the policy. See Definition below.
"-
policyId" The Policy ID of the policy.
">
policyStoreId" )The Policy Store ID of the policy store.
*‹
S
verifiedpermissionsPolicyStore/aws:verifiedpermissions/policyStore:PolicyStoreÉ## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedpermissions.PolicyStore("example", {validationSettings: {
    mode: "STRICT",
}});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedpermissions.PolicyStore("example", validation_settings={
    "mode": "STRICT",
})
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedPermissions.PolicyStore("example", new()
    {
        ValidationSettings = new Aws.VerifiedPermissions.Inputs.PolicyStoreValidationSettingsArgs
        {
            Mode = "STRICT",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedpermissions"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedpermissions.NewPolicyStore(ctx, "example", &verifiedpermissions.PolicyStoreArgs{
			ValidationSettings: &verifiedpermissions.PolicyStoreValidationSettingsArgs{
				Mode: pulumi.String("STRICT"),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedpermissions.PolicyStore;
import com.pulumi.aws.verifiedpermissions.PolicyStoreArgs;
import com.pulumi.aws.verifiedpermissions.inputs.PolicyStoreValidationSettingsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new PolicyStore("example", PolicyStoreArgs.builder()
            .validationSettings(PolicyStoreValidationSettingsArgs.builder()
                .mode("STRICT")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedpermissions:PolicyStore
    properties:
      validationSettings:
        mode: STRICT
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Verified Permissions Policy Store using the `policy_store_id`. For example:

console

 % pulumi import aws_verifiedpermissions_policy_store.example DxQg2j8xvXJQ1tQCYNWj9T

8
descriptionB" #A description of the Policy Store.
’
validationSettingsíBè:å
â
verifiedpermissionsPolicyStoreValidationSettingsSaws:verifiedpermissions/PolicyStoreValidationSettings:PolicyStoreValidationSettings*Validation settings for the policy store.
"(
arn" The ARN of the Policy Store.
"8
descriptionB" #A description of the Policy Store.
"1
policyStoreId" The ID of the Policy Store.
"’
validationSettingsíBè:å
â
verifiedpermissionsPolicyStoreValidationSettingsSaws:verifiedpermissions/PolicyStoreValidationSettings:PolicyStoreValidationSettings*Validation settings for the policy store.
*ÿ 
\
verifiedpermissionsPolicyTemplate5aws:verifiedpermissions/policyTemplate:PolicyTemplateãResource for managing an AWS Verified Permissions Policy Template.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.verifiedpermissions.PolicyTemplate("example", {
    policyStoreId: exampleAwsVerifiedpermissionsPolicyStore.id,
    statement: "permit (principal in ?principal, action in PhotoFlash::Action::\"FullPhotoAccess\", resource == ?resource) unless { resource.IsPrivate };",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedpermissions.PolicyTemplate("example",
    policy_store_id=example_aws_verifiedpermissions_policy_store["id"],
    statement="permit (principal in ?principal, action in PhotoFlash::Action::\"FullPhotoAccess\", resource == ?resource) unless { resource.IsPrivate };")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.VerifiedPermissions.PolicyTemplate("example", new()
    {
        PolicyStoreId = exampleAwsVerifiedpermissionsPolicyStore.Id,
        Statement = "permit (principal in ?principal, action in PhotoFlash::Action::\"FullPhotoAccess\", resource == ?resource) unless { resource.IsPrivate };",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedpermissions"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedpermissions.NewPolicyTemplate(ctx, "example", &verifiedpermissions.PolicyTemplateArgs{
			PolicyStoreId: pulumi.Any(exampleAwsVerifiedpermissionsPolicyStore.Id),
			Statement:     pulumi.String("permit (principal in ?principal, action in PhotoFlash::Action::\"FullPhotoAccess\", resource == ?resource) unless { resource.IsPrivate };"),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedpermissions.PolicyTemplate;
import com.pulumi.aws.verifiedpermissions.PolicyTemplateArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new PolicyTemplate("example", PolicyTemplateArgs.builder()
            .policyStoreId(exampleAwsVerifiedpermissionsPolicyStore.id())
            .statement("permit (principal in ?principal, action in PhotoFlash::Action::\"FullPhotoAccess\", resource == ?resource) unless { resource.IsPrivate };")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedpermissions:PolicyTemplate
    properties:
      policyStoreId: ${exampleAwsVerifiedpermissionsPolicyStore.id}
      statement: permit (principal in ?principal, action in PhotoFlash::Action::"FullPhotoAccess", resource == ?resource) unless { resource.IsPrivate };
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Verified Permissions Policy Store using the `policy_store_id:policy_template_id`. For example:

```sh
$ pulumi import aws:verifiedpermissions/policyTemplate:PolicyTemplate example policyStoreId:policyTemplateId
```
E
descriptionB" 0Provides a description for the policy template.
1
policyStoreId" The ID of the Policy Store.
Ä
	statement" oDefines the content of the statement, written in Cedar policy language.

The following arguments are optional:
":
createdDate" 'The date the Policy Store was created.
"E
descriptionB" 0Provides a description for the policy template.
"1
policyStoreId" The ID of the Policy Store.
"4
policyTemplateId" The ID of the Policy Store.
"Ä
	statement" oDefines the content of the statement, written in Cedar policy language.

The following arguments are optional:
*Ö
D
verifiedpermissionsSchema%aws:verifiedpermissions/schema:Schemaæ## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedpermissions.Schema;
import com.pulumi.aws.verifiedpermissions.SchemaArgs;
import static com.pulumi.codegen.internal.Serialization.*;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        var example = new Schema("example", SchemaArgs.builder()
            .policyStoreId(exampleAwsVerifiedpermissionsPolicyStore.policyStoreId())
            .definition(SchemaDefinitionArgs.builder()
                .value(serializeJson(
                    jsonObject(
                        jsonProperty("Namespace", jsonObject(
                            jsonProperty("entityTypes", jsonObject(

                            )),
                            jsonProperty("actions", jsonObject(

                            ))
                        ))
                    )))
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:verifiedpermissions:Schema
    properties:
      policyStoreId: ${exampleAwsVerifiedpermissionsPolicyStore.policyStoreId}
      definition:
        - value:
            fn::toJSON:
              Namespace:
                entityTypes: {}
                actions: {}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Verified Permissions Policy Store Schema using the `policy_store_id`. For example:

console

 % pulumi import aws_verifiedpermissions_schema.example DxQg2j8xvXJQ1tQCYNWj9T

ñ

definitionhBf:d
b
verifiedpermissionsSchemaDefinition9aws:verifiedpermissions/SchemaDefinition:SchemaDefinitionThe definition of the schema.
1
policyStoreId" The ID of the Policy Store.
"ñ

definitionhBf:d
b
verifiedpermissionsSchemaDefinition9aws:verifiedpermissions/SchemaDefinition:SchemaDefinitionThe definition of the schema.
"d

namespaces*" P(Optional) Identifies the namespaces of the entities referenced by this schema.
"1
policyStoreId" The ID of the Policy Store.
2¯
!getArnaws:index/getArn:getArnÛParses an ARN into its constituent parts.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const dbInstance = aws.getArn({
    arn: "arn:aws:rds:eu-west-1:123456789012:db:mysql-db",
});
```
```python
import pulumi
import pulumi_aws as aws

db_instance = aws.get_arn(arn="arn:aws:rds:eu-west-1:123456789012:db:mysql-db")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var dbInstance = Aws.GetArn.Invoke(new()
    {
        Arn = "arn:aws:rds:eu-west-1:123456789012:db:mysql-db",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetArn(ctx, &aws.GetArnArgs{
			Arn: "arn:aws:rds:eu-west-1:123456789012:db:mysql-db",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetArnArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var dbInstance = AwsFunctions.getArn(GetArnArgs.builder()
            .arn("arn:aws:rds:eu-west-1:123456789012:db:mysql-db")
            .build());

    }
}
```
```yaml
variables:
  dbInstance:
    fn::invoke:
      function: aws:getArn
      arguments:
        arn: arn:aws:rds:eu-west-1:123456789012:db:mysql-db
```
<!--End PulumiCodeChooser -->

arn" ARN to parse.


idB" "û
account" éThe [ID](https://docs.aws.amazon.com/general/latest/gr/acct-identifiers.html) of the AWS account that owns the resource, without the hyphens.
"	
arn" "
id" "4
	partition" #Partition that the resource is in.
"í
region" ÉRegion the resource resides in.
Note that the ARNs for some resources do not require a region, so this component might be omitted.
"˛
resource" ÌContent of this part of the ARN varies by service.
It often includes an indicator of the type of resource‚Äîfor example, an IAM user or Amazon RDS database ‚Äîfollowed by a slash (/) or a colon (:), followed by the resource name itself.
"≥
service" £The [service namespace](https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#genref-aws-service-namespaces) that identifies the AWS product.
2Â[
HgetAvailabilityZone1aws:index/getAvailabilityZone:getAvailabilityZone¢J`aws.getAvailabilityZone` provides details about a specific availability zone (AZ)
in the current region.

This can be used both to validate an availability zone given in a variable
and to split the AZ name into its component parts of an AWS region and an
AZ identifier letter. The latter may be useful e.g., for implementing a
consistent subnet numbering scheme across several regions by mapping both
the region and the subnet letter to network numbers.

This is different from the `aws.getAvailabilityZones` (plural) data source,
which provides a list of the available zones.

## Example Usage

The following example shows how this data source might be used to derive
VPC and subnet CIDR prefixes systematically for an availability zone.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const config = new pulumi.Config();
const regionNumber = config.getObject("regionNumber") || {
    "ap-northeast-1": 5,
    "eu-central-1": 4,
    "us-east-1": 1,
    "us-west-1": 2,
    "us-west-2": 3,
};
const azNumber = config.getObject("azNumber") || {
    a: 1,
    b: 2,
    c: 3,
    d: 4,
    e: 5,
    f: 6,
};
// Retrieve the AZ where we want to create network resources
// This must be in the region selected on the AWS provider.
const example = aws.getAvailabilityZone({
    name: "eu-central-1a",
});
// Create a VPC for the region associated with the AZ
const exampleVpc = new aws.ec2.Vpc("example", {cidrBlock: example.then(example => std.cidrsubnet({
    input: "10.0.0.0/8",
    newbits: 4,
    netnum: regionNumber[example.region],
})).then(invoke => invoke.result)});
// Create a subnet for the AZ within the regional VPC
const exampleSubnet = new aws.ec2.Subnet("example", {
    vpcId: exampleVpc.id,
    cidrBlock: pulumi.all([exampleVpc.cidrBlock, example]).apply(([cidrBlock, example]) => std.cidrsubnetOutput({
        input: cidrBlock,
        newbits: 4,
        netnum: azNumber[example.nameSuffix],
    })).apply(invoke => invoke.result),
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

config = pulumi.Config()
region_number = config.get_object("regionNumber")
if region_number is None:
    region_number = {
        "ap-northeast-1": 5,
        "eu-central-1": 4,
        "us-east-1": 1,
        "us-west-1": 2,
        "us-west-2": 3,
    }
az_number = config.get_object("azNumber")
if az_number is None:
    az_number = {
        "a": 1,
        "b": 2,
        "c": 3,
        "d": 4,
        "e": 5,
        "f": 6,
    }
# Retrieve the AZ where we want to create network resources
# This must be in the region selected on the AWS provider.
example = aws.get_availability_zone(name="eu-central-1a")
# Create a VPC for the region associated with the AZ
example_vpc = aws.ec2.Vpc("example", cidr_block=std.cidrsubnet(input="10.0.0.0/8",
    newbits=4,
    netnum=region_number[example.region]).result)
# Create a subnet for the AZ within the regional VPC
example_subnet = aws.ec2.Subnet("example",
    vpc_id=example_vpc.id,
    cidr_block=example_vpc.cidr_block.apply(lambda cidr_block: std.cidrsubnet_output(input=cidr_block,
        newbits=4,
        netnum=az_number[example.name_suffix])).apply(lambda invoke: invoke.result))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var config = new Config();
    var regionNumber = config.GetObject<dynamic>("regionNumber") ?? 
    {
        { "ap-northeast-1", 5 },
        { "eu-central-1", 4 },
        { "us-east-1", 1 },
        { "us-west-1", 2 },
        { "us-west-2", 3 },
    };
    var azNumber = config.GetObject<dynamic>("azNumber") ?? 
    {
        { "a", 1 },
        { "b", 2 },
        { "c", 3 },
        { "d", 4 },
        { "e", 5 },
        { "f", 6 },
    };
    // Retrieve the AZ where we want to create network resources
    // This must be in the region selected on the AWS provider.
    var example = Aws.GetAvailabilityZone.Invoke(new()
    {
        Name = "eu-central-1a",
    });

    // Create a VPC for the region associated with the AZ
    var exampleVpc = new Aws.Ec2.Vpc("example", new()
    {
        CidrBlock = Std.Cidrsubnet.Invoke(new()
        {
            Input = "10.0.0.0/8",
            Newbits = 4,
            Netnum = regionNumber[example.Apply(getAvailabilityZoneResult => getAvailabilityZoneResult.Region)],
        }).Apply(invoke => invoke.Result),
    });

    // Create a subnet for the AZ within the regional VPC
    var exampleSubnet = new Aws.Ec2.Subnet("example", new()
    {
        VpcId = exampleVpc.Id,
        CidrBlock = Output.Tuple(exampleVpc.CidrBlock, example).Apply(values =>
        {
            var cidrBlock = values.Item1;
            var example = values.Item2;
            return Std.Cidrsubnet.Invoke(new()
            {
                Input = cidrBlock,
                Newbits = 4,
                Netnum = azNumber[example.Apply(getAvailabilityZoneResult => getAvailabilityZoneResult.NameSuffix)],
            });
        }).Apply(invoke => invoke.Result),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi/config"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		cfg := config.New(ctx, "")
		regionNumber := map[string]interface{}{
			"ap-northeast-1": 5,
			"eu-central-1":   4,
			"us-east-1":      1,
			"us-west-1":      2,
			"us-west-2":      3,
		}
		if param := cfg.GetObject("regionNumber"); param != nil {
			regionNumber = param
		}
		azNumber := map[string]interface{}{
			"a": 1,
			"b": 2,
			"c": 3,
			"d": 4,
			"e": 5,
			"f": 6,
		}
		if param := cfg.GetObject("azNumber"); param != nil {
			azNumber = param
		}
		// Retrieve the AZ where we want to create network resources
		// This must be in the region selected on the AWS provider.
		example, err := aws.GetAvailabilityZone(ctx, &aws.GetAvailabilityZoneArgs{
			Name: pulumi.StringRef("eu-central-1a"),
		}, nil)
		if err != nil {
			return err
		}
		invokeCidrsubnet, err := std.Cidrsubnet(ctx, &std.CidrsubnetArgs{
			Input:   "10.0.0.0/8",
			Newbits: 4,
			Netnum:  regionNumber[example.Region],
		}, nil)
		if err != nil {
			return err
		}
		// Create a VPC for the region associated with the AZ
		exampleVpc, err := ec2.NewVpc(ctx, "example", &ec2.VpcArgs{
			CidrBlock: pulumi.String(invokeCidrsubnet.Result),
		})
		if err != nil {
			return err
		}
		// Create a subnet for the AZ within the regional VPC
		_, err = ec2.NewSubnet(ctx, "example", &ec2.SubnetArgs{
			VpcId: exampleVpc.ID(),
			CidrBlock: pulumi.String(exampleVpc.CidrBlock.ApplyT(func(cidrBlock string) (std.CidrsubnetResult, error) {
				return std.CidrsubnetResult(interface{}(std.CidrsubnetOutput(ctx, std.CidrsubnetOutputArgs{
					Input:   cidrBlock,
					Newbits: 4,
					Netnum:  pulumi.Int(azNumber[example.NameSuffix]),
				}, nil))), nil
			}).(std.CidrsubnetResultOutput).ApplyT(func(invoke std.CidrsubnetResult) (*string, error) {
				return invoke.Result, nil
			}).(pulumi.StringPtrOutput)),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZoneArgs;
import com.pulumi.aws.ec2.Vpc;
import com.pulumi.aws.ec2.VpcArgs;
import com.pulumi.aws.ec2.Subnet;
import com.pulumi.aws.ec2.SubnetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var config = ctx.config();
        final var regionNumber = config.get("regionNumber").orElse(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference));
        final var azNumber = config.get("azNumber").orElse(%!v(PANIC=Format method: runtime error: invalid memory address or nil pointer dereference));
        // Retrieve the AZ where we want to create network resources
        // This must be in the region selected on the AWS provider.
        final var example = AwsFunctions.getAvailabilityZone(GetAvailabilityZoneArgs.builder()
            .name("eu-central-1a")
            .build());

        // Create a VPC for the region associated with the AZ
        var exampleVpc = new Vpc("exampleVpc", VpcArgs.builder()
            .cidrBlock(StdFunctions.cidrsubnet(CidrsubnetArgs.builder()
                .input("10.0.0.0/8")
                .newbits(4)
                .netnum(regionNumber[example.applyValue(getAvailabilityZoneResult -> getAvailabilityZoneResult.region())])
                .build()).result())
            .build());

        // Create a subnet for the AZ within the regional VPC
        var exampleSubnet = new Subnet("exampleSubnet", SubnetArgs.builder()
            .vpcId(exampleVpc.id())
            .cidrBlock(exampleVpc.cidrBlock().applyValue(cidrBlock -> StdFunctions.cidrsubnet()).applyValue(invoke -> invoke.result()))
            .build());

    }
}
```
<!--End PulumiCodeChooser -->
Ä
allAvailabilityZonesB
 bSet to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
•
filtersbB`*^:\
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter6Configuration block(s) for filtering. Detailed below.
<
nameB" .Full name of the availability zone to select.
z
stateB" kSpecific availability zone state to require. May be any of `"available"`, `"information"` or `"impaired"`.
<
zoneIdB" ,Zone ID of the availability zone to select.
"
allAvailabilityZonesB
 "m
filtersbB`*^:\
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter"ß
	groupName" ïFor Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "Ÿ

nameSuffix" ∆Part of the AZ name that appears after the region name, uniquely identifying the AZ within its region.
For Availability Zones this is usually a single letter, for example `a` for the `us-west-2a` zone.
For Local and Wavelength Zones this is a longer string, for example `wl1-sfo-wlz-1` for the `us-west-2-wl1-sfo-wlz-1` zone.
"Y
networkBorderGroup" ?The name of the location from which the address is advertised.
"√
optInStatus" ØFor Availability Zones, this always has the value of `opt-in-not-required`. For Local Zones, this is the opt in status. The possible values are `opted-in` and `not-opted-in`.
"á
parentZoneId" sID of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
"ã
parentZoneName" uName of the zone that handles some of the Local Zone or Wavelength Zone control plane operations, such as API calls.
"±
region" ¢Region where the selected availability zone resides. This is always the region selected on the provider, since this data source searches only within that region.
"
state" "
zoneId" "c
zoneType" SType of zone. Values are `availability-zone`, `local-zone`, and `wavelength-zone`.
2Ã\
KgetAvailabilityZones3aws:index/getAvailabilityZones:getAvailabilityZones˙PThe Availability Zones data source allows access to the list of AWS
Availability Zones which can be accessed by an AWS account within the region
configured in the provider.

This is different from the `aws.getAvailabilityZone` (singular) data source,
which provides some details about a specific availability zone.

> When [Local Zones](https://aws.amazon.com/about-aws/global-infrastructure/localzones/) are enabled in a region, by default the API and this data source include both Local Zones and Availability Zones. To return only Availability Zones, see the example section below.

## Example Usage

### By State

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

// Declare the data source
const available = aws.getAvailabilityZones({
    state: "available",
});
// e.g., Create subnets in the first two available availability zones
const primary = new aws.ec2.Subnet("primary", {availabilityZone: available.then(available => available.names?.[0])});
const secondary = new aws.ec2.Subnet("secondary", {availabilityZone: available.then(available => available.names?.[1])});
```
```python
import pulumi
import pulumi_aws as aws

# Declare the data source
available = aws.get_availability_zones(state="available")
# e.g., Create subnets in the first two available availability zones
primary = aws.ec2.Subnet("primary", availability_zone=available.names[0])
secondary = aws.ec2.Subnet("secondary", availability_zone=available.names[1])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    // Declare the data source
    var available = Aws.GetAvailabilityZones.Invoke(new()
    {
        State = "available",
    });

    // e.g., Create subnets in the first two available availability zones
    var primary = new Aws.Ec2.Subnet("primary", new()
    {
        AvailabilityZone = available.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[0]),
    });

    var secondary = new Aws.Ec2.Subnet("secondary", new()
    {
        AvailabilityZone = available.Apply(getAvailabilityZonesResult => getAvailabilityZonesResult.Names[1]),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		// Declare the data source
		available, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			State: pulumi.StringRef("available"),
		}, nil)
		if err != nil {
			return err
		}
		// e.g., Create subnets in the first two available availability zones
		_, err = ec2.NewSubnet(ctx, "primary", &ec2.SubnetArgs{
			AvailabilityZone: pulumi.String(available.Names[0]),
		})
		if err != nil {
			return err
		}
		_, err = ec2.NewSubnet(ctx, "secondary", &ec2.SubnetArgs{
			AvailabilityZone: pulumi.String(available.Names[1]),
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import com.pulumi.aws.ec2.Subnet;
import com.pulumi.aws.ec2.SubnetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        // Declare the data source
        final var available = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .state("available")
            .build());

        // e.g., Create subnets in the first two available availability zones
        var primary = new Subnet("primary", SubnetArgs.builder()
            .availabilityZone(available.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[0]))
            .build());

        var secondary = new Subnet("secondary", SubnetArgs.builder()
            .availabilityZone(available.applyValue(getAvailabilityZonesResult -> getAvailabilityZonesResult.names()[1]))
            .build());

    }
}
```
```yaml
resources:
  # e.g., Create subnets in the first two available availability zones
  primary:
    type: aws:ec2:Subnet
    properties:
      availabilityZone: ${available.names[0]}
  secondary:
    type: aws:ec2:Subnet
    properties:
      availabilityZone: ${available.names[1]}
variables:
  # Declare the data source
  available:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        state: available
```
<!--End PulumiCodeChooser -->

### By Filter

All Local Zones (regardless of opt-in status):

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getAvailabilityZones({
    allAvailabilityZones: true,
    filters: [{
        name: "opt-in-status",
        values: [
            "not-opted-in",
            "opted-in",
        ],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_availability_zones(all_availability_zones=True,
    filters=[{
        "name": "opt-in-status",
        "values": [
            "not-opted-in",
            "opted-in",
        ],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetAvailabilityZones.Invoke(new()
    {
        AllAvailabilityZones = true,
        Filters = new[]
        {
            new Aws.Inputs.GetAvailabilityZonesFilterInputArgs
            {
                Name = "opt-in-status",
                Values = new[]
                {
                    "not-opted-in",
                    "opted-in",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			AllAvailabilityZones: pulumi.BoolRef(true),
			Filters: []aws.GetAvailabilityZonesFilter{
				{
					Name: "opt-in-status",
					Values: []string{
						"not-opted-in",
						"opted-in",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .allAvailabilityZones(true)
            .filters(GetAvailabilityZonesFilterArgs.builder()
                .name("opt-in-status")
                .values(                
                    "not-opted-in",
                    "opted-in")
                .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        allAvailabilityZones: true
        filters:
          - name: opt-in-status
            values:
              - not-opted-in
              - opted-in
```
<!--End PulumiCodeChooser -->

Only Availability Zones (no Local Zones):

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getAvailabilityZones({
    filters: [{
        name: "opt-in-status",
        values: ["opt-in-not-required"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_availability_zones(filters=[{
    "name": "opt-in-status",
    "values": ["opt-in-not-required"],
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetAvailabilityZones.Invoke(new()
    {
        Filters = new[]
        {
            new Aws.Inputs.GetAvailabilityZonesFilterInputArgs
            {
                Name = "opt-in-status",
                Values = new[]
                {
                    "opt-in-not-required",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetAvailabilityZones(ctx, &aws.GetAvailabilityZonesArgs{
			Filters: []aws.GetAvailabilityZonesFilter{
				{
					Name: "opt-in-status",
					Values: []string{
						"opt-in-not-required",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetAvailabilityZonesArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = AwsFunctions.getAvailabilityZones(GetAvailabilityZonesArgs.builder()
            .filters(GetAvailabilityZonesFilterArgs.builder()
                .name("opt-in-status")
                .values("opt-in-not-required")
                .build())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:getAvailabilityZones
      arguments:
        filters:
          - name: opt-in-status
            values:
              - opt-in-not-required
```
<!--End PulumiCodeChooser -->
Ä
allAvailabilityZonesB
 bSet to `true` to include all Availability Zones and Local Zones regardless of your opt in status.
D
excludeNamesB*" ,List of Availability Zone names to exclude.
D
excludeZoneIdsB*" *List of Availability Zone IDs to exclude.
®
filterseBc*a:_
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter6Configuration block(s) for filtering. Detailed below.
∂
stateB" ¶Allows to filter list of Availability Zones based on their
current state. Can be either `"available"`, `"information"`, `"impaired"` or
`"unavailable"`. By default the list includes a complete set of Availability Zones
to which the underlying AWS account has access, regardless of their state.
"
allAvailabilityZonesB
 "
excludeNamesB*" "
excludeZoneIdsB*" "p
filterseBc*a:_
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter"÷

groupNames*" ¡A set of the Availability Zone Group names. For Availability Zones, this is the same value as the Region name. For Local Zones, the name of the associated group, for example `us-west-2-lax-1`.
"E
id" ;The provider-assigned unique ID for this managed resource.
"M
names*" >List of the Availability Zone names available to the account.
"
stateB" "M
zoneIds*" <List of the Availability Zone IDs available to the account.
2Û[
WgetBillingServiceAccount;aws:index/getBillingServiceAccount:getBillingServiceAccount˙YUse this data source to get the Account ID of the [AWS Billing and Cost Management Service Account](http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/billing-getting-started.html#step-2) for the purpose of permitting in S3 bucket policy.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const main = aws.getBillingServiceAccount({});
const billingLogs = new aws.s3.BucketV2("billing_logs", {bucket: "my-billing-tf-test-bucket"});
const billingLogsAcl = new aws.s3.BucketAclV2("billing_logs_acl", {
    bucket: billingLogs.id,
    acl: "private",
});
const allowBillingLogging = pulumi.all([main, billingLogs.arn, main, billingLogs.arn]).apply(([main, billingLogsArn, main1, billingLogsArn1]) => aws.iam.getPolicyDocumentOutput({
    statements: [
        {
            effect: "Allow",
            principals: [{
                type: "AWS",
                identifiers: [main.arn],
            }],
            actions: [
                "s3:GetBucketAcl",
                "s3:GetBucketPolicy",
            ],
            resources: [billingLogsArn],
        },
        {
            effect: "Allow",
            principals: [{
                type: "AWS",
                identifiers: [main1.arn],
            }],
            actions: ["s3:PutObject"],
            resources: [`${billingLogsArn1}/*`],
        },
    ],
}));
const allowBillingLoggingBucketPolicy = new aws.s3.BucketPolicy("allow_billing_logging", {
    bucket: billingLogs.id,
    policy: allowBillingLogging.apply(allowBillingLogging => allowBillingLogging.json),
});
```
```python
import pulumi
import pulumi_aws as aws

main = aws.get_billing_service_account()
billing_logs = aws.s3.BucketV2("billing_logs", bucket="my-billing-tf-test-bucket")
billing_logs_acl = aws.s3.BucketAclV2("billing_logs_acl",
    bucket=billing_logs.id,
    acl="private")
allow_billing_logging = pulumi.Output.all(
    billingLogsArn=billing_logs.arn,
    billingLogsArn1=billing_logs.arn
).apply(lambda resolved_outputs: aws.iam.get_policy_document_output(statements=[
    {
        "effect": "Allow",
        "principals": [{
            "type": "AWS",
            "identifiers": [main.arn],
        }],
        "actions": [
            "s3:GetBucketAcl",
            "s3:GetBucketPolicy",
        ],
        "resources": [resolved_outputs['billingLogsArn']],
    },
    {
        "effect": "Allow",
        "principals": [{
            "type": "AWS",
            "identifiers": [main.arn],
        }],
        "actions": ["s3:PutObject"],
        "resources": [f"{resolved_outputs['billingLogsArn1']}/*"],
    },
]))

allow_billing_logging_bucket_policy = aws.s3.BucketPolicy("allow_billing_logging",
    bucket=billing_logs.id,
    policy=allow_billing_logging.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var main = Aws.GetBillingServiceAccount.Invoke();

    var billingLogs = new Aws.S3.BucketV2("billing_logs", new()
    {
        Bucket = "my-billing-tf-test-bucket",
    });

    var billingLogsAcl = new Aws.S3.BucketAclV2("billing_logs_acl", new()
    {
        Bucket = billingLogs.Id,
        Acl = "private",
    });

    var allowBillingLogging = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            main.Apply(getBillingServiceAccountResult => getBillingServiceAccountResult.Arn),
                        },
                    },
                },
                Actions = new[]
                {
                    "s3:GetBucketAcl",
                    "s3:GetBucketPolicy",
                },
                Resources = new[]
                {
                    billingLogs.Arn,
                },
            },
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            main.Apply(getBillingServiceAccountResult => getBillingServiceAccountResult.Arn),
                        },
                    },
                },
                Actions = new[]
                {
                    "s3:PutObject",
                },
                Resources = new[]
                {
                    $"{billingLogs.Arn}/*",
                },
            },
        },
    });

    var allowBillingLoggingBucketPolicy = new Aws.S3.BucketPolicy("allow_billing_logging", new()
    {
        Bucket = billingLogs.Id,
        Policy = allowBillingLogging.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
main, err := aws.GetBillingServiceAccount(ctx, &aws.GetBillingServiceAccountArgs{
}, nil);
if err != nil {
return err
}
billingLogs, err := s3.NewBucketV2(ctx, "billing_logs", &s3.BucketV2Args{
Bucket: pulumi.String("my-billing-tf-test-bucket"),
})
if err != nil {
return err
}
_, err = s3.NewBucketAclV2(ctx, "billing_logs_acl", &s3.BucketAclV2Args{
Bucket: billingLogs.ID(),
Acl: pulumi.String("private"),
})
if err != nil {
return err
}
allowBillingLogging := pulumi.All(billingLogs.Arn,billingLogs.Arn).ApplyT(func(_args []interface{}) (iam.GetPolicyDocumentResult, error) {
billingLogsArn := _args[0].(string)
billingLogsArn1 := _args[1].(string)
return iam.GetPolicyDocumentResult(interface{}(iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
Statements: []iam.GetPolicyDocumentStatement{
{
Effect: "Allow",
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "AWS",
Identifiers: interface{}{
main.Arn,
},
},
},
Actions: []string{
"s3:GetBucketAcl",
"s3:GetBucketPolicy",
},
Resources: []string{
billingLogsArn,
},
},
{
Effect: "Allow",
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "AWS",
Identifiers: interface{}{
main.Arn,
},
},
},
Actions: []string{
"s3:PutObject",
},
Resources: []string{
fmt.Sprintf("%v/*", billingLogsArn1),
},
},
},
}, nil))), nil
}).(iam.GetPolicyDocumentResultOutput)
_, err = s3.NewBucketPolicy(ctx, "allow_billing_logging", &s3.BucketPolicyArgs{
Bucket: billingLogs.ID(),
Policy: pulumi.String(allowBillingLogging.ApplyT(func(allowBillingLogging iam.GetPolicyDocumentResult) (*string, error) {
return &allowBillingLogging.Json, nil
}).(pulumi.StringPtrOutput)),
})
if err != nil {
return err
}
return nil
})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetBillingServiceAccountArgs;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.s3.BucketAclV2;
import com.pulumi.aws.s3.BucketAclV2Args;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.s3.BucketPolicy;
import com.pulumi.aws.s3.BucketPolicyArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var main = AwsFunctions.getBillingServiceAccount();

        var billingLogs = new BucketV2("billingLogs", BucketV2Args.builder()
            .bucket("my-billing-tf-test-bucket")
            .build());

        var billingLogsAcl = new BucketAclV2("billingLogsAcl", BucketAclV2Args.builder()
            .bucket(billingLogs.id())
            .acl("private")
            .build());

        final var allowBillingLogging = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(            
                GetPolicyDocumentStatementArgs.builder()
                    .effect("Allow")
                    .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                        .type("AWS")
                        .identifiers(main.applyValue(getBillingServiceAccountResult -> getBillingServiceAccountResult.arn()))
                        .build())
                    .actions(                    
                        "s3:GetBucketAcl",
                        "s3:GetBucketPolicy")
                    .resources(billingLogs.arn())
                    .build(),
                GetPolicyDocumentStatementArgs.builder()
                    .effect("Allow")
                    .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                        .type("AWS")
                        .identifiers(main.applyValue(getBillingServiceAccountResult -> getBillingServiceAccountResult.arn()))
                        .build())
                    .actions("s3:PutObject")
                    .resources(billingLogs.arn().applyValue(arn -> String.format("%s/*", arn)))
                    .build())
            .build());

        var allowBillingLoggingBucketPolicy = new BucketPolicy("allowBillingLoggingBucketPolicy", BucketPolicyArgs.builder()
            .bucket(billingLogs.id())
            .policy(allowBillingLogging.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(allowBillingLogging -> allowBillingLogging.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

    }
}
```
```yaml
resources:
  billingLogs:
    type: aws:s3:BucketV2
    name: billing_logs
    properties:
      bucket: my-billing-tf-test-bucket
  billingLogsAcl:
    type: aws:s3:BucketAclV2
    name: billing_logs_acl
    properties:
      bucket: ${billingLogs.id}
      acl: private
  allowBillingLoggingBucketPolicy:
    type: aws:s3:BucketPolicy
    name: allow_billing_logging
    properties:
      bucket: ${billingLogs.id}
      policy: ${allowBillingLogging.json}
variables:
  main:
    fn::invoke:
      function: aws:getBillingServiceAccount
      arguments: {}
  allowBillingLogging:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - ${main.arn}
            actions:
              - s3:GetBucketAcl
              - s3:GetBucketPolicy
            resources:
              - ${billingLogs.arn}
          - effect: Allow
            principals:
              - type: AWS
                identifiers:
                  - ${main.arn}
            actions:
              - s3:PutObject
            resources:
              - ${billingLogs.arn}/*
```
<!--End PulumiCodeChooser -->
3
idB" 'ID of the AWS billing service account.
"3
arn" (ARN of the AWS billing service account.
"1
id" 'ID of the AWS billing service account.
2û
BgetCallerIdentity-aws:index/getCallerIdentity:getCallerIdentity◊Use this data source to get the access to the effective Account ID, User ID, and ARN in
which this provider is authorized.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getCallerIdentity({});
export const accountId = current.then(current => current.accountId);
export const callerArn = current.then(current => current.arn);
export const callerUser = current.then(current => current.userId);
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_caller_identity()
pulumi.export("accountId", current.account_id)
pulumi.export("callerArn", current.arn)
pulumi.export("callerUser", current.user_id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetCallerIdentity.Invoke();

    return new Dictionary<string, object?>
    {
        ["accountId"] = current.Apply(getCallerIdentityResult => getCallerIdentityResult.AccountId),
        ["callerArn"] = current.Apply(getCallerIdentityResult => getCallerIdentityResult.Arn),
        ["callerUser"] = current.Apply(getCallerIdentityResult => getCallerIdentityResult.UserId),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetCallerIdentity(ctx, &aws.GetCallerIdentityArgs{}, nil)
		if err != nil {
			return err
		}
		ctx.Export("accountId", current.AccountId)
		ctx.Export("callerArn", current.Arn)
		ctx.Export("callerUser", current.UserId)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetCallerIdentityArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getCallerIdentity();

        ctx.export("accountId", current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.accountId()));
        ctx.export("callerArn", current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.arn()));
        ctx.export("callerUser", current.applyValue(getCallerIdentityResult -> getCallerIdentityResult.userId()));
    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getCallerIdentity
      arguments: {}
outputs:
  accountId: ${current.accountId}
  callerArn: ${current.arn}
  callerUser: ${current.userId}
```
<!--End PulumiCodeChooser -->
W
idB" KAccount ID number of the account that owns or contains the calling entity.
"`
	accountId" OAWS Account ID number of the account that owns or contains the calling entity.
"3
arn" (ARN associated with the calling entity.
"U
id" KAccount ID number of the account that owns or contains the calling entity.
"7
userId" )Unique identifier of the calling entity.
2Ç
9getDefaultTags'aws:index/getDefaultTags:getDefaultTagsÚUse this data source to get the default tags configured on the provider.

With this data source, you can apply default tags to resources not _directly_ managed by a resource, such as the instances underneath an Auto Scaling group or the volumes created for an EC2 instance.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getDefaultTags({});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_default_tags()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetDefaultTags.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetDefaultTags(ctx, &aws.GetDefaultTagsArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetDefaultTagsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = AwsFunctions.getDefaultTags();

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:getDefaultTags
      arguments: {}
```
<!--End PulumiCodeChooser -->

### Dynamically Apply Default Tags to Auto Scaling Group

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.getDefaultTags({});
const exampleGroup = new aws.autoscaling.Group("example", {tags: .map(entry => ({
    key: entry.key,
    value: entry.value,
    propagateAtLaunch: true,
}))});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.get_default_tags()
example_group = aws.autoscaling.Group("example", tags=[{
    "key": entry["key"],
    "value": entry["value"],
    "propagate_at_launch": True,
} for entry in [{"key": k, "value": v} for k, v in example.tags]])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.GetDefaultTags.Invoke();

    var exampleGroup = new Aws.AutoScaling.Group("example", new()
    {
        Tags = ,
    });

});
```
<!--End PulumiCodeChooser -->


idB" "
id" ":
tags2" ,Key-value mapping of provider default tags.
2”;
0getIpRanges!aws:index/getIpRanges:getIpRangesç1Use this data source to get the IP ranges of various AWS products and services. For more information about the contents of this data source and required JSON syntax if referencing a custom URL, see the [AWS IP Address Ranges documentation](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html).

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const europeanEc2 = aws.getIpRanges({
    regions: [
        "eu-west-1",
        "eu-central-1",
    ],
    services: ["ec2"],
});
const fromEurope = new aws.ec2.SecurityGroup("from_europe", {
    name: "from_europe",
    ingress: [{
        fromPort: 443,
        toPort: 443,
        protocol: "tcp",
        cidrBlocks: europeanEc2.then(europeanEc2 => europeanEc2.cidrBlocks),
        ipv6CidrBlocks: europeanEc2.then(europeanEc2 => europeanEc2.ipv6CidrBlocks),
    }],
    tags: {
        CreateDate: europeanEc2.then(europeanEc2 => europeanEc2.createDate),
        SyncToken: europeanEc2.then(europeanEc2 => europeanEc2.syncToken),
    },
});
```
```python
import pulumi
import pulumi_aws as aws

european_ec2 = aws.get_ip_ranges(regions=[
        "eu-west-1",
        "eu-central-1",
    ],
    services=["ec2"])
from_europe = aws.ec2.SecurityGroup("from_europe",
    name="from_europe",
    ingress=[{
        "from_port": 443,
        "to_port": 443,
        "protocol": "tcp",
        "cidr_blocks": european_ec2.cidr_blocks,
        "ipv6_cidr_blocks": european_ec2.ipv6_cidr_blocks,
    }],
    tags={
        "CreateDate": european_ec2.create_date,
        "SyncToken": european_ec2.sync_token,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var europeanEc2 = Aws.GetIpRanges.Invoke(new()
    {
        Regions = new[]
        {
            "eu-west-1",
            "eu-central-1",
        },
        Services = new[]
        {
            "ec2",
        },
    });

    var fromEurope = new Aws.Ec2.SecurityGroup("from_europe", new()
    {
        Name = "from_europe",
        Ingress = new[]
        {
            new Aws.Ec2.Inputs.SecurityGroupIngressArgs
            {
                FromPort = 443,
                ToPort = 443,
                Protocol = "tcp",
                CidrBlocks = europeanEc2.Apply(getIpRangesResult => getIpRangesResult.CidrBlocks),
                Ipv6CidrBlocks = europeanEc2.Apply(getIpRangesResult => getIpRangesResult.Ipv6CidrBlocks),
            },
        },
        Tags = 
        {
            { "CreateDate", europeanEc2.Apply(getIpRangesResult => getIpRangesResult.CreateDate) },
            { "SyncToken", europeanEc2.Apply(getIpRangesResult => getIpRangesResult.SyncToken) },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		europeanEc2, err := aws.GetIpRanges(ctx, &aws.GetIpRangesArgs{
			Regions: []string{
				"eu-west-1",
				"eu-central-1",
			},
			Services: []string{
				"ec2",
			},
		}, nil)
		if err != nil {
			return err
		}
		_, err = ec2.NewSecurityGroup(ctx, "from_europe", &ec2.SecurityGroupArgs{
			Name: pulumi.String("from_europe"),
			Ingress: ec2.SecurityGroupIngressArray{
				&ec2.SecurityGroupIngressArgs{
					FromPort:       pulumi.Int(443),
					ToPort:         pulumi.Int(443),
					Protocol:       pulumi.String("tcp"),
					CidrBlocks:     interface{}(europeanEc2.CidrBlocks),
					Ipv6CidrBlocks: interface{}(europeanEc2.Ipv6CidrBlocks),
				},
			},
			Tags: pulumi.StringMap{
				"CreateDate": pulumi.String(europeanEc2.CreateDate),
				"SyncToken":  pulumi.Int(europeanEc2.SyncToken),
			},
		})
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetIpRangesArgs;
import com.pulumi.aws.ec2.SecurityGroup;
import com.pulumi.aws.ec2.SecurityGroupArgs;
import com.pulumi.aws.ec2.inputs.SecurityGroupIngressArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var europeanEc2 = AwsFunctions.getIpRanges(GetIpRangesArgs.builder()
            .regions(            
                "eu-west-1",
                "eu-central-1")
            .services("ec2")
            .build());

        var fromEurope = new SecurityGroup("fromEurope", SecurityGroupArgs.builder()
            .name("from_europe")
            .ingress(SecurityGroupIngressArgs.builder()
                .fromPort("443")
                .toPort("443")
                .protocol("tcp")
                .cidrBlocks(europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.cidrBlocks()))
                .ipv6CidrBlocks(europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.ipv6CidrBlocks()))
                .build())
            .tags(Map.ofEntries(
                Map.entry("CreateDate", europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.createDate())),
                Map.entry("SyncToken", europeanEc2.applyValue(getIpRangesResult -> getIpRangesResult.syncToken()))
            ))
            .build());

    }
}
```
```yaml
resources:
  fromEurope:
    type: aws:ec2:SecurityGroup
    name: from_europe
    properties:
      name: from_europe
      ingress:
        - fromPort: '443'
          toPort: '443'
          protocol: tcp
          cidrBlocks: ${europeanEc2.cidrBlocks}
          ipv6CidrBlocks: ${europeanEc2.ipv6CidrBlocks}
      tags:
        CreateDate: ${europeanEc2.createDate}
        SyncToken: ${europeanEc2.syncToken}
variables:
  europeanEc2:
    fn::invoke:
      function: aws:getIpRanges
      arguments:
        regions:
          - eu-west-1
          - eu-central-1
        services:
          - ec2
```
<!--End PulumiCodeChooser -->


idB" ¥
regionsB*" †Filter IP ranges by regions (or include all regions, if
omitted). Valid items are `global` (for `cloudfront`) as well as all AWS regions
(e.g., `eu-central-1`)
Ê
services*" ”Filter IP ranges by services. Valid items are `amazon`
(for amazon.com), `amazon_connect`, `api_gateway`, `cloud9`, `cloudfront`,
`codebuild`, `dynamodb`, `ec2`, `ec2_instance_connect`, `globalaccelerator`,
`route53`, `route53_healthchecks`, `s3` and `workspaces_gateways`. See the
[`service` attribute][2] documentation for other possible values.

> **NOTE:** If the specified combination of regions and services does not yield any
CIDR blocks, this call will fail.
Ë
urlB" ⁄Custom URL for source JSON file. Syntax must match [AWS IP Address Ranges documentation](https://docs.aws.amazon.com/general/latest/gr/aws-ip-ranges.html). Defaults to `https://ip-ranges.amazonaws.com/ip-ranges.json`.
";

cidrBlocks*" 'Lexically ordered list of CIDR blocks.
"S

createDate" APublication time of the IP ranges (e.g., `2016-08-03-23-46-05`).
"
id" "D
ipv6CidrBlocks*" ,Lexically ordered list of IPv6 CIDR blocks.
"
regionsB*" "
services*" "d
	syncToken SPublication time of the IP ranges, in Unix epoch time format
(e.g., `1470267965`).
"
urlB" 2Å"
3getPartition#aws:index/getPartition:getPartition˚Use this data source to lookup information about the current AWS partition in
which the provider is working.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getPartition({});
const s3Policy = current.then(current => aws.iam.getPolicyDocument({
    statements: [{
        sid: "1",
        actions: ["s3:ListBucket"],
        resources: [`arn:${current.partition}:s3:::my-bucket`],
    }],
}));
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_partition()
s3_policy = aws.iam.get_policy_document(statements=[{
    "sid": "1",
    "actions": ["s3:ListBucket"],
    "resources": [f"arn:{current.partition}:s3:::my-bucket"],
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetPartition.Invoke();

    var s3Policy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Sid = "1",
                Actions = new[]
                {
                    "s3:ListBucket",
                },
                Resources = new[]
                {
                    $"arn:{current.Apply(getPartitionResult => getPartitionResult.Partition)}:s3:::my-bucket",
                },
            },
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetPartition(ctx, &aws.GetPartitionArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Sid: pulumi.StringRef("1"),
					Actions: []string{
						"s3:ListBucket",
					},
					Resources: []string{
						fmt.Sprintf("arn:%v:s3:::my-bucket", current.Partition),
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetPartitionArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getPartition();

        final var s3Policy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .sid("1")
                .actions("s3:ListBucket")
                .resources(String.format("arn:%s:s3:::my-bucket", current.applyValue(getPartitionResult -> getPartitionResult.partition())))
                .build())
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getPartition
      arguments: {}
  s3Policy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - sid: '1'
            actions:
              - s3:ListBucket
            resources:
              - arn:${current.partition}:s3:::my-bucket
```
<!--End PulumiCodeChooser -->
h
idB" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"å
	dnsSuffix" {Base DNS domain name for the current partition (e.g., `amazonaws.com` in AWS Commercial, `amazonaws.com.cn` in AWS China).
"f
id" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"m
	partition" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"|
reverseDnsPrefix" dPrefix of service names (e.g., `com.amazonaws` in AWS Commercial, `cn.com.amazonaws` in AWS China).
2œ
*	getRegionaws:index/getRegion:getRegionÊ`aws.getRegion` provides details about a specific AWS region.

As well as validating a given region name this resource can be used to
discover the name of the region configured within the provider. The latter
can be useful in a child module which is inheriting an AWS provider
configuration from its parent module.

## Example Usage

The following example shows how the resource might be used to obtain
the name of the AWS region configured on the provider.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegion({});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_region()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegion.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegion();

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
```
<!--End PulumiCodeChooser -->
8
endpointB" &EC2 endpoint of the region to select.


idB" 1
nameB" #Full name of the region to select.
"R
description" ?Region's description in this format: "Location (Region name)".
"6
endpoint" &EC2 endpoint for the selected region.
"
id" ")
name" Name of the selected region.
2Û4
-
getRegionsaws:index/getRegions:getRegionsﬂ/Provides information about AWS Regions. Can be used to filter regions i.e., by Opt-In status or only regions enabled for current account. To get details like endpoint and description of each region the data source can be combined with the `aws.getRegion` data source.

## Example Usage

Enabled AWS Regions:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegions({});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_regions()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegions.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegions(ctx, &aws.GetRegionsArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegions();

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegions
      arguments: {}
```
<!--End PulumiCodeChooser -->

All the regions regardless of the availability

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegions({
    allRegions: true,
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_regions(all_regions=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegions.Invoke(new()
    {
        AllRegions = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegions(ctx, &aws.GetRegionsArgs{
			AllRegions: pulumi.BoolRef(true),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegions(GetRegionsArgs.builder()
            .allRegions(true)
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegions
      arguments:
        allRegions: true
```
<!--End PulumiCodeChooser -->

To see regions that are filtered by `"not-opted-in"`, the `all_regions` argument needs to be set to `true` or no results will be returned.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegions({
    allRegions: true,
    filters: [{
        name: "opt-in-status",
        values: ["not-opted-in"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_regions(all_regions=True,
    filters=[{
        "name": "opt-in-status",
        "values": ["not-opted-in"],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegions.Invoke(new()
    {
        AllRegions = true,
        Filters = new[]
        {
            new Aws.Inputs.GetRegionsFilterInputArgs
            {
                Name = "opt-in-status",
                Values = new[]
                {
                    "not-opted-in",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetRegions(ctx, &aws.GetRegionsArgs{
			AllRegions: pulumi.BoolRef(true),
			Filters: []aws.GetRegionsFilter{
				{
					Name: "opt-in-status",
					Values: []string{
						"not-opted-in",
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegions(GetRegionsArgs.builder()
            .allRegions(true)
            .filters(GetRegionsFilterArgs.builder()
                .name("opt-in-status")
                .values("not-opted-in")
                .build())
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegions
      arguments:
        allRegions: true
        filters:
          - name: opt-in-status
            values:
              - not-opted-in
```
<!--End PulumiCodeChooser -->
Z

allRegionsB
 FIf true the source will query all regions regardless of availability.
é
filtersGBE*C:A
?getRegionsFilter+aws:index/getRegionsFilter:getRegionsFilter:Configuration block(s) to use as filters. Detailed below.
h
idB" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"

allRegionsB
 "R
filtersGBE*C:A
?getRegionsFilter+aws:index/getRegionsFilter:getRegionsFilter"f
id" \Identifier of the current partition (e.g., `aws` in AWS Commercial, `aws-cn` in AWS China).
"9
names*" *Names of regions that meets the criteria.
2Ω9
-
getServiceaws:index/getService:getServiceÃ0Use this data source to compose and decompose AWS service DNS names.

## Example Usage

### Get Service DNS Name

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getRegion({});
const test = current.then(current => aws.getService({
    region: current.name,
    serviceId: "ec2",
}));
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_region()
test = aws.get_service(region=current.name,
    service_id="ec2")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetRegion.Invoke();

    var test = Aws.GetService.Invoke(new()
    {
        Region = current.Apply(getRegionResult => getRegionResult.Name),
        ServiceId = "ec2",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := aws.GetRegion(ctx, &aws.GetRegionArgs{}, nil)
		if err != nil {
			return err
		}
		_, err = aws.GetService(ctx, &aws.GetServiceArgs{
			Region:    pulumi.StringRef(current.Name),
			ServiceId: pulumi.StringRef("ec2"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetRegionArgs;
import com.pulumi.aws.inputs.GetServiceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getRegion();

        final var test = AwsFunctions.getService(GetServiceArgs.builder()
            .region(current.applyValue(getRegionResult -> getRegionResult.name()))
            .serviceId("ec2")
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getRegion
      arguments: {}
  test:
    fn::invoke:
      function: aws:getService
      arguments:
        region: ${current.name}
        serviceId: ec2
```
<!--End PulumiCodeChooser -->

### Use Service Reverse DNS Name to Get Components

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const s3 = aws.getService({
    reverseDnsName: "cn.com.amazonaws.cn-north-1.s3",
});
```
```python
import pulumi
import pulumi_aws as aws

s3 = aws.get_service(reverse_dns_name="cn.com.amazonaws.cn-north-1.s3")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var s3 = Aws.GetService.Invoke(new()
    {
        ReverseDnsName = "cn.com.amazonaws.cn-north-1.s3",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetService(ctx, &aws.GetServiceArgs{
			ReverseDnsName: pulumi.StringRef("cn.com.amazonaws.cn-north-1.s3"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetServiceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var s3 = AwsFunctions.getService(GetServiceArgs.builder()
            .reverseDnsName("cn.com.amazonaws.cn-north-1.s3")
            .build());

    }
}
```
```yaml
variables:
  s3:
    fn::invoke:
      function: aws:getService
      arguments:
        reverseDnsName: cn.com.amazonaws.cn-north-1.s3
```
<!--End PulumiCodeChooser -->

### Determine Regional Support for a Service

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const s3 = aws.getService({
    reverseDnsName: "com.amazonaws.us-gov-west-1.waf",
});
```
```python
import pulumi
import pulumi_aws as aws

s3 = aws.get_service(reverse_dns_name="com.amazonaws.us-gov-west-1.waf")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var s3 = Aws.GetService.Invoke(new()
    {
        ReverseDnsName = "com.amazonaws.us-gov-west-1.waf",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetService(ctx, &aws.GetServiceArgs{
			ReverseDnsName: pulumi.StringRef("com.amazonaws.us-gov-west-1.waf"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetServiceArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var s3 = AwsFunctions.getService(GetServiceArgs.builder()
            .reverseDnsName("com.amazonaws.us-gov-west-1.waf")
            .build());

    }
}
```
```yaml
variables:
  s3:
    fn::invoke:
      function: aws:getService
      arguments:
        reverseDnsName: com.amazonaws.us-gov-west-1.waf
```
<!--End PulumiCodeChooser -->
ó
dnsNameB" ÖDNS name of the service (_e.g.,_ `rds.us-east-1.amazonaws.com`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.


idB" O
regionB" ?Region of the service (_e.g.,_ `us-west-2`, `ap-northeast-1`).
•
reverseDnsNameB" åReverse DNS name of the service (_e.g.,_ `com.amazonaws.us-west-2.s3`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required.
~
reverseDnsPrefixB" dPrefix of the service (_e.g.,_ `com.amazonaws` in AWS Commercial, `cn.com.amazonaws` in AWS China).
ù
	serviceIdB" âService endpoint ID (_e.g.,_ `s3`, `rds`, `ec2`). One of `dns_name`, `reverse_dns_name`, or `service_id` is required. A service's endpoint ID can be found in the [_AWS General Reference_](https://docs.aws.amazon.com/general/latest/gr/aws-service-information.html).
"
dnsName" "
id" "
	partition" "
region" "
reverseDnsName" "
reverseDnsPrefix" "
	serviceId" "Ü
	supported
 uWhether the service is supported in the region's partition. New services may not be listed immediately as supported.
2 
HgetServicePrincipal1aws:index/getServicePrincipal:getServicePrincipalâUse this data source to create a Service Principal Name for a service in a given region. Service Principal Names should always end in the standard global format: `{servicename}.amazonaws.com`. However, in some AWS partitions, AWS may expect a different format.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.getServicePrincipal({
    serviceName: "s3",
});
const test = aws.getServicePrincipal({
    serviceName: "s3",
    region: "us-iso-east-1",
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.get_service_principal(service_name="s3")
test = aws.get_service_principal(service_name="s3",
    region="us-iso-east-1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.GetServicePrincipal.Invoke(new()
    {
        ServiceName = "s3",
    });

    var test = Aws.GetServicePrincipal.Invoke(new()
    {
        ServiceName = "s3",
        Region = "us-iso-east-1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := aws.GetServicePrincipal(ctx, &aws.GetServicePrincipalArgs{
			ServiceName: "s3",
		}, nil)
		if err != nil {
			return err
		}
		_, err = aws.GetServicePrincipal(ctx, &aws.GetServicePrincipalArgs{
			ServiceName: "s3",
			Region:      pulumi.StringRef("us-iso-east-1"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetServicePrincipalArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var current = AwsFunctions.getServicePrincipal(GetServicePrincipalArgs.builder()
            .serviceName("s3")
            .build());

        final var test = AwsFunctions.getServicePrincipal(GetServicePrincipalArgs.builder()
            .serviceName("s3")
            .region("us-iso-east-1")
            .build());

    }
}
```
```yaml
variables:
  current:
    fn::invoke:
      function: aws:getServicePrincipal
      arguments:
        serviceName: s3
  test:
    fn::invoke:
      function: aws:getServicePrincipal
      arguments:
        serviceName: s3
        region: us-iso-east-1
```
<!--End PulumiCodeChooser -->
T
regionB" DRegion you'd like the SPN for. By default, uses the current region.
Z
serviceName" GName of the service you want to generate a Service Principal Name for.
" 
id" øIdentifier of the current Service Principal (compound of service, region and suffix). (e.g. `logs.us-east-1.amazonaws.com`in AWS Commercial, `logs.cn-north-1.amazonaws.com.cn` in AWS China).
"y
name" mService Principal Name (e.g., `logs.amazonaws.com` in AWS Commercial, `logs.amazonaws.com.cn` in AWS China).
"w
region" iRegion identifier of the generated SPN (e.g., `us-east-1` in AWS Commercial, `cn-north-1` in AWS China).
"
serviceName" "l
suffix" ^Suffix of the SPN (e.g., `amazonaws.com` in AWS Commercial, `amazonaws.com.cn` in AWS China).
2«
F
ssoadmingetApplication*aws:ssoadmin/getApplication:getApplication≠Data source for managing an AWS SSO Admin Application.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getApplication({
    applicationArn: "arn:aws:sso::123456789012:application/ssoins-1234/apl-5678",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_application(application_arn="arn:aws:sso::123456789012:application/ssoins-1234/apl-5678")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetApplication.Invoke(new()
    {
        ApplicationArn = "arn:aws:sso::123456789012:application/ssoins-1234/apl-5678",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssoadmin.LookupApplication(ctx, &ssoadmin.LookupApplicationArgs{
			ApplicationArn: "arn:aws:sso::123456789012:application/ssoins-1234/apl-5678",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.inputs.GetApplicationArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getApplication(GetApplicationArgs.builder()
            .applicationArn("arn:aws:sso::123456789012:application/ssoins-1234/apl-5678")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getApplication
      arguments:
        applicationArn: arn:aws:sso::123456789012:application/ssoins-1234/apl-5678
```
<!--End PulumiCodeChooser -->
.
applicationArn" ARN of the application.
ï
portalOptionsrBp*n:l
j
ssoadmingetApplicationPortalOptionBaws:ssoadmin/getApplicationPortalOption:getApplicationPortalOptionèOptions for the portal associated with an application. See the `aws.ssoadmin.Application` resource documentation. The attributes are the same.
"*
applicationAccount" AWS account ID.
"
applicationArn" "?
applicationProviderArn" !ARN of the application provider.
"3
description"  Description of the application.
""
id" ARN of the application.
"?
instanceArn" ,ARN of the instance of IAM Identity Center.
"%
name" Name of the application.
"ï
portalOptionsrBp*n:l
j
ssoadmingetApplicationPortalOptionBaws:ssoadmin/getApplicationPortalOption:getApplicationPortalOptionèOptions for the portal associated with an application. See the `aws.ssoadmin.Application` resource documentation. The attributes are the same.
")
status" Status of the application.
2ï
g
ssoadmingetApplicationAssignments@aws:ssoadmin/getApplicationAssignments:getApplicationAssignments¡Data source for managing AWS SSO Admin Application Assignments.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getApplicationAssignments({
    applicationArn: exampleAwsSsoadminApplication.applicationArn,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_application_assignments(application_arn=example_aws_ssoadmin_application["applicationArn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetApplicationAssignments.Invoke(new()
    {
        ApplicationArn = exampleAwsSsoadminApplication.ApplicationArn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssoadmin.GetApplicationAssignments(ctx, &ssoadmin.GetApplicationAssignmentsArgs{
			ApplicationArn: exampleAwsSsoadminApplication.ApplicationArn,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.inputs.GetApplicationAssignmentsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getApplicationAssignments(GetApplicationAssignmentsArgs.builder()
            .applicationArn(exampleAwsSsoadminApplication.applicationArn())
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getApplicationAssignments
      arguments:
        applicationArn: ${exampleAwsSsoadminApplication.applicationArn}
```
<!--End PulumiCodeChooser -->
.
applicationArn" ARN of the application.
º
applicationAssignments≤BØ*¨:©
¶
ssoadmin.getApplicationAssignmentsApplicationAssignmentjaws:ssoadmin/getApplicationAssignmentsApplicationAssignment:getApplicationAssignmentsApplicationAssignmentmList of principals assigned to the application. See the `application_assignments` attribute reference below.
".
applicationArn" ARN of the application.
"º
applicationAssignments≤BØ*¨:©
¶
ssoadmin.getApplicationAssignmentsApplicationAssignmentjaws:ssoadmin/getApplicationAssignmentsApplicationAssignment:getApplicationAssignmentsApplicationAssignmentmList of principals assigned to the application. See the `application_assignments` attribute reference below.
"
id" 2è
a
ssoadmingetApplicationProviders<aws:ssoadmin/getApplicationProviders:getApplicationProviders¡Data source for managing AWS SSO Admin Application Providers.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getApplicationProviders({});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_application_providers()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetApplicationProviders.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssoadmin.GetApplicationProviders(ctx, &ssoadmin.GetApplicationProvidersArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.inputs.GetApplicationProvidersArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getApplicationProviders();

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getApplicationProviders
      arguments: {}
```
<!--End PulumiCodeChooser -->
•
applicationProviders¶B£*†:ù
ö
ssoadmin*getApplicationProvidersApplicationProviderbaws:ssoadmin/getApplicationProvidersApplicationProvider:getApplicationProvidersApplicationProviderdA list of application providers available in the current region. See `application_providers` below.
"•
applicationProviders¶B£*†:ù
ö
ssoadmin*getApplicationProvidersApplicationProviderbaws:ssoadmin/getApplicationProvidersApplicationProvider:getApplicationProvidersApplicationProviderdA list of application providers available in the current region. See `application_providers` below.
"
id" AWS region.
2Å
@
ssoadmingetInstances&aws:ssoadmin/getInstances:getInstances≈Use this data source to get ARNs and Identity Store IDs of Single Sign-On (SSO) Instances.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
export const arn = example.then(example => example.arns?.[0]);
export const identityStoreId = example.then(example => example.identityStoreIds?.[0]);
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
pulumi.export("arn", example.arns[0])
pulumi.export("identityStoreId", example.identity_store_ids[0])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    return new Dictionary<string, object?>
    {
        ["arn"] = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        ["identityStoreId"] = example.Apply(getInstancesResult => getInstancesResult.IdentityStoreIds[0]),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		ctx.Export("arn", example.Arns[0])
		ctx.Export("identityStoreId", example.IdentityStoreIds[0])
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        ctx.export("arn", example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]));
        ctx.export("identityStoreId", example.applyValue(getInstancesResult -> getInstancesResult.identityStoreIds()[0]));
    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
outputs:
  arn: ${example.arns[0]}
  identityStoreId: ${example.identityStoreIds[0]}
```
<!--End PulumiCodeChooser -->
"H
arns*" :Set of Amazon Resource Names (ARNs) of the SSO Instances.
"E
id" ;The provider-assigned unique ID for this managed resource.
"d
identityStoreIds*" JSet of identifiers of the identity stores connected to the SSO Instances.
2‡
L
ssoadmingetPermissionSet.aws:ssoadmin/getPermissionSet:getPermissionSet◊Use this data source to get a Single Sign-On (SSO) Permission Set.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const exampleGetPermissionSet = example.then(example => aws.ssoadmin.getPermissionSet({
    instanceArn: example.arns?.[0],
    name: "Example",
}));
export const arn = exampleGetPermissionSet.then(exampleGetPermissionSet => exampleGetPermissionSet.arn);
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_get_permission_set = aws.ssoadmin.get_permission_set(instance_arn=example.arns[0],
    name="Example")
pulumi.export("arn", example_get_permission_set.arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var exampleGetPermissionSet = Aws.SsoAdmin.GetPermissionSet.Invoke(new()
    {
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
        Name = "Example",
    });

    return new Dictionary<string, object?>
    {
        ["arn"] = exampleGetPermissionSet.Apply(getPermissionSetResult => getPermissionSetResult.Arn),
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		exampleGetPermissionSet, err := ssoadmin.LookupPermissionSet(ctx, &ssoadmin.LookupPermissionSetArgs{
			InstanceArn: example.Arns[0],
			Name:        pulumi.StringRef("Example"),
		}, nil)
		if err != nil {
			return err
		}
		ctx.Export("arn", exampleGetPermissionSet.Arn)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.inputs.GetPermissionSetArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        final var exampleGetPermissionSet = SsoadminFunctions.getPermissionSet(GetPermissionSetArgs.builder()
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .name("Example")
            .build());

        ctx.export("arn", exampleGetPermissionSet.applyValue(getPermissionSetResult -> getPermissionSetResult.arn()));
    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
  exampleGetPermissionSet:
    fn::invoke:
      function: aws:ssoadmin:getPermissionSet
      arguments:
        instanceArn: ${example.arns[0]}
        name: Example
outputs:
  arn: ${exampleGetPermissionSet.arn}
```
<!--End PulumiCodeChooser -->
(
arnB" ARN of the permission set.
O
instanceArn" <ARN of the SSO Instance associated with the permission set.
.
nameB"  Name of the SSO Permission Set.
0
tagsB2"  Key-value map of resource tags.
"	
arn" "
createdDate" "6
description" #Description of the Permission Set.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
instanceArn" "

name" "~

relayState" lRelay state URL used to redirect users within the application during the federation authentication process.
"m
sessionDuration" VLength of time that the application user sessions are valid in the ISO-8601 standard.
".
tags2"  Key-value map of resource tags.
2´
O
ssoadmingetPermissionSets0aws:ssoadmin/getPermissionSets:getPermissionSets£Data source returning the ARN of all AWS SSO Admin Permission Sets.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getInstances({});
const exampleGetPermissionSets = example.then(example => aws.ssoadmin.getPermissionSets({
    instanceArn: example.arns?.[0],
}));
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_instances()
example_get_permission_sets = aws.ssoadmin.get_permission_sets(instance_arn=example.arns[0])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetInstances.Invoke();

    var exampleGetPermissionSets = Aws.SsoAdmin.GetPermissionSets.Invoke(new()
    {
        InstanceArn = example.Apply(getInstancesResult => getInstancesResult.Arns[0]),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := ssoadmin.GetInstances(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		_, err = ssoadmin.GetPermissionSets(ctx, &ssoadmin.GetPermissionSetsArgs{
			InstanceArn: example.Arns[0],
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.inputs.GetPermissionSetsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getInstances();

        final var exampleGetPermissionSets = SsoadminFunctions.getPermissionSets(GetPermissionSetsArgs.builder()
            .instanceArn(example.applyValue(getInstancesResult -> getInstancesResult.arns()[0]))
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getInstances
      arguments: {}
  exampleGetPermissionSets:
    fn::invoke:
      function: aws:ssoadmin:getPermissionSets
      arguments:
        instanceArn: ${example.arns[0]}
```
<!--End PulumiCodeChooser -->
O
instanceArn" <ARN of the SSO Instance associated with the permission set.
"D
arns*" 6Set of string contain the ARN of all Permission Sets.
"
id" "
instanceArn" 2˝
Ç
ssoadmin"getPrincipalApplicationAssignmentsRaws:ssoadmin/getPrincipalApplicationAssignments:getPrincipalApplicationAssignmentsΩData source for viewing AWS SSO Admin Principal Application Assignments.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.ssoadmin.getPrincipalApplicationAssignments({
    instanceArn: test.arns[0],
    principalId: testAwsIdentitystoreUser.userId,
    principalType: "USER",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.ssoadmin.get_principal_application_assignments(instance_arn=test["arns"],
    principal_id=test_aws_identitystore_user["userId"],
    principal_type="USER")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.SsoAdmin.GetPrincipalApplicationAssignments.Invoke(new()
    {
        InstanceArn = test.Arns[0],
        PrincipalId = testAwsIdentitystoreUser.UserId,
        PrincipalType = "USER",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ssoadmin"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := ssoadmin.GetPrincipalApplicationAssignments(ctx, &ssoadmin.GetPrincipalApplicationAssignmentsArgs{
			InstanceArn:   test.Arns[0],
			PrincipalId:   testAwsIdentitystoreUser.UserId,
			PrincipalType: "USER",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.ssoadmin.SsoadminFunctions;
import com.pulumi.aws.ssoadmin.inputs.GetPrincipalApplicationAssignmentsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SsoadminFunctions.getPrincipalApplicationAssignments(GetPrincipalApplicationAssignmentsArgs.builder()
            .instanceArn(test.arns()[0])
            .principalId(testAwsIdentitystoreUser.userId())
            .principalType("USER")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:ssoadmin:getPrincipalApplicationAssignments
      arguments:
        instanceArn: ${test.arns[0]}
        principalId: ${testAwsIdentitystoreUser.userId}
        principalType: USER
```
<!--End PulumiCodeChooser -->
◊
applicationAssignmentsÕB *«:ƒ
¡
ssoadmin7getPrincipalApplicationAssignmentsApplicationAssignment|aws:ssoadmin/getPrincipalApplicationAssignmentsApplicationAssignment:getPrincipalApplicationAssignmentsApplicationAssignmentmList of principals assigned to the application. See the `application_assignments` attribute reference below.
?
instanceArn" ,ARN of the instance of IAM Identity Center.
`
principalId" MAn identifier for an object in IAM Identity Center, such as a user or group.
o
principalType" ZEntity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
"◊
applicationAssignmentsÕB *«:ƒ
¡
ssoadmin7getPrincipalApplicationAssignmentsApplicationAssignment|aws:ssoadmin/getPrincipalApplicationAssignmentsApplicationAssignment:getPrincipalApplicationAssignmentsApplicationAssignmentmList of principals assigned to the application. See the `application_assignments` attribute reference below.
"
id" "
instanceArn" "`
principalId" MAn identifier for an object in IAM Identity Center, such as a user or group.
"o
principalType" ZEntity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
2∑
L
storagegatewaygetLocalDisk,aws:storagegateway/getLocalDisk:getLocalDiskΩRetrieve information about a Storage Gateway local disk. The disk identifier is useful for adding the disk as a cache or upload buffer to a gateway.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.storagegateway.getLocalDisk({
    diskPath: testAwsVolumeAttachment.deviceName,
    gatewayArn: testAwsStoragegatewayGateway.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.storagegateway.get_local_disk(disk_path=test_aws_volume_attachment["deviceName"],
    gateway_arn=test_aws_storagegateway_gateway["arn"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.StorageGateway.GetLocalDisk.Invoke(new()
    {
        DiskPath = testAwsVolumeAttachment.DeviceName,
        GatewayArn = testAwsStoragegatewayGateway.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/storagegateway"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := storagegateway.GetLocalDisk(ctx, &storagegateway.GetLocalDiskArgs{
			DiskPath:   pulumi.StringRef(testAwsVolumeAttachment.DeviceName),
			GatewayArn: testAwsStoragegatewayGateway.Arn,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.storagegateway.StoragegatewayFunctions;
import com.pulumi.aws.storagegateway.inputs.GetLocalDiskArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = StoragegatewayFunctions.getLocalDisk(GetLocalDiskArgs.builder()
            .diskPath(testAwsVolumeAttachment.deviceName())
            .gatewayArn(testAwsStoragegatewayGateway.arn())
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:storagegateway:getLocalDisk
      arguments:
        diskPath: ${testAwsVolumeAttachment.deviceName}
        gatewayArn: ${testAwsStoragegatewayGateway.arn}
```
<!--End PulumiCodeChooser -->
V
diskNodeB" DDevice node of the local disk to retrieve. For example, `/dev/sdb`.
i
diskPathB" WDevice path of the local disk to retrieve. For example, `/dev/xvdb` or `/dev/nvme1n1`.
&

gatewayArn" ARN of the gateway.
"C
diskId" 5Disk identifierE.g., `pci-0000:03:00.0-scsi-0:0:0:0`
"
diskNode" "
diskPath" "

gatewayArn" "E
id" ;The provider-assigned unique ID for this managed resource.
2ò*
S

syntheticsgetRuntimeVersion2aws:synthetics/getRuntimeVersion:getRuntimeVersionä!Data source for managing an AWS CloudWatch Synthetics Runtime Version.

## Example Usage

### Latest Runtime Version

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.synthetics.getRuntimeVersion({
    prefix: "syn-nodejs-puppeteer",
    latest: true,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.synthetics.get_runtime_version(prefix="syn-nodejs-puppeteer",
    latest=True)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Synthetics.GetRuntimeVersion.Invoke(new()
    {
        Prefix = "syn-nodejs-puppeteer",
        Latest = true,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/synthetics"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := synthetics.GetRuntimeVersion(ctx, &synthetics.GetRuntimeVersionArgs{
			Prefix: "syn-nodejs-puppeteer",
			Latest: pulumi.BoolRef(true),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.synthetics.SyntheticsFunctions;
import com.pulumi.aws.synthetics.inputs.GetRuntimeVersionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SyntheticsFunctions.getRuntimeVersion(GetRuntimeVersionArgs.builder()
            .prefix("syn-nodejs-puppeteer")
            .latest(true)
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:synthetics:getRuntimeVersion
      arguments:
        prefix: syn-nodejs-puppeteer
        latest: true
```
<!--End PulumiCodeChooser -->

### Specific Runtime Version

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.synthetics.getRuntimeVersion({
    prefix: "syn-nodejs-puppeteer",
    version: "9.0",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.synthetics.get_runtime_version(prefix="syn-nodejs-puppeteer",
    version="9.0")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Synthetics.GetRuntimeVersion.Invoke(new()
    {
        Prefix = "syn-nodejs-puppeteer",
        Version = "9.0",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/synthetics"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := synthetics.GetRuntimeVersion(ctx, &synthetics.GetRuntimeVersionArgs{
			Prefix:  "syn-nodejs-puppeteer",
			Version: pulumi.StringRef("9.0"),
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.synthetics.SyntheticsFunctions;
import com.pulumi.aws.synthetics.inputs.GetRuntimeVersionArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SyntheticsFunctions.getRuntimeVersion(GetRuntimeVersionArgs.builder()
            .prefix("syn-nodejs-puppeteer")
            .version("9.0")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:synthetics:getRuntimeVersion
      arguments:
        prefix: syn-nodejs-puppeteer
        version: '9.0'
```
<!--End PulumiCodeChooser -->
}
latestB
 mWhether the latest version of the runtime should be fetched. Conflicts with `version`. Valid values: `true`.

prefix" qName prefix of the runtime version (for example, `syn-nodejs-puppeteer`).

The following arguments are optional:
e
versionB" TVersion of the runtime to be fetched (for example, `9.0`). Conflicts with `latest`.
"P
deprecationDate" 9Date of deprecation if the runtme version is deprecated.
"J
description" 7Description of the runtime version, created by Amazon.
"Ÿ
id" ŒName of the runtime version. For a list of valid runtime versions, see [Canary Runtime Versions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html).
"
latestB
 "
prefix" "?
releaseDate" ,Date that the runtime version was released.
"
versionB" "‚
versionName" ŒName of the runtime version. For a list of valid runtime versions, see [Canary Runtime Versions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html).
2∂
V

syntheticsgetRuntimeVersions4aws:synthetics/getRuntimeVersions:getRuntimeVersionsπData source for managing an AWS CloudWatch Synthetics Runtime Versions.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.synthetics.getRuntimeVersions({});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.synthetics.get_runtime_versions()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Synthetics.GetRuntimeVersions.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/synthetics"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := synthetics.GetRuntimeVersions(ctx, &synthetics.GetRuntimeVersionsArgs{}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.synthetics.SyntheticsFunctions;
import com.pulumi.aws.synthetics.inputs.GetRuntimeVersionsArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = SyntheticsFunctions.getRuntimeVersions();

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:synthetics:getRuntimeVersions
      arguments: {}
```
<!--End PulumiCodeChooser -->
Ë
runtimeVersionsåBâ*Ü:É
Ä

synthetics getRuntimeVersionsRuntimeVersionPaws:synthetics/getRuntimeVersionsRuntimeVersion:getRuntimeVersionsRuntimeVersionFList of runtime versions. See `runtime_versions` attribute reference.
"J
id" @Name of the AWS region from which runtime versions are fetched.
"Ë
runtimeVersionsåBâ*Ü:É
Ä

synthetics getRuntimeVersionsRuntimeVersionPaws:synthetics/getRuntimeVersionsRuntimeVersion:getRuntimeVersionsRuntimeVersionFList of runtime versions. See `runtime_versions` attribute reference.
2ë
K
timestreamwritegetDatabase+aws:timestreamwrite/getDatabase:getDatabaseõData source for managing an AWS Timestream Write Database.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.timestreamwrite.getDatabase({
    name: "database-example",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.timestreamwrite.get_database(name="database-example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.TimestreamWrite.GetDatabase.Invoke(new()
    {
        Name = "database-example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/timestreamwrite"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := timestreamwrite.LookupDatabase(ctx, &timestreamwrite.LookupDatabaseArgs{
			Name: "database-example",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.timestreamwrite.TimestreamwriteFunctions;
import com.pulumi.aws.timestreamwrite.inputs.GetDatabaseArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = TimestreamwriteFunctions.getDatabase(GetDatabaseArgs.builder()
            .name("database-example")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:timestreamwrite:getDatabase
      arguments:
        name: database-example
```
<!--End PulumiCodeChooser -->


name" ";
arn" 0The ARN that uniquely identifies this database.
".
createdTime" Creation time of database.
"E
id" ;The provider-assigned unique ID for this managed resource.
"X
kmsKeyId" HThe ARN of the KMS key used to encrypt the data stored in the database.
"7
lastUpdatedTime"  Last time database was updated.
"

name" "E

tableCount 3Total number of tables in the Timestream database.
2Ï
B
timestreamwritegetTable%aws:timestreamwrite/getTable:getTableòData source for managing an AWS Timestream Write Table.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.timestreamwrite.getTable({
    databaseName: testAwsTimestreamwriteDatabase.databaseName,
    name: testAwsTimestreamwriteTable.tableName,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.timestreamwrite.get_table(database_name=test_aws_timestreamwrite_database["databaseName"],
    name=test_aws_timestreamwrite_table["tableName"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.TimestreamWrite.GetTable.Invoke(new()
    {
        DatabaseName = testAwsTimestreamwriteDatabase.DatabaseName,
        Name = testAwsTimestreamwriteTable.TableName,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/timestreamwrite"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := timestreamwrite.LookupTable(ctx, &timestreamwrite.LookupTableArgs{
			DatabaseName: testAwsTimestreamwriteDatabase.DatabaseName,
			Name:         testAwsTimestreamwriteTable.TableName,
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.timestreamwrite.TimestreamwriteFunctions;
import com.pulumi.aws.timestreamwrite.inputs.GetTableArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var test = TimestreamwriteFunctions.getTable(GetTableArgs.builder()
            .databaseName(testAwsTimestreamwriteDatabase.databaseName())
            .name(testAwsTimestreamwriteTable.tableName())
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:timestreamwrite:getTable
      arguments:
        databaseName: ${testAwsTimestreamwriteDatabase.databaseName}
        name: ${testAwsTimestreamwriteTable.tableName}
```
<!--End PulumiCodeChooser -->
5
databaseName" !Name of the Timestream database.
*
name" Name of the Timestream table.
"3
arn" (ARN that uniquely identifies the table.
"1
creationTime" Time that table was created.
"&
databaseName" Name of database.
"E
id" ;The provider-assigned unique ID for this managed resource.
"4
lastUpdatedTime" Last time table was updated.
"â
magneticStoreWritePropertiesô*ñ:ì
ê
timestreamwrite"getTableMagneticStoreWritePropertyYaws:timestreamwrite/getTableMagneticStoreWriteProperty:getTableMagneticStoreWritePropertyMObject containing the following attributes to desribe magnetic store writes.
"
name" Name of the table.
"Ü
retentionProperties{*y:w
u
timestreamwritegetTableRetentionPropertyGaws:timestreamwrite/getTableRetentionProperty:getTableRetentionPropertyrObject containing the following attributes to describe the retention duration for the memory and magnetic stores.
"∑
schemasZ*X:V
T
timestreamwritegetTableSchema1aws:timestreamwrite/getTableSchema:getTableSchemaPObject containing the following attributes to describe the schema of the table.
"+
tableStatus" Current state of table.
2Ç
@
transfergetConnector&aws:transfer/getConnector:getConnector;Data source for managing an AWS Transfer Family Connector.
*
id"  Unique identifier for connector
"F

accessRole" 4ARN of the AWS Identity and Access Management role.
"!
arn" ARN of the Connector.
"’

as2Configsa*_:]
[
transfergetConnectorAs2Config8aws:transfer/getConnectorAs2Config:getConnectorAs2ConfigdStructure containing the parameters for an AS2 connector object. Contains the following attributes:
"
id" "s
loggingRole" `ARN of the IAM role that allows a connector to turn on CLoudwatch logging for Amazon S3 events.
"3
securityPolicyName" Name of security policy.
"F
serviceManagedEgressIpAddresses*" List of egress Ip addresses.
"°
sftpConfigsd*b:`
^
transfergetConnectorSftpConfig:aws:transfer/getConnectorSftpConfig:getConnectorSftpConfig,Object containing the following attributes:
":
tags2" ,Object containing the following attributes:
"6
url" +URL of the partner's AS2 or SFTP endpoint.
2Ô
7
transfer	getServer aws:transfer/getServer:getServer»Use this data source to get the ARN of an AWS Transfer Server for use in other
resources.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.transfer.getServer({
    serverId: "s-1234567",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.transfer.get_server(server_id="s-1234567")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Transfer.GetServer.Invoke(new()
    {
        ServerId = "s-1234567",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/transfer"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := transfer.LookupServer(ctx, &transfer.LookupServerArgs{
			ServerId: "s-1234567",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.transfer.TransferFunctions;
import com.pulumi.aws.transfer.inputs.GetServerArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = TransferFunctions.getServer(GetServerArgs.builder()
            .serverId("s-1234567")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:transfer:getServer
      arguments:
        serverId: s-1234567
```
<!--End PulumiCodeChooser -->
'
serverId" ID for an SFTP server.
6
tagsB2" &Map of tags assigned to the resource.
"#
arn" ARN of Transfer Server.
"+
certificate" ARN of any certificate.
"P
domain" BThe domain of the storage system that is used for file transfers.
"k
endpoint" [Endpoint of the Transfer Server (e.g., `s-12345678.server.transfer.REGION.amazonaws.com`).
"F
endpointType" 2Type of endpoint that the server is connected to.
"E
id" ;The provider-assigned unique ID for this managed resource.
"Î
identityProviderType" ŒThe mode of authentication enabled for this service. The default value is `SERVICE_MANAGED`, which allows you to store and access SFTP user credentials within the service. `API_GATEWAY` indicates that user authentication requires a call to an API Gateway endpoint URL provided by you to integrate an identity provider of your choice.
"É
invocationRole" mARN of the IAM role used to authenticate the user account with an `identity_provider_type` of `API_GATEWAY`.
"©
loggingRole" ïARN of an IAM role that allows the service to write your SFTP users‚Äô activity to your Amazon CloudWatch logs for monitoring and auditing purposes.
"å
	protocols*" yFile transfer protocol or protocols over which your file transfer protocol client can connect to your server's endpoint.
"Z
securityPolicyName" @The name of the security policy that is attached to the server.
"
serverId" "ü
structuredLogDestinations*" |A set of ARNs of destinations that will receive structured logs from the transfer server such as CloudWatch Log Group ARNs.
"4
tags2" &Map of tags assigned to the resource.
"u
url" jURL of the service endpoint used to authenticate users with an `identity_provider_type` of `API_GATEWAY`.
2Á
\
verifiedpermissionsgetPolicyStore5aws:verifiedpermissions/getPolicyStore:getPolicyStore∏Data source for managing an AWS Verified Permissions Policy Store.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.verifiedpermissions.getPolicyStore({
    id: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.verifiedpermissions.get_policy_store(id="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.VerifiedPermissions.GetPolicyStore.Invoke(new()
    {
        Id = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/verifiedpermissions"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := verifiedpermissions.LookupPolicyStore(ctx, &verifiedpermissions.LookupPolicyStoreArgs{
			Id: "example",
		}, nil)
		if err != nil {
			return err
		}
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.verifiedpermissions.VerifiedpermissionsFunctions;
import com.pulumi.aws.verifiedpermissions.inputs.GetPolicyStoreArgs;
import java.util.List;
import java.util.ArrayList;
import java.util.Map;
import java.io.File;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        Pulumi.run(App::stack);
    }

    public static void stack(Context ctx) {
        final var example = VerifiedpermissionsFunctions.getPolicyStore(GetPolicyStoreArgs.builder()
            .id("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:verifiedpermissions:getPolicyStore
      arguments:
        id: example
```
<!--End PulumiCodeChooser -->
&
id" The ID of the Policy Store.
"(
arn" The ARN of the Policy Store.
":
createdDate" 'The date the Policy Store was created.
"
description" "
id" "C
lastUpdatedDate" ,The date the Policy Store was last updated.
"€
validationSettingsò*ï:í
è
verifiedpermissionsgetPolicyStoreValidationSettingWaws:verifiedpermissions/getPolicyStoreValidationSetting:getPolicyStoreValidationSetting*Validation settings for the policy store.
:ª
ZgetAvailabilityZoneFilter=aws:index/getAvailabilityZoneFilter:getAvailabilityZoneFilter‹
Ÿ“
name" ≈Name of the filter field. Valid values can be found in the [EC2 DescribeAvailabilityZones API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeAvailabilityZones.html).
Å
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:æ
]getAvailabilityZonesFilter?aws:index/getAvailabilityZonesFilter:getAvailabilityZonesFilter‹
Ÿ“
name" ≈Name of the filter field. Valid values can be found in the [EC2 DescribeAvailabilityZones API Reference](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DescribeAvailabilityZones.html).
Å
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:Ω
?getRegionsFilter+aws:index/getRegionsFilter:getRegionsFilter˘
ˆp
name" dName of the filter field. Valid values can be found in the [describe-regions AWS CLI Reference][1].
Å
values*" qSet of values that are accepted for the given filter field. Results will be selected if any given value matches.
:·
d
ssoadminApplicationPortalOptions>aws:ssoadmin/ApplicationPortalOptions:ApplicationPortalOptions¯
ıÏ
signInOptionsîBë:é
ã
ssoadmin%ApplicationPortalOptionsSignInOptionsXaws:ssoadmin/ApplicationPortalOptionsSignInOptions:ApplicationPortalOptionsSignInOptionsDSign-in options for the access portal. See `sign_in_options` below.
É

visibilityB" oIndicates whether this application is visible in the access portal. Valid values are `ENABLED` and `DISABLED`.
:ê
ã
ssoadmin%ApplicationPortalOptionsSignInOptionsXaws:ssoadmin/ApplicationPortalOptionsSignInOptions:ApplicationPortalOptionsSignInOptionsˇ
¸U
applicationUrlB" =URL that accepts authentication requests for an application.
¢
origin" ìDetermines how IAM Identity Center navigates the user to the target application.
Valid values are `APPLICATION` and `IDENTITY_CENTER`.
If `APPLICATION` is set, IAM Identity Center redirects the customer to the configured `application_url`.
If `IDENTITY_CENTER` is set, IAM Identity Center uses SAML identity-provider initiated authentication to sign the customer directly into a SAML-based application.
:à
‘
ssoadmin=CustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceàaws:ssoadmin/CustomerManagedPolicyAttachmentCustomerManagedPolicyReference:CustomerManagedPolicyAttachmentCustomerManagedPolicyReferenceÆ
´D
name" 8Name of the customer managed IAM Policy to be attached.
‚
pathB" ”The path to the IAM policy to be attached. The default is `/`. See [IAM Identifiers](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-friendly-names) for more information.
:É
î
ssoadmin(InstanceAccessControlAttributesAttribute^aws:ssoadmin/InstanceAccessControlAttributesAttribute:InstanceAccessControlAttributesAttributeÈ
Êø
key" ≥The name of the attribute associated with your identities in your identity source. This is used to map a specified attribute in your identity source with an attribute in AWS SSO.
°
values¨*©:¶
£
ssoadmin-InstanceAccessControlAttributesAttributeValuehaws:ssoadmin/InstanceAccessControlAttributesAttributeValue:InstanceAccessControlAttributesAttributeValuehThe value used for mapping a specified attribute to an identity source. See AccessControlAttributeValue
:á
£
ssoadmin-InstanceAccessControlAttributesAttributeValuehaws:ssoadmin/InstanceAccessControlAttributesAttributeValue:InstanceAccessControlAttributesAttributeValue_
][
sources*" JThe identity source to use when mapping a specified attribute to AWS SSO.
:ç
¨
ssoadmin0PermissionsBoundaryAttachmentPermissionsBoundarynaws:ssoadmin/PermissionsBoundaryAttachmentPermissionsBoundary:PermissionsBoundaryAttachmentPermissionsBoundary€
ÿ˙
customerManagedPolicyReferenceêBç:ä
á
ssoadminNPermissionsBoundaryAttachmentPermissionsBoundaryCustomerManagedPolicyReference™aws:ssoadmin/PermissionsBoundaryAttachmentPermissionsBoundaryCustomerManagedPolicyReference:PermissionsBoundaryAttachmentPermissionsBoundaryCustomerManagedPolicyReferenceESpecifies the name and path of a customer managed policy. See below.
Y
managedPolicyArnB" ?AWS-managed IAM policy ARN to use as the permissions boundary.
:ª
á
ssoadminNPermissionsBoundaryAttachmentPermissionsBoundaryCustomerManagedPolicyReference™aws:ssoadmin/PermissionsBoundaryAttachmentPermissionsBoundaryCustomerManagedPolicyReference:PermissionsBoundaryAttachmentPermissionsBoundaryCustomerManagedPolicyReferenceÆ
´D
name" 8Name of the customer managed IAM Policy to be attached.
‚
pathB" ”The path to the IAM policy to be attached. The default is `/`. See [IAM Identifiers](https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-friendly-names) for more information.
:Ì
Ø
ssoadmin1TrustedTokenIssuerTrustedTokenIssuerConfigurationpaws:ssoadmin/TrustedTokenIssuerTrustedTokenIssuerConfiguration:TrustedTokenIssuerTrustedTokenIssuerConfiguration∏
µ≤
oidcJwtConfigurationıBÚ:Ô
Ï
ssoadminETrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfigurationòaws:ssoadmin/TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfiguration:TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfiguration°A block that describes the settings for a trusted token issuer that works with OpenID Connect (OIDC) by using JSON Web Tokens (JWT). See Documented below below.
:°
Ï
ssoadminETrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfigurationòaws:ssoadmin/TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfiguration:TrustedTokenIssuerTrustedTokenIssuerConfigurationOidcJwtConfigurationØ
¨o
claimAttributePath" USpecifies the path of the source attribute in the JWT from the trusted token issuer.
∑
identityStoreAttributePath" îSpecifies path of the destination attribute in a JWT from IAM Identity Center. The attribute mapped by this JMESPath expression is compared against the attribute mapped by `claim_attribute_path` when a trusted token issuer token is exchanged for an IAM Identity Center token.
“
	issuerUrl" ¿Specifies the URL that IAM Identity Center uses for OpenID Discovery. OpenID Discovery is used to obtain the information required to verify the tokens that the trusted token issuer generates.
©
jwksRetrievalOption" çThe method that the trusted token issuer can use to retrieve the JSON Web Key Set used to verify a JWT. Valid values are `OPEN_ID_DISCOVERY`
:≤
¶
ssoadmin.getApplicationAssignmentsApplicationAssignmentjaws:ssoadmin/getApplicationAssignmentsApplicationAssignment:getApplicationAssignmentsApplicationAssignmentÜ
É.
applicationArn" ARN of the application.
`
principalId" MAn identifier for an object in IAM Identity Center, such as a user or group.
o
principalType" ZEntity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
:≥
j
ssoadmingetApplicationPortalOptionBaws:ssoadmin/getApplicationPortalOption:getApplicationPortalOptionƒ
¡¨
signInOptionsöBó*î:ë
é
ssoadmin&getApplicationPortalOptionSignInOptionZaws:ssoadmin/getApplicationPortalOptionSignInOption:getApplicationPortalOptionSignInOption

visibility" :π
é
ssoadmin&getApplicationPortalOptionSignInOptionZaws:ssoadmin/getApplicationPortalOptionSignInOption:getApplicationPortalOptionSignInOption&
$
applicationUrl" 
origin" :¿
ö
ssoadmin*getApplicationProvidersApplicationProviderbaws:ssoadmin/getApplicationProvidersApplicationProvider:getApplicationProvidersApplicationProvider†
ù?
applicationProviderArn" !ARN of the application provider.
‘
displayDatas«Bƒ*¡:æ
ª
ssoadmin5getApplicationProvidersApplicationProviderDisplayDataxaws:ssoadmin/getApplicationProvidersApplicationProviderDisplayData:getApplicationProvidersApplicationProviderDisplayDatazAn object describing how IAM Identity Center represents the application provider in the portal. See `display_data` below.
Ç
federationProtocol" hProtocol that the application provider uses to perform federation. Valid values are `SAML` and `OAUTH`.
:è
ª
ssoadmin5getApplicationProvidersApplicationProviderDisplayDataxaws:ssoadmin/getApplicationProvidersApplicationProviderDisplayData:getApplicationProvidersApplicationProviderDisplayDataŒ
À<
description" )Description of the application provider.
5
displayName" "Name of the application provider.
T
iconUrl" EURL that points to an icon that represents the application provider.
:Õ
¡
ssoadmin7getPrincipalApplicationAssignmentsApplicationAssignment|aws:ssoadmin/getPrincipalApplicationAssignmentsApplicationAssignment:getPrincipalApplicationAssignmentsApplicationAssignmentÜ
É.
applicationArn" ARN of the application.
`
principalId" MAn identifier for an object in IAM Identity Center, such as a user or group.
o
principalType" ZEntity type for which the assignment will be created. Valid values are `USER` or `GROUP`.
:ë
î
storagegateway$FileSystemAssociationCacheAttributes\aws:storagegateway/FileSystemAssociationCacheAttributes:FileSystemAssociationCacheAttributes˜
ÙÒ
cacheStaleTimeoutInSecondsB ÃRefreshes a file share's cache by using Time To Live (TTL).
TTL is the length of time since the last refresh after which access to the directory would cause the file gateway
to first refresh that directory's contents from the Amazon S3 bucket. Valid Values: `0` or `300` to `2592000` seconds (5 minutes to 30 days). Defaults to `0`
:·
Ç
storagegatewayGatewayGatewayNetworkInterfacePaws:storagegateway/GatewayGatewayNetworkInterface:GatewayGatewayNetworkInterfaceZ
XV
ipv4AddressB" AThe Internet Protocol version 4 (IPv4) address of the interface.
:à
y
storagegatewayGatewayMaintenanceStartTimeJaws:storagegateway/GatewayMaintenanceStartTime:GatewayMaintenanceStartTimeä
á‹

dayOfMonthB" «The day of the month component of the maintenance start time represented as an ordinal number from 1 to 28, where 1 represents the first day of the month and 28 represents the last day of the month.
≠
	dayOfWeekB" ôThe day of the week component of the maintenance start time week represented as an ordinal number from 0 to 6, where 0 represents Sunday and 6 Saturday.
≥
	hourOfDay °The hour component of the maintenance start time represented as _hh_, where _hh_ is the hour (00 to 23). The hour of the day is in the time zone of the gateway.
ø
minuteOfHourB ®The minute component of the maintenance start time represented as _mm_, where _mm_ is the minute (00 to 59). The minute of the hour is in the time zone of the gateway.
:∏
ã
storagegateway!GatewaySmbActiveDirectorySettingsVaws:storagegateway/GatewaySmbActiveDirectorySettings:GatewaySmbActiveDirectorySettingsß
§
activeDirectoryStatusB" ›
domainControllersB*" øList of IPv4 addresses, NetBIOS names, or host names of your domain server.
If you need to specify the port number include it after the colon (‚Äú:‚Äù). For example, `mydc.mydomain.com:389`.
L

domainName" :The name of the domain that you want the gateway to join.
Î
organizationalUnitB" ŒThe organizational unit (OU) is a container in an Active Directory that can hold users, groups,
computers, and other OUs and this parameter specifies the OU that the gateway will join within the AD domain.
o
password" _The password of the user who has permission to add the gateway to the Active Directory domain.
á
timeoutInSecondsB mSpecifies the time in seconds, in which the JoinDomain operation must complete. The default is `20` seconds.
l
username" \The user name of user who has permission to add the gateway to the Active Directory domain.
:€
y
storagegatewayNfsFileShareCacheAttributesJaws:storagegateway/NfsFileShareCacheAttributes:NfsFileShareCacheAttributes›
⁄◊
cacheStaleTimeoutInSecondsB ≤Refreshes a file share's cache by using Time To Live (TTL).
TTL is the length of time since the last refresh after which access to the directory would cause the file gateway
to first refresh that directory's contents from the Amazon S3 bucket. Valid Values: 300 to 2,592,000 seconds (5 minutes to 30 days)
:√
à
storagegateway NfsFileShareNfsFileShareDefaultsTaws:storagegateway/NfsFileShareNfsFileShareDefaults:NfsFileShareNfsFileShareDefaultsµ
≤`
directoryModeB" IThe Unix directory mode in the string form "nnnn". Defaults to `"0777"`.
V
fileModeB" DThe Unix file mode in the string form "nnnn". Defaults to `"0666"`.
π
groupIdB" ßThe default group ID for the file share (unless the files have another group ID specified). Defaults to `65534` (`nfsnobody`). Valid values: `0` through `4294967294`.
π
ownerIdB" ßThe default owner ID for the file share (unless the files have another owner ID specified). Defaults to `65534` (`nfsnobody`). Valid values: `0` through `4294967294`.
:€
y
storagegatewaySmbFileShareCacheAttributesJaws:storagegateway/SmbFileShareCacheAttributes:SmbFileShareCacheAttributes›
⁄◊
cacheStaleTimeoutInSecondsB ≤Refreshes a file share's cache by using Time To Live (TTL).
TTL is the length of time since the last refresh after which access to the directory would cause the file gateway
to first refresh that directory's contents from the Amazon S3 bucket. Valid Values: 300 to 2,592,000 seconds (5 minutes to 30 days)
:˚
\

syntheticsCanaryArtifactConfig8aws:synthetics/CanaryArtifactConfig:CanaryArtifactConfigö
óî
s3EncryptionâBÜ:É
Ä

synthetics CanaryArtifactConfigS3EncryptionPaws:synthetics/CanaryArtifactConfigS3Encryption:CanaryArtifactConfigS3EncryptionxConfiguration of the encryption-at-rest settings for artifacts that the canary uploads to Amazon S3. See S3 Encryption.
:Ü
Ä

synthetics CanaryArtifactConfigS3EncryptionPaws:synthetics/CanaryArtifactConfigS3Encryption:CanaryArtifactConfigS3EncryptionÄ
˝Ö
encryptionModeB" mThe encryption method to use for artifacts created by this canary. Valid values are: `SSE_S3` and `SSE_KMS`.
s
	kmsKeyArnB" `The ARN of the customer-managed KMS key to use, if you specify `SSE_KMS` for `encryption_mode`.
:ó
M

syntheticsCanaryRunConfig.aws:synthetics/CanaryRunConfig:CanaryRunConfig≈
¬–
activeTracingB
 ∏Whether this canary is to use active AWS X-Ray tracing when it runs. You can enable active tracing only for canaries that use version syn-nodejs-2.0 or later for their canary runtime.
î
environmentVariablesB2" ÛMap of environment variables that are accessible from the canary during execution. Please see [AWS Docs](https://docs.aws.amazon.com/lambda/latest/dg/configuration-envvars.html#configuration-envvars-runtime) for variables reserved for Lambda.
ë

memoryInMbB }Maximum amount of memory available to the canary while it is running, in MB. The value you specify must be a multiple of 64.
¡
timeoutInSecondsB ¶Number of seconds the canary is allowed to run before it must stop. If you omit this field, the frequency of the canary is used, up to a maximum of 840 (14 minutes).
:∏
J

syntheticsCanarySchedule,aws:synthetics/CanarySchedule:CanaryScheduleÈ
Êí
durationInSecondsB wDuration in seconds, for the canary to continue making regular runs according to the schedule in the Expression value.
Œ

expression" ªRate expression or cron expression that defines how often the canary is to run. For rate expression, the syntax is `rate(number unit)`. _unit_ can be `minute`, `minutes`, or `hour`. For cron expression, the syntax is `cron(expression)`. For more information about the syntax for cron expressions, see [Scheduling canary runs using cron](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_cron.html).
:ˆ
J

syntheticsCanaryTimeline,aws:synthetics/CanaryTimeline:CanaryTimelineß
§7
createdB" &Date and time the canary was created.
K
lastModifiedB" 5Date and time the canary was most recently modified.
N
lastStartedB" 9Date and time that the canary's most recent run started.
L
lastStoppedB" 7Date and time that the canary's most recent run ended.
:¢
M

syntheticsCanaryVpcConfig.aws:synthetics/CanaryVpcConfig:CanaryVpcConfig–
ÕH
securityGroupIdsB*" ,IDs of the security groups for this canary.
E
	subnetIdsB*" 0IDs of the subnets where this canary is to run.
:
vpcIdB" +ID of the VPC where this canary is to run.
:Õ
Ä

synthetics getRuntimeVersionsRuntimeVersionPaws:synthetics/getRuntimeVersionsRuntimeVersion:getRuntimeVersionsRuntimeVersion«
ƒP
deprecationDate" 9Date of deprecation if the runtme version is deprecated.
J
description" 7Description of the runtime version, created by Amazon.
?
releaseDate" ,Date that the runtime version was released.
‚
versionName" ŒName of the runtime version.
For a list of valid runtime versions, see [Canary Runtime Versions](https://docs.aws.amazon.com/AmazonCloudWatch/latest/monitoring/CloudWatch_Synthetics_Canaries_Library.html).
:Æ
ñ
timestreaminfluxdb"DbInstanceLogDeliveryConfiguration\aws:timestreaminfluxdb/DbInstanceLogDeliveryConfiguration:DbInstanceLogDeliveryConfigurationí
èå
s3ConfigurationÃB…:∆
√
timestreaminfluxdb1DbInstanceLogDeliveryConfigurationS3Configurationzaws:timestreaminfluxdb/DbInstanceLogDeliveryConfigurationS3Configuration:DbInstanceLogDeliveryConfigurationS3Configuration*Configuration for S3 bucket log delivery.
:”
√
timestreaminfluxdb1DbInstanceLogDeliveryConfigurationS3Configurationzaws:timestreaminfluxdb/DbInstanceLogDeliveryConfigurationS3Configuration:DbInstanceLogDeliveryConfigurationS3Configurationä
á<

bucketName" *Name of the S3 bucket to deliver logs to.
∆
enabled
 ∂Indicates whether log delivery to the S3 bucket is enabled.

**Note**: Only three arguments do updates in-place: `db_parameter_group_identifier`, `log_delivery_configuration`, and `tags`. Changes to any other argument after a DB instance has been deployed will cause destruction and re-creation of the DB instance. Additionally, when `db_parameter_group_identifier` is added to a DB instance or modified, the DB instance will be updated in-place but if `db_parameter_group_identifier` is removed from a DB instance, the DB instance will be destroyed and re-created.
:≠
f
timestreaminfluxdbDbInstanceTimeouts<aws:timestreaminfluxdb/DbInstanceTimeouts:DbInstanceTimeouts¬
øÁ
createB" ÷A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
Ë
deleteB" ◊A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
Á
updateB" ÷A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:±
ç
timestreamwrite!TableMagneticStoreWritePropertiesWaws:timestreamwrite/TableMagneticStoreWriteProperties:TableMagneticStoreWritePropertiesû
õK
enableMagneticStoreWritesB
 (A flag to enable magnetic store writes.
À
!magneticStoreRejectedDataLocation˙B˜:Ù
Ò
timestreamwriteBTableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationôaws:timestreamwrite/TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocation:TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocation®The location to write error reports for records rejected asynchronously during magnetic store writes. See Magnetic Store Rejected Data Location below for more details.
:„
Ò
timestreamwriteBTableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationôaws:timestreamwrite/TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocation:TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationÏ
ÈÊ
s3ConfigurationßB§:°
û
timestreamwriteQTableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration∑aws:timestreamwrite/TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration:TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration®Configuration of an S3 location to write error reports for records rejected, asynchronously, during magnetic store writes. See S3 Configuration below for more details.
:‹
û
timestreamwriteQTableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration∑aws:timestreamwrite/TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration:TableMagneticStoreWritePropertiesMagneticStoreRejectedDataLocationS3Configuration∏
µ;

bucketNameB" 'Bucket name of the customer S3 bucket.
≈
encryptionOptionB" ™Encryption option for the customer s3 location. Options are S3 server side encryption with an S3-managed key or KMS managed key. Valid values are `SSE_KMS` and `SSE_S3`.
c
kmsKeyIdB" QKMS key arn for the customer s3 location when encrypting with a KMS managed key.
I
objectKeyPrefixB" 0Object key prefix for the customer S3 location.
:¨
r
timestreamwriteTableRetentionPropertiesEaws:timestreamwrite/TableRetentionProperties:TableRetentionPropertiesµ
≤ò
"magneticStoreRetentionPeriodInDays nThe duration for which data must be stored in the magnetic store. Minimum value of 1. Maximum value of 73000.
î
!memoryStoreRetentionPeriodInHours kThe duration for which data must be stored in the memory store. Minimum value of 1. Maximum value of 8766.
:è
K
timestreamwriteTableSchema+aws:timestreamwrite/TableSchema:TableSchemaø
ºπ
compositePartitionKeyìBê:ç
ä
timestreamwrite TableSchemaCompositePartitionKeyUaws:timestreamwrite/TableSchemaCompositePartitionKey:TableSchemaCompositePartitionKeyâA non-empty list of partition keys defining the attributes used to partition the table data. The order of the list determines the partition hierarchy. The name and type of each partition key as well as the partition key order cannot be changed after the table is created. However, the enforcement level of each partition key can be changed. See Composite Partition Key below for more details.
:«
ä
timestreamwrite TableSchemaCompositePartitionKeyUaws:timestreamwrite/TableSchemaCompositePartitionKey:TableSchemaCompositePartitionKey∑
¥ö
enforcementInRecordB" }The level of enforcement for the specification of a dimension key in ingested records. Valid values: `REQUIRED`, `OPTIONAL`.
B
nameB" 4The name of the attribute used for a dimension key.
Q
type" EThe type of the partition key. Valid values: `DIMENSION`, `MEASURE`.
:ù
ê
timestreamwrite"getTableMagneticStoreWritePropertyYaws:timestreamwrite/getTableMagneticStoreWriteProperty:getTableMagneticStoreWritePropertyá
Ña
enableMagneticStoreWrites
 @Flag that is set based on if magnetic store writes are enabled.
û
"magneticStoreRejectedDataLocations˝*˙:˜
Ù
timestreamwriteCgetTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocationõaws:timestreamwrite/getTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocation:getTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocationxObject containing the following attributes to describe error reports for records rejected during magnetic store writes.
: 
Ù
timestreamwriteCgetTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocationõaws:timestreamwrite/getTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocation:getTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocation–
Õ 
s3Configurations™*ß:§
°
timestreamwriteRgetTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocationS3Configurationπaws:timestreamwrite/getTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocationS3Configuration:getTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocationS3ConfigurationàObject containing the following attributes to describe the configuration of an s3 location to write error reports for records rejected.
:Ì
°
timestreamwriteRgetTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocationS3Configurationπaws:timestreamwrite/getTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocationS3Configuration:getTableMagneticStoreWritePropertyMagneticStoreRejectedDataLocationS3Configuration∆
√%

bucketName" Name of S3 bucket.

encryptionOption" E
kmsKeyId" 5AWS KMS key ID for S3 location with AWS maanged key.
;
objectKeyPrefix" $Object key preview for S3 location.
:›
u
timestreamwritegetTableRetentionPropertyGaws:timestreamwrite/getTableRetentionProperty:getTableRetentionProperty„
‡o
"magneticStoreRetentionPeriodInDays EDuration in days in which the data must be stored in magnetic store.
m
!memoryStoreRetentionPeriodInHours DDuration in hours in which the data must be stored in memory store.
:ñ
T
timestreamwritegetTableSchema1aws:timestreamwrite/getTableSchema:getTableSchemaΩ
∫∑
compositePartitionKeysú*ô:ñ
ì
timestreamwrite#getTableSchemaCompositePartitionKey[aws:timestreamwrite/getTableSchemaCompositePartitionKey:getTableSchemaCompositePartitionKey:Ü
ì
timestreamwrite#getTableSchemaCompositePartitionKey[aws:timestreamwrite/getTableSchemaCompositePartitionKey:getTableSchemaCompositePartitionKeyn
l
enforcementInRecord" *
name" Name of the Timestream table.
#
type" Type of partition key.
:◊
t

transcribeLanguageModelInputDataConfigHaws:transcribe/LanguageModelInputDataConfig:LanguageModelInputDataConfigﬁ
€<
dataAccessRoleArn" #IAM role with access to S3 bucket.
4
s3Uri" 'S3 URI where training data is located.
e
tuningDataS3UriB" LS3 URI where tuning data is located.

The following arguments are optional:
:Ã
j
transferAccessHomeDirectoryMappingBaws:transfer/AccessHomeDirectoryMapping:AccessHomeDirectoryMapping^
\/
entry" "Represents an entry and a target.
)
target" Represents the map target.
:–
R
transferAccessPosixProfile2aws:transfer/AccessPosixProfile:AccessPosixProfile˘
ˆH
gid =The POSIX group ID used for all EFS operations by this user.
a
secondaryGidsB* HThe secondary POSIX group IDs used for all EFS operations by this user.
G
uid <The POSIX user ID used for all EFS operations by this user.
:¡
R
transferConnectorAs2Config2aws:transfer/ConnectorAs2Config:ConnectorAs2ConfigÍ
Áf
compression" SSpecifies weather AS2 file is compressed. The valud values are ZLIB and  DISABLED.
ç
encryptionAlgorithm" rThe algorithm that is used to encrypt the file. The valid values are AES128_CBC | AES192_CBC | AES256_CBC | NONE.
G
localProfileId" 1The unique identifier for the AS2 local profile.
§
mdnResponse" êUsed for outbound requests to determine if a partner response for transfers is synchronous or asynchronous. The valid values are SYNC and NONE.
ê
mdnSigningAlgorithmB" sThe signing algorithm for the Mdn response. The valid values are SHA256 | SHA384 | SHA512 | SHA1 | NONE | DEFAULT.
z
messageSubjectB" bUsed as the subject HTTP header attribute in AS2 messages that are being sent with the connector.
K
partnerProfileId" 3The unique identifier for the AS2 partner profile.
†
signingAlgorithm" áThe algorithm that is used to sign AS2 messages sent with the connector. The valid values are SHA256 | SHA384 | SHA512 | SHA1 | NONE .
:ƒ
U
transferConnectorSftpConfig4aws:transfer/ConnectorSftpConfig:ConnectorSftpConfigÍ
Á˛
trustedHostKeysB*" ‚A list of public portion of the host key, or keys, that are used to authenticate the user to the external server to which you are connecting.(https://docs.aws.amazon.com/transfer/latest/userguide/API_SftpConnectorConfig.html)
„
userSecretIdB" ÃThe identifier for the secret (in AWS Secrets Manager) that contains the SFTP user's private key, password, or both. The identifier can be either the Amazon Resource Name (ARN) or the name of the secret.
:Ÿ
[
transferServerEndpointDetails8aws:transfer/ServerEndpointDetails:ServerEndpointDetails˘
ˆŸ
addressAllocationIdsB*" ∏A list of address allocation IDs that are required to attach an Elastic IP address to your SFTP server's endpoint. This property can only be used when `endpoint_type` is set to `VPC`.
¨
securityGroupIdsB*" èA list of security groups IDs that are available to attach to your server's endpoint. If no security groups are specified, the VPC's default security groups are automatically assigned to your endpoint. This property can only be used when `endpoint_type` is set to `VPC`.
±
	subnetIdsB*" õA list of subnet IDs that are required to host your SFTP server endpoint in your VPC. This property can only be used when `endpoint_type` is set to `VPC`.
Ä
vpcEndpointIdB" iThe ID of the VPC endpoint. This property can only be used when `endpoint_type` is set to `VPC_ENDPOINT`
±
vpcIdB" °The VPC ID of the virtual private cloud in which the SFTP server's endpoint will be hosted. This property can only be used when `endpoint_type` is set to `VPC`.
:È
[
transferServerProtocolDetails8aws:transfer/ServerProtocolDetails:ServerProtocolDetailsâ
Üs
as2TransportsB*" ZIndicates the transport method for the AS2 messages. Currently, only `HTTP` is supported.
¨
	passiveIpB" òIndicates passive mode, for FTP and FTPS protocols. Enter a single IPv4 address, such as the public IP address of a firewall, router, or load balancer.
¬
setStatOptionB" ™Use to ignore the error that is generated when the client attempts to use `SETSTAT` on a file you are uploading to an S3 bucket. Valid values: `DEFAULT`, `ENABLE_NO_OP`.
ö
tlsSessionResumptionModeB" ˜A property used with Transfer Family servers that use the FTPS protocol. Provides a mechanism to resume or share a negotiated secret key between the control and data connection for an FTPS session. Valid values: `DISABLED`, `ENABLED`, `ENFORCED`.
:ö
^
transferServerS3StorageOptions:aws:transfer/ServerS3StorageOptions:ServerS3StorageOptions∑
¥±
directoryListingOptimizationB" äSpecifies whether or not performance for your Amazon S3 directories is optimized. Valid values are `DISABLED`, `ENABLED`.

By default, home directory mappings have a `TYPE` of `DIRECTORY`. If you enable this option, you would then need to explicitly set the `HomeDirectoryMapEntry` Type to `FILE` if you want a mapping to have a file target. See [Using logical directories to simplify your Transfer Family directory structures](https://docs.aws.amazon.com/transfer/latest/userguide/logical-dir-mappings.html) for details.
:∞
[
transferServerWorkflowDetails8aws:transfer/ServerWorkflowDetails:ServerWorkflowDetails–
Õª
onPartialUploadëBé:ã
à
transfer$ServerWorkflowDetailsOnPartialUploadVaws:transfer/ServerWorkflowDetailsOnPartialUpload:ServerWorkflowDetailsOnPartialUploadìA trigger that starts a workflow if a file is only partially uploaded. See Workflow Detail below. See `on_partial_upload` Block below for details.
å
onUploadyBw:u
s
transferServerWorkflowDetailsOnUploadHaws:transfer/ServerWorkflowDetailsOnUpload:ServerWorkflowDetailsOnUploadÑA trigger that starts a workflow: the workflow begins to execute after a file is uploaded. See `on_upload` Block below for details.
:Ü
à
transfer$ServerWorkflowDetailsOnPartialUploadVaws:transfer/ServerWorkflowDetailsOnPartialUpload:ServerWorkflowDetailsOnPartialUpload¯
ı∏
executionRole" ¢Includes the necessary permissions for S3, EFS, and Lambda operations that Transfer can assume, so that all workflow steps can operate on the required resources.
8

workflowId" &A unique identifier for the workflow.
:
s
transferServerWorkflowDetailsOnUploadHaws:transfer/ServerWorkflowDetailsOnUpload:ServerWorkflowDetailsOnUpload¯
ı∏
executionRole" ¢Includes the necessary permissions for S3, EFS, and Lambda operations that Transfer can assume, so that all workflow steps can operate on the required resources.
8

workflowId" &A unique identifier for the workflow.
:¯
d
transferUserHomeDirectoryMapping>aws:transfer/UserHomeDirectoryMapping:UserHomeDirectoryMappingè
å/
entry" "Represents an entry and a target.
ÿ
target" …Represents the map target.

The `Restricted` option is achieved using the following mapping:

```
home_directory_mappings {
entry  = "/"
target = "/${aws_s3_bucket.foo.id}/$${Transfer:UserName}"
}
```
: 
L
transferUserPosixProfile.aws:transfer/UserPosixProfile:UserPosixProfile˘
ˆH
gid =The POSIX group ID used for all EFS operations by this user.
a
secondaryGidsB* HThe secondary POSIX group IDs used for all EFS operations by this user.
G
uid <The POSIX user ID used for all EFS operations by this user.
:Ç

a
transferWorkflowOnExceptionStep<aws:transfer/WorkflowOnExceptionStep:WorkflowOnExceptionStepú	
ô	¯
copyStepDetailsóBî:ë
é
transfer&WorkflowOnExceptionStepCopyStepDetailsZaws:transfer/WorkflowOnExceptionStepCopyStepDetails:WorkflowOnExceptionStepCopyStepDetailsKDetails for a step that performs a file copy. See Copy Step Details below.
Ë
customStepDetailsùBö:ó
î
transfer(WorkflowOnExceptionStepCustomStepDetails^aws:transfer/WorkflowOnExceptionStepCustomStepDetails:WorkflowOnExceptionStepCustomStepDetails3Details for a step that invokes a lambda function.
‰
decryptStepDetails†Bù:ö
ó
transfer)WorkflowOnExceptionStepDecryptStepDetails`aws:transfer/WorkflowOnExceptionStepDecryptStepDetails:WorkflowOnExceptionStepDecryptStepDetails+Details for a step that decrypts the file.
ﬂ
deleteStepDetailsùBö:ó
î
transfer(WorkflowOnExceptionStepDeleteStepDetails^aws:transfer/WorkflowOnExceptionStepDeleteStepDetails:WorkflowOnExceptionStepDeleteStepDetails*Details for a step that deletes the file.
€
tagStepDetailsîBë:é
ã
transfer%WorkflowOnExceptionStepTagStepDetailsXaws:transfer/WorkflowOnExceptionStepTagStepDetails:WorkflowOnExceptionStepTagStepDetails2Details for a step that creates one or more tags.


type" :‹	
é
transfer&WorkflowOnExceptionStepCopyStepDetailsZaws:transfer/WorkflowOnExceptionStepCopyStepDetails:WorkflowOnExceptionStepCopyStepDetails»
≈à
destinationFileLocation›B⁄:◊
‘
transfer=WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationàaws:transfer/WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocation:WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationåSpecifies the location for the file being copied. Use ${Transfer:username} in this field to parametrize the destination prefix by username.
;
nameB" -The name of the step, used as an identifier.
Æ
overwriteExistingB" íA flag that indicates whether or not to overwrite an existing file of the same name. The default is `FALSE`. Valid values are `TRUE` and `FALSE`.
…
sourceFileLocationB" ¨Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file for the workflow. Enter ${previous.file} to use the previous file as the input. In this case, this workflow step uses the output file from the previous workflow step as input. This is the default value. Enter ${original.file} to use the originally-uploaded file location as input for this step.
:à
‘
transfer=WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationàaws:transfer/WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocation:WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationÆ
´’
efsFileLocationäBá:Ñ
Å
transferLWorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationEfsFileLocation¶aws:transfer/WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationEfsFileLocation:WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationEfsFileLocation5Specifies the details for the EFS file being copied.
–
s3FileLocationáBÑ:Å
˛
transferKWorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationS3FileLocation§aws:transfer/WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationS3FileLocation:WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationS3FileLocation4Specifies the details for the S3 file being copied.
:õ
Å
transferLWorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationEfsFileLocation¶aws:transfer/WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationEfsFileLocation:WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationEfsFileLocationî
ëI
fileSystemIdB" 3The ID of the file system, assigned by Amazon EFS.
D
pathB" 6The pathname for the folder being used by a workflow.
:≈
˛
transferKWorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationS3FileLocation§aws:transfer/WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationS3FileLocation:WorkflowOnExceptionStepCopyStepDetailsDestinationFileLocationS3FileLocation¡
æE
bucketB" 5Specifies the S3 bucket for the customer input file.
u
keyB" hThe name assigned to the file when it was created in S3. You use the object key to retrieve the object.
:´
î
transfer(WorkflowOnExceptionStepCustomStepDetails^aws:transfer/WorkflowOnExceptionStepCustomStepDetails:WorkflowOnExceptionStepCustomStepDetailsë
é;
nameB" -The name of the step, used as an identifier.
…
sourceFileLocationB" ¨Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file for the workflow. Enter ${previous.file} to use the previous file as the input. In this case, this workflow step uses the output file from the previous workflow step as input. This is the default value. Enter ${original.file} to use the originally-uploaded file location as input for this step.
F
targetB" 6The ARN for the lambda function that is being called.
;
timeoutSecondsB #Timeout, in seconds, for the step.
:¿

ó
transfer)WorkflowOnExceptionStepDecryptStepDetails`aws:transfer/WorkflowOnExceptionStepDecryptStepDetails:WorkflowOnExceptionStepDecryptStepDetails£	
†	ë
destinationFileLocationÊB„:‡
›
transfer@WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationéaws:transfer/WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocation:WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationåSpecifies the location for the file being copied. Use ${Transfer:username} in this field to parametrize the destination prefix by username.
;
nameB" -The name of the step, used as an identifier.
Æ
overwriteExistingB" íA flag that indicates whether or not to overwrite an existing file of the same name. The default is `FALSE`. Valid values are `TRUE` and `FALSE`.
…
sourceFileLocationB" ¨Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file for the workflow. Enter ${previous.file} to use the previous file as the input. In this case, this workflow step uses the output file from the previous workflow step as input. This is the default value. Enter ${original.file} to use the originally-uploaded file location as input for this step.
P
type" DThe type of encryption used. Currently, this value must be `"PGP"`.
:£
›
transfer@WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationéaws:transfer/WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocation:WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocation¿
Ωﬁ
efsFileLocationìBê:ç
ä
transferOWorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationEfsFileLocation¨aws:transfer/WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationEfsFileLocation:WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationEfsFileLocation5Specifies the details for the EFS file being copied.
Ÿ
s3FileLocationêBç:ä
á
transferNWorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationS3FileLocation™aws:transfer/WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationS3FileLocation:WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationS3FileLocation4Specifies the details for the S3 file being copied.
:§
ä
transferOWorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationEfsFileLocation¨aws:transfer/WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationEfsFileLocation:WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationEfsFileLocationî
ëI
fileSystemIdB" 3The ID of the file system, assigned by Amazon EFS.
D
pathB" 6The pathname for the folder being used by a workflow.
:Œ
á
transferNWorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationS3FileLocation™aws:transfer/WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationS3FileLocation:WorkflowOnExceptionStepDecryptStepDetailsDestinationFileLocationS3FileLocation¡
æE
bucketB" 5Specifies the S3 bucket for the customer input file.
u
keyB" hThe name assigned to the file when it was created in S3. You use the object key to retrieve the object.
:¶
î
transfer(WorkflowOnExceptionStepDeleteStepDetails^aws:transfer/WorkflowOnExceptionStepDeleteStepDetails:WorkflowOnExceptionStepDeleteStepDetailså
â;
nameB" -The name of the step, used as an identifier.
…
sourceFileLocationB" ¨Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file for the workflow. Enter ${previous.file} to use the previous file as the input. In this case, this workflow step uses the output file from the previous workflow step as input. This is the default value. Enter ${original.file} to use the originally-uploaded file location as input for this step.
:ê
ã
transfer%WorkflowOnExceptionStepTagStepDetailsXaws:transfer/WorkflowOnExceptionStepTagStepDetails:WorkflowOnExceptionStepTagStepDetailsˇ
¸;
nameB" -The name of the step, used as an identifier.
…
sourceFileLocationB" ¨Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file for the workflow. Enter ${previous.file} to use the previous file as the input. In this case, this workflow step uses the output file from the previous workflow step as input. This is the default value. Enter ${original.file} to use the originally-uploaded file location as input for this step.

tags†Bù*ö:ó
î
transfer(WorkflowOnExceptionStepTagStepDetailsTag^aws:transfer/WorkflowOnExceptionStepTagStepDetailsTag:WorkflowOnExceptionStepTagStepDetailsTagEArray that contains from 1 to 10 key/value pairs. See S3 Tags below.
:‹
î
transfer(WorkflowOnExceptionStepTagStepDetailsTag^aws:transfer/WorkflowOnExceptionStepTagStepDetailsTag:WorkflowOnExceptionStepTagStepDetailsTagC
A	
key" 4
value" 'The value that corresponds to the key.
:®
@
transferWorkflowStep&aws:transfer/WorkflowStep:WorkflowStep„
‡”
copyStepDetailssBq:o
m
transferWorkflowStepCopyStepDetailsDaws:transfer/WorkflowStepCopyStepDetails:WorkflowStepCopyStepDetailsKDetails for a step that performs a file copy. See Copy Step Details below.
√
customStepDetailsyBw:u
s
transferWorkflowStepCustomStepDetailsHaws:transfer/WorkflowStepCustomStepDetails:WorkflowStepCustomStepDetails3Details for a step that invokes a lambda function.
ø
decryptStepDetails|Bz:x
v
transferWorkflowStepDecryptStepDetailsJaws:transfer/WorkflowStepDecryptStepDetails:WorkflowStepDecryptStepDetails+Details for a step that decrypts the file.
∫
deleteStepDetailsyBw:u
s
transferWorkflowStepDeleteStepDetailsHaws:transfer/WorkflowStepDeleteStepDetails:WorkflowStepDeleteStepDetails*Details for a step that deletes the file.
∂
tagStepDetailspBn:l
j
transferWorkflowStepTagStepDetailsBaws:transfer/WorkflowStepTagStepDetails:WorkflowStepTagStepDetails2Details for a step that creates one or more tags.


type" :ò	
m
transferWorkflowStepCopyStepDetailsDaws:transfer/WorkflowStepCopyStepDetails:WorkflowStepCopyStepDetails¶
£Ê
destinationFileLocationªB∏:µ
≤
transfer2WorkflowStepCopyStepDetailsDestinationFileLocationraws:transfer/WorkflowStepCopyStepDetailsDestinationFileLocation:WorkflowStepCopyStepDetailsDestinationFileLocationåSpecifies the location for the file being copied. Use ${Transfer:username} in this field to parametrize the destination prefix by username.
;
nameB" -The name of the step, used as an identifier.
Æ
overwriteExistingB" íA flag that indicates whether or not to overwrite an existing file of the same name. The default is `FALSE`. Valid values are `TRUE` and `FALSE`.
…
sourceFileLocationB" ¨Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file for the workflow. Enter ${previous.file} to use the previous file as the input. In this case, this workflow step uses the output file from the previous workflow step as input. This is the default value. Enter ${original.file} to use the originally-uploaded file location as input for this step.
:§
≤
transfer2WorkflowStepCopyStepDetailsDestinationFileLocationraws:transfer/WorkflowStepCopyStepDetailsDestinationFileLocation:WorkflowStepCopyStepDetailsDestinationFileLocationÏ
È¥
efsFileLocationÈBÊ:„
‡
transferAWorkflowStepCopyStepDetailsDestinationFileLocationEfsFileLocationêaws:transfer/WorkflowStepCopyStepDetailsDestinationFileLocationEfsFileLocation:WorkflowStepCopyStepDetailsDestinationFileLocationEfsFileLocation5Specifies the details for the EFS file being copied.
Ø
s3FileLocationÊB„:‡
›
transfer@WorkflowStepCopyStepDetailsDestinationFileLocationS3FileLocationéaws:transfer/WorkflowStepCopyStepDetailsDestinationFileLocationS3FileLocation:WorkflowStepCopyStepDetailsDestinationFileLocationS3FileLocation4Specifies the details for the S3 file being copied.
:˙
‡
transferAWorkflowStepCopyStepDetailsDestinationFileLocationEfsFileLocationêaws:transfer/WorkflowStepCopyStepDetailsDestinationFileLocationEfsFileLocation:WorkflowStepCopyStepDetailsDestinationFileLocationEfsFileLocationî
ëI
fileSystemIdB" 3The ID of the file system, assigned by Amazon EFS.
D
pathB" 6The pathname for the folder being used by a workflow.
:§
›
transfer@WorkflowStepCopyStepDetailsDestinationFileLocationS3FileLocationéaws:transfer/WorkflowStepCopyStepDetailsDestinationFileLocationS3FileLocation:WorkflowStepCopyStepDetailsDestinationFileLocationS3FileLocation¡
æE
bucketB" 5Specifies the S3 bucket for the customer input file.
u
keyB" hThe name assigned to the file when it was created in S3. You use the object key to retrieve the object.
:â
s
transferWorkflowStepCustomStepDetailsHaws:transfer/WorkflowStepCustomStepDetails:WorkflowStepCustomStepDetailsë
é;
nameB" -The name of the step, used as an identifier.
…
sourceFileLocationB" ¨Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file for the workflow. Enter ${previous.file} to use the previous file as the input. In this case, this workflow step uses the output file from the previous workflow step as input. This is the default value. Enter ${original.file} to use the originally-uploaded file location as input for this step.
F
targetB" 6The ARN for the lambda function that is being called.
;
timeoutSecondsB #Timeout, in seconds, for the step.
:¸	
v
transferWorkflowStepDecryptStepDetailsJaws:transfer/WorkflowStepDecryptStepDetails:WorkflowStepDecryptStepDetailsÅ	
˛Ô
destinationFileLocationƒB¡:æ
ª
transfer5WorkflowStepDecryptStepDetailsDestinationFileLocationxaws:transfer/WorkflowStepDecryptStepDetailsDestinationFileLocation:WorkflowStepDecryptStepDetailsDestinationFileLocationåSpecifies the location for the file being copied. Use ${Transfer:username} in this field to parametrize the destination prefix by username.
;
nameB" -The name of the step, used as an identifier.
Æ
overwriteExistingB" íA flag that indicates whether or not to overwrite an existing file of the same name. The default is `FALSE`. Valid values are `TRUE` and `FALSE`.
…
sourceFileLocationB" ¨Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file for the workflow. Enter ${previous.file} to use the previous file as the input. In this case, this workflow step uses the output file from the previous workflow step as input. This is the default value. Enter ${original.file} to use the originally-uploaded file location as input for this step.
P
type" DThe type of encryption used. Currently, this value must be `"PGP"`.
:ø
ª
transfer5WorkflowStepDecryptStepDetailsDestinationFileLocationxaws:transfer/WorkflowStepDecryptStepDetailsDestinationFileLocation:WorkflowStepDecryptStepDetailsDestinationFileLocation˛
˚Ω
efsFileLocationÚBÔ:Ï
È
transferDWorkflowStepDecryptStepDetailsDestinationFileLocationEfsFileLocationñaws:transfer/WorkflowStepDecryptStepDetailsDestinationFileLocationEfsFileLocation:WorkflowStepDecryptStepDetailsDestinationFileLocationEfsFileLocation5Specifies the details for the EFS file being copied.
∏
s3FileLocationÔBÏ:È
Ê
transferCWorkflowStepDecryptStepDetailsDestinationFileLocationS3FileLocationîaws:transfer/WorkflowStepDecryptStepDetailsDestinationFileLocationS3FileLocation:WorkflowStepDecryptStepDetailsDestinationFileLocationS3FileLocation4Specifies the details for the S3 file being copied.
:É
È
transferDWorkflowStepDecryptStepDetailsDestinationFileLocationEfsFileLocationñaws:transfer/WorkflowStepDecryptStepDetailsDestinationFileLocationEfsFileLocation:WorkflowStepDecryptStepDetailsDestinationFileLocationEfsFileLocationî
ëI
fileSystemIdB" 3The ID of the file system, assigned by Amazon EFS.
D
pathB" 6The pathname for the folder being used by a workflow.
:≠
Ê
transferCWorkflowStepDecryptStepDetailsDestinationFileLocationS3FileLocationîaws:transfer/WorkflowStepDecryptStepDetailsDestinationFileLocationS3FileLocation:WorkflowStepDecryptStepDetailsDestinationFileLocationS3FileLocation¡
æE
bucketB" 5Specifies the S3 bucket for the customer input file.
u
keyB" hThe name assigned to the file when it was created in S3. You use the object key to retrieve the object.
:Ñ
s
transferWorkflowStepDeleteStepDetailsHaws:transfer/WorkflowStepDeleteStepDetails:WorkflowStepDeleteStepDetailså
â;
nameB" -The name of the step, used as an identifier.
…
sourceFileLocationB" ¨Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file for the workflow. Enter ${previous.file} to use the previous file as the input. In this case, this workflow step uses the output file from the previous workflow step as input. This is the default value. Enter ${original.file} to use the originally-uploaded file location as input for this step.
:»
j
transferWorkflowStepTagStepDetailsBaws:transfer/WorkflowStepTagStepDetails:WorkflowStepTagStepDetailsŸ
÷;
nameB" -The name of the step, used as an identifier.
…
sourceFileLocationB" ¨Specifies which file to use as input to the workflow step: either the output from the previous step, or the originally uploaded file for the workflow. Enter ${previous.file} to use the previous file as the input. In this case, this workflow step uses the output file from the previous workflow step as input. This is the default value. Enter ${original.file} to use the originally-uploaded file location as input for this step.
 
tags{By*w:u
s
transferWorkflowStepTagStepDetailsTagHaws:transfer/WorkflowStepTagStepDetailsTag:WorkflowStepTagStepDetailsTagEArray that contains from 1 to 10 key/value pairs. See S3 Tags below.
:∫
s
transferWorkflowStepTagStepDetailsTagHaws:transfer/WorkflowStepTagStepDetailsTag:WorkflowStepTagStepDetailsTagC
A	
key" 4
value" 'The value that corresponds to the key.
:Ø
[
transfergetConnectorAs2Config8aws:transfer/getConnectorAs2Config:getConnectorAs2Configœ
Ãf
basicAuthSecretId" MBasic authentication for AS2 connector API. Returns a null value if not set.
V
compression" CSpecifies whether AS2 file is compressed. Will be ZLIB or DISABLED
É
encryptionAlgorithm" hAlgorithm used to encrypt file. Will be AES128_CBC or AES192_CBC or AES256_CBC or DES_EDE3_CBC or NONE.
?
localProfileId" )Unique identifier for AS2 local profile.
w
mdnResponse" dUsed for outbound requests to tell if response is asynchronous or not. Will be either SYNC or NONE.
~
mdnSigningAlgorithm" cSigning algorithm for MDN response. Will be SHA256 or SHA384 or SHA512 or SHA1 or NONE or DEFAULT.
_
messageSubject" ISubject HTTP header attribute in outbound AS2 messages to the connector.
Q
partnerProfileId" 9Unique identifier used by connector for partner profile.

singingAlgorithm" :˜
^
transfergetConnectorSftpConfig:aws:transfer/getConnectorSftpConfig:getConnectorSftpConfigî
ëã
trustedHostKeys*" rList of the public portions of the host keys that are used to identify the servers the connector is connected to.
Ä
userSecretId" lIdentifer for the secret in AWS Secrets Manager that contains the SFTP user's private key, and/or password.
:Õ
y
verifiedaccessEndpointLoadBalancerOptionsJaws:verifiedaccess/EndpointLoadBalancerOptions:EndpointLoadBalancerOptionsP
N
loadBalancerArnB" 
portB 
protocolB" 
	subnetIdsB*" :»
Ö
verifiedaccessEndpointNetworkInterfaceOptionsRaws:verifiedaccess/EndpointNetworkInterfaceOptions:EndpointNetworkInterfaceOptions>
<
networkInterfaceIdB" 
portB 
protocolB" :¨
p
verifiedaccessEndpointSseSpecificationDaws:verifiedaccess/EndpointSseSpecification:EndpointSseSpecification8
6!
customerManagedKeyEnabledB
 
	kmsKeyArnB" :˙
g
verifiedaccessGroupSseConfiguration>aws:verifiedaccess/GroupSseConfiguration:GroupSseConfigurationé
ãY
customerManagedKeyEnabledB
 6Boolean flag to indicate that the CMK should be used.
.
	kmsKeyArnB" ARN of the KMS key to use.
:Ó

ö
verifiedaccess&InstanceLoggingConfigurationAccessLogs`aws:verifiedaccess/InstanceLoggingConfigurationAccessLogs:InstanceLoggingConfigurationAccessLogsŒ	
À	≈
cloudwatchLogsÕB :«
ƒ
verifiedaccess4InstanceLoggingConfigurationAccessLogsCloudwatchLogs|aws:verifiedaccess/InstanceLoggingConfigurationAccessLogsCloudwatchLogs:InstanceLoggingConfigurationAccessLogsCloudwatchLogscA block that specifies configures sending Verified Access logs to CloudWatch Logs. Detailed below.
W
includeTrustContextB
 :Include trust data sent by trust providers into the logs.
“
kinesisDataFirehose›B⁄:◊
‘
verifiedaccess9InstanceLoggingConfigurationAccessLogsKinesisDataFirehoseÜaws:verifiedaccess/InstanceLoggingConfigurationAccessLogsKinesisDataFirehose:InstanceLoggingConfigurationAccessLogsKinesisDataFirehose[A block that specifies configures sending Verified Access logs to Kinesis. Detailed below.
«

logVersionB" ≤The logging version to use. Refer to [VerifiedAccessLogOptions](https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_VerifiedAccessLogOptions.html) for the allowed values.
à
s3©B¶:£
†
verifiedaccess(InstanceLoggingConfigurationAccessLogsS3daws:verifiedaccess/InstanceLoggingConfigurationAccessLogsS3:InstanceLoggingConfigurationAccessLogsS3VA block that specifies configures sending Verified Access logs to S3. Detailed below.
:¡
ƒ
verifiedaccess4InstanceLoggingConfigurationAccessLogsCloudwatchLogs|aws:verifiedaccess/InstanceLoggingConfigurationAccessLogsCloudwatchLogs:InstanceLoggingConfigurationAccessLogsCloudwatchLogsx
v5
enabled
 &Indicates whether logging is enabled.
=
logGroupB" +The name of the CloudWatch Logs Log Group.
:Õ
‘
verifiedaccess9InstanceLoggingConfigurationAccessLogsKinesisDataFirehoseÜaws:verifiedaccess/InstanceLoggingConfigurationAccessLogsKinesisDataFirehose:InstanceLoggingConfigurationAccessLogsKinesisDataFirehoset
r9
deliveryStreamB" !The name of the delivery stream.
5
enabled
 &Indicates whether logging is enabled.
:É
†
verifiedaccess(InstanceLoggingConfigurationAccessLogsS3daws:verifiedaccess/InstanceLoggingConfigurationAccessLogsS3:InstanceLoggingConfigurationAccessLogsS3›
⁄+

bucketNameB" The name of S3 bucket.
O
bucketOwnerB" :The ID of the AWS account that owns the Amazon S3 bucket.
5
enabled
 &Indicates whether logging is enabled.
#
prefixB" The bucket prefix.
:ë
ë
verifiedaccess#InstanceVerifiedAccessTrustProviderZaws:verifiedaccess/InstanceVerifiedAccessTrustProvider:InstanceVerifiedAccessTrustProvider˙
˜I
descriptionB" 4A description for the AWS Verified Access Instance.
J
deviceTrustProviderTypeB" )The type of device-based trust provider.
O
trustProviderTypeB" 4The type of trust provider (user- or device-based).
F
userTrustProviderTypeB" 'The type of user-based trust provider.
E
verifiedAccessTrustProviderIdB" The ID of the trust provider.
:é
v
verifiedaccessTrustProviderDeviceOptionsHaws:verifiedaccess/TrustProviderDeviceOptions:TrustProviderDeviceOptions

tenantIdB" :ç
p
verifiedaccessTrustProviderOidcOptionsDaws:verifiedaccess/TrustProviderOidcOptions:TrustProviderOidcOptionsò
ï
authorizationEndpointB" 
clientIdB" 
clientSecret" 
issuerB" 
scopeB" 
tokenEndpointB" 
userInfoEndpointB" :ø
É
verifiedpermissionsIdentitySourceConfigurationOaws:verifiedpermissions/IdentitySourceConfiguration:IdentitySourceConfiguration∂
≥ 
cognitoUserPoolConfiguration·Bﬁ:€
ÿ
verifiedpermissions7IdentitySourceConfigurationCognitoUserPoolConfigurationáaws:verifiedpermissions/IdentitySourceConfigurationCognitoUserPoolConfiguration:IdentitySourceConfigurationCognitoUserPoolConfiguration≈Specifies the configuration details of an Amazon Cognito user pool that Verified Permissions can use as a source of authenticated identities as entities. See Cognito User Pool Configuration below.
„
openIdConnectConfiguration€Bÿ:’
“
verifiedpermissions5IdentitySourceConfigurationOpenIdConnectConfigurationÉaws:verifiedpermissions/IdentitySourceConfigurationOpenIdConnectConfiguration:IdentitySourceConfigurationOpenIdConnectConfigurationÊSpecifies the configuration details of an OpenID Connect (OIDC) identity provider, or identity source, that Verified Permissions can use to generate entities from authenticated identities. See Open ID Connect Configuration below.
:ö
ÿ
verifiedpermissions7IdentitySourceConfigurationCognitoUserPoolConfigurationáaws:verifiedpermissions/IdentitySourceConfigurationCognitoUserPoolConfiguration:IdentitySourceConfigurationCognitoUserPoolConfigurationº
πx
	clientIdsB*" cThe unique application client IDs that are associated with the specified Amazon Cognito user pool.
∏
groupConfigurationóBî:ë
é
verifiedpermissionsIIdentitySourceConfigurationCognitoUserPoolConfigurationGroupConfiguration´aws:verifiedpermissions/IdentitySourceConfigurationCognitoUserPoolConfigurationGroupConfiguration:IdentitySourceConfigurationCognitoUserPoolConfigurationGroupConfigurationáThe type of entity that a policy store maps to groups from an Amazon Cognito user pool identity source. See Group Configuration below.
Å
userPoolArn" nThe Amazon Resource Name (ARN) of the Amazon Cognito user pool that contains the identities to be authorized.
:õ
é
verifiedpermissionsIIdentitySourceConfigurationCognitoUserPoolConfigurationGroupConfiguration´aws:verifiedpermissions/IdentitySourceConfigurationCognitoUserPoolConfigurationGroupConfiguration:IdentitySourceConfigurationCognitoUserPoolConfigurationGroupConfigurationá
ÑÅ
groupEntityType" jThe name of the schema entity type that's mapped to the user pool group. Defaults to `AWS::CognitoGroup`.
:ë
“
verifiedpermissions5IdentitySourceConfigurationOpenIdConnectConfigurationÉaws:verifiedpermissions/IdentitySourceConfigurationOpenIdConnectConfiguration:IdentitySourceConfigurationOpenIdConnectConfigurationπ	
∂	x
entityIdPrefixB" `A descriptive string that you want to prefix to user entities from your OIDC identity provider.
≤
groupConfigurationëBé:ã
à
verifiedpermissionsGIdentitySourceConfigurationOpenIdConnectConfigurationGroupConfigurationßaws:verifiedpermissions/IdentitySourceConfigurationOpenIdConnectConfigurationGroupConfiguration:IdentitySourceConfigurationOpenIdConnectConfigurationGroupConfigurationáThe type of entity that a policy store maps to groups from an Amazon Cognito user pool identity source. See Group Configuration below.
ö
issuer" ãThe issuer URL of an OIDC identity provider. This URL must have an OIDC discovery endpoint at the path `.well-known/openid-configuration`.
Á
tokenSelectionÖBÇ:ˇ
¸
verifiedpermissionsCIdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionüaws:verifiedpermissions/IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelection:IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionÃThe token type that you want to process from your OIDC identity provider. Your policy store can process either identity (ID) or access tokens from a given OIDC identity source. See Token Selection below.
:ï
à
verifiedpermissionsGIdentitySourceConfigurationOpenIdConnectConfigurationGroupConfigurationßaws:verifiedpermissions/IdentitySourceConfigurationOpenIdConnectConfigurationGroupConfiguration:IdentitySourceConfigurationOpenIdConnectConfigurationGroupConfigurationá
Ñ~

groupClaim" lThe token claim that you want Verified Permissions to interpret as group membership. For example, `groups`.
Å
groupEntityType" jThe name of the schema entity type that's mapped to the user pool group. Defaults to `AWS::CognitoGroup`.
:–
¸
verifiedpermissionsCIdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionüaws:verifiedpermissions/IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelection:IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionŒ
Àö
accessTokenOnly≤BØ:¨
©
verifiedpermissionsRIdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnlyΩaws:verifiedpermissions/IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnly:IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnlyRThe OIDC configuration for processing access tokens. See Access Token Only below.
´
identityTokenOnly∏Bµ:≤
Ø
verifiedpermissionsTIdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionIdentityTokenOnly¡aws:verifiedpermissions/IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionIdentityTokenOnly:IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionIdentityTokenOnly[The OIDC configuration for processing identity (ID) tokens. See Identity Token Only below.
:Ù
©
verifiedpermissionsRIdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnlyΩaws:verifiedpermissions/IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnly:IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionAccessTokenOnly≈
¬e
	audiencesB*" PThe access token aud claim values that you want to accept in your policy store.
Y
principalIdClaimB" ?The claim that determines the principal in OIDC access tokens.
:™
Ø
verifiedpermissionsTIdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionIdentityTokenOnly¡aws:verifiedpermissions/IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionIdentityTokenOnly:IdentitySourceConfigurationOpenIdConnectConfigurationTokenSelectionIdentityTokenOnlyı
Úî
	clientIdsB*" The ID token audience, or client ID, claim values that you want to accept in your policy store from an OIDC identity provider.
Y
principalIdClaimB" ?The claim that determines the principal in OIDC access tokens.
:Ü
b
verifiedpermissionsPolicyDefinition9aws:verifiedpermissions/PolicyDefinition:PolicyDefinitionü
úµ
staticzBx:v
t
verifiedpermissionsPolicyDefinitionStaticEaws:verifiedpermissions/PolicyDefinitionStatic:PolicyDefinitionStatic/The static policy statement. See Static below.
·
templateLinkedïBí:è
å
verifiedpermissionsPolicyDefinitionTemplateLinkedUaws:verifiedpermissions/PolicyDefinitionTemplateLinked:PolicyDefinitionTemplateLinked7The template linked policy. See Template Linked below.
:Ó
t
verifiedpermissionsPolicyDefinitionStaticEaws:verifiedpermissions/PolicyDefinitionStatic:PolicyDefinitionStaticv
t;
descriptionB" &The description of the static policy.
5
	statement" $The statement of the static policy.
:¢
å
verifiedpermissionsPolicyDefinitionTemplateLinkedUaws:verifiedpermissions/PolicyDefinitionTemplateLinked:PolicyDefinitionTemplateLinkedê
ç0
policyTemplateId" The ID of the template.
Ì
	principal∞B≠:™
ß
verifiedpermissions'PolicyDefinitionTemplateLinkedPrincipalgaws:verifiedpermissions/PolicyDefinitionTemplateLinkedPrincipal:PolicyDefinitionTemplateLinkedPrincipal-The principal of the template linked policy.
Ë
resource≠B™:ß
§
verifiedpermissions&PolicyDefinitionTemplateLinkedResourceeaws:verifiedpermissions/PolicyDefinitionTemplateLinkedResource:PolicyDefinitionTemplateLinkedResource,The resource of the template linked policy.
:ñ
ß
verifiedpermissions'PolicyDefinitionTemplateLinkedPrincipalgaws:verifiedpermissions/PolicyDefinitionTemplateLinkedPrincipal:PolicyDefinitionTemplateLinkedPrincipalj
h0
entityId"  The entity ID of the principal.
4

entityType" "The entity type of the principal.
:ë
§
verifiedpermissions&PolicyDefinitionTemplateLinkedResourceeaws:verifiedpermissions/PolicyDefinitionTemplateLinkedResource:PolicyDefinitionTemplateLinkedResourceh
f/
entityId" The entity ID of the resource.
3

entityType" !The entity type of the resource.
:ä
â
verifiedpermissionsPolicyStoreValidationSettingsSaws:verifiedpermissions/PolicyStoreValidationSettings:PolicyStoreValidationSettings|
zx
mode" lThe mode for the validation settings. Valid values: `OFF`, `STRICT`.

The following arguments are optional:
:£
b
verifiedpermissionsSchemaDefinition9aws:verifiedpermissions/SchemaDefinition:SchemaDefinition=
;9
value" ,A JSON string representation of the schema.
:¢
è
verifiedpermissionsgetPolicyStoreValidationSettingWaws:verifiedpermissions/getPolicyStoreValidationSetting:getPolicyStoreValidationSetting


mode" 