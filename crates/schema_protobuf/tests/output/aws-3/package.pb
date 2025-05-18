
awsAWS"6.66.2*Ôö
,
budgetsBudgetaws:budgets/budget:BudgetÎÔProvides a budgets budget resource. Budgets use the cost visualization provided by Cost Explorer to show you the status of your budgets, to provide forecasts of your estimated costs, and to track your AWS usage, including your free tier usage.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const ec2 = new aws.budgets.Budget("ec2", {
    name: "budget-ec2-monthly",
    budgetType: "COST",
    limitAmount: "1200",
    limitUnit: "USD",
    timePeriodEnd: "2087-06-15_00:00",
    timePeriodStart: "2017-07-01_00:00",
    timeUnit: "MONTHLY",
    costFilters: [{
        name: "Service",
        values: ["Amazon Elastic Compute Cloud - Compute"],
    }],
    notifications: [{
        comparisonOperator: "GREATER_THAN",
        threshold: 100,
        thresholdType: "PERCENTAGE",
        notificationType: "FORECASTED",
        subscriberEmailAddresses: ["test@example.com"],
    }],
    tags: {
        Tag1: "Value1",
        Tag2: "Value2",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

ec2 = aws.budgets.Budget("ec2",
    name="budget-ec2-monthly",
    budget_type="COST",
    limit_amount="1200",
    limit_unit="USD",
    time_period_end="2087-06-15_00:00",
    time_period_start="2017-07-01_00:00",
    time_unit="MONTHLY",
    cost_filters=[{
        "name": "Service",
        "values": ["Amazon Elastic Compute Cloud - Compute"],
    }],
    notifications=[{
        "comparison_operator": "GREATER_THAN",
        "threshold": 100,
        "threshold_type": "PERCENTAGE",
        "notification_type": "FORECASTED",
        "subscriber_email_addresses": ["test@example.com"],
    }],
    tags={
        "Tag1": "Value1",
        "Tag2": "Value2",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var ec2 = new Aws.Budgets.Budget("ec2", new()
    {
        Name = "budget-ec2-monthly",
        BudgetType = "COST",
        LimitAmount = "1200",
        LimitUnit = "USD",
        TimePeriodEnd = "2087-06-15_00:00",
        TimePeriodStart = "2017-07-01_00:00",
        TimeUnit = "MONTHLY",
        CostFilters = new[]
        {
            new Aws.Budgets.Inputs.BudgetCostFilterArgs
            {
                Name = "Service",
                Values = new[]
                {
                    "Amazon Elastic Compute Cloud - Compute",
                },
            },
        },
        Notifications = new[]
        {
            new Aws.Budgets.Inputs.BudgetNotificationArgs
            {
                ComparisonOperator = "GREATER_THAN",
                Threshold = 100,
                ThresholdType = "PERCENTAGE",
                NotificationType = "FORECASTED",
                SubscriberEmailAddresses = new[]
                {
                    "test@example.com",
                },
            },
        },
        Tags = 
        {
            { "Tag1", "Value1" },
            { "Tag2", "Value2" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/budgets"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := budgets.NewBudget(ctx, "ec2", &budgets.BudgetArgs{
			Name:            pulumi.String("budget-ec2-monthly"),
			BudgetType:      pulumi.String("COST"),
			LimitAmount:     pulumi.String("1200"),
			LimitUnit:       pulumi.String("USD"),
			TimePeriodEnd:   pulumi.String("2087-06-15_00:00"),
			TimePeriodStart: pulumi.String("2017-07-01_00:00"),
			TimeUnit:        pulumi.String("MONTHLY"),
			CostFilters: budgets.BudgetCostFilterArray{
				&budgets.BudgetCostFilterArgs{
					Name: pulumi.String("Service"),
					Values: pulumi.StringArray{
						pulumi.String("Amazon Elastic Compute Cloud - Compute"),
					},
				},
			},
			Notifications: budgets.BudgetNotificationArray{
				&budgets.BudgetNotificationArgs{
					ComparisonOperator: pulumi.String("GREATER_THAN"),
					Threshold:          pulumi.Float64(100),
					ThresholdType:      pulumi.String("PERCENTAGE"),
					NotificationType:   pulumi.String("FORECASTED"),
					SubscriberEmailAddresses: pulumi.StringArray{
						pulumi.String("test@example.com"),
					},
				},
			},
			Tags: pulumi.StringMap{
				"Tag1": pulumi.String("Value1"),
				"Tag2": pulumi.String("Value2"),
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
import com.pulumi.aws.budgets.Budget;
import com.pulumi.aws.budgets.BudgetArgs;
import com.pulumi.aws.budgets.inputs.BudgetCostFilterArgs;
import com.pulumi.aws.budgets.inputs.BudgetNotificationArgs;
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
        var ec2 = new Budget("ec2", BudgetArgs.builder()
            .name("budget-ec2-monthly")
            .budgetType("COST")
            .limitAmount("1200")
            .limitUnit("USD")
            .timePeriodEnd("2087-06-15_00:00")
            .timePeriodStart("2017-07-01_00:00")
            .timeUnit("MONTHLY")
            .costFilters(BudgetCostFilterArgs.builder()
                .name("Service")
                .values("Amazon Elastic Compute Cloud - Compute")
                .build())
            .notifications(BudgetNotificationArgs.builder()
                .comparisonOperator("GREATER_THAN")
                .threshold(100)
                .thresholdType("PERCENTAGE")
                .notificationType("FORECASTED")
                .subscriberEmailAddresses("test@example.com")
                .build())
            .tags(Map.ofEntries(
                Map.entry("Tag1", "Value1"),
                Map.entry("Tag2", "Value2")
            ))
            .build());

    }
}
```
```yaml
resources:
  ec2:
    type: aws:budgets:Budget
    properties:
      name: budget-ec2-monthly
      budgetType: COST
      limitAmount: '1200'
      limitUnit: USD
      timePeriodEnd: 2087-06-15_00:00
      timePeriodStart: 2017-07-01_00:00
      timeUnit: MONTHLY
      costFilters:
        - name: Service
          values:
            - Amazon Elastic Compute Cloud - Compute
      notifications:
        - comparisonOperator: GREATER_THAN
          threshold: 100
          thresholdType: PERCENTAGE
          notificationType: FORECASTED
          subscriberEmailAddresses:
            - test@example.com
      tags:
        Tag1: Value1
        Tag2: Value2
```
<!--End PulumiCodeChooser -->

Create a budget for *$100*.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const cost = new aws.budgets.Budget("cost", {
    budgetType: "COST",
    limitAmount: "100",
    limitUnit: "USD",
});
```
```python
import pulumi
import pulumi_aws as aws

cost = aws.budgets.Budget("cost",
    budget_type="COST",
    limit_amount="100",
    limit_unit="USD")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var cost = new Aws.Budgets.Budget("cost", new()
    {
        BudgetType = "COST",
        LimitAmount = "100",
        LimitUnit = "USD",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/budgets"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := budgets.NewBudget(ctx, "cost", &budgets.BudgetArgs{
			BudgetType:  pulumi.String("COST"),
			LimitAmount: pulumi.String("100"),
			LimitUnit:   pulumi.String("USD"),
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
import com.pulumi.aws.budgets.Budget;
import com.pulumi.aws.budgets.BudgetArgs;
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
        var cost = new Budget("cost", BudgetArgs.builder()
            .budgetType("COST")
            .limitAmount("100")
            .limitUnit("USD")
            .build());

    }
}
```
```yaml
resources:
  cost:
    type: aws:budgets:Budget
    properties:
      budgetType: COST
      limitAmount: '100'
      limitUnit: USD
```
<!--End PulumiCodeChooser -->

Create a budget with planned budget limits.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const cost = new aws.budgets.Budget("cost", {plannedLimits: [
    {
        startTime: "2017-07-01_00:00",
        amount: "100",
        unit: "USD",
    },
    {
        startTime: "2017-08-01_00:00",
        amount: "200",
        unit: "USD",
    },
]});
```
```python
import pulumi
import pulumi_aws as aws

cost = aws.budgets.Budget("cost", planned_limits=[
    {
        "start_time": "2017-07-01_00:00",
        "amount": "100",
        "unit": "USD",
    },
    {
        "start_time": "2017-08-01_00:00",
        "amount": "200",
        "unit": "USD",
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
    var cost = new Aws.Budgets.Budget("cost", new()
    {
        PlannedLimits = new[]
        {
            new Aws.Budgets.Inputs.BudgetPlannedLimitArgs
            {
                StartTime = "2017-07-01_00:00",
                Amount = "100",
                Unit = "USD",
            },
            new Aws.Budgets.Inputs.BudgetPlannedLimitArgs
            {
                StartTime = "2017-08-01_00:00",
                Amount = "200",
                Unit = "USD",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/budgets"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := budgets.NewBudget(ctx, "cost", &budgets.BudgetArgs{
			PlannedLimits: budgets.BudgetPlannedLimitArray{
				&budgets.BudgetPlannedLimitArgs{
					StartTime: pulumi.String("2017-07-01_00:00"),
					Amount:    pulumi.String("100"),
					Unit:      pulumi.String("USD"),
				},
				&budgets.BudgetPlannedLimitArgs{
					StartTime: pulumi.String("2017-08-01_00:00"),
					Amount:    pulumi.String("200"),
					Unit:      pulumi.String("USD"),
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
import com.pulumi.aws.budgets.Budget;
import com.pulumi.aws.budgets.BudgetArgs;
import com.pulumi.aws.budgets.inputs.BudgetPlannedLimitArgs;
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
        var cost = new Budget("cost", BudgetArgs.builder()
            .plannedLimits(            
                BudgetPlannedLimitArgs.builder()
                    .startTime("2017-07-01_00:00")
                    .amount("100")
                    .unit("USD")
                    .build(),
                BudgetPlannedLimitArgs.builder()
                    .startTime("2017-08-01_00:00")
                    .amount("200")
                    .unit("USD")
                    .build())
            .build());

    }
}
```
```yaml
resources:
  cost:
    type: aws:budgets:Budget
    properties:
      plannedLimits:
        - startTime: 2017-07-01_00:00
          amount: '100'
          unit: USD
        - startTime: 2017-08-01_00:00
          amount: '200'
          unit: USD
```
<!--End PulumiCodeChooser -->

Create a budget for s3 with a limit of *3 GB* of storage.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const s3 = new aws.budgets.Budget("s3", {
    budgetType: "USAGE",
    limitAmount: "3",
    limitUnit: "GB",
});
```
```python
import pulumi
import pulumi_aws as aws

s3 = aws.budgets.Budget("s3",
    budget_type="USAGE",
    limit_amount="3",
    limit_unit="GB")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var s3 = new Aws.Budgets.Budget("s3", new()
    {
        BudgetType = "USAGE",
        LimitAmount = "3",
        LimitUnit = "GB",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/budgets"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := budgets.NewBudget(ctx, "s3", &budgets.BudgetArgs{
			BudgetType:  pulumi.String("USAGE"),
			LimitAmount: pulumi.String("3"),
			LimitUnit:   pulumi.String("GB"),
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
import com.pulumi.aws.budgets.Budget;
import com.pulumi.aws.budgets.BudgetArgs;
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
        var s3 = new Budget("s3", BudgetArgs.builder()
            .budgetType("USAGE")
            .limitAmount("3")
            .limitUnit("GB")
            .build());

    }
}
```
```yaml
resources:
  s3:
    type: aws:budgets:Budget
    properties:
      budgetType: USAGE
      limitAmount: '3'
      limitUnit: GB
```
<!--End PulumiCodeChooser -->

Create a Savings Plan Utilization Budget

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const savingsPlanUtilization = new aws.budgets.Budget("savings_plan_utilization", {
    budgetType: "SAVINGS_PLANS_UTILIZATION",
    limitAmount: "100.0",
    limitUnit: "PERCENTAGE",
    costTypes: {
        includeCredit: false,
        includeDiscount: false,
        includeOtherSubscription: false,
        includeRecurring: false,
        includeRefund: false,
        includeSubscription: true,
        includeSupport: false,
        includeTax: false,
        includeUpfront: false,
        useBlended: false,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

savings_plan_utilization = aws.budgets.Budget("savings_plan_utilization",
    budget_type="SAVINGS_PLANS_UTILIZATION",
    limit_amount="100.0",
    limit_unit="PERCENTAGE",
    cost_types={
        "include_credit": False,
        "include_discount": False,
        "include_other_subscription": False,
        "include_recurring": False,
        "include_refund": False,
        "include_subscription": True,
        "include_support": False,
        "include_tax": False,
        "include_upfront": False,
        "use_blended": False,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var savingsPlanUtilization = new Aws.Budgets.Budget("savings_plan_utilization", new()
    {
        BudgetType = "SAVINGS_PLANS_UTILIZATION",
        LimitAmount = "100.0",
        LimitUnit = "PERCENTAGE",
        CostTypes = new Aws.Budgets.Inputs.BudgetCostTypesArgs
        {
            IncludeCredit = false,
            IncludeDiscount = false,
            IncludeOtherSubscription = false,
            IncludeRecurring = false,
            IncludeRefund = false,
            IncludeSubscription = true,
            IncludeSupport = false,
            IncludeTax = false,
            IncludeUpfront = false,
            UseBlended = false,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/budgets"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := budgets.NewBudget(ctx, "savings_plan_utilization", &budgets.BudgetArgs{
			BudgetType:  pulumi.String("SAVINGS_PLANS_UTILIZATION"),
			LimitAmount: pulumi.String("100.0"),
			LimitUnit:   pulumi.String("PERCENTAGE"),
			CostTypes: &budgets.BudgetCostTypesArgs{
				IncludeCredit:            pulumi.Bool(false),
				IncludeDiscount:          pulumi.Bool(false),
				IncludeOtherSubscription: pulumi.Bool(false),
				IncludeRecurring:         pulumi.Bool(false),
				IncludeRefund:            pulumi.Bool(false),
				IncludeSubscription:      pulumi.Bool(true),
				IncludeSupport:           pulumi.Bool(false),
				IncludeTax:               pulumi.Bool(false),
				IncludeUpfront:           pulumi.Bool(false),
				UseBlended:               pulumi.Bool(false),
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
import com.pulumi.aws.budgets.Budget;
import com.pulumi.aws.budgets.BudgetArgs;
import com.pulumi.aws.budgets.inputs.BudgetCostTypesArgs;
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
        var savingsPlanUtilization = new Budget("savingsPlanUtilization", BudgetArgs.builder()
            .budgetType("SAVINGS_PLANS_UTILIZATION")
            .limitAmount("100.0")
            .limitUnit("PERCENTAGE")
            .costTypes(BudgetCostTypesArgs.builder()
                .includeCredit(false)
                .includeDiscount(false)
                .includeOtherSubscription(false)
                .includeRecurring(false)
                .includeRefund(false)
                .includeSubscription(true)
                .includeSupport(false)
                .includeTax(false)
                .includeUpfront(false)
                .useBlended(false)
                .build())
            .build());

    }
}
```
```yaml
resources:
  savingsPlanUtilization:
    type: aws:budgets:Budget
    name: savings_plan_utilization
    properties:
      budgetType: SAVINGS_PLANS_UTILIZATION
      limitAmount: '100.0'
      limitUnit: PERCENTAGE
      costTypes:
        includeCredit: false
        includeDiscount: false
        includeOtherSubscription: false
        includeRecurring: false
        includeRefund: false
        includeSubscription: true
        includeSupport: false
        includeTax: false
        includeUpfront: false
        useBlended: false
```
<!--End PulumiCodeChooser -->

Create a RI Utilization Budget

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const riUtilization = new aws.budgets.Budget("ri_utilization", {
    budgetType: "RI_UTILIZATION",
    limitAmount: "100.0",
    limitUnit: "PERCENTAGE",
    costTypes: {
        includeCredit: false,
        includeDiscount: false,
        includeOtherSubscription: false,
        includeRecurring: false,
        includeRefund: false,
        includeSubscription: true,
        includeSupport: false,
        includeTax: false,
        includeUpfront: false,
        useBlended: false,
    },
    costFilters: [{
        name: "Service",
        values: ["Amazon Relational Database Service"],
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

ri_utilization = aws.budgets.Budget("ri_utilization",
    budget_type="RI_UTILIZATION",
    limit_amount="100.0",
    limit_unit="PERCENTAGE",
    cost_types={
        "include_credit": False,
        "include_discount": False,
        "include_other_subscription": False,
        "include_recurring": False,
        "include_refund": False,
        "include_subscription": True,
        "include_support": False,
        "include_tax": False,
        "include_upfront": False,
        "use_blended": False,
    },
    cost_filters=[{
        "name": "Service",
        "values": ["Amazon Relational Database Service"],
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var riUtilization = new Aws.Budgets.Budget("ri_utilization", new()
    {
        BudgetType = "RI_UTILIZATION",
        LimitAmount = "100.0",
        LimitUnit = "PERCENTAGE",
        CostTypes = new Aws.Budgets.Inputs.BudgetCostTypesArgs
        {
            IncludeCredit = false,
            IncludeDiscount = false,
            IncludeOtherSubscription = false,
            IncludeRecurring = false,
            IncludeRefund = false,
            IncludeSubscription = true,
            IncludeSupport = false,
            IncludeTax = false,
            IncludeUpfront = false,
            UseBlended = false,
        },
        CostFilters = new[]
        {
            new Aws.Budgets.Inputs.BudgetCostFilterArgs
            {
                Name = "Service",
                Values = new[]
                {
                    "Amazon Relational Database Service",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/budgets"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := budgets.NewBudget(ctx, "ri_utilization", &budgets.BudgetArgs{
			BudgetType:  pulumi.String("RI_UTILIZATION"),
			LimitAmount: pulumi.String("100.0"),
			LimitUnit:   pulumi.String("PERCENTAGE"),
			CostTypes: &budgets.BudgetCostTypesArgs{
				IncludeCredit:            pulumi.Bool(false),
				IncludeDiscount:          pulumi.Bool(false),
				IncludeOtherSubscription: pulumi.Bool(false),
				IncludeRecurring:         pulumi.Bool(false),
				IncludeRefund:            pulumi.Bool(false),
				IncludeSubscription:      pulumi.Bool(true),
				IncludeSupport:           pulumi.Bool(false),
				IncludeTax:               pulumi.Bool(false),
				IncludeUpfront:           pulumi.Bool(false),
				UseBlended:               pulumi.Bool(false),
			},
			CostFilters: budgets.BudgetCostFilterArray{
				&budgets.BudgetCostFilterArgs{
					Name: pulumi.String("Service"),
					Values: pulumi.StringArray{
						pulumi.String("Amazon Relational Database Service"),
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
import com.pulumi.aws.budgets.Budget;
import com.pulumi.aws.budgets.BudgetArgs;
import com.pulumi.aws.budgets.inputs.BudgetCostTypesArgs;
import com.pulumi.aws.budgets.inputs.BudgetCostFilterArgs;
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
        var riUtilization = new Budget("riUtilization", BudgetArgs.builder()
            .budgetType("RI_UTILIZATION")
            .limitAmount("100.0")
            .limitUnit("PERCENTAGE")
            .costTypes(BudgetCostTypesArgs.builder()
                .includeCredit(false)
                .includeDiscount(false)
                .includeOtherSubscription(false)
                .includeRecurring(false)
                .includeRefund(false)
                .includeSubscription(true)
                .includeSupport(false)
                .includeTax(false)
                .includeUpfront(false)
                .useBlended(false)
                .build())
            .costFilters(BudgetCostFilterArgs.builder()
                .name("Service")
                .values("Amazon Relational Database Service")
                .build())
            .build());

    }
}
```
```yaml
resources:
  riUtilization:
    type: aws:budgets:Budget
    name: ri_utilization
    properties:
      budgetType: RI_UTILIZATION
      limitAmount: '100.0'
      limitUnit: PERCENTAGE
      costTypes:
        includeCredit: false
        includeDiscount: false
        includeOtherSubscription: false
        includeRecurring: false
        includeRefund: false
        includeSubscription: true
        includeSupport: false
        includeTax: false
        includeUpfront: false
        useBlended: false
      costFilters:
        - name: Service
          values:
            - Amazon Relational Database Service
```
<!--End PulumiCodeChooser -->

Create a Cost Filter using Resource Tags

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const cost = new aws.budgets.Budget("cost", {costFilters: [{
    name: "TagKeyValue",
    values: ["TagKey$TagValue"],
}]});
```
```python
import pulumi
import pulumi_aws as aws

cost = aws.budgets.Budget("cost", cost_filters=[{
    "name": "TagKeyValue",
    "values": ["TagKey$TagValue"],
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var cost = new Aws.Budgets.Budget("cost", new()
    {
        CostFilters = new[]
        {
            new Aws.Budgets.Inputs.BudgetCostFilterArgs
            {
                Name = "TagKeyValue",
                Values = new[]
                {
                    "TagKey$TagValue",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/budgets"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := budgets.NewBudget(ctx, "cost", &budgets.BudgetArgs{
			CostFilters: budgets.BudgetCostFilterArray{
				&budgets.BudgetCostFilterArgs{
					Name: pulumi.String("TagKeyValue"),
					Values: pulumi.StringArray{
						pulumi.String("TagKey$TagValue"),
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
import com.pulumi.aws.budgets.Budget;
import com.pulumi.aws.budgets.BudgetArgs;
import com.pulumi.aws.budgets.inputs.BudgetCostFilterArgs;
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
        var cost = new Budget("cost", BudgetArgs.builder()
            .costFilters(BudgetCostFilterArgs.builder()
                .name("TagKeyValue")
                .values("TagKey$TagValue")
                .build())
            .build());

    }
}
```
```yaml
resources:
  cost:
    type: aws:budgets:Budget
    properties:
      costFilters:
        - name: TagKeyValue
          values:
            - TagKey$TagValue
```
<!--End PulumiCodeChooser -->

Create a cost_filter using resource tags, obtaining the tag value from a variable

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const cost = new aws.budgets.Budget("cost", {costFilters: [{
    name: "TagKeyValue",
    values: [`TagKey${"$"}${tagValue}`],
}]});
```
```python
import pulumi
import pulumi_aws as aws

cost = aws.budgets.Budget("cost", cost_filters=[{
    "name": "TagKeyValue",
    "values": [f"TagKey{'$'}{tag_value}"],
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var cost = new Aws.Budgets.Budget("cost", new()
    {
        CostFilters = new[]
        {
            new Aws.Budgets.Inputs.BudgetCostFilterArgs
            {
                Name = "TagKeyValue",
                Values = new[]
                {
                    $"TagKey{"$"}{tagValue}",
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

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/budgets"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := budgets.NewBudget(ctx, "cost", &budgets.BudgetArgs{
			CostFilters: budgets.BudgetCostFilterArray{
				&budgets.BudgetCostFilterArgs{
					Name: pulumi.String("TagKeyValue"),
					Values: pulumi.StringArray{
						pulumi.Sprintf("TagKey%v%v", "$", tagValue),
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
import com.pulumi.aws.budgets.Budget;
import com.pulumi.aws.budgets.BudgetArgs;
import com.pulumi.aws.budgets.inputs.BudgetCostFilterArgs;
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
        var cost = new Budget("cost", BudgetArgs.builder()
            .costFilters(BudgetCostFilterArgs.builder()
                .name("TagKeyValue")
                .values(String.format("TagKey%s%s", "$",tagValue))
                .build())
            .build());

    }
}
```
```yaml
resources:
  cost:
    type: aws:budgets:Budget
    properties:
      costFilters:
        - name: TagKeyValue
          values:
            - TagKey$${tagValue}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import budgets using `AccountID:BudgetName`. For example:

```sh
$ pulumi import aws:budgets/budget:Budget myBudget 123456789012:myBudget
```
v
	accountIdB" cThe ID of the target account for budget. Will use current user's account_id by default if omitted.
“
autoAdjustData\BZ:X
V
budgetsBudgetAutoAdjustData5aws:budgets/BudgetAutoAdjustData:BudgetAutoAdjustDatabObject containing AutoAdjustData which determines the budget amount for an auto-adjusting budget.
E

budgetType" 3Whether this budget tracks monetary cost or usage.
ù
costFiltersRBP*N:L
J
budgetsBudgetCostFilter-aws:budgets/BudgetCostFilter:BudgetCostFilter:A list of CostFilter name/values pair to apply to budget.
ø
	costTypesMBK:I
G
budgetsBudgetCostTypes+aws:budgets/BudgetCostTypes:BudgetCostTypescObject containing CostTypes The types of cost included in a budget, such as tax and subscriptions.
N
limitAmountB" 9The amount of cost or usage being measured for a budget.
Ò
	limitUnitB" ›The unit of measurement used for the budget forecast, actual spend, or budget threshold, such as dollars or GB. See [Spend](http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/data-type-spend.html) documentation.
<
nameB" .The name of a budget. Unique within accounts.
P

namePrefixB" <The prefix of the name of a budget. Unique within accounts.
€
notificationsXBV*T:R
P
budgetsBudgetNotification1aws:budgets/BudgetNotification:BudgetNotificationpObject containing Budget Notifications. Can be used multiple times to define more than one budget notification.
î
plannedLimitsXBV*T:R
P
budgetsBudgetPlannedLimit1aws:budgets/BudgetPlannedLimit:BudgetPlannedLimit®Object containing Planned Budget Limits. Can be used multiple times to plan more than one budget limit. See [PlannedBudgetLimits](https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_Budget.html#awscostmanagement-Type-budgets_Budget-PlannedBudgetLimits) documentation.
Õ
tagsB2" ºMap of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
ê
timePeriodEndB" yThe end of the time period covered by the budget. There are no restrictions on the end date. Format: `2017-01-01_12:00`.
Ò
timePeriodStartB" ◊The start of the time period covered by the budget. If you don't specify a start date, AWS defaults to the start of your chosen time period. The start date must come before the end date. Format: `2017-01-01_12:00`.
¡
timeUnit" ∞The length of time until a budget resets the actual and forecasted spend. Valid values: `MONTHLY`, `QUARTERLY`, `ANNUALLY`, and `DAILY`.

The following arguments are optional:
"t
	accountId" cThe ID of the target account for budget. Will use current user's account_id by default if omitted.
""
arn" The ARN of the budget.
"“
autoAdjustData\BZ:X
V
budgetsBudgetAutoAdjustData5aws:budgets/BudgetAutoAdjustData:BudgetAutoAdjustDatabObject containing AutoAdjustData which determines the budget amount for an auto-adjusting budget.
"E

budgetType" 3Whether this budget tracks monetary cost or usage.
"õ
costFiltersP*N:L
J
budgetsBudgetCostFilter-aws:budgets/BudgetCostFilter:BudgetCostFilter:A list of CostFilter name/values pair to apply to budget.
"Ω
	costTypesK:I
G
budgetsBudgetCostTypes+aws:budgets/BudgetCostTypes:BudgetCostTypescObject containing CostTypes The types of cost included in a budget, such as tax and subscriptions.
"L
limitAmount" 9The amount of cost or usage being measured for a budget.
"Ô
	limitUnit" ›The unit of measurement used for the budget forecast, actual spend, or budget threshold, such as dollars or GB. See [Spend](http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/data-type-spend.html) documentation.
":
name" .The name of a budget. Unique within accounts.
"N

namePrefix" <The prefix of the name of a budget. Unique within accounts.
"€
notificationsXBV*T:R
P
budgetsBudgetNotification1aws:budgets/BudgetNotification:BudgetNotificationpObject containing Budget Notifications. Can be used multiple times to define more than one budget notification.
"î
plannedLimitsXBV*T:R
P
budgetsBudgetPlannedLimit1aws:budgets/BudgetPlannedLimit:BudgetPlannedLimit®Object containing Planned Budget Limits. Can be used multiple times to plan more than one budget limit. See [PlannedBudgetLimits](https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_Budget.html#awscostmanagement-Type-budgets_Budget-PlannedBudgetLimits) documentation.
"Õ
tagsB2" ºMap of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"ê
timePeriodEndB" yThe end of the time period covered by the budget. There are no restrictions on the end date. Format: `2017-01-01_12:00`.
"Ô
timePeriodStart" ◊The start of the time period covered by the budget. If you don't specify a start date, AWS defaults to the start of your chosen time period. The start date must come before the end date. Format: `2017-01-01_12:00`.
"¡
timeUnit" ∞The length of time until a budget resets the actual and forecasted spend. Valid values: `MONTHLY`, `QUARTERLY`, `ANNUALLY`, and `DAILY`.

The following arguments are optional:
*§ö
>
budgetsBudgetAction%aws:budgets/budgetAction:BudgetActionëÄProvides a budget action resource. Budget actions are cost savings controls that run either automatically on your behalf or by using a workflow approval process.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        actions: ["ec2:Describe*"],
        resources: ["*"],
    }],
});
const examplePolicy = new aws.iam.Policy("example", {
    name: "example",
    description: "My example policy",
    policy: example.then(example => example.json),
});
const current = aws.getPartition({});
const assumeRole = current.then(current => aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: [`budgets.${current.dnsSuffix}`],
        }],
        actions: ["sts:AssumeRole"],
    }],
}));
const exampleRole = new aws.iam.Role("example", {
    name: "example",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const exampleBudget = new aws.budgets.Budget("example", {
    name: "example",
    budgetType: "USAGE",
    limitAmount: "10.0",
    limitUnit: "dollars",
    timePeriodStart: "2006-01-02_15:04",
    timeUnit: "MONTHLY",
});
const exampleBudgetAction = new aws.budgets.BudgetAction("example", {
    budgetName: exampleBudget.name,
    actionType: "APPLY_IAM_POLICY",
    approvalModel: "AUTOMATIC",
    notificationType: "ACTUAL",
    executionRoleArn: exampleRole.arn,
    actionThreshold: {
        actionThresholdType: "ABSOLUTE_VALUE",
        actionThresholdValue: 100,
    },
    definition: {
        iamActionDefinition: {
            policyArn: examplePolicy.arn,
            roles: [exampleRole.name],
        },
    },
    subscribers: [{
        address: "example@example.example",
        subscriptionType: "EMAIL",
    }],
    tags: {
        Tag1: "Value1",
        Tag2: "Value2",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "actions": ["ec2:Describe*"],
    "resources": ["*"],
}])
example_policy = aws.iam.Policy("example",
    name="example",
    description="My example policy",
    policy=example.json)
current = aws.get_partition()
assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": [f"budgets.{current.dns_suffix}"],
    }],
    "actions": ["sts:AssumeRole"],
}])
example_role = aws.iam.Role("example",
    name="example",
    assume_role_policy=assume_role.json)
example_budget = aws.budgets.Budget("example",
    name="example",
    budget_type="USAGE",
    limit_amount="10.0",
    limit_unit="dollars",
    time_period_start="2006-01-02_15:04",
    time_unit="MONTHLY")
example_budget_action = aws.budgets.BudgetAction("example",
    budget_name=example_budget.name,
    action_type="APPLY_IAM_POLICY",
    approval_model="AUTOMATIC",
    notification_type="ACTUAL",
    execution_role_arn=example_role.arn,
    action_threshold={
        "action_threshold_type": "ABSOLUTE_VALUE",
        "action_threshold_value": 100,
    },
    definition={
        "iam_action_definition": {
            "policy_arn": example_policy.arn,
            "roles": [example_role.name],
        },
    },
    subscribers=[{
        "address": "example@example.example",
        "subscription_type": "EMAIL",
    }],
    tags={
        "Tag1": "Value1",
        "Tag2": "Value2",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
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
                Effect = "Allow",
                Actions = new[]
                {
                    "ec2:Describe*",
                },
                Resources = new[]
                {
                    "*",
                },
            },
        },
    });

    var examplePolicy = new Aws.Iam.Policy("example", new()
    {
        Name = "example",
        Description = "My example policy",
        PolicyDocument = example.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var current = Aws.GetPartition.Invoke();

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
                            $"budgets.{current.Apply(getPartitionResult => getPartitionResult.DnsSuffix)}",
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
        Name = "example",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var exampleBudget = new Aws.Budgets.Budget("example", new()
    {
        Name = "example",
        BudgetType = "USAGE",
        LimitAmount = "10.0",
        LimitUnit = "dollars",
        TimePeriodStart = "2006-01-02_15:04",
        TimeUnit = "MONTHLY",
    });

    var exampleBudgetAction = new Aws.Budgets.BudgetAction("example", new()
    {
        BudgetName = exampleBudget.Name,
        ActionType = "APPLY_IAM_POLICY",
        ApprovalModel = "AUTOMATIC",
        NotificationType = "ACTUAL",
        ExecutionRoleArn = exampleRole.Arn,
        ActionThreshold = new Aws.Budgets.Inputs.BudgetActionActionThresholdArgs
        {
            ActionThresholdType = "ABSOLUTE_VALUE",
            ActionThresholdValue = 100,
        },
        Definition = new Aws.Budgets.Inputs.BudgetActionDefinitionArgs
        {
            IamActionDefinition = new Aws.Budgets.Inputs.BudgetActionDefinitionIamActionDefinitionArgs
            {
                PolicyArn = examplePolicy.Arn,
                Roles = new[]
                {
                    exampleRole.Name,
                },
            },
        },
        Subscribers = new[]
        {
            new Aws.Budgets.Inputs.BudgetActionSubscriberArgs
            {
                Address = "example@example.example",
                SubscriptionType = "EMAIL",
            },
        },
        Tags = 
        {
            { "Tag1", "Value1" },
            { "Tag2", "Value2" },
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/budgets"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Actions: []string{
						"ec2:Describe*",
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
		examplePolicy, err := iam.NewPolicy(ctx, "example", &iam.PolicyArgs{
			Name:        pulumi.String("example"),
			Description: pulumi.String("My example policy"),
			Policy:      pulumi.String(example.Json),
		})
		if err != nil {
			return err
		}
		current, err := aws.GetPartition(ctx, &aws.GetPartitionArgs{}, nil)
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
								fmt.Sprintf("budgets.%v", current.DnsSuffix),
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
			Name:             pulumi.String("example"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		exampleBudget, err := budgets.NewBudget(ctx, "example", &budgets.BudgetArgs{
			Name:            pulumi.String("example"),
			BudgetType:      pulumi.String("USAGE"),
			LimitAmount:     pulumi.String("10.0"),
			LimitUnit:       pulumi.String("dollars"),
			TimePeriodStart: pulumi.String("2006-01-02_15:04"),
			TimeUnit:        pulumi.String("MONTHLY"),
		})
		if err != nil {
			return err
		}
		_, err = budgets.NewBudgetAction(ctx, "example", &budgets.BudgetActionArgs{
			BudgetName:       exampleBudget.Name,
			ActionType:       pulumi.String("APPLY_IAM_POLICY"),
			ApprovalModel:    pulumi.String("AUTOMATIC"),
			NotificationType: pulumi.String("ACTUAL"),
			ExecutionRoleArn: exampleRole.Arn,
			ActionThreshold: &budgets.BudgetActionActionThresholdArgs{
				ActionThresholdType:  pulumi.String("ABSOLUTE_VALUE"),
				ActionThresholdValue: pulumi.Float64(100),
			},
			Definition: &budgets.BudgetActionDefinitionArgs{
				IamActionDefinition: &budgets.BudgetActionDefinitionIamActionDefinitionArgs{
					PolicyArn: examplePolicy.Arn,
					Roles: pulumi.StringArray{
						exampleRole.Name,
					},
				},
			},
			Subscribers: budgets.BudgetActionSubscriberArray{
				&budgets.BudgetActionSubscriberArgs{
					Address:          pulumi.String("example@example.example"),
					SubscriptionType: pulumi.String("EMAIL"),
				},
			},
			Tags: pulumi.StringMap{
				"Tag1": pulumi.String("Value1"),
				"Tag2": pulumi.String("Value2"),
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
import com.pulumi.aws.iam.Policy;
import com.pulumi.aws.iam.PolicyArgs;
import com.pulumi.aws.AwsFunctions;
import com.pulumi.aws.inputs.GetPartitionArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.budgets.Budget;
import com.pulumi.aws.budgets.BudgetArgs;
import com.pulumi.aws.budgets.BudgetAction;
import com.pulumi.aws.budgets.BudgetActionArgs;
import com.pulumi.aws.budgets.inputs.BudgetActionActionThresholdArgs;
import com.pulumi.aws.budgets.inputs.BudgetActionDefinitionArgs;
import com.pulumi.aws.budgets.inputs.BudgetActionDefinitionIamActionDefinitionArgs;
import com.pulumi.aws.budgets.inputs.BudgetActionSubscriberArgs;
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
                .effect("Allow")
                .actions("ec2:Describe*")
                .resources("*")
                .build())
            .build());

        var examplePolicy = new Policy("examplePolicy", PolicyArgs.builder()
            .name("example")
            .description("My example policy")
            .policy(example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        final var current = AwsFunctions.getPartition();

        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers(String.format("budgets.%s", current.applyValue(getPartitionResult -> getPartitionResult.dnsSuffix())))
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var exampleRole = new Role("exampleRole", RoleArgs.builder()
            .name("example")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var exampleBudget = new Budget("exampleBudget", BudgetArgs.builder()
            .name("example")
            .budgetType("USAGE")
            .limitAmount("10.0")
            .limitUnit("dollars")
            .timePeriodStart("2006-01-02_15:04")
            .timeUnit("MONTHLY")
            .build());

        var exampleBudgetAction = new BudgetAction("exampleBudgetAction", BudgetActionArgs.builder()
            .budgetName(exampleBudget.name())
            .actionType("APPLY_IAM_POLICY")
            .approvalModel("AUTOMATIC")
            .notificationType("ACTUAL")
            .executionRoleArn(exampleRole.arn())
            .actionThreshold(BudgetActionActionThresholdArgs.builder()
                .actionThresholdType("ABSOLUTE_VALUE")
                .actionThresholdValue(100)
                .build())
            .definition(BudgetActionDefinitionArgs.builder()
                .iamActionDefinition(BudgetActionDefinitionIamActionDefinitionArgs.builder()
                    .policyArn(examplePolicy.arn())
                    .roles(exampleRole.name())
                    .build())
                .build())
            .subscribers(BudgetActionSubscriberArgs.builder()
                .address("example@example.example")
                .subscriptionType("EMAIL")
                .build())
            .tags(Map.ofEntries(
                Map.entry("Tag1", "Value1"),
                Map.entry("Tag2", "Value2")
            ))
            .build());

    }
}
```
```yaml
resources:
  exampleBudgetAction:
    type: aws:budgets:BudgetAction
    name: example
    properties:
      budgetName: ${exampleBudget.name}
      actionType: APPLY_IAM_POLICY
      approvalModel: AUTOMATIC
      notificationType: ACTUAL
      executionRoleArn: ${exampleRole.arn}
      actionThreshold:
        actionThresholdType: ABSOLUTE_VALUE
        actionThresholdValue: 100
      definition:
        iamActionDefinition:
          policyArn: ${examplePolicy.arn}
          roles:
            - ${exampleRole.name}
      subscribers:
        - address: example@example.example
          subscriptionType: EMAIL
      tags:
        Tag1: Value1
        Tag2: Value2
  examplePolicy:
    type: aws:iam:Policy
    name: example
    properties:
      name: example
      description: My example policy
      policy: ${example.json}
  exampleRole:
    type: aws:iam:Role
    name: example
    properties:
      name: example
      assumeRolePolicy: ${assumeRole.json}
  exampleBudget:
    type: aws:budgets:Budget
    name: example
    properties:
      name: example
      budgetType: USAGE
      limitAmount: '10.0'
      limitUnit: dollars
      timePeriodStart: 2006-01-02_15:04
      timeUnit: MONTHLY
variables:
  example:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            actions:
              - ec2:Describe*
            resources:
              - '*'
  current:
    fn::invoke:
      function: aws:getPartition
      arguments: {}
  assumeRole:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: Service
                identifiers:
                  - budgets.${current.dnsSuffix}
            actions:
              - sts:AssumeRole
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import budget actions using `AccountID:ActionID:BudgetName`. For example:

```sh
$ pulumi import aws:budgets/budgetAction:BudgetAction myBudget 123456789012:some-id:myBudget
```
v
	accountIdB" cThe ID of the target account for budget. Will use current user's account_id by default if omitted.
ø
actionThresholdo:m
k
budgetsBudgetActionActionThresholdCaws:budgets/BudgetActionActionThreshold:BudgetActionActionThreshold;The trigger threshold of the action. See Action Threshold.
ˆ

actionType" „The type of action. This defines the type of tasks that can be carried out by this action. This field also determines the format for definition. Valid values are `APPLY_IAM_POLICY`, `APPLY_SCP_POLICY`, and `RUN_SSM_DOCUMENTS`.
Å
approvalModel" lThis specifies if the action needs manual or automatic approval. Valid values are `AUTOMATIC` and `MANUAL`.
(

budgetName" The name of a budget.
Ø

definition`:^
\
budgetsBudgetActionDefinition9aws:budgets/BudgetActionDefinition:BudgetActionDefinition?Specifies all of the type-specific parameters. See Definition.
{
executionRoleArn" cThe role passed for action execution and reversion. Roles and actions must be in the same account.
_
notificationType" GThe type of a notification. Valid values are `ACTUAL` or `FORECASTED`.
ö
subscribersb*`:^
\
budgetsBudgetActionSubscriber9aws:budgets/BudgetActionSubscriber:BudgetActionSubscriber'A list of subscribers. See Subscriber.
Õ
tagsB2" ºMap of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"t
	accountId" cThe ID of the target account for budget. Will use current user's account_id by default if omitted.
"-
actionId" The id of the budget action.
"ø
actionThresholdo:m
k
budgetsBudgetActionActionThresholdCaws:budgets/BudgetActionActionThreshold:BudgetActionActionThreshold;The trigger threshold of the action. See Action Threshold.
"ˆ

actionType" „The type of action. This defines the type of tasks that can be carried out by this action. This field also determines the format for definition. Valid values are `APPLY_IAM_POLICY`, `APPLY_SCP_POLICY`, and `RUN_SSM_DOCUMENTS`.
"Å
approvalModel" lThis specifies if the action needs manual or automatic approval. Valid values are `AUTOMATIC` and `MANUAL`.
")
arn" The ARN of the budget action.
"(

budgetName" The name of a budget.
"Ø

definition`:^
\
budgetsBudgetActionDefinition9aws:budgets/BudgetActionDefinition:BudgetActionDefinition?Specifies all of the type-specific parameters. See Definition.
"{
executionRoleArn" cThe role passed for action execution and reversion. Roles and actions must be in the same account.
"_
notificationType" GThe type of a notification. Valid values are `ACTUAL` or `FORECASTED`.
"/
status" !The status of the budget action.
"ö
subscribersb*`:^
\
budgetsBudgetActionSubscriber9aws:budgets/BudgetActionSubscriber:BudgetActionSubscriber'A list of subscribers. See Subscriber.
"Õ
tagsB2" ºMap of tags assigned to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*ƒ
T
cfgAggregateAuthorization5aws:cfg/aggregateAuthorization:AggregateAuthorization•Manages an AWS Config Aggregate Authorization

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cfg.AggregateAuthorization("example", {
    accountId: "123456789012",
    region: "eu-west-2",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cfg.AggregateAuthorization("example",
    account_id="123456789012",
    region="eu-west-2")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cfg.AggregateAuthorization("example", new()
    {
        AccountId = "123456789012",
        Region = "eu-west-2",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cfg.NewAggregateAuthorization(ctx, "example", &cfg.AggregateAuthorizationArgs{
			AccountId: pulumi.String("123456789012"),
			Region:    pulumi.String("eu-west-2"),
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
import com.pulumi.aws.cfg.AggregateAuthorization;
import com.pulumi.aws.cfg.AggregateAuthorizationArgs;
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
        var example = new AggregateAuthorization("example", AggregateAuthorizationArgs.builder()
            .accountId("123456789012")
            .region("eu-west-2")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cfg:AggregateAuthorization
    properties:
      accountId: '123456789012'
      region: eu-west-2
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Config aggregate authorizations using `account_id:region`. For example:

```sh
$ pulumi import aws:cfg/aggregateAuthorization:AggregateAuthorization example 123456789012:us-east-1
```

	accountId" Account ID

region" Region
–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"
	accountId" Account ID
"(
arn" The ARN of the authorization
"
region" Region
"–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*Úo
W
cfgConfigurationAggregator7aws:cfg/configurationAggregator:ConfigurationAggregator≥_Manages an AWS Config Configuration Aggregator

## Example Usage

### Account Based Aggregation

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const account = new aws.cfg.ConfigurationAggregator("account", {
    name: "example",
    accountAggregationSource: {
        accountIds: ["123456789012"],
        regions: ["us-west-2"],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

account = aws.cfg.ConfigurationAggregator("account",
    name="example",
    account_aggregation_source={
        "account_ids": ["123456789012"],
        "regions": ["us-west-2"],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var account = new Aws.Cfg.ConfigurationAggregator("account", new()
    {
        Name = "example",
        AccountAggregationSource = new Aws.Cfg.Inputs.ConfigurationAggregatorAccountAggregationSourceArgs
        {
            AccountIds = new[]
            {
                "123456789012",
            },
            Regions = new[]
            {
                "us-west-2",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cfg.NewConfigurationAggregator(ctx, "account", &cfg.ConfigurationAggregatorArgs{
			Name: pulumi.String("example"),
			AccountAggregationSource: &cfg.ConfigurationAggregatorAccountAggregationSourceArgs{
				AccountIds: pulumi.StringArray{
					pulumi.String("123456789012"),
				},
				Regions: pulumi.StringArray{
					pulumi.String("us-west-2"),
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
import com.pulumi.aws.cfg.ConfigurationAggregator;
import com.pulumi.aws.cfg.ConfigurationAggregatorArgs;
import com.pulumi.aws.cfg.inputs.ConfigurationAggregatorAccountAggregationSourceArgs;
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
        var account = new ConfigurationAggregator("account", ConfigurationAggregatorArgs.builder()
            .name("example")
            .accountAggregationSource(ConfigurationAggregatorAccountAggregationSourceArgs.builder()
                .accountIds("123456789012")
                .regions("us-west-2")
                .build())
            .build());

    }
}
```
```yaml
resources:
  account:
    type: aws:cfg:ConfigurationAggregator
    properties:
      name: example
      accountAggregationSource:
        accountIds:
          - '123456789012'
        regions:
          - us-west-2
```
<!--End PulumiCodeChooser -->

### Organization Based Aggregation

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["config.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const organizationRole = new aws.iam.Role("organization", {
    name: "example",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const organizationRolePolicyAttachment = new aws.iam.RolePolicyAttachment("organization", {
    role: organizationRole.name,
    policyArn: "arn:aws:iam::aws:policy/service-role/AWSConfigRoleForOrganizations",
});
const organization = new aws.cfg.ConfigurationAggregator("organization", {
    name: "example",
    organizationAggregationSource: {
        allRegions: true,
        roleArn: organizationRole.arn,
    },
}, {
    dependsOn: [organizationRolePolicyAttachment],
});
```
```python
import pulumi
import pulumi_aws as aws

assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["config.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
organization_role = aws.iam.Role("organization",
    name="example",
    assume_role_policy=assume_role.json)
organization_role_policy_attachment = aws.iam.RolePolicyAttachment("organization",
    role=organization_role.name,
    policy_arn="arn:aws:iam::aws:policy/service-role/AWSConfigRoleForOrganizations")
organization = aws.cfg.ConfigurationAggregator("organization",
    name="example",
    organization_aggregation_source={
        "all_regions": True,
        "role_arn": organization_role.arn,
    },
    opts = pulumi.ResourceOptions(depends_on=[organization_role_policy_attachment]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
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
                            "config.amazonaws.com",
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

    var organizationRole = new Aws.Iam.Role("organization", new()
    {
        Name = "example",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var organizationRolePolicyAttachment = new Aws.Iam.RolePolicyAttachment("organization", new()
    {
        Role = organizationRole.Name,
        PolicyArn = "arn:aws:iam::aws:policy/service-role/AWSConfigRoleForOrganizations",
    });

    var organization = new Aws.Cfg.ConfigurationAggregator("organization", new()
    {
        Name = "example",
        OrganizationAggregationSource = new Aws.Cfg.Inputs.ConfigurationAggregatorOrganizationAggregationSourceArgs
        {
            AllRegions = true,
            RoleArn = organizationRole.Arn,
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            organizationRolePolicyAttachment,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		assumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"config.amazonaws.com",
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
		organizationRole, err := iam.NewRole(ctx, "organization", &iam.RoleArgs{
			Name:             pulumi.String("example"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		organizationRolePolicyAttachment, err := iam.NewRolePolicyAttachment(ctx, "organization", &iam.RolePolicyAttachmentArgs{
			Role:      organizationRole.Name,
			PolicyArn: pulumi.String("arn:aws:iam::aws:policy/service-role/AWSConfigRoleForOrganizations"),
		})
		if err != nil {
			return err
		}
		_, err = cfg.NewConfigurationAggregator(ctx, "organization", &cfg.ConfigurationAggregatorArgs{
			Name: pulumi.String("example"),
			OrganizationAggregationSource: &cfg.ConfigurationAggregatorOrganizationAggregationSourceArgs{
				AllRegions: pulumi.Bool(true),
				RoleArn:    organizationRole.Arn,
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			organizationRolePolicyAttachment,
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
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.iam.RolePolicyAttachment;
import com.pulumi.aws.iam.RolePolicyAttachmentArgs;
import com.pulumi.aws.cfg.ConfigurationAggregator;
import com.pulumi.aws.cfg.ConfigurationAggregatorArgs;
import com.pulumi.aws.cfg.inputs.ConfigurationAggregatorOrganizationAggregationSourceArgs;
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
        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("config.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var organizationRole = new Role("organizationRole", RoleArgs.builder()
            .name("example")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var organizationRolePolicyAttachment = new RolePolicyAttachment("organizationRolePolicyAttachment", RolePolicyAttachmentArgs.builder()
            .role(organizationRole.name())
            .policyArn("arn:aws:iam::aws:policy/service-role/AWSConfigRoleForOrganizations")
            .build());

        var organization = new ConfigurationAggregator("organization", ConfigurationAggregatorArgs.builder()
            .name("example")
            .organizationAggregationSource(ConfigurationAggregatorOrganizationAggregationSourceArgs.builder()
                .allRegions(true)
                .roleArn(organizationRole.arn())
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(organizationRolePolicyAttachment)
                .build());

    }
}
```
```yaml
resources:
  organization:
    type: aws:cfg:ConfigurationAggregator
    properties:
      name: example
      organizationAggregationSource:
        allRegions: true
        roleArn: ${organizationRole.arn}
    options:
      dependsOn:
        - ${organizationRolePolicyAttachment}
  organizationRole:
    type: aws:iam:Role
    name: organization
    properties:
      name: example
      assumeRolePolicy: ${assumeRole.json}
  organizationRolePolicyAttachment:
    type: aws:iam:RolePolicyAttachment
    name: organization
    properties:
      role: ${organizationRole.name}
      policyArn: arn:aws:iam::aws:policy/service-role/AWSConfigRoleForOrganizations
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
                  - config.amazonaws.com
            actions:
              - sts:AssumeRole
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Configuration Aggregators using the name. For example:

```sh
$ pulumi import aws:cfg/configurationAggregator:ConfigurationAggregator example foo
```
â
accountAggregationSource®B•:¢
ü
cfg/ConfigurationAggregatorAccountAggregationSourcegaws:cfg/ConfigurationAggregatorAccountAggregationSource:ConfigurationAggregatorAccountAggregationSourceBThe account(s) to aggregate config data from as documented below.
8
nameB" *The name of the configuration aggregator.
ü
organizationAggregationSource∑B¥:±
Æ
cfg4ConfigurationAggregatorOrganizationAggregationSourceqaws:cfg/ConfigurationAggregatorOrganizationAggregationSource:ConfigurationAggregatorOrganizationAggregationSourceDThe organization to aggregate config data from as documented below.
Æ
tagsB2" ùA map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.

Either `account_aggregation_source` or `organization_aggregation_source` must be specified.
"â
accountAggregationSource®B•:¢
ü
cfg/ConfigurationAggregatorAccountAggregationSourcegaws:cfg/ConfigurationAggregatorAccountAggregationSource:ConfigurationAggregatorAccountAggregationSourceBThe account(s) to aggregate config data from as documented below.
"%
arn" The ARN of the aggregator
"6
name" *The name of the configuration aggregator.
"ü
organizationAggregationSource∑B¥:±
Æ
cfg4ConfigurationAggregatorOrganizationAggregationSourceqaws:cfg/ConfigurationAggregatorOrganizationAggregationSource:ConfigurationAggregatorOrganizationAggregationSourceDThe organization to aggregate config data from as documented below.
"Æ
tagsB2" ùA map of tags to assign to the resource. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.

Either `account_aggregation_source` or `organization_aggregation_source` must be specified.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*…z
?
cfgConformancePack'aws:cfg/conformancePack:ConformancePack¥fManages a Config Conformance Pack. More information about this collection of Config rules and remediation actions can be found in the
[Conformance Packs](https://docs.aws.amazon.com/config/latest/developerguide/conformance-packs.html) documentation.
Sample Conformance Pack templates may be found in the
[AWS Config Rules Repository](https://github.com/awslabs/aws-config-rules/tree/master/aws-config-conformance-packs).

> **NOTE:** The account must have a Configuration Recorder with proper IAM permissions before the Conformance Pack will
successfully create or update. See also the
`aws.cfg.Recorder` resource.

## Example Usage

### Template Body

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cfg.ConformancePack("example", {
    name: "example",
    inputParameters: [{
        parameterName: "AccessKeysRotatedParameterMaxAccessKeyAge",
        parameterValue: "90",
    }],
    templateBody: `Parameters:
  AccessKeysRotatedParameterMaxAccessKeyAge:
    Type: String
Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
`,
}, {
    dependsOn: [exampleAwsConfigConfigurationRecorder],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cfg.ConformancePack("example",
    name="example",
    input_parameters=[{
        "parameter_name": "AccessKeysRotatedParameterMaxAccessKeyAge",
        "parameter_value": "90",
    }],
    template_body="""Parameters:
  AccessKeysRotatedParameterMaxAccessKeyAge:
    Type: String
Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
""",
    opts = pulumi.ResourceOptions(depends_on=[example_aws_config_configuration_recorder]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cfg.ConformancePack("example", new()
    {
        Name = "example",
        InputParameters = new[]
        {
            new Aws.Cfg.Inputs.ConformancePackInputParameterArgs
            {
                ParameterName = "AccessKeysRotatedParameterMaxAccessKeyAge",
                ParameterValue = "90",
            },
        },
        TemplateBody = @"Parameters:
  AccessKeysRotatedParameterMaxAccessKeyAge:
    Type: String
Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsConfigConfigurationRecorder,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cfg.NewConformancePack(ctx, "example", &cfg.ConformancePackArgs{
			Name: pulumi.String("example"),
			InputParameters: cfg.ConformancePackInputParameterArray{
				&cfg.ConformancePackInputParameterArgs{
					ParameterName:  pulumi.String("AccessKeysRotatedParameterMaxAccessKeyAge"),
					ParameterValue: pulumi.String("90"),
				},
			},
			TemplateBody: pulumi.String(`Parameters:
  AccessKeysRotatedParameterMaxAccessKeyAge:
    Type: String
Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
`),
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsConfigConfigurationRecorder,
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
import com.pulumi.aws.cfg.ConformancePack;
import com.pulumi.aws.cfg.ConformancePackArgs;
import com.pulumi.aws.cfg.inputs.ConformancePackInputParameterArgs;
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
        var example = new ConformancePack("example", ConformancePackArgs.builder()
            .name("example")
            .inputParameters(ConformancePackInputParameterArgs.builder()
                .parameterName("AccessKeysRotatedParameterMaxAccessKeyAge")
                .parameterValue("90")
                .build())
            .templateBody("""
Parameters:
  AccessKeysRotatedParameterMaxAccessKeyAge:
    Type: String
Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
            """)
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsConfigConfigurationRecorder)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cfg:ConformancePack
    properties:
      name: example
      inputParameters:
        - parameterName: AccessKeysRotatedParameterMaxAccessKeyAge
          parameterValue: '90'
      templateBody: |
        Parameters:
          AccessKeysRotatedParameterMaxAccessKeyAge:
            Type: String
        Resources:
          IAMPasswordPolicy:
            Properties:
              ConfigRuleName: IAMPasswordPolicy
              Source:
                Owner: AWS
                SourceIdentifier: IAM_PASSWORD_POLICY
            Type: AWS::Config::ConfigRule
    options:
      dependsOn:
        - ${exampleAwsConfigConfigurationRecorder}
```
<!--End PulumiCodeChooser -->

### Template S3 URI

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleBucketV2 = new aws.s3.BucketV2("example", {bucket: "example"});
const exampleBucketObjectv2 = new aws.s3.BucketObjectv2("example", {
    bucket: exampleBucketV2.id,
    key: "example-key",
    content: `Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
`,
});
const example = new aws.cfg.ConformancePack("example", {
    name: "example",
    templateS3Uri: pulumi.interpolate`s3://${exampleBucketV2.bucket}/${exampleBucketObjectv2.key}`,
}, {
    dependsOn: [exampleAwsConfigConfigurationRecorder],
});
```
```python
import pulumi
import pulumi_aws as aws

example_bucket_v2 = aws.s3.BucketV2("example", bucket="example")
example_bucket_objectv2 = aws.s3.BucketObjectv2("example",
    bucket=example_bucket_v2.id,
    key="example-key",
    content="""Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
""")
example = aws.cfg.ConformancePack("example",
    name="example",
    template_s3_uri=pulumi.Output.all(
        bucket=example_bucket_v2.bucket,
        key=example_bucket_objectv2.key
).apply(lambda resolved_outputs: f"s3://{resolved_outputs['bucket']}/{resolved_outputs['key']}")
,
    opts = pulumi.ResourceOptions(depends_on=[example_aws_config_configuration_recorder]))
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
        Bucket = "example",
    });

    var exampleBucketObjectv2 = new Aws.S3.BucketObjectv2("example", new()
    {
        Bucket = exampleBucketV2.Id,
        Key = "example-key",
        Content = @"Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
",
    });

    var example = new Aws.Cfg.ConformancePack("example", new()
    {
        Name = "example",
        TemplateS3Uri = Output.Tuple(exampleBucketV2.Bucket, exampleBucketObjectv2.Key).Apply(values =>
        {
            var bucket = values.Item1;
            var key = values.Item2;
            return $"s3://{bucket}/{key}";
        }),
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsConfigConfigurationRecorder,
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleBucketV2, err := s3.NewBucketV2(ctx, "example", &s3.BucketV2Args{
			Bucket: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		exampleBucketObjectv2, err := s3.NewBucketObjectv2(ctx, "example", &s3.BucketObjectv2Args{
			Bucket: exampleBucketV2.ID(),
			Key:    pulumi.String("example-key"),
			Content: pulumi.String(`Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
`),
		})
		if err != nil {
			return err
		}
		_, err = cfg.NewConformancePack(ctx, "example", &cfg.ConformancePackArgs{
			Name: pulumi.String("example"),
			TemplateS3Uri: pulumi.All(exampleBucketV2.Bucket, exampleBucketObjectv2.Key).ApplyT(func(_args []interface{}) (string, error) {
				bucket := _args[0].(string)
				key := _args[1].(string)
				return fmt.Sprintf("s3://%v/%v", bucket, key), nil
			}).(pulumi.StringOutput),
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsConfigConfigurationRecorder,
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
import com.pulumi.aws.cfg.ConformancePack;
import com.pulumi.aws.cfg.ConformancePackArgs;
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
        var exampleBucketV2 = new BucketV2("exampleBucketV2", BucketV2Args.builder()
            .bucket("example")
            .build());

        var exampleBucketObjectv2 = new BucketObjectv2("exampleBucketObjectv2", BucketObjectv2Args.builder()
            .bucket(exampleBucketV2.id())
            .key("example-key")
            .content("""
Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
            """)
            .build());

        var example = new ConformancePack("example", ConformancePackArgs.builder()
            .name("example")
            .templateS3Uri(Output.tuple(exampleBucketV2.bucket(), exampleBucketObjectv2.key()).applyValue(values -> {
                var bucket = values.t1;
                var key = values.t2;
                return String.format("s3://%s/%s", bucket,key);
            }))
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleAwsConfigConfigurationRecorder)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cfg:ConformancePack
    properties:
      name: example
      templateS3Uri: s3://${exampleBucketV2.bucket}/${exampleBucketObjectv2.key}
    options:
      dependsOn:
        - ${exampleAwsConfigConfigurationRecorder}
  exampleBucketV2:
    type: aws:s3:BucketV2
    name: example
    properties:
      bucket: example
  exampleBucketObjectv2:
    type: aws:s3:BucketObjectv2
    name: example
    properties:
      bucket: ${exampleBucketV2.id}
      key: example-key
      content: |
        Resources:
          IAMPasswordPolicy:
            Properties:
              ConfigRuleName: IAMPasswordPolicy
              Source:
                Owner: AWS
                SourceIdentifier: IAM_PASSWORD_POLICY
            Type: AWS::Config::ConfigRule
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Config Conformance Packs using the `name`. For example:

```sh
$ pulumi import aws:cfg/conformancePack:ConformancePack example example
```
u
deliveryS3BucketB" [Amazon S3 bucket where AWS Config stores conformance pack templates. Maximum length of 63.
Z
deliveryS3KeyPrefixB" =The prefix for the Amazon S3 bucket. Maximum length of 1024.
ä
inputParametersqBo*m:k
i
cfgConformancePackInputParameterCaws:cfg/ConformancePackInputParameter:ConformancePackInputParameterÉSet of configuration blocks describing input parameters passed to the conformance pack template. Documented below. When configured, the parameters must also be included in the `template_body` or in the template stored in Amazon S3 if using `template_s3_uri`.
à
nameB" zThe name of the conformance pack. Must begin with a letter and contain from 1 to 256 alphanumeric characters and hyphens.
ù
templateBodyB" ÜA string containing full conformance pack template body. Maximum length of 51200. Drift detection is not possible with this argument.
ª
templateS3UriB" £Location of file, e.g., `s3://bucketname/prefix`, containing the template body. The uri must point to the conformance pack template that is located in an Amazon S3 bucket in the same region as the conformance pack. Maximum length of 1024. Drift detection is not possible with this argument.
"?
arn" 4Amazon Resource Name (ARN) of the conformance pack.
"u
deliveryS3BucketB" [Amazon S3 bucket where AWS Config stores conformance pack templates. Maximum length of 63.
"Z
deliveryS3KeyPrefixB" =The prefix for the Amazon S3 bucket. Maximum length of 1024.
"ä
inputParametersqBo*m:k
i
cfgConformancePackInputParameterCaws:cfg/ConformancePackInputParameter:ConformancePackInputParameterÉSet of configuration blocks describing input parameters passed to the conformance pack template. Documented below. When configured, the parameters must also be included in the `template_body` or in the template stored in Amazon S3 if using `template_s3_uri`.
"Ü
name" zThe name of the conformance pack. Must begin with a letter and contain from 1 to 256 alphanumeric characters and hyphens.
"ù
templateBodyB" ÜA string containing full conformance pack template body. Maximum length of 51200. Drift detection is not possible with this argument.
"ª
templateS3UriB" £Location of file, e.g., `s3://bucketname/prefix`, containing the template body. The uri must point to the conformance pack template that is located in an Amazon S3 bucket in the same region as the conformance pack. Maximum length of 1024. Drift detection is not possible with this argument.
*ùf
?
cfgDeliveryChannel'aws:cfg/deliveryChannel:DeliveryChannel’YProvides an AWS Config Delivery Channel.

> **Note:** Delivery Channel requires a Configuration Recorder to be present. Use of `depends_on` (as shown below) is recommended to avoid race conditions.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const b = new aws.s3.BucketV2("b", {
    bucket: "example-awsconfig",
    forceDestroy: true,
});
const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["config.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const r = new aws.iam.Role("r", {
    name: "awsconfig-example",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const fooRecorder = new aws.cfg.Recorder("foo", {
    name: "example",
    roleArn: r.arn,
});
const foo = new aws.cfg.DeliveryChannel("foo", {
    name: "example",
    s3BucketName: b.bucket,
}, {
    dependsOn: [fooRecorder],
});
const p = aws.iam.getPolicyDocumentOutput({
    statements: [{
        effect: "Allow",
        actions: ["s3:*"],
        resources: [
            b.arn,
            pulumi.interpolate`${b.arn}/*`,
        ],
    }],
});
const pRolePolicy = new aws.iam.RolePolicy("p", {
    name: "awsconfig-example",
    role: r.id,
    policy: p.apply(p => p.json),
});
```
```python
import pulumi
import pulumi_aws as aws

b = aws.s3.BucketV2("b",
    bucket="example-awsconfig",
    force_destroy=True)
assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["config.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
r = aws.iam.Role("r",
    name="awsconfig-example",
    assume_role_policy=assume_role.json)
foo_recorder = aws.cfg.Recorder("foo",
    name="example",
    role_arn=r.arn)
foo = aws.cfg.DeliveryChannel("foo",
    name="example",
    s3_bucket_name=b.bucket,
    opts = pulumi.ResourceOptions(depends_on=[foo_recorder]))
p = aws.iam.get_policy_document_output(statements=[{
    "effect": "Allow",
    "actions": ["s3:*"],
    "resources": [
        b.arn,
        b.arn.apply(lambda arn: f"{arn}/*"),
    ],
}])
p_role_policy = aws.iam.RolePolicy("p",
    name="awsconfig-example",
    role=r.id,
    policy=p.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var b = new Aws.S3.BucketV2("b", new()
    {
        Bucket = "example-awsconfig",
        ForceDestroy = true,
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
                            "config.amazonaws.com",
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

    var r = new Aws.Iam.Role("r", new()
    {
        Name = "awsconfig-example",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var fooRecorder = new Aws.Cfg.Recorder("foo", new()
    {
        Name = "example",
        RoleArn = r.Arn,
    });

    var foo = new Aws.Cfg.DeliveryChannel("foo", new()
    {
        Name = "example",
        S3BucketName = b.Bucket,
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            fooRecorder,
        },
    });

    var p = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Actions = new[]
                {
                    "s3:*",
                },
                Resources = new[]
                {
                    b.Arn,
                    $"{b.Arn}/*",
                },
            },
        },
    });

    var pRolePolicy = new Aws.Iam.RolePolicy("p", new()
    {
        Name = "awsconfig-example",
        Role = r.Id,
        Policy = p.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		b, err := s3.NewBucketV2(ctx, "b", &s3.BucketV2Args{
			Bucket:       pulumi.String("example-awsconfig"),
			ForceDestroy: pulumi.Bool(true),
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
								"config.amazonaws.com",
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
		r, err := iam.NewRole(ctx, "r", &iam.RoleArgs{
			Name:             pulumi.String("awsconfig-example"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		fooRecorder, err := cfg.NewRecorder(ctx, "foo", &cfg.RecorderArgs{
			Name:    pulumi.String("example"),
			RoleArn: r.Arn,
		})
		if err != nil {
			return err
		}
		_, err = cfg.NewDeliveryChannel(ctx, "foo", &cfg.DeliveryChannelArgs{
			Name:         pulumi.String("example"),
			S3BucketName: b.Bucket,
		}, pulumi.DependsOn([]pulumi.Resource{
			fooRecorder,
		}))
		if err != nil {
			return err
		}
		p := iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
			Statements: iam.GetPolicyDocumentStatementArray{
				&iam.GetPolicyDocumentStatementArgs{
					Effect: pulumi.String("Allow"),
					Actions: pulumi.StringArray{
						pulumi.String("s3:*"),
					},
					Resources: pulumi.StringArray{
						b.Arn,
						b.Arn.ApplyT(func(arn string) (string, error) {
							return fmt.Sprintf("%v/*", arn), nil
						}).(pulumi.StringOutput),
					},
				},
			},
		}, nil)
		_, err = iam.NewRolePolicy(ctx, "p", &iam.RolePolicyArgs{
			Name: pulumi.String("awsconfig-example"),
			Role: r.ID(),
			Policy: pulumi.String(p.ApplyT(func(p iam.GetPolicyDocumentResult) (*string, error) {
				return &p.Json, nil
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
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.cfg.Recorder;
import com.pulumi.aws.cfg.RecorderArgs;
import com.pulumi.aws.cfg.DeliveryChannel;
import com.pulumi.aws.cfg.DeliveryChannelArgs;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
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
        var b = new BucketV2("b", BucketV2Args.builder()
            .bucket("example-awsconfig")
            .forceDestroy(true)
            .build());

        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("config.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var r = new Role("r", RoleArgs.builder()
            .name("awsconfig-example")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var fooRecorder = new Recorder("fooRecorder", RecorderArgs.builder()
            .name("example")
            .roleArn(r.arn())
            .build());

        var foo = new DeliveryChannel("foo", DeliveryChannelArgs.builder()
            .name("example")
            .s3BucketName(b.bucket())
            .build(), CustomResourceOptions.builder()
                .dependsOn(fooRecorder)
                .build());

        final var p = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .actions("s3:*")
                .resources(                
                    b.arn(),
                    b.arn().applyValue(arn -> String.format("%s/*", arn)))
                .build())
            .build());

        var pRolePolicy = new RolePolicy("pRolePolicy", RolePolicyArgs.builder()
            .name("awsconfig-example")
            .role(r.id())
            .policy(p.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(p -> p.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

    }
}
```
```yaml
resources:
  foo:
    type: aws:cfg:DeliveryChannel
    properties:
      name: example
      s3BucketName: ${b.bucket}
    options:
      dependsOn:
        - ${fooRecorder}
  b:
    type: aws:s3:BucketV2
    properties:
      bucket: example-awsconfig
      forceDestroy: true
  fooRecorder:
    type: aws:cfg:Recorder
    name: foo
    properties:
      name: example
      roleArn: ${r.arn}
  r:
    type: aws:iam:Role
    properties:
      name: awsconfig-example
      assumeRolePolicy: ${assumeRole.json}
  pRolePolicy:
    type: aws:iam:RolePolicy
    name: p
    properties:
      name: awsconfig-example
      role: ${r.id}
      policy: ${p.json}
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
                  - config.amazonaws.com
            actions:
              - sts:AssumeRole
  p:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            actions:
              - s3:*
            resources:
              - ${b.arn}
              - ${b.arn}/*
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Delivery Channel using the name. For example:

```sh
$ pulumi import aws:cfg/deliveryChannel:DeliveryChannel foo example
```
k
nameB" ]The name of the delivery channel. Defaults to `default`. Changing it recreates the resource.
W
s3BucketName" CThe name of the S3 bucket used to store the configuration history.
=
s3KeyPrefixB" (The prefix for the specified S3 bucket.
ü
s3KmsKeyArnB" âThe ARN of the AWS KMS key used to encrypt objects delivered by AWS Config. Must belong to the same Region as the destination S3 bucket.
˛
snapshotDeliveryPropertiesñBì:ê
ç
cfg)DeliveryChannelSnapshotDeliveryProperties[aws:cfg/DeliveryChannelSnapshotDeliveryProperties:DeliveryChannelSnapshotDeliveryPropertiesGOptions for how AWS Config delivers configuration snapshots. See below
Y
snsTopicArnB" DThe ARN of the SNS topic that AWS Config delivers notifications to.
"i
name" ]The name of the delivery channel. Defaults to `default`. Changing it recreates the resource.
"W
s3BucketName" CThe name of the S3 bucket used to store the configuration history.
"=
s3KeyPrefixB" (The prefix for the specified S3 bucket.
"ü
s3KmsKeyArnB" âThe ARN of the AWS KMS key used to encrypt objects delivered by AWS Config. Must belong to the same Region as the destination S3 bucket.
"˛
snapshotDeliveryPropertiesñBì:ê
ç
cfg)DeliveryChannelSnapshotDeliveryProperties[aws:cfg/DeliveryChannelSnapshotDeliveryProperties:DeliveryChannelSnapshotDeliveryPropertiesGOptions for how AWS Config delivers configuration snapshots. See below
"Y
snsTopicArnB" DThe ARN of the SNS topic that AWS Config delivers notifications to.
*‰†
c
cfgOrganizationConformancePack?aws:cfg/organizationConformancePack:OrganizationConformancePackÂáManages a Config Organization Conformance Pack. More information can be found in the [Managing Conformance Packs Across all Accounts in Your Organization](https://docs.aws.amazon.com/config/latest/developerguide/conformance-pack-organization-apis.html) and [AWS Config Managed Rules](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html) documentation. Example conformance pack templates may be found in the [AWS Config Rules Repository](https://github.com/awslabs/aws-config-rules/tree/master/aws-config-conformance-packs).

> **NOTE:** This resource must be created in the Organization master account or a delegated administrator account, and the Organization must have all features enabled. Every Organization account except those configured in the `excluded_accounts` argument must have a Configuration Recorder with proper IAM permissions before the Organization Conformance Pack will successfully create or update. See also the `aws.cfg.Recorder` resource.

## Example Usage

### Using Template Body

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleOrganization = new aws.organizations.Organization("example", {
    awsServiceAccessPrincipals: ["config-multiaccountsetup.amazonaws.com"],
    featureSet: "ALL",
});
const example = new aws.cfg.OrganizationConformancePack("example", {
    name: "example",
    inputParameters: [{
        parameterName: "AccessKeysRotatedParameterMaxAccessKeyAge",
        parameterValue: "90",
    }],
    templateBody: `Parameters:
  AccessKeysRotatedParameterMaxAccessKeyAge:
    Type: String
Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
`,
}, {
    dependsOn: [
        exampleAwsConfigConfigurationRecorder,
        exampleOrganization,
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

example_organization = aws.organizations.Organization("example",
    aws_service_access_principals=["config-multiaccountsetup.amazonaws.com"],
    feature_set="ALL")
example = aws.cfg.OrganizationConformancePack("example",
    name="example",
    input_parameters=[{
        "parameter_name": "AccessKeysRotatedParameterMaxAccessKeyAge",
        "parameter_value": "90",
    }],
    template_body="""Parameters:
  AccessKeysRotatedParameterMaxAccessKeyAge:
    Type: String
Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
""",
    opts = pulumi.ResourceOptions(depends_on=[
            example_aws_config_configuration_recorder,
            example_organization,
        ]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleOrganization = new Aws.Organizations.Organization("example", new()
    {
        AwsServiceAccessPrincipals = new[]
        {
            "config-multiaccountsetup.amazonaws.com",
        },
        FeatureSet = "ALL",
    });

    var example = new Aws.Cfg.OrganizationConformancePack("example", new()
    {
        Name = "example",
        InputParameters = new[]
        {
            new Aws.Cfg.Inputs.OrganizationConformancePackInputParameterArgs
            {
                ParameterName = "AccessKeysRotatedParameterMaxAccessKeyAge",
                ParameterValue = "90",
            },
        },
        TemplateBody = @"Parameters:
  AccessKeysRotatedParameterMaxAccessKeyAge:
    Type: String
Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsConfigConfigurationRecorder,
            exampleOrganization,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/organizations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleOrganization, err := organizations.NewOrganization(ctx, "example", &organizations.OrganizationArgs{
			AwsServiceAccessPrincipals: pulumi.StringArray{
				pulumi.String("config-multiaccountsetup.amazonaws.com"),
			},
			FeatureSet: pulumi.String("ALL"),
		})
		if err != nil {
			return err
		}
		_, err = cfg.NewOrganizationConformancePack(ctx, "example", &cfg.OrganizationConformancePackArgs{
			Name: pulumi.String("example"),
			InputParameters: cfg.OrganizationConformancePackInputParameterArray{
				&cfg.OrganizationConformancePackInputParameterArgs{
					ParameterName:  pulumi.String("AccessKeysRotatedParameterMaxAccessKeyAge"),
					ParameterValue: pulumi.String("90"),
				},
			},
			TemplateBody: pulumi.String(`Parameters:
  AccessKeysRotatedParameterMaxAccessKeyAge:
    Type: String
Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
`),
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsConfigConfigurationRecorder,
			exampleOrganization,
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
import com.pulumi.aws.organizations.Organization;
import com.pulumi.aws.organizations.OrganizationArgs;
import com.pulumi.aws.cfg.OrganizationConformancePack;
import com.pulumi.aws.cfg.OrganizationConformancePackArgs;
import com.pulumi.aws.cfg.inputs.OrganizationConformancePackInputParameterArgs;
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
        var exampleOrganization = new Organization("exampleOrganization", OrganizationArgs.builder()
            .awsServiceAccessPrincipals("config-multiaccountsetup.amazonaws.com")
            .featureSet("ALL")
            .build());

        var example = new OrganizationConformancePack("example", OrganizationConformancePackArgs.builder()
            .name("example")
            .inputParameters(OrganizationConformancePackInputParameterArgs.builder()
                .parameterName("AccessKeysRotatedParameterMaxAccessKeyAge")
                .parameterValue("90")
                .build())
            .templateBody("""
Parameters:
  AccessKeysRotatedParameterMaxAccessKeyAge:
    Type: String
Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
            """)
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    exampleAwsConfigConfigurationRecorder,
                    exampleOrganization)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cfg:OrganizationConformancePack
    properties:
      name: example
      inputParameters:
        - parameterName: AccessKeysRotatedParameterMaxAccessKeyAge
          parameterValue: '90'
      templateBody: |
        Parameters:
          AccessKeysRotatedParameterMaxAccessKeyAge:
            Type: String
        Resources:
          IAMPasswordPolicy:
            Properties:
              ConfigRuleName: IAMPasswordPolicy
              Source:
                Owner: AWS
                SourceIdentifier: IAM_PASSWORD_POLICY
            Type: AWS::Config::ConfigRule
    options:
      dependsOn:
        - ${exampleAwsConfigConfigurationRecorder}
        - ${exampleOrganization}
  exampleOrganization:
    type: aws:organizations:Organization
    name: example
    properties:
      awsServiceAccessPrincipals:
        - config-multiaccountsetup.amazonaws.com
      featureSet: ALL
```
<!--End PulumiCodeChooser -->

### Using Template S3 URI

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const exampleOrganization = new aws.organizations.Organization("example", {
    awsServiceAccessPrincipals: ["config-multiaccountsetup.amazonaws.com"],
    featureSet: "ALL",
});
const exampleBucketV2 = new aws.s3.BucketV2("example", {bucket: "example"});
const exampleBucketObjectv2 = new aws.s3.BucketObjectv2("example", {
    bucket: exampleBucketV2.id,
    key: "example-key",
    content: `Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
`,
});
const example = new aws.cfg.OrganizationConformancePack("example", {
    name: "example",
    templateS3Uri: pulumi.interpolate`s3://${exampleBucketV2.bucket}/${exampleBucketObjectv2.key}`,
}, {
    dependsOn: [
        exampleAwsConfigConfigurationRecorder,
        exampleOrganization,
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

example_organization = aws.organizations.Organization("example",
    aws_service_access_principals=["config-multiaccountsetup.amazonaws.com"],
    feature_set="ALL")
example_bucket_v2 = aws.s3.BucketV2("example", bucket="example")
example_bucket_objectv2 = aws.s3.BucketObjectv2("example",
    bucket=example_bucket_v2.id,
    key="example-key",
    content="""Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
""")
example = aws.cfg.OrganizationConformancePack("example",
    name="example",
    template_s3_uri=pulumi.Output.all(
        bucket=example_bucket_v2.bucket,
        key=example_bucket_objectv2.key
).apply(lambda resolved_outputs: f"s3://{resolved_outputs['bucket']}/{resolved_outputs['key']}")
,
    opts = pulumi.ResourceOptions(depends_on=[
            example_aws_config_configuration_recorder,
            example_organization,
        ]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var exampleOrganization = new Aws.Organizations.Organization("example", new()
    {
        AwsServiceAccessPrincipals = new[]
        {
            "config-multiaccountsetup.amazonaws.com",
        },
        FeatureSet = "ALL",
    });

    var exampleBucketV2 = new Aws.S3.BucketV2("example", new()
    {
        Bucket = "example",
    });

    var exampleBucketObjectv2 = new Aws.S3.BucketObjectv2("example", new()
    {
        Bucket = exampleBucketV2.Id,
        Key = "example-key",
        Content = @"Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
",
    });

    var example = new Aws.Cfg.OrganizationConformancePack("example", new()
    {
        Name = "example",
        TemplateS3Uri = Output.Tuple(exampleBucketV2.Bucket, exampleBucketObjectv2.Key).Apply(values =>
        {
            var bucket = values.Item1;
            var key = values.Item2;
            return $"s3://{bucket}/{key}";
        }),
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleAwsConfigConfigurationRecorder,
            exampleOrganization,
        },
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/organizations"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		exampleOrganization, err := organizations.NewOrganization(ctx, "example", &organizations.OrganizationArgs{
			AwsServiceAccessPrincipals: pulumi.StringArray{
				pulumi.String("config-multiaccountsetup.amazonaws.com"),
			},
			FeatureSet: pulumi.String("ALL"),
		})
		if err != nil {
			return err
		}
		exampleBucketV2, err := s3.NewBucketV2(ctx, "example", &s3.BucketV2Args{
			Bucket: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		exampleBucketObjectv2, err := s3.NewBucketObjectv2(ctx, "example", &s3.BucketObjectv2Args{
			Bucket: exampleBucketV2.ID(),
			Key:    pulumi.String("example-key"),
			Content: pulumi.String(`Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
`),
		})
		if err != nil {
			return err
		}
		_, err = cfg.NewOrganizationConformancePack(ctx, "example", &cfg.OrganizationConformancePackArgs{
			Name: pulumi.String("example"),
			TemplateS3Uri: pulumi.All(exampleBucketV2.Bucket, exampleBucketObjectv2.Key).ApplyT(func(_args []interface{}) (string, error) {
				bucket := _args[0].(string)
				key := _args[1].(string)
				return fmt.Sprintf("s3://%v/%v", bucket, key), nil
			}).(pulumi.StringOutput),
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleAwsConfigConfigurationRecorder,
			exampleOrganization,
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
import com.pulumi.aws.organizations.Organization;
import com.pulumi.aws.organizations.OrganizationArgs;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.s3.BucketObjectv2;
import com.pulumi.aws.s3.BucketObjectv2Args;
import com.pulumi.aws.cfg.OrganizationConformancePack;
import com.pulumi.aws.cfg.OrganizationConformancePackArgs;
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
        var exampleOrganization = new Organization("exampleOrganization", OrganizationArgs.builder()
            .awsServiceAccessPrincipals("config-multiaccountsetup.amazonaws.com")
            .featureSet("ALL")
            .build());

        var exampleBucketV2 = new BucketV2("exampleBucketV2", BucketV2Args.builder()
            .bucket("example")
            .build());

        var exampleBucketObjectv2 = new BucketObjectv2("exampleBucketObjectv2", BucketObjectv2Args.builder()
            .bucket(exampleBucketV2.id())
            .key("example-key")
            .content("""
Resources:
  IAMPasswordPolicy:
    Properties:
      ConfigRuleName: IAMPasswordPolicy
      Source:
        Owner: AWS
        SourceIdentifier: IAM_PASSWORD_POLICY
    Type: AWS::Config::ConfigRule
            """)
            .build());

        var example = new OrganizationConformancePack("example", OrganizationConformancePackArgs.builder()
            .name("example")
            .templateS3Uri(Output.tuple(exampleBucketV2.bucket(), exampleBucketObjectv2.key()).applyValue(values -> {
                var bucket = values.t1;
                var key = values.t2;
                return String.format("s3://%s/%s", bucket,key);
            }))
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    exampleAwsConfigConfigurationRecorder,
                    exampleOrganization)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cfg:OrganizationConformancePack
    properties:
      name: example
      templateS3Uri: s3://${exampleBucketV2.bucket}/${exampleBucketObjectv2.key}
    options:
      dependsOn:
        - ${exampleAwsConfigConfigurationRecorder}
        - ${exampleOrganization}
  exampleOrganization:
    type: aws:organizations:Organization
    name: example
    properties:
      awsServiceAccessPrincipals:
        - config-multiaccountsetup.amazonaws.com
      featureSet: ALL
  exampleBucketV2:
    type: aws:s3:BucketV2
    name: example
    properties:
      bucket: example
  exampleBucketObjectv2:
    type: aws:s3:BucketObjectv2
    name: example
    properties:
      bucket: ${exampleBucketV2.id}
      key: example-key
      content: |
        Resources:
          IAMPasswordPolicy:
            Properties:
              ConfigRuleName: IAMPasswordPolicy
              Source:
                Owner: AWS
                SourceIdentifier: IAM_PASSWORD_POLICY
            Type: AWS::Config::ConfigRule
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Config Organization Conformance Packs using the `name`. For example:

```sh
$ pulumi import aws:cfg/organizationConformancePack:OrganizationConformancePack example example
```
≤
deliveryS3BucketB" óAmazon S3 bucket where AWS Config stores conformance pack templates. Delivery bucket must begin with `awsconfigconforms` prefix. Maximum length of 63.
Z
deliveryS3KeyPrefixB" =The prefix for the Amazon S3 bucket. Maximum length of 1024.
§
excludedAccountsB*" áSet of AWS accounts to be excluded from an organization conformance pack while deploying a conformance pack. Maximum of 1000 accounts.
≥
inputParametersôBñ*ì:ê
ç
cfg)OrganizationConformancePackInputParameter[aws:cfg/OrganizationConformancePackInputParameter:OrganizationConformancePackInputParameterÉSet of configuration blocks describing input parameters passed to the conformance pack template. Documented below. When configured, the parameters must also be included in the `template_body` or in the template stored in Amazon S3 if using `template_s3_uri`.
ñ
nameB" áThe name of the organization conformance pack. Must begin with a letter and contain from 1 to 128 alphanumeric characters and hyphens.
ù
templateBodyB" ÜA string containing full conformance pack template body. Maximum length of 51200. Drift detection is not possible with this argument.
ª
templateS3UriB" £Location of file, e.g., `s3://bucketname/prefix`, containing the template body. The uri must point to the conformance pack template that is located in an Amazon S3 bucket in the same region as the conformance pack. Maximum length of 1024. Drift detection is not possible with this argument.
"L
arn" AAmazon Resource Name (ARN) of the organization conformance pack.
"≤
deliveryS3BucketB" óAmazon S3 bucket where AWS Config stores conformance pack templates. Delivery bucket must begin with `awsconfigconforms` prefix. Maximum length of 63.
"Z
deliveryS3KeyPrefixB" =The prefix for the Amazon S3 bucket. Maximum length of 1024.
"§
excludedAccountsB*" áSet of AWS accounts to be excluded from an organization conformance pack while deploying a conformance pack. Maximum of 1000 accounts.
"≥
inputParametersôBñ*ì:ê
ç
cfg)OrganizationConformancePackInputParameter[aws:cfg/OrganizationConformancePackInputParameter:OrganizationConformancePackInputParameterÉSet of configuration blocks describing input parameters passed to the conformance pack template. Documented below. When configured, the parameters must also be included in the `template_body` or in the template stored in Amazon S3 if using `template_s3_uri`.
"î
name" áThe name of the organization conformance pack. Must begin with a letter and contain from 1 to 128 alphanumeric characters and hyphens.
"ù
templateBodyB" ÜA string containing full conformance pack template body. Maximum length of 51200. Drift detection is not possible with this argument.
"ª
templateS3UriB" £Location of file, e.g., `s3://bucketname/prefix`, containing the template body. The uri must point to the conformance pack template that is located in an Amazon S3 bucket in the same region as the conformance pack. Maximum length of 1024. Drift detection is not possible with this argument.
*ﬂG
f
cfgOrganizationCustomPolicyRuleAaws:cfg/organizationCustomPolicyRule:OrganizationCustomPolicyRule∫0Manages a Config Organization Custom Policy Rule. More information about these rules can be found in the [Enabling AWS Config Rules Across all Accounts in Your Organization](https://docs.aws.amazon.com/config/latest/developerguide/config-rule-multi-account-deployment.html) and [AWS Config Managed Rules](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html) documentation. For working with Organization Managed Rules (those invoking an AWS managed rule), see the `aws_config_organization_managed__rule` resource.

> **NOTE:** This resource must be created in the Organization master account and rules will include the master account unless its ID is added to the `excluded_accounts` argument.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cfg.OrganizationCustomPolicyRule("example", {
    name: "example_rule_name",
    policyRuntime: "guard-2.x.x",
    policyText: `let status = ['ACTIVE']

rule tableisactive when
    resourceType == "AWS::DynamoDB::Table" {
    configuration.tableStatus == %status
}

rule checkcompliance when
    resourceType == "AWS::DynamoDB::Table"
    tableisactive {
        let pitr = supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus
        %pitr == "ENABLED"
    }
`,
    resourceTypesScopes: ["AWS::DynamoDB::Table"],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cfg.OrganizationCustomPolicyRule("example",
    name="example_rule_name",
    policy_runtime="guard-2.x.x",
    policy_text="""let status = ['ACTIVE']

rule tableisactive when
    resourceType == "AWS::DynamoDB::Table" {
    configuration.tableStatus == %status
}

rule checkcompliance when
    resourceType == "AWS::DynamoDB::Table"
    tableisactive {
        let pitr = supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus
        %pitr == "ENABLED"
    }
""",
    resource_types_scopes=["AWS::DynamoDB::Table"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cfg.OrganizationCustomPolicyRule("example", new()
    {
        Name = "example_rule_name",
        PolicyRuntime = "guard-2.x.x",
        PolicyText = @"let status = ['ACTIVE']

rule tableisactive when
    resourceType == ""AWS::DynamoDB::Table"" {
    configuration.tableStatus == %status
}

rule checkcompliance when
    resourceType == ""AWS::DynamoDB::Table""
    tableisactive {
        let pitr = supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus
        %pitr == ""ENABLED""
    }
",
        ResourceTypesScopes = new[]
        {
            "AWS::DynamoDB::Table",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cfg.NewOrganizationCustomPolicyRule(ctx, "example", &cfg.OrganizationCustomPolicyRuleArgs{
			Name:          pulumi.String("example_rule_name"),
			PolicyRuntime: pulumi.String("guard-2.x.x"),
			PolicyText: pulumi.String(`let status = ['ACTIVE']

rule tableisactive when
    resourceType == "AWS::DynamoDB::Table" {
    configuration.tableStatus == %status
}

rule checkcompliance when
    resourceType == "AWS::DynamoDB::Table"
    tableisactive {
        let pitr = supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus
        %pitr == "ENABLED"
    }
`),
			ResourceTypesScopes: pulumi.StringArray{
				pulumi.String("AWS::DynamoDB::Table"),
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
import com.pulumi.aws.cfg.OrganizationCustomPolicyRule;
import com.pulumi.aws.cfg.OrganizationCustomPolicyRuleArgs;
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
        var example = new OrganizationCustomPolicyRule("example", OrganizationCustomPolicyRuleArgs.builder()
            .name("example_rule_name")
            .policyRuntime("guard-2.x.x")
            .policyText("""
let status = ['ACTIVE']

rule tableisactive when
    resourceType == "AWS::DynamoDB::Table" {
    configuration.tableStatus == %status
}

rule checkcompliance when
    resourceType == "AWS::DynamoDB::Table"
    tableisactive {
        let pitr = supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus
        %pitr == "ENABLED"
    }
            """)
            .resourceTypesScopes("AWS::DynamoDB::Table")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cfg:OrganizationCustomPolicyRule
    properties:
      name: example_rule_name
      policyRuntime: guard-2.x.x
      policyText: |
        let status = ['ACTIVE']

        rule tableisactive when
            resourceType == "AWS::DynamoDB::Table" {
            configuration.tableStatus == %status
        }

        rule checkcompliance when
            resourceType == "AWS::DynamoDB::Table"
            tableisactive {
                let pitr = supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus
                %pitr == "ENABLED"
            }
      resourceTypesScopes:
        - AWS::DynamoDB::Table
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a Config Organization Custom Policy Rule using the `name` argument. For example:

```sh
$ pulumi import aws:cfg/organizationCustomPolicyRule:OrganizationCustomPolicyRule example example_rule_name
```
]
debugLogDeliveryAccountsB*" 9List of AWS account identifiers to exclude from the rule
-
descriptionB" Description of the rule
U
excludedAccountsB*" 9List of AWS account identifiers to exclude from the rule
g
inputParametersB" NA string in JSON format that is passed to the AWS Config Rule Lambda Function
ø
maximumExecutionFrequencyB" õMaximum frequency with which AWS Config runs evaluations for a rule, if the rule is triggered at a periodic frequency. Defaults to `TwentyFour_Hours` for periodic frequency triggered rules. Valid values: `One_Hour`, `Three_Hours`, `Six_Hours`, `Twelve_Hours`, or `TwentyFour_Hours`.

nameB" name of the rule
Y
policyRuntime" Druntime system for your organization AWS Config Custom Policy rules
m

policyText" [policy definition containing the logic for your organization AWS Config Custom Policy rule
D
resourceIdScopeB" +Identifier of the AWS resource to evaluate
J
resourceTypesScopesB*" +List of types of AWS resources to evaluate
:
tagKeyScopeB" %Tag key of AWS resources to evaluate
>
tagValueScopeB" 'Tag value of AWS resources to evaluate
¯
triggerTypes*" ·List of notification types that trigger AWS Config to run an evaluation for the rule. Valid values: `ConfigurationItemChangeNotification`, `OversizedConfigurationItemChangeNotification`

The following arguments are optional:
"2
arn" 'Amazon Resource Name (ARN) of the rule
"]
debugLogDeliveryAccountsB*" 9List of AWS account identifiers to exclude from the rule
"-
descriptionB" Description of the rule
"U
excludedAccountsB*" 9List of AWS account identifiers to exclude from the rule
"g
inputParametersB" NA string in JSON format that is passed to the AWS Config Rule Lambda Function
"ø
maximumExecutionFrequencyB" õMaximum frequency with which AWS Config runs evaluations for a rule, if the rule is triggered at a periodic frequency. Defaults to `TwentyFour_Hours` for periodic frequency triggered rules. Valid values: `One_Hour`, `Three_Hours`, `Six_Hours`, `Twelve_Hours`, or `TwentyFour_Hours`.
"
name" name of the rule
"Y
policyRuntime" Druntime system for your organization AWS Config Custom Policy rules
"m

policyText" [policy definition containing the logic for your organization AWS Config Custom Policy rule
"D
resourceIdScopeB" +Identifier of the AWS resource to evaluate
"J
resourceTypesScopesB*" +List of types of AWS resources to evaluate
":
tagKeyScopeB" %Tag key of AWS resources to evaluate
">
tagValueScopeB" 'Tag value of AWS resources to evaluate
"¯
triggerTypes*" ·List of notification types that trigger AWS Config to run an evaluation for the rule. Valid values: `ConfigurationItemChangeNotification`, `OversizedConfigurationItemChangeNotification`

The following arguments are optional:
*ïQ
T
cfgOrganizationCustomRule5aws:cfg/organizationCustomRule:OrganizationCustomRule¥=Manages a Config Organization Custom Rule. More information about these rules can be found in the [Enabling AWS Config Rules Across all Accounts in Your Organization](https://docs.aws.amazon.com/config/latest/developerguide/config-rule-multi-account-deployment.html) and [AWS Config Managed Rules](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html) documentation. For working with Organization Managed Rules (those invoking an AWS managed rule), see the `aws_config_organization_managed__rule` resource.

> **NOTE:** This resource must be created in the Organization master account and rules will include the master account unless its ID is added to the `excluded_accounts` argument.

> **NOTE:** The proper Lambda permission to allow the AWS Config service invoke the Lambda Function must be in place before the rule will successfully create or update. See also the `aws.lambda.Permission` resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.lambda.Permission("example", {
    action: "lambda:InvokeFunction",
    "function": exampleAwsLambdaFunction.arn,
    principal: "config.amazonaws.com",
    statementId: "AllowExecutionFromConfig",
});
const exampleOrganization = new aws.organizations.Organization("example", {
    awsServiceAccessPrincipals: ["config-multiaccountsetup.amazonaws.com"],
    featureSet: "ALL",
});
const exampleOrganizationCustomRule = new aws.cfg.OrganizationCustomRule("example", {
    lambdaFunctionArn: exampleAwsLambdaFunction.arn,
    name: "example",
    triggerTypes: ["ConfigurationItemChangeNotification"],
}, {
    dependsOn: [
        example,
        exampleOrganization,
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.lambda_.Permission("example",
    action="lambda:InvokeFunction",
    function=example_aws_lambda_function["arn"],
    principal="config.amazonaws.com",
    statement_id="AllowExecutionFromConfig")
example_organization = aws.organizations.Organization("example",
    aws_service_access_principals=["config-multiaccountsetup.amazonaws.com"],
    feature_set="ALL")
example_organization_custom_rule = aws.cfg.OrganizationCustomRule("example",
    lambda_function_arn=example_aws_lambda_function["arn"],
    name="example",
    trigger_types=["ConfigurationItemChangeNotification"],
    opts = pulumi.ResourceOptions(depends_on=[
            example,
            example_organization,
        ]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Lambda.Permission("example", new()
    {
        Action = "lambda:InvokeFunction",
        Function = exampleAwsLambdaFunction.Arn,
        Principal = "config.amazonaws.com",
        StatementId = "AllowExecutionFromConfig",
    });

    var exampleOrganization = new Aws.Organizations.Organization("example", new()
    {
        AwsServiceAccessPrincipals = new[]
        {
            "config-multiaccountsetup.amazonaws.com",
        },
        FeatureSet = "ALL",
    });

    var exampleOrganizationCustomRule = new Aws.Cfg.OrganizationCustomRule("example", new()
    {
        LambdaFunctionArn = exampleAwsLambdaFunction.Arn,
        Name = "example",
        TriggerTypes = new[]
        {
            "ConfigurationItemChangeNotification",
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
            exampleOrganization,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/lambda"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/organizations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := lambda.NewPermission(ctx, "example", &lambda.PermissionArgs{
			Action:      pulumi.String("lambda:InvokeFunction"),
			Function:    pulumi.Any(exampleAwsLambdaFunction.Arn),
			Principal:   pulumi.String("config.amazonaws.com"),
			StatementId: pulumi.String("AllowExecutionFromConfig"),
		})
		if err != nil {
			return err
		}
		exampleOrganization, err := organizations.NewOrganization(ctx, "example", &organizations.OrganizationArgs{
			AwsServiceAccessPrincipals: pulumi.StringArray{
				pulumi.String("config-multiaccountsetup.amazonaws.com"),
			},
			FeatureSet: pulumi.String("ALL"),
		})
		if err != nil {
			return err
		}
		_, err = cfg.NewOrganizationCustomRule(ctx, "example", &cfg.OrganizationCustomRuleArgs{
			LambdaFunctionArn: pulumi.Any(exampleAwsLambdaFunction.Arn),
			Name:              pulumi.String("example"),
			TriggerTypes: pulumi.StringArray{
				pulumi.String("ConfigurationItemChangeNotification"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
			exampleOrganization,
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
import com.pulumi.aws.lambda.Permission;
import com.pulumi.aws.lambda.PermissionArgs;
import com.pulumi.aws.organizations.Organization;
import com.pulumi.aws.organizations.OrganizationArgs;
import com.pulumi.aws.cfg.OrganizationCustomRule;
import com.pulumi.aws.cfg.OrganizationCustomRuleArgs;
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
        var example = new Permission("example", PermissionArgs.builder()
            .action("lambda:InvokeFunction")
            .function(exampleAwsLambdaFunction.arn())
            .principal("config.amazonaws.com")
            .statementId("AllowExecutionFromConfig")
            .build());

        var exampleOrganization = new Organization("exampleOrganization", OrganizationArgs.builder()
            .awsServiceAccessPrincipals("config-multiaccountsetup.amazonaws.com")
            .featureSet("ALL")
            .build());

        var exampleOrganizationCustomRule = new OrganizationCustomRule("exampleOrganizationCustomRule", OrganizationCustomRuleArgs.builder()
            .lambdaFunctionArn(exampleAwsLambdaFunction.arn())
            .name("example")
            .triggerTypes("ConfigurationItemChangeNotification")
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    example,
                    exampleOrganization)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:lambda:Permission
    properties:
      action: lambda:InvokeFunction
      function: ${exampleAwsLambdaFunction.arn}
      principal: config.amazonaws.com
      statementId: AllowExecutionFromConfig
  exampleOrganization:
    type: aws:organizations:Organization
    name: example
    properties:
      awsServiceAccessPrincipals:
        - config-multiaccountsetup.amazonaws.com
      featureSet: ALL
  exampleOrganizationCustomRule:
    type: aws:cfg:OrganizationCustomRule
    name: example
    properties:
      lambdaFunctionArn: ${exampleAwsLambdaFunction.arn}
      name: example
      triggerTypes:
        - ConfigurationItemChangeNotification
    options:
      dependsOn:
        - ${example}
        - ${exampleOrganization}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Config Organization Custom Rules using the name. For example:

```sh
$ pulumi import aws:cfg/organizationCustomRule:OrganizationCustomRule example example
```
-
descriptionB" Description of the rule
U
excludedAccountsB*" 9List of AWS account identifiers to exclude from the rule
g
inputParametersB" NA string in JSON format that is passed to the AWS Config Rule Lambda Function
P
lambdaFunctionArn" 7Amazon Resource Name (ARN) of the rule Lambda Function
√
maximumExecutionFrequencyB" üThe maximum frequency with which AWS Config runs evaluations for a rule, if the rule is triggered at a periodic frequency. Defaults to `TwentyFour_Hours` for periodic frequency triggered rules. Valid values: `One_Hour`, `Three_Hours`, `Six_Hours`, `Twelve_Hours`, or `TwentyFour_Hours`.
#
nameB" The name of the rule
D
resourceIdScopeB" +Identifier of the AWS resource to evaluate
J
resourceTypesScopesB*" +List of types of AWS resources to evaluate
:
tagKeyScopeB" %Tag key of AWS resources to evaluate
>
tagValueScopeB" 'Tag value of AWS resources to evaluate
Ó
triggerTypes*" ◊List of notification types that trigger AWS Config to run an evaluation for the rule. Valid values: `ConfigurationItemChangeNotification`, `OversizedConfigurationItemChangeNotification`, and `ScheduledNotification`
"2
arn" 'Amazon Resource Name (ARN) of the rule
"-
descriptionB" Description of the rule
"U
excludedAccountsB*" 9List of AWS account identifiers to exclude from the rule
"g
inputParametersB" NA string in JSON format that is passed to the AWS Config Rule Lambda Function
"P
lambdaFunctionArn" 7Amazon Resource Name (ARN) of the rule Lambda Function
"√
maximumExecutionFrequencyB" üThe maximum frequency with which AWS Config runs evaluations for a rule, if the rule is triggered at a periodic frequency. Defaults to `TwentyFour_Hours` for periodic frequency triggered rules. Valid values: `One_Hour`, `Three_Hours`, `Six_Hours`, `Twelve_Hours`, or `TwentyFour_Hours`.
"!
name" The name of the rule
"D
resourceIdScopeB" +Identifier of the AWS resource to evaluate
"J
resourceTypesScopesB*" +List of types of AWS resources to evaluate
":
tagKeyScopeB" %Tag key of AWS resources to evaluate
">
tagValueScopeB" 'Tag value of AWS resources to evaluate
"Ó
triggerTypes*" ◊List of notification types that trigger AWS Config to run an evaluation for the rule. Valid values: `ConfigurationItemChangeNotification`, `OversizedConfigurationItemChangeNotification`, and `ScheduledNotification`
*ü<
W
cfgOrganizationManagedRule7aws:cfg/organizationManagedRule:OrganizationManagedRule¡)Manages a Config Organization Managed Rule. More information about these rules can be found in the [Enabling AWS Config Rules Across all Accounts in Your Organization](https://docs.aws.amazon.com/config/latest/developerguide/config-rule-multi-account-deployment.html) and [AWS Config Managed Rules](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html) documentation. For working with Organization Custom Rules (those invoking a custom Lambda Function), see the `aws.cfg.OrganizationCustomRule` resource.

> **NOTE:** This resource must be created in the Organization master account and rules will include the master account unless its ID is added to the `excluded_accounts` argument.

> **NOTE:** Every Organization account except those configured in the `excluded_accounts` argument must have a Configuration Recorder with proper IAM permissions before the rule will successfully create or update. See also the `aws.cfg.Recorder` resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.organizations.Organization("example", {
    awsServiceAccessPrincipals: ["config-multiaccountsetup.amazonaws.com"],
    featureSet: "ALL",
});
const exampleOrganizationManagedRule = new aws.cfg.OrganizationManagedRule("example", {
    name: "example",
    ruleIdentifier: "IAM_PASSWORD_POLICY",
}, {
    dependsOn: [example],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.organizations.Organization("example",
    aws_service_access_principals=["config-multiaccountsetup.amazonaws.com"],
    feature_set="ALL")
example_organization_managed_rule = aws.cfg.OrganizationManagedRule("example",
    name="example",
    rule_identifier="IAM_PASSWORD_POLICY",
    opts = pulumi.ResourceOptions(depends_on=[example]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Organizations.Organization("example", new()
    {
        AwsServiceAccessPrincipals = new[]
        {
            "config-multiaccountsetup.amazonaws.com",
        },
        FeatureSet = "ALL",
    });

    var exampleOrganizationManagedRule = new Aws.Cfg.OrganizationManagedRule("example", new()
    {
        Name = "example",
        RuleIdentifier = "IAM_PASSWORD_POLICY",
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/organizations"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := organizations.NewOrganization(ctx, "example", &organizations.OrganizationArgs{
			AwsServiceAccessPrincipals: pulumi.StringArray{
				pulumi.String("config-multiaccountsetup.amazonaws.com"),
			},
			FeatureSet: pulumi.String("ALL"),
		})
		if err != nil {
			return err
		}
		_, err = cfg.NewOrganizationManagedRule(ctx, "example", &cfg.OrganizationManagedRuleArgs{
			Name:           pulumi.String("example"),
			RuleIdentifier: pulumi.String("IAM_PASSWORD_POLICY"),
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
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
import com.pulumi.aws.organizations.Organization;
import com.pulumi.aws.organizations.OrganizationArgs;
import com.pulumi.aws.cfg.OrganizationManagedRule;
import com.pulumi.aws.cfg.OrganizationManagedRuleArgs;
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
        var example = new Organization("example", OrganizationArgs.builder()
            .awsServiceAccessPrincipals("config-multiaccountsetup.amazonaws.com")
            .featureSet("ALL")
            .build());

        var exampleOrganizationManagedRule = new OrganizationManagedRule("exampleOrganizationManagedRule", OrganizationManagedRuleArgs.builder()
            .name("example")
            .ruleIdentifier("IAM_PASSWORD_POLICY")
            .build(), CustomResourceOptions.builder()
                .dependsOn(example)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:organizations:Organization
    properties:
      awsServiceAccessPrincipals:
        - config-multiaccountsetup.amazonaws.com
      featureSet: ALL
  exampleOrganizationManagedRule:
    type: aws:cfg:OrganizationManagedRule
    name: example
    properties:
      name: example
      ruleIdentifier: IAM_PASSWORD_POLICY
    options:
      dependsOn:
        - ${example}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Config Organization Managed Rules using the name. For example:

```sh
$ pulumi import aws:cfg/organizationManagedRule:OrganizationManagedRule example example
```
-
descriptionB" Description of the rule
U
excludedAccountsB*" 9List of AWS account identifiers to exclude from the rule
g
inputParametersB" NA string in JSON format that is passed to the AWS Config Rule Lambda Function
√
maximumExecutionFrequencyB" üThe maximum frequency with which AWS Config runs evaluations for a rule, if the rule is triggered at a periodic frequency. Defaults to `TwentyFour_Hours` for periodic frequency triggered rules. Valid values: `One_Hour`, `Three_Hours`, `Six_Hours`, `Twelve_Hours`, or `TwentyFour_Hours`.
#
nameB" The name of the rule
D
resourceIdScopeB" +Identifier of the AWS resource to evaluate
J
resourceTypesScopesB*" +List of types of AWS resources to evaluate
˝
ruleIdentifier" ÊIdentifier of an available AWS Config Managed Rule to call. For available values, see the [List of AWS Config Managed Rules](https://docs.aws.amazon.com/config/latest/developerguide/managed-rules-by-aws-config.html) documentation
:
tagKeyScopeB" %Tag key of AWS resources to evaluate
>
tagValueScopeB" 'Tag value of AWS resources to evaluate
"2
arn" 'Amazon Resource Name (ARN) of the rule
"-
descriptionB" Description of the rule
"U
excludedAccountsB*" 9List of AWS account identifiers to exclude from the rule
"g
inputParametersB" NA string in JSON format that is passed to the AWS Config Rule Lambda Function
"√
maximumExecutionFrequencyB" üThe maximum frequency with which AWS Config runs evaluations for a rule, if the rule is triggered at a periodic frequency. Defaults to `TwentyFour_Hours` for periodic frequency triggered rules. Valid values: `One_Hour`, `Three_Hours`, `Six_Hours`, `Twelve_Hours`, or `TwentyFour_Hours`.
"!
name" The name of the rule
"D
resourceIdScopeB" +Identifier of the AWS resource to evaluate
"J
resourceTypesScopesB*" +List of types of AWS resources to evaluate
"˝
ruleIdentifier" ÊIdentifier of an available AWS Config Managed Rule to call. For available values, see the [List of AWS Config Managed Rules](https://docs.aws.amazon.com/config/latest/developerguide/managed-rules-by-aws-config.html) documentation
":
tagKeyScopeB" %Tag key of AWS resources to evaluate
">
tagValueScopeB" 'Tag value of AWS resources to evaluate
*Øâ
*
cfgRecorderaws:cfg/recorder:Recorder¥~Provides an AWS Config Configuration Recorder. Please note that this resource **does not start** the created recorder automatically.

> **Note:** _Starting_ the Configuration Recorder requires a delivery channel (while delivery channel creation requires Configuration Recorder). This is why `aws.cfg.RecorderStatus` is a separate resource.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["config.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const r = new aws.iam.Role("r", {
    name: "awsconfig-example",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const foo = new aws.cfg.Recorder("foo", {
    name: "example",
    roleArn: r.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["config.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
r = aws.iam.Role("r",
    name="awsconfig-example",
    assume_role_policy=assume_role.json)
foo = aws.cfg.Recorder("foo",
    name="example",
    role_arn=r.arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
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
                            "config.amazonaws.com",
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

    var r = new Aws.Iam.Role("r", new()
    {
        Name = "awsconfig-example",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var foo = new Aws.Cfg.Recorder("foo", new()
    {
        Name = "example",
        RoleArn = r.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		assumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"config.amazonaws.com",
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
		r, err := iam.NewRole(ctx, "r", &iam.RoleArgs{
			Name:             pulumi.String("awsconfig-example"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		_, err = cfg.NewRecorder(ctx, "foo", &cfg.RecorderArgs{
			Name:    pulumi.String("example"),
			RoleArn: r.Arn,
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
import com.pulumi.aws.cfg.Recorder;
import com.pulumi.aws.cfg.RecorderArgs;
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
        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("config.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var r = new Role("r", RoleArgs.builder()
            .name("awsconfig-example")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var foo = new Recorder("foo", RecorderArgs.builder()
            .name("example")
            .roleArn(r.arn())
            .build());

    }
}
```
```yaml
resources:
  foo:
    type: aws:cfg:Recorder
    properties:
      name: example
      roleArn: ${r.arn}
  r:
    type: aws:iam:Role
    properties:
      name: awsconfig-example
      assumeRolePolicy: ${assumeRole.json}
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
                  - config.amazonaws.com
            actions:
              - sts:AssumeRole
```
<!--End PulumiCodeChooser -->

### Exclude Resources Types Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const foo = new aws.cfg.Recorder("foo", {
    name: "example",
    roleArn: r.arn,
    recordingGroup: {
        allSupported: false,
        exclusionByResourceTypes: [{
            resourceTypes: ["AWS::EC2::Instance"],
        }],
        recordingStrategies: [{
            useOnly: "EXCLUSION_BY_RESOURCE_TYPES",
        }],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

foo = aws.cfg.Recorder("foo",
    name="example",
    role_arn=r["arn"],
    recording_group={
        "all_supported": False,
        "exclusion_by_resource_types": [{
            "resource_types": ["AWS::EC2::Instance"],
        }],
        "recording_strategies": [{
            "use_only": "EXCLUSION_BY_RESOURCE_TYPES",
        }],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var foo = new Aws.Cfg.Recorder("foo", new()
    {
        Name = "example",
        RoleArn = r.Arn,
        RecordingGroup = new Aws.Cfg.Inputs.RecorderRecordingGroupArgs
        {
            AllSupported = false,
            ExclusionByResourceTypes = new[]
            {
                new Aws.Cfg.Inputs.RecorderRecordingGroupExclusionByResourceTypeArgs
                {
                    ResourceTypes = new[]
                    {
                        "AWS::EC2::Instance",
                    },
                },
            },
            RecordingStrategies = new[]
            {
                new Aws.Cfg.Inputs.RecorderRecordingGroupRecordingStrategyArgs
                {
                    UseOnly = "EXCLUSION_BY_RESOURCE_TYPES",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cfg.NewRecorder(ctx, "foo", &cfg.RecorderArgs{
			Name:    pulumi.String("example"),
			RoleArn: pulumi.Any(r.Arn),
			RecordingGroup: &cfg.RecorderRecordingGroupArgs{
				AllSupported: pulumi.Bool(false),
				ExclusionByResourceTypes: cfg.RecorderRecordingGroupExclusionByResourceTypeArray{
					&cfg.RecorderRecordingGroupExclusionByResourceTypeArgs{
						ResourceTypes: pulumi.StringArray{
							pulumi.String("AWS::EC2::Instance"),
						},
					},
				},
				RecordingStrategies: cfg.RecorderRecordingGroupRecordingStrategyArray{
					&cfg.RecorderRecordingGroupRecordingStrategyArgs{
						UseOnly: pulumi.String("EXCLUSION_BY_RESOURCE_TYPES"),
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
import com.pulumi.aws.cfg.Recorder;
import com.pulumi.aws.cfg.RecorderArgs;
import com.pulumi.aws.cfg.inputs.RecorderRecordingGroupArgs;
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
        var foo = new Recorder("foo", RecorderArgs.builder()
            .name("example")
            .roleArn(r.arn())
            .recordingGroup(RecorderRecordingGroupArgs.builder()
                .allSupported(false)
                .exclusionByResourceTypes(RecorderRecordingGroupExclusionByResourceTypeArgs.builder()
                    .resourceTypes("AWS::EC2::Instance")
                    .build())
                .recordingStrategies(RecorderRecordingGroupRecordingStrategyArgs.builder()
                    .useOnly("EXCLUSION_BY_RESOURCE_TYPES")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  foo:
    type: aws:cfg:Recorder
    properties:
      name: example
      roleArn: ${r.arn}
      recordingGroup:
        allSupported: false
        exclusionByResourceTypes:
          - resourceTypes:
              - AWS::EC2::Instance
        recordingStrategies:
          - useOnly: EXCLUSION_BY_RESOURCE_TYPES
```
<!--End PulumiCodeChooser -->

### Periodic Recording

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const foo = new aws.cfg.Recorder("foo", {
    name: "example",
    roleArn: r.arn,
    recordingGroup: {
        allSupported: false,
        includeGlobalResourceTypes: false,
        resourceTypes: [
            "AWS::EC2::Instance",
            "AWS::EC2::NetworkInterface",
        ],
    },
    recordingMode: {
        recordingFrequency: "CONTINUOUS",
        recordingModeOverride: {
            description: "Only record EC2 network interfaces daily",
            resourceTypes: ["AWS::EC2::NetworkInterface"],
            recordingFrequency: "DAILY",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

foo = aws.cfg.Recorder("foo",
    name="example",
    role_arn=r["arn"],
    recording_group={
        "all_supported": False,
        "include_global_resource_types": False,
        "resource_types": [
            "AWS::EC2::Instance",
            "AWS::EC2::NetworkInterface",
        ],
    },
    recording_mode={
        "recording_frequency": "CONTINUOUS",
        "recording_mode_override": {
            "description": "Only record EC2 network interfaces daily",
            "resource_types": ["AWS::EC2::NetworkInterface"],
            "recording_frequency": "DAILY",
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
    var foo = new Aws.Cfg.Recorder("foo", new()
    {
        Name = "example",
        RoleArn = r.Arn,
        RecordingGroup = new Aws.Cfg.Inputs.RecorderRecordingGroupArgs
        {
            AllSupported = false,
            IncludeGlobalResourceTypes = false,
            ResourceTypes = new[]
            {
                "AWS::EC2::Instance",
                "AWS::EC2::NetworkInterface",
            },
        },
        RecordingMode = new Aws.Cfg.Inputs.RecorderRecordingModeArgs
        {
            RecordingFrequency = "CONTINUOUS",
            RecordingModeOverride = new Aws.Cfg.Inputs.RecorderRecordingModeRecordingModeOverrideArgs
            {
                Description = "Only record EC2 network interfaces daily",
                ResourceTypes = new[]
                {
                    "AWS::EC2::NetworkInterface",
                },
                RecordingFrequency = "DAILY",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cfg.NewRecorder(ctx, "foo", &cfg.RecorderArgs{
			Name:    pulumi.String("example"),
			RoleArn: pulumi.Any(r.Arn),
			RecordingGroup: &cfg.RecorderRecordingGroupArgs{
				AllSupported:               pulumi.Bool(false),
				IncludeGlobalResourceTypes: pulumi.Bool(false),
				ResourceTypes: pulumi.StringArray{
					pulumi.String("AWS::EC2::Instance"),
					pulumi.String("AWS::EC2::NetworkInterface"),
				},
			},
			RecordingMode: &cfg.RecorderRecordingModeArgs{
				RecordingFrequency: pulumi.String("CONTINUOUS"),
				RecordingModeOverride: &cfg.RecorderRecordingModeRecordingModeOverrideArgs{
					Description: pulumi.String("Only record EC2 network interfaces daily"),
					ResourceTypes: pulumi.StringArray{
						pulumi.String("AWS::EC2::NetworkInterface"),
					},
					RecordingFrequency: pulumi.String("DAILY"),
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
import com.pulumi.aws.cfg.Recorder;
import com.pulumi.aws.cfg.RecorderArgs;
import com.pulumi.aws.cfg.inputs.RecorderRecordingGroupArgs;
import com.pulumi.aws.cfg.inputs.RecorderRecordingModeArgs;
import com.pulumi.aws.cfg.inputs.RecorderRecordingModeRecordingModeOverrideArgs;
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
        var foo = new Recorder("foo", RecorderArgs.builder()
            .name("example")
            .roleArn(r.arn())
            .recordingGroup(RecorderRecordingGroupArgs.builder()
                .allSupported(false)
                .includeGlobalResourceTypes(false)
                .resourceTypes(                
                    "AWS::EC2::Instance",
                    "AWS::EC2::NetworkInterface")
                .build())
            .recordingMode(RecorderRecordingModeArgs.builder()
                .recordingFrequency("CONTINUOUS")
                .recordingModeOverride(RecorderRecordingModeRecordingModeOverrideArgs.builder()
                    .description("Only record EC2 network interfaces daily")
                    .resourceTypes("AWS::EC2::NetworkInterface")
                    .recordingFrequency("DAILY")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  foo:
    type: aws:cfg:Recorder
    properties:
      name: example
      roleArn: ${r.arn}
      recordingGroup:
        allSupported: false
        includeGlobalResourceTypes: false
        resourceTypes:
          - AWS::EC2::Instance
          - AWS::EC2::NetworkInterface
      recordingMode:
        recordingFrequency: CONTINUOUS
        recordingModeOverride:
          description: Only record EC2 network interfaces daily
          resourceTypes:
            - AWS::EC2::NetworkInterface
          recordingFrequency: DAILY
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Configuration Recorder using the name. For example:

```sh
$ pulumi import aws:cfg/recorder:Recorder foo example
```
c
nameB" UThe name of the recorder. Defaults to `default`. Changing it recreates the resource.
ã
recordingGroupZBX:V
T
cfgRecorderRecordingGroup5aws:cfg/RecorderRecordingGroup:RecorderRecordingGroupRecording group - see below.
Ü
recordingModeWBU:S
Q
cfgRecorderRecordingMode3aws:cfg/RecorderRecordingMode:RecorderRecordingModeRecording mode - see below.
™
roleArn" öAmazon Resource Name (ARN) of the IAM role. Used to make read or write requests to the delivery channel and to describe the AWS resources associated with the account. See [AWS Docs](http://docs.aws.amazon.com/config/latest/developerguide/iamrole-permissions.html) for more details.
"a
name" UThe name of the recorder. Defaults to `default`. Changing it recreates the resource.
"â
recordingGroupX:V
T
cfgRecorderRecordingGroup5aws:cfg/RecorderRecordingGroup:RecorderRecordingGroupRecording group - see below.
"Ñ
recordingModeU:S
Q
cfgRecorderRecordingMode3aws:cfg/RecorderRecordingMode:RecorderRecordingModeRecording mode - see below.
"™
roleArn" öAmazon Resource Name (ARN) of the IAM role. Used to make read or write requests to the delivery channel and to describe the AWS resources associated with the account. See [AWS Docs](http://docs.aws.amazon.com/config/latest/developerguide/iamrole-permissions.html) for more details.
*ål
<
cfgRecorderStatus%aws:cfg/recorderStatus:RecorderStatus—iManages status (recording / stopped) of an AWS Config Configuration Recorder.

> **Note:** Starting Configuration Recorder requires a Delivery Channel to be present. Use of `depends_on` (as shown below) is recommended to avoid race conditions.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const b = new aws.s3.BucketV2("b", {bucket: "awsconfig-example"});
const fooDeliveryChannel = new aws.cfg.DeliveryChannel("foo", {
    name: "example",
    s3BucketName: b.bucket,
});
const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["config.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const r = new aws.iam.Role("r", {
    name: "example-awsconfig",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const fooRecorder = new aws.cfg.Recorder("foo", {
    name: "example",
    roleArn: r.arn,
});
const foo = new aws.cfg.RecorderStatus("foo", {
    name: fooRecorder.name,
    isEnabled: true,
}, {
    dependsOn: [fooDeliveryChannel],
});
const a = new aws.iam.RolePolicyAttachment("a", {
    role: r.name,
    policyArn: "arn:aws:iam::aws:policy/service-role/AWS_ConfigRole",
});
const p = aws.iam.getPolicyDocumentOutput({
    statements: [{
        effect: "Allow",
        actions: ["s3:*"],
        resources: [
            b.arn,
            pulumi.interpolate`${b.arn}/*`,
        ],
    }],
});
const pRolePolicy = new aws.iam.RolePolicy("p", {
    name: "awsconfig-example",
    role: r.id,
    policy: p.apply(p => p.json),
});
```
```python
import pulumi
import pulumi_aws as aws

b = aws.s3.BucketV2("b", bucket="awsconfig-example")
foo_delivery_channel = aws.cfg.DeliveryChannel("foo",
    name="example",
    s3_bucket_name=b.bucket)
assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["config.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
r = aws.iam.Role("r",
    name="example-awsconfig",
    assume_role_policy=assume_role.json)
foo_recorder = aws.cfg.Recorder("foo",
    name="example",
    role_arn=r.arn)
foo = aws.cfg.RecorderStatus("foo",
    name=foo_recorder.name,
    is_enabled=True,
    opts = pulumi.ResourceOptions(depends_on=[foo_delivery_channel]))
a = aws.iam.RolePolicyAttachment("a",
    role=r.name,
    policy_arn="arn:aws:iam::aws:policy/service-role/AWS_ConfigRole")
p = aws.iam.get_policy_document_output(statements=[{
    "effect": "Allow",
    "actions": ["s3:*"],
    "resources": [
        b.arn,
        b.arn.apply(lambda arn: f"{arn}/*"),
    ],
}])
p_role_policy = aws.iam.RolePolicy("p",
    name="awsconfig-example",
    role=r.id,
    policy=p.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var b = new Aws.S3.BucketV2("b", new()
    {
        Bucket = "awsconfig-example",
    });

    var fooDeliveryChannel = new Aws.Cfg.DeliveryChannel("foo", new()
    {
        Name = "example",
        S3BucketName = b.Bucket,
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
                            "config.amazonaws.com",
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

    var r = new Aws.Iam.Role("r", new()
    {
        Name = "example-awsconfig",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var fooRecorder = new Aws.Cfg.Recorder("foo", new()
    {
        Name = "example",
        RoleArn = r.Arn,
    });

    var foo = new Aws.Cfg.RecorderStatus("foo", new()
    {
        Name = fooRecorder.Name,
        IsEnabled = true,
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            fooDeliveryChannel,
        },
    });

    var a = new Aws.Iam.RolePolicyAttachment("a", new()
    {
        Role = r.Name,
        PolicyArn = "arn:aws:iam::aws:policy/service-role/AWS_ConfigRole",
    });

    var p = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Actions = new[]
                {
                    "s3:*",
                },
                Resources = new[]
                {
                    b.Arn,
                    $"{b.Arn}/*",
                },
            },
        },
    });

    var pRolePolicy = new Aws.Iam.RolePolicy("p", new()
    {
        Name = "awsconfig-example",
        Role = r.Id,
        Policy = p.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		b, err := s3.NewBucketV2(ctx, "b", &s3.BucketV2Args{
			Bucket: pulumi.String("awsconfig-example"),
		})
		if err != nil {
			return err
		}
		fooDeliveryChannel, err := cfg.NewDeliveryChannel(ctx, "foo", &cfg.DeliveryChannelArgs{
			Name:         pulumi.String("example"),
			S3BucketName: b.Bucket,
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
								"config.amazonaws.com",
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
		r, err := iam.NewRole(ctx, "r", &iam.RoleArgs{
			Name:             pulumi.String("example-awsconfig"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		fooRecorder, err := cfg.NewRecorder(ctx, "foo", &cfg.RecorderArgs{
			Name:    pulumi.String("example"),
			RoleArn: r.Arn,
		})
		if err != nil {
			return err
		}
		_, err = cfg.NewRecorderStatus(ctx, "foo", &cfg.RecorderStatusArgs{
			Name:      fooRecorder.Name,
			IsEnabled: pulumi.Bool(true),
		}, pulumi.DependsOn([]pulumi.Resource{
			fooDeliveryChannel,
		}))
		if err != nil {
			return err
		}
		_, err = iam.NewRolePolicyAttachment(ctx, "a", &iam.RolePolicyAttachmentArgs{
			Role:      r.Name,
			PolicyArn: pulumi.String("arn:aws:iam::aws:policy/service-role/AWS_ConfigRole"),
		})
		if err != nil {
			return err
		}
		p := iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
			Statements: iam.GetPolicyDocumentStatementArray{
				&iam.GetPolicyDocumentStatementArgs{
					Effect: pulumi.String("Allow"),
					Actions: pulumi.StringArray{
						pulumi.String("s3:*"),
					},
					Resources: pulumi.StringArray{
						b.Arn,
						b.Arn.ApplyT(func(arn string) (string, error) {
							return fmt.Sprintf("%v/*", arn), nil
						}).(pulumi.StringOutput),
					},
				},
			},
		}, nil)
		_, err = iam.NewRolePolicy(ctx, "p", &iam.RolePolicyArgs{
			Name: pulumi.String("awsconfig-example"),
			Role: r.ID(),
			Policy: pulumi.String(p.ApplyT(func(p iam.GetPolicyDocumentResult) (*string, error) {
				return &p.Json, nil
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
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.cfg.DeliveryChannel;
import com.pulumi.aws.cfg.DeliveryChannelArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.cfg.Recorder;
import com.pulumi.aws.cfg.RecorderArgs;
import com.pulumi.aws.cfg.RecorderStatus;
import com.pulumi.aws.cfg.RecorderStatusArgs;
import com.pulumi.aws.iam.RolePolicyAttachment;
import com.pulumi.aws.iam.RolePolicyAttachmentArgs;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
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
        var b = new BucketV2("b", BucketV2Args.builder()
            .bucket("awsconfig-example")
            .build());

        var fooDeliveryChannel = new DeliveryChannel("fooDeliveryChannel", DeliveryChannelArgs.builder()
            .name("example")
            .s3BucketName(b.bucket())
            .build());

        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("config.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var r = new Role("r", RoleArgs.builder()
            .name("example-awsconfig")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var fooRecorder = new Recorder("fooRecorder", RecorderArgs.builder()
            .name("example")
            .roleArn(r.arn())
            .build());

        var foo = new RecorderStatus("foo", RecorderStatusArgs.builder()
            .name(fooRecorder.name())
            .isEnabled(true)
            .build(), CustomResourceOptions.builder()
                .dependsOn(fooDeliveryChannel)
                .build());

        var a = new RolePolicyAttachment("a", RolePolicyAttachmentArgs.builder()
            .role(r.name())
            .policyArn("arn:aws:iam::aws:policy/service-role/AWS_ConfigRole")
            .build());

        final var p = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .actions("s3:*")
                .resources(                
                    b.arn(),
                    b.arn().applyValue(arn -> String.format("%s/*", arn)))
                .build())
            .build());

        var pRolePolicy = new RolePolicy("pRolePolicy", RolePolicyArgs.builder()
            .name("awsconfig-example")
            .role(r.id())
            .policy(p.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(p -> p.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .build());

    }
}
```
```yaml
resources:
  foo:
    type: aws:cfg:RecorderStatus
    properties:
      name: ${fooRecorder.name}
      isEnabled: true
    options:
      dependsOn:
        - ${fooDeliveryChannel}
  a:
    type: aws:iam:RolePolicyAttachment
    properties:
      role: ${r.name}
      policyArn: arn:aws:iam::aws:policy/service-role/AWS_ConfigRole
  b:
    type: aws:s3:BucketV2
    properties:
      bucket: awsconfig-example
  fooDeliveryChannel:
    type: aws:cfg:DeliveryChannel
    name: foo
    properties:
      name: example
      s3BucketName: ${b.bucket}
  fooRecorder:
    type: aws:cfg:Recorder
    name: foo
    properties:
      name: example
      roleArn: ${r.arn}
  r:
    type: aws:iam:Role
    properties:
      name: example-awsconfig
      assumeRolePolicy: ${assumeRole.json}
  pRolePolicy:
    type: aws:iam:RolePolicy
    name: p
    properties:
      name: awsconfig-example
      role: ${r.id}
      policy: ${p.json}
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
                  - config.amazonaws.com
            actions:
              - sts:AssumeRole
  p:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            actions:
              - s3:*
            resources:
              - ${b.arn}
              - ${b.arn}/*
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Configuration Recorder Status using the name of the Configuration Recorder. For example:

```sh
$ pulumi import aws:cfg/recorderStatus:RecorderStatus foo example
```
S
	isEnabled
 BWhether the configuration recorder should be enabled or disabled.
'
nameB" The name of the recorder
"S
	isEnabled
 BWhether the configuration recorder should be enabled or disabled.
"%
name" The name of the recorder
*ãc
Z
cfgRemediationConfiguration9aws:cfg/remediationConfiguration:RemediationConfiguration‡NProvides an AWS Config Remediation Configuration.

> **Note:** Config Remediation Configuration requires an existing Config Rule to be present.

## Example Usage

AWS managed rules can be used by setting the source owner to `AWS` and the source identifier to the name of the managed rule. More information about AWS managed rules can be found in the [AWS Config Developer Guide](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html).

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const _this = new aws.cfg.Rule("this", {
    name: "example",
    source: {
        owner: "AWS",
        sourceIdentifier: "S3_BUCKET_VERSIONING_ENABLED",
    },
});
const thisRemediationConfiguration = new aws.cfg.RemediationConfiguration("this", {
    configRuleName: _this.name,
    resourceType: "AWS::S3::Bucket",
    targetType: "SSM_DOCUMENT",
    targetId: "AWS-EnableS3BucketEncryption",
    targetVersion: "1",
    parameters: [
        {
            name: "AutomationAssumeRole",
            staticValue: "arn:aws:iam::875924563244:role/security_config",
        },
        {
            name: "BucketName",
            resourceValue: "RESOURCE_ID",
        },
        {
            name: "SSEAlgorithm",
            staticValue: "AES256",
        },
    ],
    automatic: true,
    maximumAutomaticAttempts: 10,
    retryAttemptSeconds: 600,
    executionControls: {
        ssmControls: {
            concurrentExecutionRatePercentage: 25,
            errorPercentage: 20,
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

this = aws.cfg.Rule("this",
    name="example",
    source={
        "owner": "AWS",
        "source_identifier": "S3_BUCKET_VERSIONING_ENABLED",
    })
this_remediation_configuration = aws.cfg.RemediationConfiguration("this",
    config_rule_name=this.name,
    resource_type="AWS::S3::Bucket",
    target_type="SSM_DOCUMENT",
    target_id="AWS-EnableS3BucketEncryption",
    target_version="1",
    parameters=[
        {
            "name": "AutomationAssumeRole",
            "static_value": "arn:aws:iam::875924563244:role/security_config",
        },
        {
            "name": "BucketName",
            "resource_value": "RESOURCE_ID",
        },
        {
            "name": "SSEAlgorithm",
            "static_value": "AES256",
        },
    ],
    automatic=True,
    maximum_automatic_attempts=10,
    retry_attempt_seconds=600,
    execution_controls={
        "ssm_controls": {
            "concurrent_execution_rate_percentage": 25,
            "error_percentage": 20,
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
    var @this = new Aws.Cfg.Rule("this", new()
    {
        Name = "example",
        Source = new Aws.Cfg.Inputs.RuleSourceArgs
        {
            Owner = "AWS",
            SourceIdentifier = "S3_BUCKET_VERSIONING_ENABLED",
        },
    });

    var thisRemediationConfiguration = new Aws.Cfg.RemediationConfiguration("this", new()
    {
        ConfigRuleName = @this.Name,
        ResourceType = "AWS::S3::Bucket",
        TargetType = "SSM_DOCUMENT",
        TargetId = "AWS-EnableS3BucketEncryption",
        TargetVersion = "1",
        Parameters = new[]
        {
            new Aws.Cfg.Inputs.RemediationConfigurationParameterArgs
            {
                Name = "AutomationAssumeRole",
                StaticValue = "arn:aws:iam::875924563244:role/security_config",
            },
            new Aws.Cfg.Inputs.RemediationConfigurationParameterArgs
            {
                Name = "BucketName",
                ResourceValue = "RESOURCE_ID",
            },
            new Aws.Cfg.Inputs.RemediationConfigurationParameterArgs
            {
                Name = "SSEAlgorithm",
                StaticValue = "AES256",
            },
        },
        Automatic = true,
        MaximumAutomaticAttempts = 10,
        RetryAttemptSeconds = 600,
        ExecutionControls = new Aws.Cfg.Inputs.RemediationConfigurationExecutionControlsArgs
        {
            SsmControls = new Aws.Cfg.Inputs.RemediationConfigurationExecutionControlsSsmControlsArgs
            {
                ConcurrentExecutionRatePercentage = 25,
                ErrorPercentage = 20,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		this, err := cfg.NewRule(ctx, "this", &cfg.RuleArgs{
			Name: pulumi.String("example"),
			Source: &cfg.RuleSourceArgs{
				Owner:            pulumi.String("AWS"),
				SourceIdentifier: pulumi.String("S3_BUCKET_VERSIONING_ENABLED"),
			},
		})
		if err != nil {
			return err
		}
		_, err = cfg.NewRemediationConfiguration(ctx, "this", &cfg.RemediationConfigurationArgs{
			ConfigRuleName: this.Name,
			ResourceType:   pulumi.String("AWS::S3::Bucket"),
			TargetType:     pulumi.String("SSM_DOCUMENT"),
			TargetId:       pulumi.String("AWS-EnableS3BucketEncryption"),
			TargetVersion:  pulumi.String("1"),
			Parameters: cfg.RemediationConfigurationParameterArray{
				&cfg.RemediationConfigurationParameterArgs{
					Name:        pulumi.String("AutomationAssumeRole"),
					StaticValue: pulumi.String("arn:aws:iam::875924563244:role/security_config"),
				},
				&cfg.RemediationConfigurationParameterArgs{
					Name:          pulumi.String("BucketName"),
					ResourceValue: pulumi.String("RESOURCE_ID"),
				},
				&cfg.RemediationConfigurationParameterArgs{
					Name:        pulumi.String("SSEAlgorithm"),
					StaticValue: pulumi.String("AES256"),
				},
			},
			Automatic:                pulumi.Bool(true),
			MaximumAutomaticAttempts: pulumi.Int(10),
			RetryAttemptSeconds:      pulumi.Int(600),
			ExecutionControls: &cfg.RemediationConfigurationExecutionControlsArgs{
				SsmControls: &cfg.RemediationConfigurationExecutionControlsSsmControlsArgs{
					ConcurrentExecutionRatePercentage: pulumi.Int(25),
					ErrorPercentage:                   pulumi.Int(20),
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
import com.pulumi.aws.cfg.Rule;
import com.pulumi.aws.cfg.RuleArgs;
import com.pulumi.aws.cfg.inputs.RuleSourceArgs;
import com.pulumi.aws.cfg.RemediationConfiguration;
import com.pulumi.aws.cfg.RemediationConfigurationArgs;
import com.pulumi.aws.cfg.inputs.RemediationConfigurationParameterArgs;
import com.pulumi.aws.cfg.inputs.RemediationConfigurationExecutionControlsArgs;
import com.pulumi.aws.cfg.inputs.RemediationConfigurationExecutionControlsSsmControlsArgs;
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
        var this_ = new Rule("this", RuleArgs.builder()
            .name("example")
            .source(RuleSourceArgs.builder()
                .owner("AWS")
                .sourceIdentifier("S3_BUCKET_VERSIONING_ENABLED")
                .build())
            .build());

        var thisRemediationConfiguration = new RemediationConfiguration("thisRemediationConfiguration", RemediationConfigurationArgs.builder()
            .configRuleName(this_.name())
            .resourceType("AWS::S3::Bucket")
            .targetType("SSM_DOCUMENT")
            .targetId("AWS-EnableS3BucketEncryption")
            .targetVersion("1")
            .parameters(            
                RemediationConfigurationParameterArgs.builder()
                    .name("AutomationAssumeRole")
                    .staticValue("arn:aws:iam::875924563244:role/security_config")
                    .build(),
                RemediationConfigurationParameterArgs.builder()
                    .name("BucketName")
                    .resourceValue("RESOURCE_ID")
                    .build(),
                RemediationConfigurationParameterArgs.builder()
                    .name("SSEAlgorithm")
                    .staticValue("AES256")
                    .build())
            .automatic(true)
            .maximumAutomaticAttempts(10)
            .retryAttemptSeconds(600)
            .executionControls(RemediationConfigurationExecutionControlsArgs.builder()
                .ssmControls(RemediationConfigurationExecutionControlsSsmControlsArgs.builder()
                    .concurrentExecutionRatePercentage(25)
                    .errorPercentage(20)
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  this:
    type: aws:cfg:Rule
    properties:
      name: example
      source:
        owner: AWS
        sourceIdentifier: S3_BUCKET_VERSIONING_ENABLED
  thisRemediationConfiguration:
    type: aws:cfg:RemediationConfiguration
    name: this
    properties:
      configRuleName: ${this.name}
      resourceType: AWS::S3::Bucket
      targetType: SSM_DOCUMENT
      targetId: AWS-EnableS3BucketEncryption
      targetVersion: '1'
      parameters:
        - name: AutomationAssumeRole
          staticValue: arn:aws:iam::875924563244:role/security_config
        - name: BucketName
          resourceValue: RESOURCE_ID
        - name: SSEAlgorithm
          staticValue: AES256
      automatic: true
      maximumAutomaticAttempts: 10
      retryAttemptSeconds: 600
      executionControls:
        ssmControls:
          concurrentExecutionRatePercentage: 25
          errorPercentage: 20
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Remediation Configurations using the name config_rule_name. For example:

```sh
$ pulumi import aws:cfg/remediationConfiguration:RemediationConfiguration this example
```
E
	automaticB
 2Remediation is triggered automatically if `true`.
3
configRuleName" Name of the AWS Config rule.
Â
executionControlsñBì:ê
ç
cfg)RemediationConfigurationExecutionControls[aws:cfg/RemediationConfigurationExecutionControls:RemediationConfigurationExecutionControls7Configuration block for execution controls. See below.
ã
maximumAutomaticAttemptsB iMaximum number of failed attempts for auto-remediation. If you do not select a number, the default is 5.


parameters}B{*y:w
u
cfg!RemediationConfigurationParameterKaws:cfg/RemediationConfigurationParameter:RemediationConfigurationParametercCan be specified multiple times for each parameter. Each parameter block supports arguments below.
(
resourceTypeB" Type of resource.
ñ
retryAttemptSecondsB yMaximum time in seconds that AWS Config runs auto-remediation. If you do not select a number, the default is 60 seconds.
>
targetId" .Target ID is the name of the public document.
Ö

targetType" sType of the target. Target executes remediation. For example, SSM document.

The following arguments are optional:
W
targetVersionB" @Version of the target. For example, version of the SSM document
"8
arn" -ARN of the Config Remediation Configuration.
"E
	automaticB
 2Remediation is triggered automatically if `true`.
"3
configRuleName" Name of the AWS Config rule.
"Â
executionControlsñBì:ê
ç
cfg)RemediationConfigurationExecutionControls[aws:cfg/RemediationConfigurationExecutionControls:RemediationConfigurationExecutionControls7Configuration block for execution controls. See below.
"ã
maximumAutomaticAttemptsB iMaximum number of failed attempts for auto-remediation. If you do not select a number, the default is 5.
"

parameters}B{*y:w
u
cfg!RemediationConfigurationParameterKaws:cfg/RemediationConfigurationParameter:RemediationConfigurationParametercCan be specified multiple times for each parameter. Each parameter block supports arguments below.
"(
resourceTypeB" Type of resource.
"ñ
retryAttemptSecondsB yMaximum time in seconds that AWS Config runs auto-remediation. If you do not select a number, the default is 60 seconds.
">
targetId" .Target ID is the name of the public document.
"Ö

targetType" sType of the target. Target executes remediation. For example, SSM document.

The following arguments are optional:
"W
targetVersionB" @Version of the target. For example, version of the SSM document
*À
T
cfgRetentionConfiguration5aws:cfg/retentionConfiguration:RetentionConfiguration‘Provides a resource to manage the AWS Config retention configuration.
The retention configuration defines the number of days that AWS Config stores historical information.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cfg.RetentionConfiguration("example", {retentionPeriodInDays: 90});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cfg.RetentionConfiguration("example", retention_period_in_days=90)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cfg.RetentionConfiguration("example", new()
    {
        RetentionPeriodInDays = 90,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cfg.NewRetentionConfiguration(ctx, "example", &cfg.RetentionConfigurationArgs{
			RetentionPeriodInDays: pulumi.Int(90),
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
import com.pulumi.aws.cfg.RetentionConfiguration;
import com.pulumi.aws.cfg.RetentionConfigurationArgs;
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
        var example = new RetentionConfiguration("example", RetentionConfigurationArgs.builder()
            .retentionPeriodInDays(90)
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cfg:RetentionConfiguration
    properties:
      retentionPeriodInDays: 90
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import the AWS Config retention configuration using the `name`. For example:

```sh
$ pulumi import aws:cfg/retentionConfiguration:RetentionConfiguration example default
```
Z
retentionPeriodInDays =The number of days AWS Config stores historical information.
"d
name" XThe name of the retention configuration object. The object is always named **default**.
"Z
retentionPeriodInDays =The number of days AWS Config stores historical information.
*ò∆

cfgRuleaws:cfg/rule:Ruleæ≥Provides an AWS Config Rule.

> **Note:** Config Rule requires an existing Configuration Recorder to be present. Use of `depends_on` is recommended (as shown below) to avoid race conditions.

## Example Usage

### AWS Managed Rules

AWS managed rules can be used by setting the source owner to `AWS` and the source identifier to the name of the managed rule. More information about AWS managed rules can be found in the [AWS Config Developer Guide](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html).

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["config.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const rRole = new aws.iam.Role("r", {
    name: "my-awsconfig-role",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const foo = new aws.cfg.Recorder("foo", {
    name: "example",
    roleArn: rRole.arn,
});
const r = new aws.cfg.Rule("r", {
    name: "example",
    source: {
        owner: "AWS",
        sourceIdentifier: "S3_BUCKET_VERSIONING_ENABLED",
    },
}, {
    dependsOn: [foo],
});
const p = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        actions: ["config:Put*"],
        resources: ["*"],
    }],
});
const pRolePolicy = new aws.iam.RolePolicy("p", {
    name: "my-awsconfig-policy",
    role: rRole.id,
    policy: p.then(p => p.json),
});
```
```python
import pulumi
import pulumi_aws as aws

assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["config.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
r_role = aws.iam.Role("r",
    name="my-awsconfig-role",
    assume_role_policy=assume_role.json)
foo = aws.cfg.Recorder("foo",
    name="example",
    role_arn=r_role.arn)
r = aws.cfg.Rule("r",
    name="example",
    source={
        "owner": "AWS",
        "source_identifier": "S3_BUCKET_VERSIONING_ENABLED",
    },
    opts = pulumi.ResourceOptions(depends_on=[foo]))
p = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "actions": ["config:Put*"],
    "resources": ["*"],
}])
p_role_policy = aws.iam.RolePolicy("p",
    name="my-awsconfig-policy",
    role=r_role.id,
    policy=p.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
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
                            "config.amazonaws.com",
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

    var rRole = new Aws.Iam.Role("r", new()
    {
        Name = "my-awsconfig-role",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var foo = new Aws.Cfg.Recorder("foo", new()
    {
        Name = "example",
        RoleArn = rRole.Arn,
    });

    var r = new Aws.Cfg.Rule("r", new()
    {
        Name = "example",
        Source = new Aws.Cfg.Inputs.RuleSourceArgs
        {
            Owner = "AWS",
            SourceIdentifier = "S3_BUCKET_VERSIONING_ENABLED",
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            foo,
        },
    });

    var p = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Actions = new[]
                {
                    "config:Put*",
                },
                Resources = new[]
                {
                    "*",
                },
            },
        },
    });

    var pRolePolicy = new Aws.Iam.RolePolicy("p", new()
    {
        Name = "my-awsconfig-policy",
        Role = rRole.Id,
        Policy = p.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		assumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"config.amazonaws.com",
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
		rRole, err := iam.NewRole(ctx, "r", &iam.RoleArgs{
			Name:             pulumi.String("my-awsconfig-role"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		foo, err := cfg.NewRecorder(ctx, "foo", &cfg.RecorderArgs{
			Name:    pulumi.String("example"),
			RoleArn: rRole.Arn,
		})
		if err != nil {
			return err
		}
		_, err = cfg.NewRule(ctx, "r", &cfg.RuleArgs{
			Name: pulumi.String("example"),
			Source: &cfg.RuleSourceArgs{
				Owner:            pulumi.String("AWS"),
				SourceIdentifier: pulumi.String("S3_BUCKET_VERSIONING_ENABLED"),
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			foo,
		}))
		if err != nil {
			return err
		}
		p, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Actions: []string{
						"config:Put*",
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
		_, err = iam.NewRolePolicy(ctx, "p", &iam.RolePolicyArgs{
			Name:   pulumi.String("my-awsconfig-policy"),
			Role:   rRole.ID(),
			Policy: pulumi.String(p.Json),
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
import com.pulumi.aws.cfg.Recorder;
import com.pulumi.aws.cfg.RecorderArgs;
import com.pulumi.aws.cfg.Rule;
import com.pulumi.aws.cfg.RuleArgs;
import com.pulumi.aws.cfg.inputs.RuleSourceArgs;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
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
        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("config.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var rRole = new Role("rRole", RoleArgs.builder()
            .name("my-awsconfig-role")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var foo = new Recorder("foo", RecorderArgs.builder()
            .name("example")
            .roleArn(rRole.arn())
            .build());

        var r = new Rule("r", RuleArgs.builder()
            .name("example")
            .source(RuleSourceArgs.builder()
                .owner("AWS")
                .sourceIdentifier("S3_BUCKET_VERSIONING_ENABLED")
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(foo)
                .build());

        final var p = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .actions("config:Put*")
                .resources("*")
                .build())
            .build());

        var pRolePolicy = new RolePolicy("pRolePolicy", RolePolicyArgs.builder()
            .name("my-awsconfig-policy")
            .role(rRole.id())
            .policy(p.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

    }
}
```
```yaml
resources:
  r:
    type: aws:cfg:Rule
    properties:
      name: example
      source:
        owner: AWS
        sourceIdentifier: S3_BUCKET_VERSIONING_ENABLED
    options:
      dependsOn:
        - ${foo}
  foo:
    type: aws:cfg:Recorder
    properties:
      name: example
      roleArn: ${rRole.arn}
  rRole:
    type: aws:iam:Role
    name: r
    properties:
      name: my-awsconfig-role
      assumeRolePolicy: ${assumeRole.json}
  pRolePolicy:
    type: aws:iam:RolePolicy
    name: p
    properties:
      name: my-awsconfig-policy
      role: ${rRole.id}
      policy: ${p.json}
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
                  - config.amazonaws.com
            actions:
              - sts:AssumeRole
  p:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            actions:
              - config:Put*
            resources:
              - '*'
```
<!--End PulumiCodeChooser -->

### Custom Rules

Custom rules can be used by setting the source owner to `CUSTOM_LAMBDA` and the source identifier to the Amazon Resource Name (ARN) of the Lambda Function. The AWS Config service must have permissions to invoke the Lambda Function, e.g., via the `aws.lambda.Permission` resource. More information about custom rules can be found in the [AWS Config Developer Guide](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_develop-rules.html).

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cfg.Recorder("example", {});
const exampleFunction = new aws.lambda.Function("example", {});
const examplePermission = new aws.lambda.Permission("example", {
    action: "lambda:InvokeFunction",
    "function": exampleFunction.arn,
    principal: "config.amazonaws.com",
    statementId: "AllowExecutionFromConfig",
});
const exampleRule = new aws.cfg.Rule("example", {source: {
    owner: "CUSTOM_LAMBDA",
    sourceIdentifier: exampleFunction.arn,
}}, {
    dependsOn: [
        example,
        examplePermission,
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cfg.Recorder("example")
example_function = aws.lambda_.Function("example")
example_permission = aws.lambda_.Permission("example",
    action="lambda:InvokeFunction",
    function=example_function.arn,
    principal="config.amazonaws.com",
    statement_id="AllowExecutionFromConfig")
example_rule = aws.cfg.Rule("example", source={
    "owner": "CUSTOM_LAMBDA",
    "source_identifier": example_function.arn,
},
opts = pulumi.ResourceOptions(depends_on=[
        example,
        example_permission,
    ]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cfg.Recorder("example");

    var exampleFunction = new Aws.Lambda.Function("example");

    var examplePermission = new Aws.Lambda.Permission("example", new()
    {
        Action = "lambda:InvokeFunction",
        Function = exampleFunction.Arn,
        Principal = "config.amazonaws.com",
        StatementId = "AllowExecutionFromConfig",
    });

    var exampleRule = new Aws.Cfg.Rule("example", new()
    {
        Source = new Aws.Cfg.Inputs.RuleSourceArgs
        {
            Owner = "CUSTOM_LAMBDA",
            SourceIdentifier = exampleFunction.Arn,
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            example,
            examplePermission,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/lambda"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := cfg.NewRecorder(ctx, "example", nil)
		if err != nil {
			return err
		}
		exampleFunction, err := lambda.NewFunction(ctx, "example", nil)
		if err != nil {
			return err
		}
		examplePermission, err := lambda.NewPermission(ctx, "example", &lambda.PermissionArgs{
			Action:      pulumi.String("lambda:InvokeFunction"),
			Function:    exampleFunction.Arn,
			Principal:   pulumi.String("config.amazonaws.com"),
			StatementId: pulumi.String("AllowExecutionFromConfig"),
		})
		if err != nil {
			return err
		}
		_, err = cfg.NewRule(ctx, "example", &cfg.RuleArgs{
			Source: &cfg.RuleSourceArgs{
				Owner:            pulumi.String("CUSTOM_LAMBDA"),
				SourceIdentifier: exampleFunction.Arn,
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			example,
			examplePermission,
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
import com.pulumi.aws.cfg.Recorder;
import com.pulumi.aws.lambda.Function;
import com.pulumi.aws.lambda.Permission;
import com.pulumi.aws.lambda.PermissionArgs;
import com.pulumi.aws.cfg.Rule;
import com.pulumi.aws.cfg.RuleArgs;
import com.pulumi.aws.cfg.inputs.RuleSourceArgs;
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
        var example = new Recorder("example");

        var exampleFunction = new Function("exampleFunction");

        var examplePermission = new Permission("examplePermission", PermissionArgs.builder()
            .action("lambda:InvokeFunction")
            .function(exampleFunction.arn())
            .principal("config.amazonaws.com")
            .statementId("AllowExecutionFromConfig")
            .build());

        var exampleRule = new Rule("exampleRule", RuleArgs.builder()
            .source(RuleSourceArgs.builder()
                .owner("CUSTOM_LAMBDA")
                .sourceIdentifier(exampleFunction.arn())
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(                
                    example,
                    examplePermission)
                .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cfg:Recorder
  exampleFunction:
    type: aws:lambda:Function
    name: example
  examplePermission:
    type: aws:lambda:Permission
    name: example
    properties:
      action: lambda:InvokeFunction
      function: ${exampleFunction.arn}
      principal: config.amazonaws.com
      statementId: AllowExecutionFromConfig
  exampleRule:
    type: aws:cfg:Rule
    name: example
    properties:
      source:
        owner: CUSTOM_LAMBDA
        sourceIdentifier: ${exampleFunction.arn}
    options:
      dependsOn:
        - ${example}
        - ${examplePermission}
```
<!--End PulumiCodeChooser -->

### Custom Policies

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cfg.Rule("example", {
    name: "example",
    source: {
        owner: "CUSTOM_POLICY",
        sourceDetails: [{
            messageType: "ConfigurationItemChangeNotification",
        }],
        customPolicyDetails: {
            policyRuntime: "guard-2.x.x",
            policyText: `\x09  rule tableisactive when
\x09\x09  resourceType == "AWS::DynamoDB::Table" {
\x09\x09  configuration.tableStatus == ['ACTIVE']
\x09  }
\x09  
\x09  rule checkcompliance when
\x09\x09  resourceType == "AWS::DynamoDB::Table"
\x09\x09  tableisactive {
\x09\x09\x09  supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus == "ENABLED"
\x09  }
`,
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cfg.Rule("example",
    name="example",
    source={
        "owner": "CUSTOM_POLICY",
        "source_details": [{
            "message_type": "ConfigurationItemChangeNotification",
        }],
        "custom_policy_details": {
            "policy_runtime": "guard-2.x.x",
            "policy_text": """\x09  rule tableisactive when
\x09\x09  resourceType == "AWS::DynamoDB::Table" {
\x09\x09  configuration.tableStatus == ['ACTIVE']
\x09  }
\x09  
\x09  rule checkcompliance when
\x09\x09  resourceType == "AWS::DynamoDB::Table"
\x09\x09  tableisactive {
\x09\x09\x09  supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus == "ENABLED"
\x09  }
""",
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
    var example = new Aws.Cfg.Rule("example", new()
    {
        Name = "example",
        Source = new Aws.Cfg.Inputs.RuleSourceArgs
        {
            Owner = "CUSTOM_POLICY",
            SourceDetails = new[]
            {
                new Aws.Cfg.Inputs.RuleSourceSourceDetailArgs
                {
                    MessageType = "ConfigurationItemChangeNotification",
                },
            },
            CustomPolicyDetails = new Aws.Cfg.Inputs.RuleSourceCustomPolicyDetailsArgs
            {
                PolicyRuntime = "guard-2.x.x",
                PolicyText = @"	  rule tableisactive when
		  resourceType == ""AWS::DynamoDB::Table"" {
		  configuration.tableStatus == ['ACTIVE']
	  }
	  
	  rule checkcompliance when
		  resourceType == ""AWS::DynamoDB::Table""
		  tableisactive {
			  supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus == ""ENABLED""
	  }
",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cfg"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cfg.NewRule(ctx, "example", &cfg.RuleArgs{
			Name: pulumi.String("example"),
			Source: &cfg.RuleSourceArgs{
				Owner: pulumi.String("CUSTOM_POLICY"),
				SourceDetails: cfg.RuleSourceSourceDetailArray{
					&cfg.RuleSourceSourceDetailArgs{
						MessageType: pulumi.String("ConfigurationItemChangeNotification"),
					},
				},
				CustomPolicyDetails: &cfg.RuleSourceCustomPolicyDetailsArgs{
					PolicyRuntime: pulumi.String("guard-2.x.x"),
					PolicyText: pulumi.String(`	  rule tableisactive when
		  resourceType == "AWS::DynamoDB::Table" {
		  configuration.tableStatus == ['ACTIVE']
	  }
	  
	  rule checkcompliance when
		  resourceType == "AWS::DynamoDB::Table"
		  tableisactive {
			  supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus == "ENABLED"
	  }
`),
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
import com.pulumi.aws.cfg.Rule;
import com.pulumi.aws.cfg.RuleArgs;
import com.pulumi.aws.cfg.inputs.RuleSourceArgs;
import com.pulumi.aws.cfg.inputs.RuleSourceCustomPolicyDetailsArgs;
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
        var example = new Rule("example", RuleArgs.builder()
            .name("example")
            .source(RuleSourceArgs.builder()
                .owner("CUSTOM_POLICY")
                .sourceDetails(RuleSourceSourceDetailArgs.builder()
                    .messageType("ConfigurationItemChangeNotification")
                    .build())
                .customPolicyDetails(RuleSourceCustomPolicyDetailsArgs.builder()
                    .policyRuntime("guard-2.x.x")
                    .policyText("""
	  rule tableisactive when
		  resourceType == "AWS::DynamoDB::Table" {
		  configuration.tableStatus == ['ACTIVE']
	  }
	  
	  rule checkcompliance when
		  resourceType == "AWS::DynamoDB::Table"
		  tableisactive {
			  supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus == "ENABLED"
	  }
                    """)
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cfg:Rule
    properties:
      name: example
      source:
        owner: CUSTOM_POLICY
        sourceDetails:
          - messageType: ConfigurationItemChangeNotification
        customPolicyDetails:
          policyRuntime: guard-2.x.x
          policyText: "\t  rule tableisactive when\n\t\t  resourceType == \"AWS::DynamoDB::Table\" {\n\t\t  configuration.tableStatus == ['ACTIVE']\n\t  }\n\t  \n\t  rule checkcompliance when\n\t\t  resourceType == \"AWS::DynamoDB::Table\"\n\t\t  tableisactive {\n\t\t\t  supplementaryConfiguration.ContinuousBackupsDescription.pointInTimeRecoveryDescription.pointInTimeRecoveryStatus == \"ENABLED\"\n\t  }\n"
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Config Rule using the name. For example:

```sh
$ pulumi import aws:cfg/rule:Rule foo example
```
-
descriptionB" Description of the rule
∫
evaluationModesPBN*L:J
H
cfgRuleEvaluationMode-aws:cfg/RuleEvaluationMode:RuleEvaluationModeUThe modes the Config rule can be evaluated in. See Evaluation Mode for more details.
h
inputParametersB" OA string in JSON format that is passed to the AWS Config rule Lambda function.
l
maximumExecutionFrequencyB" IThe maximum frequency with which AWS Config runs evaluations for a rule.
#
nameB" The name of the rule
ï
scope3B1:/
-
cfg	RuleScopeaws:cfg/RuleScope:RuleScopeWScope defines which resources can trigger an evaluation for the rule. See Scope Below.
◊
source4:2
0
cfg
RuleSourceaws:cfg/RuleSource:RuleSourceñSource specifies the rule owner, the rule identifier, and the notifications that cause the function to evaluate your AWS resources. See Source Below.
–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"&
arn" The ARN of the config rule
"-
descriptionB" Description of the rule
"∏
evaluationModesN*L:J
H
cfgRuleEvaluationMode-aws:cfg/RuleEvaluationMode:RuleEvaluationModeUThe modes the Config rule can be evaluated in. See Evaluation Mode for more details.
"h
inputParametersB" OA string in JSON format that is passed to the AWS Config rule Lambda function.
"l
maximumExecutionFrequencyB" IThe maximum frequency with which AWS Config runs evaluations for a rule.
"!
name" The name of the rule
"(
ruleId" The ID of the config rule
"ï
scope3B1:/
-
cfg	RuleScopeaws:cfg/RuleScope:RuleScopeWScope defines which resources can trigger an evaluation for the rule. See Scope Below.
"◊
source4:2
0
cfg
RuleSourceaws:cfg/RuleSource:RuleSourceñSource specifies the rule owner, the rule identifier, and the notifications that cause the function to evaluate your AWS resources. See Source Below.
"–
tagsB2" øA map of tags to assign to the resource. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*”.
e
chatbotSlackChannelConfiguration?aws:chatbot/slackChannelConfiguration:SlackChannelConfigurationáResource for managing an AWS Chatbot Slack Channel Configuration.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.chatbot.SlackChannelConfiguration("test", {
    configurationName: "min-slaka-kanal",
    iamRoleArn: testAwsIamRole.arn,
    slackChannelId: "C07EZ1ABC23",
    slackTeamId: "T07EA123LEP",
    tags: {
        Name: "min-slaka-kanal",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.chatbot.SlackChannelConfiguration("test",
    configuration_name="min-slaka-kanal",
    iam_role_arn=test_aws_iam_role["arn"],
    slack_channel_id="C07EZ1ABC23",
    slack_team_id="T07EA123LEP",
    tags={
        "Name": "min-slaka-kanal",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Chatbot.SlackChannelConfiguration("test", new()
    {
        ConfigurationName = "min-slaka-kanal",
        IamRoleArn = testAwsIamRole.Arn,
        SlackChannelId = "C07EZ1ABC23",
        SlackTeamId = "T07EA123LEP",
        Tags = 
        {
            { "Name", "min-slaka-kanal" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chatbot"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chatbot.NewSlackChannelConfiguration(ctx, "test", &chatbot.SlackChannelConfigurationArgs{
			ConfigurationName: pulumi.String("min-slaka-kanal"),
			IamRoleArn:        pulumi.Any(testAwsIamRole.Arn),
			SlackChannelId:    pulumi.String("C07EZ1ABC23"),
			SlackTeamId:       pulumi.String("T07EA123LEP"),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("min-slaka-kanal"),
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
import com.pulumi.aws.chatbot.SlackChannelConfiguration;
import com.pulumi.aws.chatbot.SlackChannelConfigurationArgs;
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
        var test = new SlackChannelConfiguration("test", SlackChannelConfigurationArgs.builder()
            .configurationName("min-slaka-kanal")
            .iamRoleArn(testAwsIamRole.arn())
            .slackChannelId("C07EZ1ABC23")
            .slackTeamId("T07EA123LEP")
            .tags(Map.of("Name", "min-slaka-kanal"))
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:chatbot:SlackChannelConfiguration
    properties:
      configurationName: min-slaka-kanal
      iamRoleArn: ${testAwsIamRole.arn}
      slackChannelId: C07EZ1ABC23
      slackTeamId: T07EA123LEP
      tags:
        Name: min-slaka-kanal
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Chatbot Slack Channel Configuration using the `chat_configuration_arn`. For example:

```sh
$ pulumi import aws:chatbot/slackChannelConfiguration:SlackChannelConfiguration example arn:aws:chatbot::123456789012:chat-configuration/slack-channel/min-slaka-kanal
```
B
configurationName" )Name of the Slack channel configuration.
∑
guardrailPolicyArnsB*" óList of IAM policy ARNs that are applied as channel guardrails. The AWS managed `AdministratorAccess` policy is applied by default if this is not set.
c

iamRoleArn" QUser-defined role that AWS Chatbot assumes. This is not the service-linked role.
I
loggingLevelB" 3Logging levels include `ERROR`, `INFO`, or `NONE`.
K
slackChannelId" 5ID of the Slack channel. For example, `C07EZ1ABC23`.
ç
slackTeamId" zID of the Slack workspace authorized with AWS Chatbot. For example, `T07EA123LEP`.

The following arguments are optional:
Z
snsTopicArnsB*" BARNs of the SNS topics that deliver notifications to AWS Chatbot.
6
tagsB2" &Map of tags assigned to the resource.
ë
timeoutsÑBÅ:
}
chatbot!SlackChannelConfigurationTimeoutsOaws:chatbot/SlackChannelConfigurationTimeouts:SlackChannelConfigurationTimeoutsf
userAuthorizationRequiredB
 CEnables use of a user role requirement in your chat configuration.
"D
chatConfigurationArn" (ARN of the Slack channel configuration.
"B
configurationName" )Name of the Slack channel configuration.
"µ
guardrailPolicyArns*" óList of IAM policy ARNs that are applied as channel guardrails. The AWS managed `AdministratorAccess` policy is applied by default if this is not set.
"c

iamRoleArn" QUser-defined role that AWS Chatbot assumes. This is not the service-linked role.
"G
loggingLevel" 3Logging levels include `ERROR`, `INFO`, or `NONE`.
"K
slackChannelId" 5ID of the Slack channel. For example, `C07EZ1ABC23`.
"3
slackChannelName" Name of the Slack channel.
"ç
slackTeamId" zID of the Slack workspace authorized with AWS Chatbot. For example, `T07EA123LEP`.

The following arguments are optional:
"-
slackTeamName" Name of the Slack team.
"X
snsTopicArns*" BARNs of the SNS topics that deliver notifications to AWS Chatbot.
"6
tagsB2" &Map of tags assigned to the resource.
"á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"ë
timeoutsÑBÅ:
}
chatbot!SlackChannelConfigurationTimeoutsOaws:chatbot/SlackChannelConfigurationTimeouts:SlackChannelConfigurationTimeouts"d
userAuthorizationRequired
 CEnables use of a user role requirement in your chat configuration.
*€6
e
chatbotTeamsChannelConfiguration?aws:chatbot/teamsChannelConfiguration:TeamsChannelConfiguration˚Resource for managing an AWS Chatbot Microsoft Teams Channel Configuration.

> **NOTE:** We provide this resource on a best-effort basis. If you are able to test it and find it useful, we welcome your input at GitHub.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.chatbot.TeamsChannelConfiguration("test", {
    channelId: "C07EZ1ABC23",
    configurationName: "mitt-lags-kanal",
    iamRoleArn: testAwsIamRole.arn,
    teamId: "74361522-da01-538d-aa2e-ac7918c6bb92",
    tenantId: "1234",
    tags: {
        Name: "mitt-lags-kanal",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.chatbot.TeamsChannelConfiguration("test",
    channel_id="C07EZ1ABC23",
    configuration_name="mitt-lags-kanal",
    iam_role_arn=test_aws_iam_role["arn"],
    team_id="74361522-da01-538d-aa2e-ac7918c6bb92",
    tenant_id="1234",
    tags={
        "Name": "mitt-lags-kanal",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Chatbot.TeamsChannelConfiguration("test", new()
    {
        ChannelId = "C07EZ1ABC23",
        ConfigurationName = "mitt-lags-kanal",
        IamRoleArn = testAwsIamRole.Arn,
        TeamId = "74361522-da01-538d-aa2e-ac7918c6bb92",
        TenantId = "1234",
        Tags = 
        {
            { "Name", "mitt-lags-kanal" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chatbot"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chatbot.NewTeamsChannelConfiguration(ctx, "test", &chatbot.TeamsChannelConfigurationArgs{
			ChannelId:         pulumi.String("C07EZ1ABC23"),
			ConfigurationName: pulumi.String("mitt-lags-kanal"),
			IamRoleArn:        pulumi.Any(testAwsIamRole.Arn),
			TeamId:            pulumi.String("74361522-da01-538d-aa2e-ac7918c6bb92"),
			TenantId:          pulumi.String("1234"),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("mitt-lags-kanal"),
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
import com.pulumi.aws.chatbot.TeamsChannelConfiguration;
import com.pulumi.aws.chatbot.TeamsChannelConfigurationArgs;
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
        var test = new TeamsChannelConfiguration("test", TeamsChannelConfigurationArgs.builder()
            .channelId("C07EZ1ABC23")
            .configurationName("mitt-lags-kanal")
            .iamRoleArn(testAwsIamRole.arn())
            .teamId("74361522-da01-538d-aa2e-ac7918c6bb92")
            .tenantId("1234")
            .tags(Map.of("Name", "mitt-lags-kanal"))
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:chatbot:TeamsChannelConfiguration
    properties:
      channelId: C07EZ1ABC23
      configurationName: mitt-lags-kanal
      iamRoleArn: ${testAwsIamRole.arn}
      teamId: 74361522-da01-538d-aa2e-ac7918c6bb92
      tenantId: '1234'
      tags:
        Name: mitt-lags-kanal
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Chatbot Microsoft Teams Channel Configuration using the `team_id`. For example:

```sh
$ pulumi import aws:chatbot/teamsChannelConfiguration:TeamsChannelConfiguration example 5f4f15d2-b958-522a-8333-124aa8bf0925
```
4
	channelId" #ID of the Microsoft Teams channel.
:
channelNameB" %Name of the Microsoft Teams channel.
L
configurationName" 3Name of the Microsoft Teams channel configuration.
∑
guardrailPolicyArnsB*" óList of IAM policy ARNs that are applied as channel guardrails. The AWS managed `AdministratorAccess` policy is applied by default if this is not set.
¥

iamRoleArn" °ARN of the IAM role that defines the permissions for AWS Chatbot. This is a user-defined role that AWS Chatbot will assume. This is not the service-linked role.
I
loggingLevelB" 3Logging levels include `ERROR`, `INFO`, or `NONE`.
Z
snsTopicArnsB*" BARNs of the SNS topics that deliver notifications to AWS Chatbot.
6
tagsB2" &Map of tags assigned to the resource.
Ù
teamId" ÂID of the Microsoft Team authorized with AWS Chatbot. To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console.
4
teamNameB" "Name of the Microsoft Teams team.
Y
tenantId" IID of the Microsoft Teams tenant.

The following arguments are optional:
ë
timeoutsÑBÅ:
}
chatbot!TeamsChannelConfigurationTimeoutsOaws:chatbot/TeamsChannelConfigurationTimeouts:TeamsChannelConfigurationTimeoutsf
userAuthorizationRequiredB
 CEnables use of a user role requirement in your chat configuration.
"4
	channelId" #ID of the Microsoft Teams channel.
"8
channelName" %Name of the Microsoft Teams channel.
"N
chatConfigurationArn" 2ARN of the Microsoft Teams channel configuration.
"L
configurationName" 3Name of the Microsoft Teams channel configuration.
"µ
guardrailPolicyArns*" óList of IAM policy ARNs that are applied as channel guardrails. The AWS managed `AdministratorAccess` policy is applied by default if this is not set.
"¥

iamRoleArn" °ARN of the IAM role that defines the permissions for AWS Chatbot. This is a user-defined role that AWS Chatbot will assume. This is not the service-linked role.
"G
loggingLevel" 3Logging levels include `ERROR`, `INFO`, or `NONE`.
"X
snsTopicArns*" BARNs of the SNS topics that deliver notifications to AWS Chatbot.
"6
tagsB2" &Map of tags assigned to the resource.
"á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"Ù
teamId" ÂID of the Microsoft Team authorized with AWS Chatbot. To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console.
"2
teamName" "Name of the Microsoft Teams team.
"Y
tenantId" IID of the Microsoft Teams tenant.

The following arguments are optional:
"ë
timeoutsÑBÅ:
}
chatbot!TeamsChannelConfigurationTimeoutsOaws:chatbot/TeamsChannelConfigurationTimeouts:TeamsChannelConfigurationTimeouts"d
userAuthorizationRequired
 CEnables use of a user role requirement in your chat configuration.
*ä
X
chimeSdkvoiceGlobalSettings7aws:chime/sdkvoiceGlobalSettings:SdkvoiceGlobalSettingsáResource for managing Amazon Chime SDK Voice Global Settings.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.chime.SdkvoiceGlobalSettings("example", {voiceConnector: {
    cdrBucket: "example-bucket-name",
}});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.chime.SdkvoiceGlobalSettings("example", voice_connector={
    "cdr_bucket": "example-bucket-name",
})
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Chime.SdkvoiceGlobalSettings("example", new()
    {
        VoiceConnector = new Aws.Chime.Inputs.SdkvoiceGlobalSettingsVoiceConnectorArgs
        {
            CdrBucket = "example-bucket-name",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chime"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chime.NewSdkvoiceGlobalSettings(ctx, "example", &chime.SdkvoiceGlobalSettingsArgs{
			VoiceConnector: &chime.SdkvoiceGlobalSettingsVoiceConnectorArgs{
				CdrBucket: pulumi.String("example-bucket-name"),
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
import com.pulumi.aws.chime.SdkvoiceGlobalSettings;
import com.pulumi.aws.chime.SdkvoiceGlobalSettingsArgs;
import com.pulumi.aws.chime.inputs.SdkvoiceGlobalSettingsVoiceConnectorArgs;
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
        var example = new SdkvoiceGlobalSettings("example", SdkvoiceGlobalSettingsArgs.builder()
            .voiceConnector(SdkvoiceGlobalSettingsVoiceConnectorArgs.builder()
                .cdrBucket("example-bucket-name")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:chime:SdkvoiceGlobalSettings
    properties:
      voiceConnector:
        cdrBucket: example-bucket-name
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AWS Chime SDK Voice Global Settings using the `id` (AWS account ID). For example:

```sh
$ pulumi import aws:chime/sdkvoiceGlobalSettings:SdkvoiceGlobalSettings example 123456789012
```
–
voiceConnectorà:Ö
Ç
chime$SdkvoiceGlobalSettingsVoiceConnectorSaws:chime/SdkvoiceGlobalSettingsVoiceConnector:SdkvoiceGlobalSettingsVoiceConnector3The Voice Connector settings. See voice_connector.
"–
voiceConnectorà:Ö
Ç
chime$SdkvoiceGlobalSettingsVoiceConnectorSaws:chime/SdkvoiceGlobalSettingsVoiceConnector:SdkvoiceGlobalSettingsVoiceConnector3The Voice Connector settings. See voice_connector.
*Ü'
g
chimeSdkvoiceSipMediaApplicationAaws:chime/sdkvoiceSipMediaApplication:SdkvoiceSipMediaApplicationãA ChimeSDKVoice SIP Media Application is a managed object that passes values from a SIP rule to a target AWS Lambda function.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.chime.SdkvoiceSipMediaApplication("example", {
    awsRegion: "us-east-1",
    name: "example-sip-media-application",
    endpoints: {
        lambdaArn: test.arn,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.chime.SdkvoiceSipMediaApplication("example",
    aws_region="us-east-1",
    name="example-sip-media-application",
    endpoints={
        "lambda_arn": test["arn"],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Chime.SdkvoiceSipMediaApplication("example", new()
    {
        AwsRegion = "us-east-1",
        Name = "example-sip-media-application",
        Endpoints = new Aws.Chime.Inputs.SdkvoiceSipMediaApplicationEndpointsArgs
        {
            LambdaArn = test.Arn,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chime"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chime.NewSdkvoiceSipMediaApplication(ctx, "example", &chime.SdkvoiceSipMediaApplicationArgs{
			AwsRegion: pulumi.String("us-east-1"),
			Name:      pulumi.String("example-sip-media-application"),
			Endpoints: &chime.SdkvoiceSipMediaApplicationEndpointsArgs{
				LambdaArn: pulumi.Any(test.Arn),
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
import com.pulumi.aws.chime.SdkvoiceSipMediaApplication;
import com.pulumi.aws.chime.SdkvoiceSipMediaApplicationArgs;
import com.pulumi.aws.chime.inputs.SdkvoiceSipMediaApplicationEndpointsArgs;
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
        var example = new SdkvoiceSipMediaApplication("example", SdkvoiceSipMediaApplicationArgs.builder()
            .awsRegion("us-east-1")
            .name("example-sip-media-application")
            .endpoints(SdkvoiceSipMediaApplicationEndpointsArgs.builder()
                .lambdaArn(test.arn())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:chime:SdkvoiceSipMediaApplication
    properties:
      awsRegion: us-east-1
      name: example-sip-media-application
      endpoints:
        lambdaArn: ${test.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a ChimeSDKVoice SIP Media Application using the `id`. For example:

```sh
$ pulumi import aws:chime/sdkvoiceSipMediaApplication:SdkvoiceSipMediaApplication example abcdef123456
```
c
	awsRegion" RThe AWS Region in which the AWS Chime SDK Voice Sip Media Application is created.
Ø
	endpointsà:Ö
Ç
chime$SdkvoiceSipMediaApplicationEndpointsSaws:chime/SdkvoiceSipMediaApplicationEndpoints:SdkvoiceSipMediaApplicationEndpointsñList of endpoints (Lambda Amazon Resource Names) specified for the SIP media application. Currently, only one endpoint is supported. See `endpoints`.
p
nameB" bThe name of the AWS Chime SDK Voice Sip Media Application.

The following arguments are optional:
À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"W
arn" LARN (Amazon Resource Name) of the AWS Chime SDK Voice Sip Media Application
"c
	awsRegion" RThe AWS Region in which the AWS Chime SDK Voice Sip Media Application is created.
"Ø
	endpointsà:Ö
Ç
chime$SdkvoiceSipMediaApplicationEndpointsSaws:chime/SdkvoiceSipMediaApplicationEndpoints:SdkvoiceSipMediaApplicationEndpointsñList of endpoints (Lambda Amazon Resource Names) specified for the SIP media application. Currently, only one endpoint is supported. See `endpoints`.
"n
name" bThe name of the AWS Chime SDK Voice Sip Media Application.

The following arguments are optional:
"À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*ª2
C
chimeSdkvoiceSipRule)aws:chime/sdkvoiceSipRule:SdkvoiceSipRuleÂ A SIP rule associates your SIP media application with a phone number or a Request URI hostname. You can associate a SIP rule with more than one SIP media application. Each application then runs only that rule.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.chime.SdkvoiceSipRule("example", {
    name: "example-sip-rule",
    triggerType: "RequestUriHostname",
    triggerValue: example_voice_connector.outboundHostName,
    targetApplications: [{
        priority: 1,
        sipMediaApplicationId: example_sma.id,
        awsRegion: "us-east-1",
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.chime.SdkvoiceSipRule("example",
    name="example-sip-rule",
    trigger_type="RequestUriHostname",
    trigger_value=example_voice_connector["outboundHostName"],
    target_applications=[{
        "priority": 1,
        "sip_media_application_id": example_sma["id"],
        "aws_region": "us-east-1",
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Chime.SdkvoiceSipRule("example", new()
    {
        Name = "example-sip-rule",
        TriggerType = "RequestUriHostname",
        TriggerValue = example_voice_connector.OutboundHostName,
        TargetApplications = new[]
        {
            new Aws.Chime.Inputs.SdkvoiceSipRuleTargetApplicationArgs
            {
                Priority = 1,
                SipMediaApplicationId = example_sma.Id,
                AwsRegion = "us-east-1",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chime"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chime.NewSdkvoiceSipRule(ctx, "example", &chime.SdkvoiceSipRuleArgs{
			Name:         pulumi.String("example-sip-rule"),
			TriggerType:  pulumi.String("RequestUriHostname"),
			TriggerValue: pulumi.Any(example_voice_connector.OutboundHostName),
			TargetApplications: chime.SdkvoiceSipRuleTargetApplicationArray{
				&chime.SdkvoiceSipRuleTargetApplicationArgs{
					Priority:              pulumi.Int(1),
					SipMediaApplicationId: pulumi.Any(example_sma.Id),
					AwsRegion:             pulumi.String("us-east-1"),
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
import com.pulumi.aws.chime.SdkvoiceSipRule;
import com.pulumi.aws.chime.SdkvoiceSipRuleArgs;
import com.pulumi.aws.chime.inputs.SdkvoiceSipRuleTargetApplicationArgs;
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
        var example = new SdkvoiceSipRule("example", SdkvoiceSipRuleArgs.builder()
            .name("example-sip-rule")
            .triggerType("RequestUriHostname")
            .triggerValue(example_voice_connector.outboundHostName())
            .targetApplications(SdkvoiceSipRuleTargetApplicationArgs.builder()
                .priority(1)
                .sipMediaApplicationId(example_sma.id())
                .awsRegion("us-east-1")
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:chime:SdkvoiceSipRule
    properties:
      name: example-sip-rule
      triggerType: RequestUriHostname
      triggerValue: ${["example-voice-connector"].outboundHostName}
      targetApplications:
        - priority: 1
          sipMediaApplicationId: ${["example-sma"].id}
          awsRegion: us-east-1
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import a ChimeSDKVoice SIP Rule using the `id`. For example:

```sh
$ pulumi import aws:chime/sdkvoiceSipRule:SdkvoiceSipRule example abcdef123456
```
a
disabledB
 OEnables or disables a rule. You must disable rules before you can delete them.
(
nameB" The name of the SIP rule.
¢
targetApplications|*z:x
v
chime SdkvoiceSipRuleTargetApplicationKaws:chime/SdkvoiceSipRuleTargetApplication:SdkvoiceSipRuleTargetApplicationçList of SIP media applications with priority and AWS Region. Only one SIP application per AWS Region can be used. See `target_applications`.
é
triggerType" {The type of trigger assigned to the SIP rule in `trigger_value`. Valid values are `RequestUriHostname` or `ToPhoneNumber`.
Ç
triggerValue" ÌIf `trigger_type` is `RequestUriHostname`, the value can be the outbound host name of an Amazon Chime Voice Connector. If `trigger_type` is `ToPhoneNumber`, the value can be a customer-owned phone number in the E164 format. The Sip Media Application specified in the Sip Rule is triggered if the request URI in an incoming SIP request matches the `RequestUriHostname`, or if the "To" header in the incoming SIP request matches the `ToPhoneNumber` value.

The following arguments are optional:
"a
disabledB
 OEnables or disables a rule. You must disable rules before you can delete them.
"&
name" The name of the SIP rule.
"¢
targetApplications|*z:x
v
chime SdkvoiceSipRuleTargetApplicationKaws:chime/SdkvoiceSipRuleTargetApplication:SdkvoiceSipRuleTargetApplicationçList of SIP media applications with priority and AWS Region. Only one SIP application per AWS Region can be used. See `target_applications`.
"é
triggerType" {The type of trigger assigned to the SIP rule in `trigger_value`. Valid values are `RequestUriHostname` or `ToPhoneNumber`.
"Ç
triggerValue" ÌIf `trigger_type` is `RequestUriHostname`, the value can be the outbound host name of an Amazon Chime Voice Connector. If `trigger_type` is `ToPhoneNumber`, the value can be a customer-owned phone number in the E164 format. The Sip Media Application specified in the Sip Rule is triggered if the request URI in an incoming SIP request matches the `RequestUriHostname`, or if the "To" header in the incoming SIP request matches the `ToPhoneNumber` value.

The following arguments are optional:
*À.
d
chimeSdkvoiceVoiceProfileDomain?aws:chime/sdkvoiceVoiceProfileDomain:SdkvoiceVoiceProfileDomainÎ&Resource for managing an AWS Chime SDK Voice Profile Domain.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.kms.Key("example", {
    description: "KMS Key for Voice Profile Domain",
    deletionWindowInDays: 7,
});
const exampleSdkvoiceVoiceProfileDomain = new aws.chime.SdkvoiceVoiceProfileDomain("example", {
    name: "ExampleVoiceProfileDomain",
    serverSideEncryptionConfiguration: {
        kmsKeyArn: example.arn,
    },
    description: "My Voice Profile Domain",
    tags: {
        key1: "value1",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.kms.Key("example",
    description="KMS Key for Voice Profile Domain",
    deletion_window_in_days=7)
example_sdkvoice_voice_profile_domain = aws.chime.SdkvoiceVoiceProfileDomain("example",
    name="ExampleVoiceProfileDomain",
    server_side_encryption_configuration={
        "kms_key_arn": example.arn,
    },
    description="My Voice Profile Domain",
    tags={
        "key1": "value1",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Kms.Key("example", new()
    {
        Description = "KMS Key for Voice Profile Domain",
        DeletionWindowInDays = 7,
    });

    var exampleSdkvoiceVoiceProfileDomain = new Aws.Chime.SdkvoiceVoiceProfileDomain("example", new()
    {
        Name = "ExampleVoiceProfileDomain",
        ServerSideEncryptionConfiguration = new Aws.Chime.Inputs.SdkvoiceVoiceProfileDomainServerSideEncryptionConfigurationArgs
        {
            KmsKeyArn = example.Arn,
        },
        Description = "My Voice Profile Domain",
        Tags = 
        {
            { "key1", "value1" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chime"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/kms"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := kms.NewKey(ctx, "example", &kms.KeyArgs{
			Description:          pulumi.String("KMS Key for Voice Profile Domain"),
			DeletionWindowInDays: pulumi.Int(7),
		})
		if err != nil {
			return err
		}
		_, err = chime.NewSdkvoiceVoiceProfileDomain(ctx, "example", &chime.SdkvoiceVoiceProfileDomainArgs{
			Name: pulumi.String("ExampleVoiceProfileDomain"),
			ServerSideEncryptionConfiguration: &chime.SdkvoiceVoiceProfileDomainServerSideEncryptionConfigurationArgs{
				KmsKeyArn: example.Arn,
			},
			Description: pulumi.String("My Voice Profile Domain"),
			Tags: pulumi.StringMap{
				"key1": pulumi.String("value1"),
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
import com.pulumi.aws.chime.SdkvoiceVoiceProfileDomain;
import com.pulumi.aws.chime.SdkvoiceVoiceProfileDomainArgs;
import com.pulumi.aws.chime.inputs.SdkvoiceVoiceProfileDomainServerSideEncryptionConfigurationArgs;
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
        var example = new Key("example", KeyArgs.builder()
            .description("KMS Key for Voice Profile Domain")
            .deletionWindowInDays(7)
            .build());

        var exampleSdkvoiceVoiceProfileDomain = new SdkvoiceVoiceProfileDomain("exampleSdkvoiceVoiceProfileDomain", SdkvoiceVoiceProfileDomainArgs.builder()
            .name("ExampleVoiceProfileDomain")
            .serverSideEncryptionConfiguration(SdkvoiceVoiceProfileDomainServerSideEncryptionConfigurationArgs.builder()
                .kmsKeyArn(example.arn())
                .build())
            .description("My Voice Profile Domain")
            .tags(Map.of("key1", "value1"))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:kms:Key
    properties:
      description: KMS Key for Voice Profile Domain
      deletionWindowInDays: 7
  exampleSdkvoiceVoiceProfileDomain:
    type: aws:chime:SdkvoiceVoiceProfileDomain
    name: example
    properties:
      name: ExampleVoiceProfileDomain
      serverSideEncryptionConfiguration:
        kmsKeyArn: ${example.arn}
      description: My Voice Profile Domain
      tags:
        key1: value1
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import AWS Chime SDK Voice Profile Domain using the `id`. For example:

```sh
$ pulumi import aws:chime/sdkvoiceVoiceProfileDomain:SdkvoiceVoiceProfileDomain example abcdef123456
```
:
descriptionB" %Description of Voice Profile Domain.
,
nameB" Name of Voice Profile Domain.
†
!serverSideEncryptionConfigurationŒ:À
»
chime;SdkvoiceVoiceProfileDomainServerSideEncryptionConfigurationÅaws:chime/SdkvoiceVoiceProfileDomainServerSideEncryptionConfiguration:SdkvoiceVoiceProfileDomainServerSideEncryptionConfiguration*Configuration for server side encryption.

tagsB2" ",
arn" !ARN of the Voice Profile Domain.
":
descriptionB" %Description of Voice Profile Domain.
"*
name" Name of Voice Profile Domain.
"†
!serverSideEncryptionConfigurationŒ:À
»
chime;SdkvoiceVoiceProfileDomainServerSideEncryptionConfigurationÅaws:chime/SdkvoiceVoiceProfileDomainServerSideEncryptionConfiguration:SdkvoiceVoiceProfileDomainServerSideEncryptionConfiguration*Configuration for server side encryption.
"
tagsB2" "
tagsAll2" *…
@
chimeVoiceConnector'aws:chime/voiceConnector:VoiceConnector¬Enables you to connect your phone system to the telephone network at a substantial cost savings by using SIP trunking.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.chime.VoiceConnector("test", {
    name: "connector-test-1",
    requireEncryption: true,
    awsRegion: "us-east-1",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.chime.VoiceConnector("test",
    name="connector-test-1",
    require_encryption=True,
    aws_region="us-east-1")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Chime.VoiceConnector("test", new()
    {
        Name = "connector-test-1",
        RequireEncryption = true,
        AwsRegion = "us-east-1",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chime"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chime.NewVoiceConnector(ctx, "test", &chime.VoiceConnectorArgs{
			Name:              pulumi.String("connector-test-1"),
			RequireEncryption: pulumi.Bool(true),
			AwsRegion:         pulumi.String("us-east-1"),
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
import com.pulumi.aws.chime.VoiceConnector;
import com.pulumi.aws.chime.VoiceConnectorArgs;
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
        var test = new VoiceConnector("test", VoiceConnectorArgs.builder()
            .name("connector-test-1")
            .requireEncryption(true)
            .awsRegion("us-east-1")
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:chime:VoiceConnector
    properties:
      name: connector-test-1
      requireEncryption: true
      awsRegion: us-east-1
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Configuration Recorder using the name. For example:

```sh
$ pulumi import aws:chime/voiceConnector:VoiceConnector test example
```
s
	awsRegionB" `The AWS Region in which the Amazon Chime Voice Connector is created. Default value: `us-east-1`
<
nameB" .The name of the Amazon Chime Voice Connector.
à
requireEncryption
 oWhen enabled, requires encryption for the Amazon Chime Voice Connector.

The following arguments are optional:
À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"K
arn" @ARN (Amazon Resource Name) of the Amazon Chime Voice Connector.
"q
	awsRegion" `The AWS Region in which the Amazon Chime Voice Connector is created. Default value: `us-east-1`
":
name" .The name of the Amazon Chime Voice Connector.
"U
outboundHostName" =The outbound host name for the Amazon Chime Voice Connector.
"à
requireEncryption
 oWhen enabled, requires encryption for the Amazon Chime Voice Connector.

The following arguments are optional:
"À
tagsB2" ∫Key-value mapping of resource tags. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
*‰5
O
chimeVoiceConnectorGroup1aws:chime/voiceConnectorGroup:VoiceConnectorGroupÜ1Creates an Amazon Chime Voice Connector group under the administrator's AWS account. You can associate Amazon Chime Voice Connectors with the Amazon Chime Voice Connector group by including VoiceConnectorItems in the request.

You can include Amazon Chime Voice Connectors from different AWS Regions in your group. This creates a fault tolerant mechanism for fallback in case of availability events.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const vc1 = new aws.chime.VoiceConnector("vc1", {
    name: "connector-test-1",
    requireEncryption: true,
    awsRegion: "us-east-1",
});
const vc2 = new aws.chime.VoiceConnector("vc2", {
    name: "connector-test-2",
    requireEncryption: true,
    awsRegion: "us-west-2",
});
const group = new aws.chime.VoiceConnectorGroup("group", {
    name: "test-group",
    connectors: [
        {
            voiceConnectorId: vc1.id,
            priority: 1,
        },
        {
            voiceConnectorId: vc2.id,
            priority: 3,
        },
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

vc1 = aws.chime.VoiceConnector("vc1",
    name="connector-test-1",
    require_encryption=True,
    aws_region="us-east-1")
vc2 = aws.chime.VoiceConnector("vc2",
    name="connector-test-2",
    require_encryption=True,
    aws_region="us-west-2")
group = aws.chime.VoiceConnectorGroup("group",
    name="test-group",
    connectors=[
        {
            "voice_connector_id": vc1.id,
            "priority": 1,
        },
        {
            "voice_connector_id": vc2.id,
            "priority": 3,
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
    var vc1 = new Aws.Chime.VoiceConnector("vc1", new()
    {
        Name = "connector-test-1",
        RequireEncryption = true,
        AwsRegion = "us-east-1",
    });

    var vc2 = new Aws.Chime.VoiceConnector("vc2", new()
    {
        Name = "connector-test-2",
        RequireEncryption = true,
        AwsRegion = "us-west-2",
    });

    var @group = new Aws.Chime.VoiceConnectorGroup("group", new()
    {
        Name = "test-group",
        Connectors = new[]
        {
            new Aws.Chime.Inputs.VoiceConnectorGroupConnectorArgs
            {
                VoiceConnectorId = vc1.Id,
                Priority = 1,
            },
            new Aws.Chime.Inputs.VoiceConnectorGroupConnectorArgs
            {
                VoiceConnectorId = vc2.Id,
                Priority = 3,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chime"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		vc1, err := chime.NewVoiceConnector(ctx, "vc1", &chime.VoiceConnectorArgs{
			Name:              pulumi.String("connector-test-1"),
			RequireEncryption: pulumi.Bool(true),
			AwsRegion:         pulumi.String("us-east-1"),
		})
		if err != nil {
			return err
		}
		vc2, err := chime.NewVoiceConnector(ctx, "vc2", &chime.VoiceConnectorArgs{
			Name:              pulumi.String("connector-test-2"),
			RequireEncryption: pulumi.Bool(true),
			AwsRegion:         pulumi.String("us-west-2"),
		})
		if err != nil {
			return err
		}
		_, err = chime.NewVoiceConnectorGroup(ctx, "group", &chime.VoiceConnectorGroupArgs{
			Name: pulumi.String("test-group"),
			Connectors: chime.VoiceConnectorGroupConnectorArray{
				&chime.VoiceConnectorGroupConnectorArgs{
					VoiceConnectorId: vc1.ID(),
					Priority:         pulumi.Int(1),
				},
				&chime.VoiceConnectorGroupConnectorArgs{
					VoiceConnectorId: vc2.ID(),
					Priority:         pulumi.Int(3),
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
import com.pulumi.aws.chime.VoiceConnector;
import com.pulumi.aws.chime.VoiceConnectorArgs;
import com.pulumi.aws.chime.VoiceConnectorGroup;
import com.pulumi.aws.chime.VoiceConnectorGroupArgs;
import com.pulumi.aws.chime.inputs.VoiceConnectorGroupConnectorArgs;
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
        var vc1 = new VoiceConnector("vc1", VoiceConnectorArgs.builder()
            .name("connector-test-1")
            .requireEncryption(true)
            .awsRegion("us-east-1")
            .build());

        var vc2 = new VoiceConnector("vc2", VoiceConnectorArgs.builder()
            .name("connector-test-2")
            .requireEncryption(true)
            .awsRegion("us-west-2")
            .build());

        var group = new VoiceConnectorGroup("group", VoiceConnectorGroupArgs.builder()
            .name("test-group")
            .connectors(            
                VoiceConnectorGroupConnectorArgs.builder()
                    .voiceConnectorId(vc1.id())
                    .priority(1)
                    .build(),
                VoiceConnectorGroupConnectorArgs.builder()
                    .voiceConnectorId(vc2.id())
                    .priority(3)
                    .build())
            .build());

    }
}
```
```yaml
resources:
  vc1:
    type: aws:chime:VoiceConnector
    properties:
      name: connector-test-1
      requireEncryption: true
      awsRegion: us-east-1
  vc2:
    type: aws:chime:VoiceConnector
    properties:
      name: connector-test-2
      requireEncryption: true
      awsRegion: us-west-2
  group:
    type: aws:chime:VoiceConnectorGroup
    properties:
      name: test-group
      connectors:
        - voiceConnectorId: ${vc1.id}
          priority: 1
        - voiceConnectorId: ${vc2.id}
          priority: 3
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Configuration Recorder using the name. For example:

```sh
$ pulumi import aws:chime/voiceConnectorGroup:VoiceConnectorGroup default example
```
ø

connectorsrBp*n:l
j
chimeVoiceConnectorGroupConnectorCaws:chime/VoiceConnectorGroupConnector:VoiceConnectorGroupConnector=The Amazon Chime Voice Connectors to route inbound calls to.
B
nameB" 4The name of the Amazon Chime Voice Connector group.
"ø

connectorsrBp*n:l
j
chimeVoiceConnectorGroupConnectorCaws:chime/VoiceConnectorGroupConnector:VoiceConnectorGroupConnector=The Amazon Chime Voice Connectors to route inbound calls to.
"@
name" 4The name of the Amazon Chime Voice Connector group.
*ø#
U
chimeVoiceConnectorLogging5aws:chime/voiceConnectorLogging:VoiceConnectorLoggingëAdds a logging configuration for the specified Amazon Chime Voice Connector. The logging configuration specifies whether SIP message logs are enabled for sending to Amazon CloudWatch Logs.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const _default = new aws.chime.VoiceConnector("default", {
    name: "vc-name-test",
    requireEncryption: true,
});
const defaultVoiceConnectorLogging = new aws.chime.VoiceConnectorLogging("default", {
    enableSipLogs: true,
    enableMediaMetricLogs: true,
    voiceConnectorId: _default.id,
});
```
```python
import pulumi
import pulumi_aws as aws

default = aws.chime.VoiceConnector("default",
    name="vc-name-test",
    require_encryption=True)
default_voice_connector_logging = aws.chime.VoiceConnectorLogging("default",
    enable_sip_logs=True,
    enable_media_metric_logs=True,
    voice_connector_id=default.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var @default = new Aws.Chime.VoiceConnector("default", new()
    {
        Name = "vc-name-test",
        RequireEncryption = true,
    });

    var defaultVoiceConnectorLogging = new Aws.Chime.VoiceConnectorLogging("default", new()
    {
        EnableSipLogs = true,
        EnableMediaMetricLogs = true,
        VoiceConnectorId = @default.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chime"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chime.NewVoiceConnector(ctx, "default", &chime.VoiceConnectorArgs{
			Name:              pulumi.String("vc-name-test"),
			RequireEncryption: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		_, err = chime.NewVoiceConnectorLogging(ctx, "default", &chime.VoiceConnectorLoggingArgs{
			EnableSipLogs:         pulumi.Bool(true),
			EnableMediaMetricLogs: pulumi.Bool(true),
			VoiceConnectorId:      _default.ID(),
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
import com.pulumi.aws.chime.VoiceConnector;
import com.pulumi.aws.chime.VoiceConnectorArgs;
import com.pulumi.aws.chime.VoiceConnectorLogging;
import com.pulumi.aws.chime.VoiceConnectorLoggingArgs;
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
        var default_ = new VoiceConnector("default", VoiceConnectorArgs.builder()
            .name("vc-name-test")
            .requireEncryption(true)
            .build());

        var defaultVoiceConnectorLogging = new VoiceConnectorLogging("defaultVoiceConnectorLogging", VoiceConnectorLoggingArgs.builder()
            .enableSipLogs(true)
            .enableMediaMetricLogs(true)
            .voiceConnectorId(default_.id())
            .build());

    }
}
```
```yaml
resources:
  default:
    type: aws:chime:VoiceConnector
    properties:
      name: vc-name-test
      requireEncryption: true
  defaultVoiceConnectorLogging:
    type: aws:chime:VoiceConnectorLogging
    name: default
    properties:
      enableSipLogs: true
      enableMediaMetricLogs: true
      voiceConnectorId: ${default.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Chime Voice Connector Logging using the `voice_connector_id`. For example:

```sh
$ pulumi import aws:chime/voiceConnectorLogging:VoiceConnectorLogging default abcdef1ghij2klmno3pqr4
```
Ñ
enableMediaMetricLogsB
 eWhen true, enables logging of detailed media metrics for Voice Connectors to Amazon CloudWatch logs.
b
enableSipLogsB
 KWhen true, enables SIP message logs for sending to Amazon CloudWatch Logs.
=
voiceConnectorId" %The Amazon Chime Voice Connector ID.
"Ñ
enableMediaMetricLogsB
 eWhen true, enables logging of detailed media metrics for Voice Connectors to Amazon CloudWatch logs.
"b
enableSipLogsB
 KWhen true, enables SIP message logs for sending to Amazon CloudWatch Logs.
"=
voiceConnectorId" %The Amazon Chime Voice Connector ID.
*ä8
d
chimeVoiceConnectorOrganization?aws:chime/voiceConnectorOrganization:VoiceConnectorOrganizationì0Enable origination settings to control inbound calling to your SIP infrastructure.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const _default = new aws.chime.VoiceConnector("default", {
    name: "test",
    requireEncryption: true,
});
const defaultVoiceConnectorOrganization = new aws.chime.VoiceConnectorOrganization("default", {
    disabled: false,
    voiceConnectorId: _default.id,
    routes: [
        {
            host: "127.0.0.1",
            port: 8081,
            protocol: "TCP",
            priority: 1,
            weight: 1,
        },
        {
            host: "127.0.0.2",
            port: 8082,
            protocol: "TCP",
            priority: 2,
            weight: 10,
        },
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

default = aws.chime.VoiceConnector("default",
    name="test",
    require_encryption=True)
default_voice_connector_organization = aws.chime.VoiceConnectorOrganization("default",
    disabled=False,
    voice_connector_id=default.id,
    routes=[
        {
            "host": "127.0.0.1",
            "port": 8081,
            "protocol": "TCP",
            "priority": 1,
            "weight": 1,
        },
        {
            "host": "127.0.0.2",
            "port": 8082,
            "protocol": "TCP",
            "priority": 2,
            "weight": 10,
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
    var @default = new Aws.Chime.VoiceConnector("default", new()
    {
        Name = "test",
        RequireEncryption = true,
    });

    var defaultVoiceConnectorOrganization = new Aws.Chime.VoiceConnectorOrganization("default", new()
    {
        Disabled = false,
        VoiceConnectorId = @default.Id,
        Routes = new[]
        {
            new Aws.Chime.Inputs.VoiceConnectorOrganizationRouteArgs
            {
                Host = "127.0.0.1",
                Port = 8081,
                Protocol = "TCP",
                Priority = 1,
                Weight = 1,
            },
            new Aws.Chime.Inputs.VoiceConnectorOrganizationRouteArgs
            {
                Host = "127.0.0.2",
                Port = 8082,
                Protocol = "TCP",
                Priority = 2,
                Weight = 10,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chime"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chime.NewVoiceConnector(ctx, "default", &chime.VoiceConnectorArgs{
			Name:              pulumi.String("test"),
			RequireEncryption: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		_, err = chime.NewVoiceConnectorOrganization(ctx, "default", &chime.VoiceConnectorOrganizationArgs{
			Disabled:         pulumi.Bool(false),
			VoiceConnectorId: _default.ID(),
			Routes: chime.VoiceConnectorOrganizationRouteArray{
				&chime.VoiceConnectorOrganizationRouteArgs{
					Host:     pulumi.String("127.0.0.1"),
					Port:     pulumi.Int(8081),
					Protocol: pulumi.String("TCP"),
					Priority: pulumi.Int(1),
					Weight:   pulumi.Int(1),
				},
				&chime.VoiceConnectorOrganizationRouteArgs{
					Host:     pulumi.String("127.0.0.2"),
					Port:     pulumi.Int(8082),
					Protocol: pulumi.String("TCP"),
					Priority: pulumi.Int(2),
					Weight:   pulumi.Int(10),
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
import com.pulumi.aws.chime.VoiceConnector;
import com.pulumi.aws.chime.VoiceConnectorArgs;
import com.pulumi.aws.chime.VoiceConnectorOrganization;
import com.pulumi.aws.chime.VoiceConnectorOrganizationArgs;
import com.pulumi.aws.chime.inputs.VoiceConnectorOrganizationRouteArgs;
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
        var default_ = new VoiceConnector("default", VoiceConnectorArgs.builder()
            .name("test")
            .requireEncryption(true)
            .build());

        var defaultVoiceConnectorOrganization = new VoiceConnectorOrganization("defaultVoiceConnectorOrganization", VoiceConnectorOrganizationArgs.builder()
            .disabled(false)
            .voiceConnectorId(default_.id())
            .routes(            
                VoiceConnectorOrganizationRouteArgs.builder()
                    .host("127.0.0.1")
                    .port(8081)
                    .protocol("TCP")
                    .priority(1)
                    .weight(1)
                    .build(),
                VoiceConnectorOrganizationRouteArgs.builder()
                    .host("127.0.0.2")
                    .port(8082)
                    .protocol("TCP")
                    .priority(2)
                    .weight(10)
                    .build())
            .build());

    }
}
```
```yaml
resources:
  default:
    type: aws:chime:VoiceConnector
    properties:
      name: test
      requireEncryption: true
  defaultVoiceConnectorOrganization:
    type: aws:chime:VoiceConnectorOrganization
    name: default
    properties:
      disabled: false
      voiceConnectorId: ${default.id}
      routes:
        - host: 127.0.0.1
          port: 8081
          protocol: TCP
          priority: 1
          weight: 1
        - host: 127.0.0.2
          port: 8082
          protocol: TCP
          priority: 2
          weight: 10
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Chime Voice Connector Origination using the `voice_connector_id`. For example:

```sh
$ pulumi import aws:chime/voiceConnectorOrganization:VoiceConnectorOrganization default abcdef1ghij2klmno3pqr4
```

disabledB
 mWhen origination settings are disabled, inbound calls are not enabled for your Amazon Chime Voice Connector.
Ñ
routesy*w:u
s
chimeVoiceConnectorOrganizationRouteIaws:chime/VoiceConnectorOrganizationRoute:VoiceConnectorOrganizationRouteSet of call distribution properties defined for your SIP hosts. See route below for more details. Minimum of 1. Maximum of 20.
=
voiceConnectorId" %The Amazon Chime Voice Connector ID.
"
disabledB
 mWhen origination settings are disabled, inbound calls are not enabled for your Amazon Chime Voice Connector.
"Ñ
routesy*w:u
s
chimeVoiceConnectorOrganizationRouteIaws:chime/VoiceConnectorOrganizationRoute:VoiceConnectorOrganizationRouteSet of call distribution properties defined for your SIP hosts. See route below for more details. Minimum of 1. Maximum of 20.
"=
voiceConnectorId" %The Amazon Chime Voice Connector ID.
*Ô¢
[
chimeVoiceConnectorStreaming9aws:chime/voiceConnectorStreaming:VoiceConnectorStreamingÇòAdds a streaming configuration for the specified Amazon Chime Voice Connector. The streaming configuration specifies whether media streaming is enabled for sending to Amazon Kinesis.
It also sets the retention period, in hours, for the Amazon Kinesis data.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const _default = new aws.chime.VoiceConnector("default", {
    name: "vc-name-test",
    requireEncryption: true,
});
const defaultVoiceConnectorStreaming = new aws.chime.VoiceConnectorStreaming("default", {
    disabled: false,
    voiceConnectorId: _default.id,
    dataRetention: 7,
    streamingNotificationTargets: ["SQS"],
});
```
```python
import pulumi
import pulumi_aws as aws

default = aws.chime.VoiceConnector("default",
    name="vc-name-test",
    require_encryption=True)
default_voice_connector_streaming = aws.chime.VoiceConnectorStreaming("default",
    disabled=False,
    voice_connector_id=default.id,
    data_retention=7,
    streaming_notification_targets=["SQS"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var @default = new Aws.Chime.VoiceConnector("default", new()
    {
        Name = "vc-name-test",
        RequireEncryption = true,
    });

    var defaultVoiceConnectorStreaming = new Aws.Chime.VoiceConnectorStreaming("default", new()
    {
        Disabled = false,
        VoiceConnectorId = @default.Id,
        DataRetention = 7,
        StreamingNotificationTargets = new[]
        {
            "SQS",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chime"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chime.NewVoiceConnector(ctx, "default", &chime.VoiceConnectorArgs{
			Name:              pulumi.String("vc-name-test"),
			RequireEncryption: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		_, err = chime.NewVoiceConnectorStreaming(ctx, "default", &chime.VoiceConnectorStreamingArgs{
			Disabled:         pulumi.Bool(false),
			VoiceConnectorId: _default.ID(),
			DataRetention:    pulumi.Int(7),
			StreamingNotificationTargets: pulumi.StringArray{
				pulumi.String("SQS"),
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
import com.pulumi.aws.chime.VoiceConnector;
import com.pulumi.aws.chime.VoiceConnectorArgs;
import com.pulumi.aws.chime.VoiceConnectorStreaming;
import com.pulumi.aws.chime.VoiceConnectorStreamingArgs;
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
        var default_ = new VoiceConnector("default", VoiceConnectorArgs.builder()
            .name("vc-name-test")
            .requireEncryption(true)
            .build());

        var defaultVoiceConnectorStreaming = new VoiceConnectorStreaming("defaultVoiceConnectorStreaming", VoiceConnectorStreamingArgs.builder()
            .disabled(false)
            .voiceConnectorId(default_.id())
            .dataRetention(7)
            .streamingNotificationTargets("SQS")
            .build());

    }
}
```
```yaml
resources:
  default:
    type: aws:chime:VoiceConnector
    properties:
      name: vc-name-test
      requireEncryption: true
  defaultVoiceConnectorStreaming:
    type: aws:chime:VoiceConnectorStreaming
    name: default
    properties:
      disabled: false
      voiceConnectorId: ${default.id}
      dataRetention: 7
      streamingNotificationTargets:
        - SQS
```
<!--End PulumiCodeChooser -->

### Example Usage With Media Insights

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const _default = new aws.chime.VoiceConnector("default", {
    name: "vc-name-test",
    requireEncryption: true,
});
const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["mediapipelines.chime.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const exampleRole = new aws.iam.Role("example", {
    name: "ExampleResourceAccessRole",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const exampleStream = new aws.kinesis.Stream("example", {
    name: "ExampleStream",
    shardCount: 2,
});
const example = new aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("example", {
    name: "ExampleConfig",
    resourceAccessRoleArn: exampleRole.arn,
    elements: [
        {
            type: "AmazonTranscribeCallAnalyticsProcessor",
            amazonTranscribeCallAnalyticsProcessorConfiguration: {
                languageCode: "en-US",
            },
        },
        {
            type: "KinesisDataStreamSink",
            kinesisDataStreamSinkConfiguration: {
                insightsTarget: exampleStream.arn,
            },
        },
    ],
});
const defaultVoiceConnectorStreaming = new aws.chime.VoiceConnectorStreaming("default", {
    disabled: false,
    voiceConnectorId: _default.id,
    dataRetention: 7,
    streamingNotificationTargets: ["SQS"],
    mediaInsightsConfiguration: {
        disabled: false,
        configurationArn: example.arn,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

default = aws.chime.VoiceConnector("default",
    name="vc-name-test",
    require_encryption=True)
assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["mediapipelines.chime.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
example_role = aws.iam.Role("example",
    name="ExampleResourceAccessRole",
    assume_role_policy=assume_role.json)
example_stream = aws.kinesis.Stream("example",
    name="ExampleStream",
    shard_count=2)
example = aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("example",
    name="ExampleConfig",
    resource_access_role_arn=example_role.arn,
    elements=[
        {
            "type": "AmazonTranscribeCallAnalyticsProcessor",
            "amazon_transcribe_call_analytics_processor_configuration": {
                "language_code": "en-US",
            },
        },
        {
            "type": "KinesisDataStreamSink",
            "kinesis_data_stream_sink_configuration": {
                "insights_target": example_stream.arn,
            },
        },
    ])
default_voice_connector_streaming = aws.chime.VoiceConnectorStreaming("default",
    disabled=False,
    voice_connector_id=default.id,
    data_retention=7,
    streaming_notification_targets=["SQS"],
    media_insights_configuration={
        "disabled": False,
        "configuration_arn": example.arn,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var @default = new Aws.Chime.VoiceConnector("default", new()
    {
        Name = "vc-name-test",
        RequireEncryption = true,
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
                            "mediapipelines.chime.amazonaws.com",
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
        Name = "ExampleResourceAccessRole",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var exampleStream = new Aws.Kinesis.Stream("example", new()
    {
        Name = "ExampleStream",
        ShardCount = 2,
    });

    var example = new Aws.ChimeSDKMediaPipelines.MediaInsightsPipelineConfiguration("example", new()
    {
        Name = "ExampleConfig",
        ResourceAccessRoleArn = exampleRole.Arn,
        Elements = new[]
        {
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "AmazonTranscribeCallAnalyticsProcessor",
                AmazonTranscribeCallAnalyticsProcessorConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs
                {
                    LanguageCode = "en-US",
                },
            },
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "KinesisDataStreamSink",
                KinesisDataStreamSinkConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs
                {
                    InsightsTarget = exampleStream.Arn,
                },
            },
        },
    });

    var defaultVoiceConnectorStreaming = new Aws.Chime.VoiceConnectorStreaming("default", new()
    {
        Disabled = false,
        VoiceConnectorId = @default.Id,
        DataRetention = 7,
        StreamingNotificationTargets = new[]
        {
            "SQS",
        },
        MediaInsightsConfiguration = new Aws.Chime.Inputs.VoiceConnectorStreamingMediaInsightsConfigurationArgs
        {
            Disabled = false,
            ConfigurationArn = example.Arn,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chime"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chimesdkmediapipelines"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/kinesis"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chime.NewVoiceConnector(ctx, "default", &chime.VoiceConnectorArgs{
			Name:              pulumi.String("vc-name-test"),
			RequireEncryption: pulumi.Bool(true),
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
								"mediapipelines.chime.amazonaws.com",
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
			Name:             pulumi.String("ExampleResourceAccessRole"),
			AssumeRolePolicy: pulumi.String(assumeRole.Json),
		})
		if err != nil {
			return err
		}
		exampleStream, err := kinesis.NewStream(ctx, "example", &kinesis.StreamArgs{
			Name:       pulumi.String("ExampleStream"),
			ShardCount: pulumi.Int(2),
		})
		if err != nil {
			return err
		}
		example, err := chimesdkmediapipelines.NewMediaInsightsPipelineConfiguration(ctx, "example", &chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs{
			Name:                  pulumi.String("ExampleConfig"),
			ResourceAccessRoleArn: exampleRole.Arn,
			Elements: chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArray{
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("AmazonTranscribeCallAnalyticsProcessor"),
					AmazonTranscribeCallAnalyticsProcessorConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs{
						LanguageCode: pulumi.String("en-US"),
					},
				},
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("KinesisDataStreamSink"),
					KinesisDataStreamSinkConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs{
						InsightsTarget: exampleStream.Arn,
					},
				},
			},
		})
		if err != nil {
			return err
		}
		_, err = chime.NewVoiceConnectorStreaming(ctx, "default", &chime.VoiceConnectorStreamingArgs{
			Disabled:         pulumi.Bool(false),
			VoiceConnectorId: _default.ID(),
			DataRetention:    pulumi.Int(7),
			StreamingNotificationTargets: pulumi.StringArray{
				pulumi.String("SQS"),
			},
			MediaInsightsConfiguration: &chime.VoiceConnectorStreamingMediaInsightsConfigurationArgs{
				Disabled:         pulumi.Bool(false),
				ConfigurationArn: example.Arn,
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
import com.pulumi.aws.chime.VoiceConnector;
import com.pulumi.aws.chime.VoiceConnectorArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.kinesis.Stream;
import com.pulumi.aws.kinesis.StreamArgs;
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration;
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs;
import com.pulumi.aws.chime.VoiceConnectorStreaming;
import com.pulumi.aws.chime.VoiceConnectorStreamingArgs;
import com.pulumi.aws.chime.inputs.VoiceConnectorStreamingMediaInsightsConfigurationArgs;
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
        var default_ = new VoiceConnector("default", VoiceConnectorArgs.builder()
            .name("vc-name-test")
            .requireEncryption(true)
            .build());

        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("mediapipelines.chime.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var exampleRole = new Role("exampleRole", RoleArgs.builder()
            .name("ExampleResourceAccessRole")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var exampleStream = new Stream("exampleStream", StreamArgs.builder()
            .name("ExampleStream")
            .shardCount(2)
            .build());

        var example = new MediaInsightsPipelineConfiguration("example", MediaInsightsPipelineConfigurationArgs.builder()
            .name("ExampleConfig")
            .resourceAccessRoleArn(exampleRole.arn())
            .elements(            
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("AmazonTranscribeCallAnalyticsProcessor")
                    .amazonTranscribeCallAnalyticsProcessorConfiguration(MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs.builder()
                        .languageCode("en-US")
                        .build())
                    .build(),
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("KinesisDataStreamSink")
                    .kinesisDataStreamSinkConfiguration(MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs.builder()
                        .insightsTarget(exampleStream.arn())
                        .build())
                    .build())
            .build());

        var defaultVoiceConnectorStreaming = new VoiceConnectorStreaming("defaultVoiceConnectorStreaming", VoiceConnectorStreamingArgs.builder()
            .disabled(false)
            .voiceConnectorId(default_.id())
            .dataRetention(7)
            .streamingNotificationTargets("SQS")
            .mediaInsightsConfiguration(VoiceConnectorStreamingMediaInsightsConfigurationArgs.builder()
                .disabled(false)
                .configurationArn(example.arn())
                .build())
            .build());

    }
}
```
```yaml
resources:
  default:
    type: aws:chime:VoiceConnector
    properties:
      name: vc-name-test
      requireEncryption: true
  defaultVoiceConnectorStreaming:
    type: aws:chime:VoiceConnectorStreaming
    name: default
    properties:
      disabled: false
      voiceConnectorId: ${default.id}
      dataRetention: 7
      streamingNotificationTargets:
        - SQS
      mediaInsightsConfiguration:
        disabled: false
        configurationArn: ${example.arn}
  example:
    type: aws:chimesdkmediapipelines:MediaInsightsPipelineConfiguration
    properties:
      name: ExampleConfig
      resourceAccessRoleArn: ${exampleRole.arn}
      elements:
        - type: AmazonTranscribeCallAnalyticsProcessor
          amazonTranscribeCallAnalyticsProcessorConfiguration:
            languageCode: en-US
        - type: KinesisDataStreamSink
          kinesisDataStreamSinkConfiguration:
            insightsTarget: ${exampleStream.arn}
  exampleRole:
    type: aws:iam:Role
    name: example
    properties:
      name: ExampleResourceAccessRole
      assumeRolePolicy: ${assumeRole.json}
  exampleStream:
    type: aws:kinesis:Stream
    name: example
    properties:
      name: ExampleStream
      shardCount: 2
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
                  - mediapipelines.chime.amazonaws.com
            actions:
              - sts:AssumeRole
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Chime Voice Connector Streaming using the `voice_connector_id`. For example:

```sh
$ pulumi import aws:chime/voiceConnectorStreaming:VoiceConnectorStreaming default abcdef1ghij2klmno3pqr4
```
R
dataRetention =The retention period, in hours, for the Amazon Kinesis data.
_
disabledB
 MWhen true, media streaming to Amazon Kinesis is turned off. Default: `false`
ô
mediaInsightsConfiguration≤BØ:¨
©
chime1VoiceConnectorStreamingMediaInsightsConfigurationmaws:chime/VoiceConnectorStreamingMediaInsightsConfiguration:VoiceConnectorStreamingMediaInsightsConfigurationFThe media insights configuration. See `media_insights_configuration`.
t
streamingNotificationTargetsB*" LThe streaming notification targets. Valid Values: `EventBridge | SNS | SQS`
=
voiceConnectorId" %The Amazon Chime Voice Connector ID.
"R
dataRetention =The retention period, in hours, for the Amazon Kinesis data.
"_
disabledB
 MWhen true, media streaming to Amazon Kinesis is turned off. Default: `false`
"ô
mediaInsightsConfiguration≤BØ:¨
©
chime1VoiceConnectorStreamingMediaInsightsConfigurationmaws:chime/VoiceConnectorStreamingMediaInsightsConfiguration:VoiceConnectorStreamingMediaInsightsConfigurationFThe media insights configuration. See `media_insights_configuration`.
"t
streamingNotificationTargetsB*" LThe streaming notification targets. Valid Values: `EventBridge | SNS | SQS`
"=
voiceConnectorId" %The Amazon Chime Voice Connector ID.
*⁄+
a
chimeVoiceConnectorTermination=aws:chime/voiceConnectorTermination:VoiceConnectorTerminationÏ"Enable Termination settings to control outbound calling from your SIP infrastructure.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const _default = new aws.chime.VoiceConnector("default", {
    name: "vc-name-test",
    requireEncryption: true,
});
const defaultVoiceConnectorTermination = new aws.chime.VoiceConnectorTermination("default", {
    disabled: false,
    cpsLimit: 1,
    cidrAllowLists: ["50.35.78.96/31"],
    callingRegions: [
        "US",
        "CA",
    ],
    voiceConnectorId: _default.id,
});
```
```python
import pulumi
import pulumi_aws as aws

default = aws.chime.VoiceConnector("default",
    name="vc-name-test",
    require_encryption=True)
default_voice_connector_termination = aws.chime.VoiceConnectorTermination("default",
    disabled=False,
    cps_limit=1,
    cidr_allow_lists=["50.35.78.96/31"],
    calling_regions=[
        "US",
        "CA",
    ],
    voice_connector_id=default.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var @default = new Aws.Chime.VoiceConnector("default", new()
    {
        Name = "vc-name-test",
        RequireEncryption = true,
    });

    var defaultVoiceConnectorTermination = new Aws.Chime.VoiceConnectorTermination("default", new()
    {
        Disabled = false,
        CpsLimit = 1,
        CidrAllowLists = new[]
        {
            "50.35.78.96/31",
        },
        CallingRegions = new[]
        {
            "US",
            "CA",
        },
        VoiceConnectorId = @default.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chime"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chime.NewVoiceConnector(ctx, "default", &chime.VoiceConnectorArgs{
			Name:              pulumi.String("vc-name-test"),
			RequireEncryption: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		_, err = chime.NewVoiceConnectorTermination(ctx, "default", &chime.VoiceConnectorTerminationArgs{
			Disabled: pulumi.Bool(false),
			CpsLimit: pulumi.Int(1),
			CidrAllowLists: pulumi.StringArray{
				pulumi.String("50.35.78.96/31"),
			},
			CallingRegions: pulumi.StringArray{
				pulumi.String("US"),
				pulumi.String("CA"),
			},
			VoiceConnectorId: _default.ID(),
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
import com.pulumi.aws.chime.VoiceConnector;
import com.pulumi.aws.chime.VoiceConnectorArgs;
import com.pulumi.aws.chime.VoiceConnectorTermination;
import com.pulumi.aws.chime.VoiceConnectorTerminationArgs;
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
        var default_ = new VoiceConnector("default", VoiceConnectorArgs.builder()
            .name("vc-name-test")
            .requireEncryption(true)
            .build());

        var defaultVoiceConnectorTermination = new VoiceConnectorTermination("defaultVoiceConnectorTermination", VoiceConnectorTerminationArgs.builder()
            .disabled(false)
            .cpsLimit(1)
            .cidrAllowLists("50.35.78.96/31")
            .callingRegions(            
                "US",
                "CA")
            .voiceConnectorId(default_.id())
            .build());

    }
}
```
```yaml
resources:
  default:
    type: aws:chime:VoiceConnector
    properties:
      name: vc-name-test
      requireEncryption: true
  defaultVoiceConnectorTermination:
    type: aws:chime:VoiceConnectorTermination
    name: default
    properties:
      disabled: false
      cpsLimit: 1
      cidrAllowLists:
        - 50.35.78.96/31
      callingRegions:
        - US
        - CA
      voiceConnectorId: ${default.id}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Chime Voice Connector Termination using the `voice_connector_id`. For example:

```sh
$ pulumi import aws:chime/voiceConnectorTermination:VoiceConnectorTermination default abcdef1ghij2klmno3pqr4
```
`
callingRegions*" HThe countries to which calls are allowed, in ISO 3166-1 alpha-2 format.
P
cidrAllowLists*" 8The IP addresses allowed to make calls, in CIDR format.
q
cpsLimitB _The limit on calls per second. Max value based on account service quota. Default value of `1`.
@
defaultPhoneNumberB" $The default caller ID phone number.
Z
disabledB
 HWhen termination settings are disabled, outbound calls can not be made.
=
voiceConnectorId" %The Amazon Chime Voice Connector ID.
"`
callingRegions*" HThe countries to which calls are allowed, in ISO 3166-1 alpha-2 format.
"P
cidrAllowLists*" 8The IP addresses allowed to make calls, in CIDR format.
"q
cpsLimitB _The limit on calls per second. Max value based on account service quota. Default value of `1`.
"@
defaultPhoneNumberB" $The default caller ID phone number.
"Z
disabledB
 HWhen termination settings are disabled, outbound calls can not be made.
"=
voiceConnectorId" %The Amazon Chime Voice Connector ID.
*Ë?
Ç
chime$VoiceConnectorTerminationCredentialsSaws:chime/voiceConnectorTerminationCredentials:VoiceConnectorTerminationCredentials§:Adds termination SIP credentials for the specified Amazon Chime Voice Connector.

> **Note:** Voice Connector Termination Credentials requires a Voice Connector Termination to be present. Use of `depends_on` (as shown below) is recommended to avoid race conditions.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const _default = new aws.chime.VoiceConnector("default", {
    name: "test",
    requireEncryption: true,
});
const defaultVoiceConnectorTermination = new aws.chime.VoiceConnectorTermination("default", {
    disabled: true,
    cpsLimit: 1,
    cidrAllowLists: ["50.35.78.96/31"],
    callingRegions: [
        "US",
        "CA",
    ],
    voiceConnectorId: _default.id,
});
const defaultVoiceConnectorTerminationCredentials = new aws.chime.VoiceConnectorTerminationCredentials("default", {
    voiceConnectorId: _default.id,
    credentials: [{
        username: "test",
        password: "test!",
    }],
}, {
    dependsOn: [defaultVoiceConnectorTermination],
});
```
```python
import pulumi
import pulumi_aws as aws

default = aws.chime.VoiceConnector("default",
    name="test",
    require_encryption=True)
default_voice_connector_termination = aws.chime.VoiceConnectorTermination("default",
    disabled=True,
    cps_limit=1,
    cidr_allow_lists=["50.35.78.96/31"],
    calling_regions=[
        "US",
        "CA",
    ],
    voice_connector_id=default.id)
default_voice_connector_termination_credentials = aws.chime.VoiceConnectorTerminationCredentials("default",
    voice_connector_id=default.id,
    credentials=[{
        "username": "test",
        "password": "test!",
    }],
    opts = pulumi.ResourceOptions(depends_on=[default_voice_connector_termination]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var @default = new Aws.Chime.VoiceConnector("default", new()
    {
        Name = "test",
        RequireEncryption = true,
    });

    var defaultVoiceConnectorTermination = new Aws.Chime.VoiceConnectorTermination("default", new()
    {
        Disabled = true,
        CpsLimit = 1,
        CidrAllowLists = new[]
        {
            "50.35.78.96/31",
        },
        CallingRegions = new[]
        {
            "US",
            "CA",
        },
        VoiceConnectorId = @default.Id,
    });

    var defaultVoiceConnectorTerminationCredentials = new Aws.Chime.VoiceConnectorTerminationCredentials("default", new()
    {
        VoiceConnectorId = @default.Id,
        Credentials = new[]
        {
            new Aws.Chime.Inputs.VoiceConnectorTerminationCredentialsCredentialArgs
            {
                Username = "test",
                Password = "test!",
            },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            defaultVoiceConnectorTermination,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chime"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chime.NewVoiceConnector(ctx, "default", &chime.VoiceConnectorArgs{
			Name:              pulumi.String("test"),
			RequireEncryption: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		defaultVoiceConnectorTermination, err := chime.NewVoiceConnectorTermination(ctx, "default", &chime.VoiceConnectorTerminationArgs{
			Disabled: pulumi.Bool(true),
			CpsLimit: pulumi.Int(1),
			CidrAllowLists: pulumi.StringArray{
				pulumi.String("50.35.78.96/31"),
			},
			CallingRegions: pulumi.StringArray{
				pulumi.String("US"),
				pulumi.String("CA"),
			},
			VoiceConnectorId: _default.ID(),
		})
		if err != nil {
			return err
		}
		_, err = chime.NewVoiceConnectorTerminationCredentials(ctx, "default", &chime.VoiceConnectorTerminationCredentialsArgs{
			VoiceConnectorId: _default.ID(),
			Credentials: chime.VoiceConnectorTerminationCredentialsCredentialArray{
				&chime.VoiceConnectorTerminationCredentialsCredentialArgs{
					Username: pulumi.String("test"),
					Password: pulumi.String("test!"),
				},
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			defaultVoiceConnectorTermination,
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
import com.pulumi.aws.chime.VoiceConnector;
import com.pulumi.aws.chime.VoiceConnectorArgs;
import com.pulumi.aws.chime.VoiceConnectorTermination;
import com.pulumi.aws.chime.VoiceConnectorTerminationArgs;
import com.pulumi.aws.chime.VoiceConnectorTerminationCredentials;
import com.pulumi.aws.chime.VoiceConnectorTerminationCredentialsArgs;
import com.pulumi.aws.chime.inputs.VoiceConnectorTerminationCredentialsCredentialArgs;
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
        var default_ = new VoiceConnector("default", VoiceConnectorArgs.builder()
            .name("test")
            .requireEncryption(true)
            .build());

        var defaultVoiceConnectorTermination = new VoiceConnectorTermination("defaultVoiceConnectorTermination", VoiceConnectorTerminationArgs.builder()
            .disabled(true)
            .cpsLimit(1)
            .cidrAllowLists("50.35.78.96/31")
            .callingRegions(            
                "US",
                "CA")
            .voiceConnectorId(default_.id())
            .build());

        var defaultVoiceConnectorTerminationCredentials = new VoiceConnectorTerminationCredentials("defaultVoiceConnectorTerminationCredentials", VoiceConnectorTerminationCredentialsArgs.builder()
            .voiceConnectorId(default_.id())
            .credentials(VoiceConnectorTerminationCredentialsCredentialArgs.builder()
                .username("test")
                .password("test!")
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(defaultVoiceConnectorTermination)
                .build());

    }
}
```
```yaml
resources:
  default:
    type: aws:chime:VoiceConnector
    properties:
      name: test
      requireEncryption: true
  defaultVoiceConnectorTermination:
    type: aws:chime:VoiceConnectorTermination
    name: default
    properties:
      disabled: true
      cpsLimit: 1
      cidrAllowLists:
        - 50.35.78.96/31
      callingRegions:
        - US
        - CA
      voiceConnectorId: ${default.id}
  defaultVoiceConnectorTerminationCredentials:
    type: aws:chime:VoiceConnectorTerminationCredentials
    name: default
    properties:
      voiceConnectorId: ${default.id}
      credentials:
        - username: test
          password: test!
    options:
      dependsOn:
        - ${defaultVoiceConnectorTermination}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Chime Voice Connector Termination Credentials using the `voice_connector_id`. For example:

```sh
$ pulumi import aws:chime/voiceConnectorTerminationCredentials:VoiceConnectorTerminationCredentials default abcdef1ghij2klmno3pqr4
```
‡
credentials©*¶:£
†
chime.VoiceConnectorTerminationCredentialsCredentialgaws:chime/VoiceConnectorTerminationCredentialsCredential:VoiceConnectorTerminationCredentialsCredential%List of termination SIP credentials.
9
voiceConnectorId" !Amazon Chime Voice Connector ID.
"‡
credentials©*¶:£
†
chime.VoiceConnectorTerminationCredentialsCredentialgaws:chime/VoiceConnectorTerminationCredentialsCredential:VoiceConnectorTerminationCredentialsCredential%List of termination SIP credentials.
"9
voiceConnectorId" !Amazon Chime Voice Connector ID.
*ßƒ
û
chimesdkmediapipelines"MediaInsightsPipelineConfiguration`aws:chimesdkmediapipelines/mediaInsightsPipelineConfiguration:MediaInsightsPipelineConfiguration∫¥Resource for managing an AWS Chime SDK Media Pipelines Media Insights Pipeline Configuration.
Consult the [Call analytics developer guide](https://docs.aws.amazon.com/chime-sdk/latest/dg/call-analytics.html) for more detailed information about usage.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.kinesis.Stream("example", {
    name: "example",
    shardCount: 2,
});
const mediaPipelinesAssumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["mediapipelines.chime.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const callAnalyticsRole = new aws.iam.Role("call_analytics_role", {
    name: "CallAnalyticsRole",
    assumeRolePolicy: mediaPipelinesAssumeRole.then(mediaPipelinesAssumeRole => mediaPipelinesAssumeRole.json),
});
const myConfiguration = new aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("my_configuration", {
    name: "MyBasicConfiguration",
    resourceAccessRoleArn: callAnalyticsRole.arn,
    elements: [
        {
            type: "AmazonTranscribeCallAnalyticsProcessor",
            amazonTranscribeCallAnalyticsProcessorConfiguration: {
                languageCode: "en-US",
            },
        },
        {
            type: "KinesisDataStreamSink",
            kinesisDataStreamSinkConfiguration: {
                insightsTarget: example.arn,
            },
        },
    ],
    tags: {
        Key1: "Value1",
        Key2: "Value2",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.kinesis.Stream("example",
    name="example",
    shard_count=2)
media_pipelines_assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["mediapipelines.chime.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
call_analytics_role = aws.iam.Role("call_analytics_role",
    name="CallAnalyticsRole",
    assume_role_policy=media_pipelines_assume_role.json)
my_configuration = aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("my_configuration",
    name="MyBasicConfiguration",
    resource_access_role_arn=call_analytics_role.arn,
    elements=[
        {
            "type": "AmazonTranscribeCallAnalyticsProcessor",
            "amazon_transcribe_call_analytics_processor_configuration": {
                "language_code": "en-US",
            },
        },
        {
            "type": "KinesisDataStreamSink",
            "kinesis_data_stream_sink_configuration": {
                "insights_target": example.arn,
            },
        },
    ],
    tags={
        "Key1": "Value1",
        "Key2": "Value2",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Kinesis.Stream("example", new()
    {
        Name = "example",
        ShardCount = 2,
    });

    var mediaPipelinesAssumeRole = Aws.Iam.GetPolicyDocument.Invoke(new()
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
                            "mediapipelines.chime.amazonaws.com",
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

    var callAnalyticsRole = new Aws.Iam.Role("call_analytics_role", new()
    {
        Name = "CallAnalyticsRole",
        AssumeRolePolicy = mediaPipelinesAssumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var myConfiguration = new Aws.ChimeSDKMediaPipelines.MediaInsightsPipelineConfiguration("my_configuration", new()
    {
        Name = "MyBasicConfiguration",
        ResourceAccessRoleArn = callAnalyticsRole.Arn,
        Elements = new[]
        {
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "AmazonTranscribeCallAnalyticsProcessor",
                AmazonTranscribeCallAnalyticsProcessorConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs
                {
                    LanguageCode = "en-US",
                },
            },
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "KinesisDataStreamSink",
                KinesisDataStreamSinkConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs
                {
                    InsightsTarget = example.Arn,
                },
            },
        },
        Tags = 
        {
            { "Key1", "Value1" },
            { "Key2", "Value2" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chimesdkmediapipelines"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/kinesis"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := kinesis.NewStream(ctx, "example", &kinesis.StreamArgs{
			Name:       pulumi.String("example"),
			ShardCount: pulumi.Int(2),
		})
		if err != nil {
			return err
		}
		mediaPipelinesAssumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"mediapipelines.chime.amazonaws.com",
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
		callAnalyticsRole, err := iam.NewRole(ctx, "call_analytics_role", &iam.RoleArgs{
			Name:             pulumi.String("CallAnalyticsRole"),
			AssumeRolePolicy: pulumi.String(mediaPipelinesAssumeRole.Json),
		})
		if err != nil {
			return err
		}
		_, err = chimesdkmediapipelines.NewMediaInsightsPipelineConfiguration(ctx, "my_configuration", &chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs{
			Name:                  pulumi.String("MyBasicConfiguration"),
			ResourceAccessRoleArn: callAnalyticsRole.Arn,
			Elements: chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArray{
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("AmazonTranscribeCallAnalyticsProcessor"),
					AmazonTranscribeCallAnalyticsProcessorConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs{
						LanguageCode: pulumi.String("en-US"),
					},
				},
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("KinesisDataStreamSink"),
					KinesisDataStreamSinkConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs{
						InsightsTarget: example.Arn,
					},
				},
			},
			Tags: pulumi.StringMap{
				"Key1": pulumi.String("Value1"),
				"Key2": pulumi.String("Value2"),
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
import com.pulumi.aws.kinesis.Stream;
import com.pulumi.aws.kinesis.StreamArgs;
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration;
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs;
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
        var example = new Stream("example", StreamArgs.builder()
            .name("example")
            .shardCount(2)
            .build());

        final var mediaPipelinesAssumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("mediapipelines.chime.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var callAnalyticsRole = new Role("callAnalyticsRole", RoleArgs.builder()
            .name("CallAnalyticsRole")
            .assumeRolePolicy(mediaPipelinesAssumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var myConfiguration = new MediaInsightsPipelineConfiguration("myConfiguration", MediaInsightsPipelineConfigurationArgs.builder()
            .name("MyBasicConfiguration")
            .resourceAccessRoleArn(callAnalyticsRole.arn())
            .elements(            
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("AmazonTranscribeCallAnalyticsProcessor")
                    .amazonTranscribeCallAnalyticsProcessorConfiguration(MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs.builder()
                        .languageCode("en-US")
                        .build())
                    .build(),
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("KinesisDataStreamSink")
                    .kinesisDataStreamSinkConfiguration(MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs.builder()
                        .insightsTarget(example.arn())
                        .build())
                    .build())
            .tags(Map.ofEntries(
                Map.entry("Key1", "Value1"),
                Map.entry("Key2", "Value2")
            ))
            .build());

    }
}
```
```yaml
resources:
  myConfiguration:
    type: aws:chimesdkmediapipelines:MediaInsightsPipelineConfiguration
    name: my_configuration
    properties:
      name: MyBasicConfiguration
      resourceAccessRoleArn: ${callAnalyticsRole.arn}
      elements:
        - type: AmazonTranscribeCallAnalyticsProcessor
          amazonTranscribeCallAnalyticsProcessorConfiguration:
            languageCode: en-US
        - type: KinesisDataStreamSink
          kinesisDataStreamSinkConfiguration:
            insightsTarget: ${example.arn}
      tags:
        Key1: Value1
        Key2: Value2
  example:
    type: aws:kinesis:Stream
    properties:
      name: example
      shardCount: 2
  callAnalyticsRole:
    type: aws:iam:Role
    name: call_analytics_role
    properties:
      name: CallAnalyticsRole
      assumeRolePolicy: ${mediaPipelinesAssumeRole.json}
variables:
  mediaPipelinesAssumeRole:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: Service
                identifiers:
                  - mediapipelines.chime.amazonaws.com
            actions:
              - sts:AssumeRole
```
<!--End PulumiCodeChooser -->

- The required policies on `call_analytics_role` will vary based on the selected processors. See [Call analytics resource access role](https://docs.aws.amazon.com/chime-sdk/latest/dg/ca-resource-access-role.html) for directions on choosing appropriate policies.

### Transcribe Call Analytics processor usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const transcribeAssumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["transcribe.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const postCallRole = new aws.iam.Role("post_call_role", {
    name: "PostCallAccessRole",
    assumeRolePolicy: transcribeAssumeRole.then(transcribeAssumeRole => transcribeAssumeRole.json),
});
const myConfiguration = new aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("my_configuration", {
    name: "MyCallAnalyticsConfiguration",
    resourceAccessRoleArn: exampleAwsIamRole.arn,
    elements: [
        {
            type: "AmazonTranscribeCallAnalyticsProcessor",
            amazonTranscribeCallAnalyticsProcessorConfiguration: {
                callAnalyticsStreamCategories: [
                    "category_1",
                    "category_2",
                ],
                contentRedactionType: "PII",
                enablePartialResultsStabilization: true,
                filterPartialResults: true,
                languageCode: "en-US",
                languageModelName: "MyLanguageModel",
                partialResultsStability: "high",
                piiEntityTypes: "ADDRESS,BANK_ACCOUNT_NUMBER",
                postCallAnalyticsSettings: {
                    contentRedactionOutput: "redacted",
                    dataAccessRoleArn: postCallRole.arn,
                    outputEncryptionKmsKeyId: "MyKmsKeyId",
                    outputLocation: "s3://MyBucket",
                },
                vocabularyFilterMethod: "mask",
                vocabularyFilterName: "MyVocabularyFilter",
                vocabularyName: "MyVocabulary",
            },
        },
        {
            type: "KinesisDataStreamSink",
            kinesisDataStreamSinkConfiguration: {
                insightsTarget: example.arn,
            },
        },
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

transcribe_assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["transcribe.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
post_call_role = aws.iam.Role("post_call_role",
    name="PostCallAccessRole",
    assume_role_policy=transcribe_assume_role.json)
my_configuration = aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("my_configuration",
    name="MyCallAnalyticsConfiguration",
    resource_access_role_arn=example_aws_iam_role["arn"],
    elements=[
        {
            "type": "AmazonTranscribeCallAnalyticsProcessor",
            "amazon_transcribe_call_analytics_processor_configuration": {
                "call_analytics_stream_categories": [
                    "category_1",
                    "category_2",
                ],
                "content_redaction_type": "PII",
                "enable_partial_results_stabilization": True,
                "filter_partial_results": True,
                "language_code": "en-US",
                "language_model_name": "MyLanguageModel",
                "partial_results_stability": "high",
                "pii_entity_types": "ADDRESS,BANK_ACCOUNT_NUMBER",
                "post_call_analytics_settings": {
                    "content_redaction_output": "redacted",
                    "data_access_role_arn": post_call_role.arn,
                    "output_encryption_kms_key_id": "MyKmsKeyId",
                    "output_location": "s3://MyBucket",
                },
                "vocabulary_filter_method": "mask",
                "vocabulary_filter_name": "MyVocabularyFilter",
                "vocabulary_name": "MyVocabulary",
            },
        },
        {
            "type": "KinesisDataStreamSink",
            "kinesis_data_stream_sink_configuration": {
                "insights_target": example["arn"],
            },
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
    var transcribeAssumeRole = Aws.Iam.GetPolicyDocument.Invoke(new()
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
                            "transcribe.amazonaws.com",
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

    var postCallRole = new Aws.Iam.Role("post_call_role", new()
    {
        Name = "PostCallAccessRole",
        AssumeRolePolicy = transcribeAssumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var myConfiguration = new Aws.ChimeSDKMediaPipelines.MediaInsightsPipelineConfiguration("my_configuration", new()
    {
        Name = "MyCallAnalyticsConfiguration",
        ResourceAccessRoleArn = exampleAwsIamRole.Arn,
        Elements = new[]
        {
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "AmazonTranscribeCallAnalyticsProcessor",
                AmazonTranscribeCallAnalyticsProcessorConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs
                {
                    CallAnalyticsStreamCategories = new[]
                    {
                        "category_1",
                        "category_2",
                    },
                    ContentRedactionType = "PII",
                    EnablePartialResultsStabilization = true,
                    FilterPartialResults = true,
                    LanguageCode = "en-US",
                    LanguageModelName = "MyLanguageModel",
                    PartialResultsStability = "high",
                    PiiEntityTypes = "ADDRESS,BANK_ACCOUNT_NUMBER",
                    PostCallAnalyticsSettings = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettingsArgs
                    {
                        ContentRedactionOutput = "redacted",
                        DataAccessRoleArn = postCallRole.Arn,
                        OutputEncryptionKmsKeyId = "MyKmsKeyId",
                        OutputLocation = "s3://MyBucket",
                    },
                    VocabularyFilterMethod = "mask",
                    VocabularyFilterName = "MyVocabularyFilter",
                    VocabularyName = "MyVocabulary",
                },
            },
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "KinesisDataStreamSink",
                KinesisDataStreamSinkConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs
                {
                    InsightsTarget = example.Arn,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chimesdkmediapipelines"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		transcribeAssumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Type: "Service",
							Identifiers: []string{
								"transcribe.amazonaws.com",
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
		postCallRole, err := iam.NewRole(ctx, "post_call_role", &iam.RoleArgs{
			Name:             pulumi.String("PostCallAccessRole"),
			AssumeRolePolicy: pulumi.String(transcribeAssumeRole.Json),
		})
		if err != nil {
			return err
		}
		_, err = chimesdkmediapipelines.NewMediaInsightsPipelineConfiguration(ctx, "my_configuration", &chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs{
			Name:                  pulumi.String("MyCallAnalyticsConfiguration"),
			ResourceAccessRoleArn: pulumi.Any(exampleAwsIamRole.Arn),
			Elements: chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArray{
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("AmazonTranscribeCallAnalyticsProcessor"),
					AmazonTranscribeCallAnalyticsProcessorConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs{
						CallAnalyticsStreamCategories: pulumi.StringArray{
							pulumi.String("category_1"),
							pulumi.String("category_2"),
						},
						ContentRedactionType:              pulumi.String("PII"),
						EnablePartialResultsStabilization: pulumi.Bool(true),
						FilterPartialResults:              pulumi.Bool(true),
						LanguageCode:                      pulumi.String("en-US"),
						LanguageModelName:                 pulumi.String("MyLanguageModel"),
						PartialResultsStability:           pulumi.String("high"),
						PiiEntityTypes:                    pulumi.String("ADDRESS,BANK_ACCOUNT_NUMBER"),
						PostCallAnalyticsSettings: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettingsArgs{
							ContentRedactionOutput:   pulumi.String("redacted"),
							DataAccessRoleArn:        postCallRole.Arn,
							OutputEncryptionKmsKeyId: pulumi.String("MyKmsKeyId"),
							OutputLocation:           pulumi.String("s3://MyBucket"),
						},
						VocabularyFilterMethod: pulumi.String("mask"),
						VocabularyFilterName:   pulumi.String("MyVocabularyFilter"),
						VocabularyName:         pulumi.String("MyVocabulary"),
					},
				},
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("KinesisDataStreamSink"),
					KinesisDataStreamSinkConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs{
						InsightsTarget: pulumi.Any(example.Arn),
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
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration;
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettingsArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs;
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
        final var transcribeAssumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("transcribe.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var postCallRole = new Role("postCallRole", RoleArgs.builder()
            .name("PostCallAccessRole")
            .assumeRolePolicy(transcribeAssumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var myConfiguration = new MediaInsightsPipelineConfiguration("myConfiguration", MediaInsightsPipelineConfigurationArgs.builder()
            .name("MyCallAnalyticsConfiguration")
            .resourceAccessRoleArn(exampleAwsIamRole.arn())
            .elements(            
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("AmazonTranscribeCallAnalyticsProcessor")
                    .amazonTranscribeCallAnalyticsProcessorConfiguration(MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs.builder()
                        .callAnalyticsStreamCategories(                        
                            "category_1",
                            "category_2")
                        .contentRedactionType("PII")
                        .enablePartialResultsStabilization(true)
                        .filterPartialResults(true)
                        .languageCode("en-US")
                        .languageModelName("MyLanguageModel")
                        .partialResultsStability("high")
                        .piiEntityTypes("ADDRESS,BANK_ACCOUNT_NUMBER")
                        .postCallAnalyticsSettings(MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettingsArgs.builder()
                            .contentRedactionOutput("redacted")
                            .dataAccessRoleArn(postCallRole.arn())
                            .outputEncryptionKmsKeyId("MyKmsKeyId")
                            .outputLocation("s3://MyBucket")
                            .build())
                        .vocabularyFilterMethod("mask")
                        .vocabularyFilterName("MyVocabularyFilter")
                        .vocabularyName("MyVocabulary")
                        .build())
                    .build(),
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("KinesisDataStreamSink")
                    .kinesisDataStreamSinkConfiguration(MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs.builder()
                        .insightsTarget(example.arn())
                        .build())
                    .build())
            .build());

    }
}
```
```yaml
resources:
  myConfiguration:
    type: aws:chimesdkmediapipelines:MediaInsightsPipelineConfiguration
    name: my_configuration
    properties:
      name: MyCallAnalyticsConfiguration
      resourceAccessRoleArn: ${exampleAwsIamRole.arn}
      elements:
        - type: AmazonTranscribeCallAnalyticsProcessor
          amazonTranscribeCallAnalyticsProcessorConfiguration:
            callAnalyticsStreamCategories:
              - category_1
              - category_2
            contentRedactionType: PII
            enablePartialResultsStabilization: true
            filterPartialResults: true
            languageCode: en-US
            languageModelName: MyLanguageModel
            partialResultsStability: high
            piiEntityTypes: ADDRESS,BANK_ACCOUNT_NUMBER
            postCallAnalyticsSettings:
              contentRedactionOutput: redacted
              dataAccessRoleArn: ${postCallRole.arn}
              outputEncryptionKmsKeyId: MyKmsKeyId
              outputLocation: s3://MyBucket
            vocabularyFilterMethod: mask
            vocabularyFilterName: MyVocabularyFilter
            vocabularyName: MyVocabulary
        - type: KinesisDataStreamSink
          kinesisDataStreamSinkConfiguration:
            insightsTarget: ${example.arn}
  postCallRole:
    type: aws:iam:Role
    name: post_call_role
    properties:
      name: PostCallAccessRole
      assumeRolePolicy: ${transcribeAssumeRole.json}
variables:
  transcribeAssumeRole:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            principals:
              - type: Service
                identifiers:
                  - transcribe.amazonaws.com
            actions:
              - sts:AssumeRole
```
<!--End PulumiCodeChooser -->

### Real time alerts usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const myConfiguration = new aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("my_configuration", {
    name: "MyRealTimeAlertConfiguration",
    resourceAccessRoleArn: callAnalyticsRole.arn,
    elements: [
        {
            type: "AmazonTranscribeCallAnalyticsProcessor",
            amazonTranscribeCallAnalyticsProcessorConfiguration: {
                languageCode: "en-US",
            },
        },
        {
            type: "KinesisDataStreamSink",
            kinesisDataStreamSinkConfiguration: {
                insightsTarget: example.arn,
            },
        },
    ],
    realTimeAlertConfiguration: {
        disabled: false,
        rules: [
            {
                type: "IssueDetection",
                issueDetectionConfiguration: {
                    ruleName: "MyIssueDetectionRule",
                },
            },
            {
                type: "KeywordMatch",
                keywordMatchConfiguration: {
                    keywords: [
                        "keyword1",
                        "keyword2",
                    ],
                    negate: false,
                    ruleName: "MyKeywordMatchRule",
                },
            },
            {
                type: "Sentiment",
                sentimentConfiguration: {
                    ruleName: "MySentimentRule",
                    sentimentType: "NEGATIVE",
                    timePeriod: 60,
                },
            },
        ],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

my_configuration = aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("my_configuration",
    name="MyRealTimeAlertConfiguration",
    resource_access_role_arn=call_analytics_role["arn"],
    elements=[
        {
            "type": "AmazonTranscribeCallAnalyticsProcessor",
            "amazon_transcribe_call_analytics_processor_configuration": {
                "language_code": "en-US",
            },
        },
        {
            "type": "KinesisDataStreamSink",
            "kinesis_data_stream_sink_configuration": {
                "insights_target": example["arn"],
            },
        },
    ],
    real_time_alert_configuration={
        "disabled": False,
        "rules": [
            {
                "type": "IssueDetection",
                "issue_detection_configuration": {
                    "rule_name": "MyIssueDetectionRule",
                },
            },
            {
                "type": "KeywordMatch",
                "keyword_match_configuration": {
                    "keywords": [
                        "keyword1",
                        "keyword2",
                    ],
                    "negate": False,
                    "rule_name": "MyKeywordMatchRule",
                },
            },
            {
                "type": "Sentiment",
                "sentiment_configuration": {
                    "rule_name": "MySentimentRule",
                    "sentiment_type": "NEGATIVE",
                    "time_period": 60,
                },
            },
        ],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var myConfiguration = new Aws.ChimeSDKMediaPipelines.MediaInsightsPipelineConfiguration("my_configuration", new()
    {
        Name = "MyRealTimeAlertConfiguration",
        ResourceAccessRoleArn = callAnalyticsRole.Arn,
        Elements = new[]
        {
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "AmazonTranscribeCallAnalyticsProcessor",
                AmazonTranscribeCallAnalyticsProcessorConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs
                {
                    LanguageCode = "en-US",
                },
            },
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "KinesisDataStreamSink",
                KinesisDataStreamSinkConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs
                {
                    InsightsTarget = example.Arn,
                },
            },
        },
        RealTimeAlertConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationArgs
        {
            Disabled = false,
            Rules = new[]
            {
                new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleArgs
                {
                    Type = "IssueDetection",
                    IssueDetectionConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleIssueDetectionConfigurationArgs
                    {
                        RuleName = "MyIssueDetectionRule",
                    },
                },
                new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleArgs
                {
                    Type = "KeywordMatch",
                    KeywordMatchConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfigurationArgs
                    {
                        Keywords = new[]
                        {
                            "keyword1",
                            "keyword2",
                        },
                        Negate = false,
                        RuleName = "MyKeywordMatchRule",
                    },
                },
                new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleArgs
                {
                    Type = "Sentiment",
                    SentimentConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfigurationArgs
                    {
                        RuleName = "MySentimentRule",
                        SentimentType = "NEGATIVE",
                        TimePeriod = 60,
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
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chimesdkmediapipelines"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chimesdkmediapipelines.NewMediaInsightsPipelineConfiguration(ctx, "my_configuration", &chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs{
			Name:                  pulumi.String("MyRealTimeAlertConfiguration"),
			ResourceAccessRoleArn: pulumi.Any(callAnalyticsRole.Arn),
			Elements: chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArray{
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("AmazonTranscribeCallAnalyticsProcessor"),
					AmazonTranscribeCallAnalyticsProcessorConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs{
						LanguageCode: pulumi.String("en-US"),
					},
				},
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("KinesisDataStreamSink"),
					KinesisDataStreamSinkConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs{
						InsightsTarget: pulumi.Any(example.Arn),
					},
				},
			},
			RealTimeAlertConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationArgs{
				Disabled: pulumi.Bool(false),
				Rules: chimesdkmediapipelines.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleArray{
					&chimesdkmediapipelines.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleArgs{
						Type: pulumi.String("IssueDetection"),
						IssueDetectionConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleIssueDetectionConfigurationArgs{
							RuleName: pulumi.String("MyIssueDetectionRule"),
						},
					},
					&chimesdkmediapipelines.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleArgs{
						Type: pulumi.String("KeywordMatch"),
						KeywordMatchConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfigurationArgs{
							Keywords: pulumi.StringArray{
								pulumi.String("keyword1"),
								pulumi.String("keyword2"),
							},
							Negate:   pulumi.Bool(false),
							RuleName: pulumi.String("MyKeywordMatchRule"),
						},
					},
					&chimesdkmediapipelines.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleArgs{
						Type: pulumi.String("Sentiment"),
						SentimentConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfigurationArgs{
							RuleName:      pulumi.String("MySentimentRule"),
							SentimentType: pulumi.String("NEGATIVE"),
							TimePeriod:    pulumi.Int(60),
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
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration;
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationRealTimeAlertConfigurationArgs;
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
        var myConfiguration = new MediaInsightsPipelineConfiguration("myConfiguration", MediaInsightsPipelineConfigurationArgs.builder()
            .name("MyRealTimeAlertConfiguration")
            .resourceAccessRoleArn(callAnalyticsRole.arn())
            .elements(            
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("AmazonTranscribeCallAnalyticsProcessor")
                    .amazonTranscribeCallAnalyticsProcessorConfiguration(MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationArgs.builder()
                        .languageCode("en-US")
                        .build())
                    .build(),
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("KinesisDataStreamSink")
                    .kinesisDataStreamSinkConfiguration(MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs.builder()
                        .insightsTarget(example.arn())
                        .build())
                    .build())
            .realTimeAlertConfiguration(MediaInsightsPipelineConfigurationRealTimeAlertConfigurationArgs.builder()
                .disabled(false)
                .rules(                
                    MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleArgs.builder()
                        .type("IssueDetection")
                        .issueDetectionConfiguration(MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleIssueDetectionConfigurationArgs.builder()
                            .ruleName("MyIssueDetectionRule")
                            .build())
                        .build(),
                    MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleArgs.builder()
                        .type("KeywordMatch")
                        .keywordMatchConfiguration(MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfigurationArgs.builder()
                            .keywords(                            
                                "keyword1",
                                "keyword2")
                            .negate(false)
                            .ruleName("MyKeywordMatchRule")
                            .build())
                        .build(),
                    MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleArgs.builder()
                        .type("Sentiment")
                        .sentimentConfiguration(MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfigurationArgs.builder()
                            .ruleName("MySentimentRule")
                            .sentimentType("NEGATIVE")
                            .timePeriod(60)
                            .build())
                        .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  myConfiguration:
    type: aws:chimesdkmediapipelines:MediaInsightsPipelineConfiguration
    name: my_configuration
    properties:
      name: MyRealTimeAlertConfiguration
      resourceAccessRoleArn: ${callAnalyticsRole.arn}
      elements:
        - type: AmazonTranscribeCallAnalyticsProcessor
          amazonTranscribeCallAnalyticsProcessorConfiguration:
            languageCode: en-US
        - type: KinesisDataStreamSink
          kinesisDataStreamSinkConfiguration:
            insightsTarget: ${example.arn}
      realTimeAlertConfiguration:
        disabled: false
        rules:
          - type: IssueDetection
            issueDetectionConfiguration:
              ruleName: MyIssueDetectionRule
          - type: KeywordMatch
            keywordMatchConfiguration:
              keywords:
                - keyword1
                - keyword2
              negate: false
              ruleName: MyKeywordMatchRule
          - type: Sentiment
            sentimentConfiguration:
              ruleName: MySentimentRule
              sentimentType: NEGATIVE
              timePeriod: 60
```
<!--End PulumiCodeChooser -->

### Transcribe processor usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const myConfiguration = new aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("my_configuration", {
    name: "MyTranscribeConfiguration",
    resourceAccessRoleArn: exampleAwsIamRole.arn,
    elements: [
        {
            type: "AmazonTranscribeProcessor",
            amazonTranscribeProcessorConfiguration: {
                contentIdentificationType: "PII",
                enablePartialResultsStabilization: true,
                filterPartialResults: true,
                languageCode: "en-US",
                languageModelName: "MyLanguageModel",
                partialResultsStability: "high",
                piiEntityTypes: "ADDRESS,BANK_ACCOUNT_NUMBER",
                showSpeakerLabel: true,
                vocabularyFilterMethod: "mask",
                vocabularyFilterName: "MyVocabularyFilter",
                vocabularyName: "MyVocabulary",
            },
        },
        {
            type: "KinesisDataStreamSink",
            kinesisDataStreamSinkConfiguration: {
                insightsTarget: example.arn,
            },
        },
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

my_configuration = aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("my_configuration",
    name="MyTranscribeConfiguration",
    resource_access_role_arn=example_aws_iam_role["arn"],
    elements=[
        {
            "type": "AmazonTranscribeProcessor",
            "amazon_transcribe_processor_configuration": {
                "content_identification_type": "PII",
                "enable_partial_results_stabilization": True,
                "filter_partial_results": True,
                "language_code": "en-US",
                "language_model_name": "MyLanguageModel",
                "partial_results_stability": "high",
                "pii_entity_types": "ADDRESS,BANK_ACCOUNT_NUMBER",
                "show_speaker_label": True,
                "vocabulary_filter_method": "mask",
                "vocabulary_filter_name": "MyVocabularyFilter",
                "vocabulary_name": "MyVocabulary",
            },
        },
        {
            "type": "KinesisDataStreamSink",
            "kinesis_data_stream_sink_configuration": {
                "insights_target": example["arn"],
            },
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
    var myConfiguration = new Aws.ChimeSDKMediaPipelines.MediaInsightsPipelineConfiguration("my_configuration", new()
    {
        Name = "MyTranscribeConfiguration",
        ResourceAccessRoleArn = exampleAwsIamRole.Arn,
        Elements = new[]
        {
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "AmazonTranscribeProcessor",
                AmazonTranscribeProcessorConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfigurationArgs
                {
                    ContentIdentificationType = "PII",
                    EnablePartialResultsStabilization = true,
                    FilterPartialResults = true,
                    LanguageCode = "en-US",
                    LanguageModelName = "MyLanguageModel",
                    PartialResultsStability = "high",
                    PiiEntityTypes = "ADDRESS,BANK_ACCOUNT_NUMBER",
                    ShowSpeakerLabel = true,
                    VocabularyFilterMethod = "mask",
                    VocabularyFilterName = "MyVocabularyFilter",
                    VocabularyName = "MyVocabulary",
                },
            },
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "KinesisDataStreamSink",
                KinesisDataStreamSinkConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs
                {
                    InsightsTarget = example.Arn,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chimesdkmediapipelines"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chimesdkmediapipelines.NewMediaInsightsPipelineConfiguration(ctx, "my_configuration", &chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs{
			Name:                  pulumi.String("MyTranscribeConfiguration"),
			ResourceAccessRoleArn: pulumi.Any(exampleAwsIamRole.Arn),
			Elements: chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArray{
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("AmazonTranscribeProcessor"),
					AmazonTranscribeProcessorConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfigurationArgs{
						ContentIdentificationType:         pulumi.String("PII"),
						EnablePartialResultsStabilization: pulumi.Bool(true),
						FilterPartialResults:              pulumi.Bool(true),
						LanguageCode:                      pulumi.String("en-US"),
						LanguageModelName:                 pulumi.String("MyLanguageModel"),
						PartialResultsStability:           pulumi.String("high"),
						PiiEntityTypes:                    pulumi.String("ADDRESS,BANK_ACCOUNT_NUMBER"),
						ShowSpeakerLabel:                  pulumi.Bool(true),
						VocabularyFilterMethod:            pulumi.String("mask"),
						VocabularyFilterName:              pulumi.String("MyVocabularyFilter"),
						VocabularyName:                    pulumi.String("MyVocabulary"),
					},
				},
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("KinesisDataStreamSink"),
					KinesisDataStreamSinkConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs{
						InsightsTarget: pulumi.Any(example.Arn),
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
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration;
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs;
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
        var myConfiguration = new MediaInsightsPipelineConfiguration("myConfiguration", MediaInsightsPipelineConfigurationArgs.builder()
            .name("MyTranscribeConfiguration")
            .resourceAccessRoleArn(exampleAwsIamRole.arn())
            .elements(            
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("AmazonTranscribeProcessor")
                    .amazonTranscribeProcessorConfiguration(MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfigurationArgs.builder()
                        .contentIdentificationType("PII")
                        .enablePartialResultsStabilization(true)
                        .filterPartialResults(true)
                        .languageCode("en-US")
                        .languageModelName("MyLanguageModel")
                        .partialResultsStability("high")
                        .piiEntityTypes("ADDRESS,BANK_ACCOUNT_NUMBER")
                        .showSpeakerLabel(true)
                        .vocabularyFilterMethod("mask")
                        .vocabularyFilterName("MyVocabularyFilter")
                        .vocabularyName("MyVocabulary")
                        .build())
                    .build(),
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("KinesisDataStreamSink")
                    .kinesisDataStreamSinkConfiguration(MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs.builder()
                        .insightsTarget(example.arn())
                        .build())
                    .build())
            .build());

    }
}
```
```yaml
resources:
  myConfiguration:
    type: aws:chimesdkmediapipelines:MediaInsightsPipelineConfiguration
    name: my_configuration
    properties:
      name: MyTranscribeConfiguration
      resourceAccessRoleArn: ${exampleAwsIamRole.arn}
      elements:
        - type: AmazonTranscribeProcessor
          amazonTranscribeProcessorConfiguration:
            contentIdentificationType: PII
            enablePartialResultsStabilization: true
            filterPartialResults: true
            languageCode: en-US
            languageModelName: MyLanguageModel
            partialResultsStability: high
            piiEntityTypes: ADDRESS,BANK_ACCOUNT_NUMBER
            showSpeakerLabel: true
            vocabularyFilterMethod: mask
            vocabularyFilterName: MyVocabularyFilter
            vocabularyName: MyVocabulary
        - type: KinesisDataStreamSink
          kinesisDataStreamSinkConfiguration:
            insightsTarget: ${example.arn}
```
<!--End PulumiCodeChooser -->

### Voice analytics processor usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const myConfiguration = new aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("my_configuration", {
    name: "MyVoiceAnalyticsConfiguration",
    resourceAccessRoleArn: example.arn,
    elements: [
        {
            type: "VoiceAnalyticsProcessor",
            voiceAnalyticsProcessorConfiguration: {
                speakerSearchStatus: "Enabled",
                voiceToneAnalysisStatus: "Enabled",
            },
        },
        {
            type: "LambdaFunctionSink",
            lambdaFunctionSinkConfiguration: {
                insightsTarget: "arn:aws:lambda:us-west-2:1111111111:function:MyFunction",
            },
        },
        {
            type: "SnsTopicSink",
            snsTopicSinkConfiguration: {
                insightsTarget: "arn:aws:sns:us-west-2:1111111111:topic/MyTopic",
            },
        },
        {
            type: "SqsQueueSink",
            sqsQueueSinkConfiguration: {
                insightsTarget: "arn:aws:sqs:us-west-2:1111111111:queue/MyQueue",
            },
        },
        {
            type: "KinesisDataStreamSink",
            kinesisDataStreamSinkConfiguration: {
                insightsTarget: test.arn,
            },
        },
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

my_configuration = aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("my_configuration",
    name="MyVoiceAnalyticsConfiguration",
    resource_access_role_arn=example["arn"],
    elements=[
        {
            "type": "VoiceAnalyticsProcessor",
            "voice_analytics_processor_configuration": {
                "speaker_search_status": "Enabled",
                "voice_tone_analysis_status": "Enabled",
            },
        },
        {
            "type": "LambdaFunctionSink",
            "lambda_function_sink_configuration": {
                "insights_target": "arn:aws:lambda:us-west-2:1111111111:function:MyFunction",
            },
        },
        {
            "type": "SnsTopicSink",
            "sns_topic_sink_configuration": {
                "insights_target": "arn:aws:sns:us-west-2:1111111111:topic/MyTopic",
            },
        },
        {
            "type": "SqsQueueSink",
            "sqs_queue_sink_configuration": {
                "insights_target": "arn:aws:sqs:us-west-2:1111111111:queue/MyQueue",
            },
        },
        {
            "type": "KinesisDataStreamSink",
            "kinesis_data_stream_sink_configuration": {
                "insights_target": test["arn"],
            },
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
    var myConfiguration = new Aws.ChimeSDKMediaPipelines.MediaInsightsPipelineConfiguration("my_configuration", new()
    {
        Name = "MyVoiceAnalyticsConfiguration",
        ResourceAccessRoleArn = example.Arn,
        Elements = new[]
        {
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "VoiceAnalyticsProcessor",
                VoiceAnalyticsProcessorConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfigurationArgs
                {
                    SpeakerSearchStatus = "Enabled",
                    VoiceToneAnalysisStatus = "Enabled",
                },
            },
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "LambdaFunctionSink",
                LambdaFunctionSinkConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfigurationArgs
                {
                    InsightsTarget = "arn:aws:lambda:us-west-2:1111111111:function:MyFunction",
                },
            },
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "SnsTopicSink",
                SnsTopicSinkConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementSnsTopicSinkConfigurationArgs
                {
                    InsightsTarget = "arn:aws:sns:us-west-2:1111111111:topic/MyTopic",
                },
            },
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "SqsQueueSink",
                SqsQueueSinkConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementSqsQueueSinkConfigurationArgs
                {
                    InsightsTarget = "arn:aws:sqs:us-west-2:1111111111:queue/MyQueue",
                },
            },
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "KinesisDataStreamSink",
                KinesisDataStreamSinkConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs
                {
                    InsightsTarget = test.Arn,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chimesdkmediapipelines"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chimesdkmediapipelines.NewMediaInsightsPipelineConfiguration(ctx, "my_configuration", &chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs{
			Name:                  pulumi.String("MyVoiceAnalyticsConfiguration"),
			ResourceAccessRoleArn: pulumi.Any(example.Arn),
			Elements: chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArray{
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("VoiceAnalyticsProcessor"),
					VoiceAnalyticsProcessorConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfigurationArgs{
						SpeakerSearchStatus:     pulumi.String("Enabled"),
						VoiceToneAnalysisStatus: pulumi.String("Enabled"),
					},
				},
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("LambdaFunctionSink"),
					LambdaFunctionSinkConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfigurationArgs{
						InsightsTarget: pulumi.String("arn:aws:lambda:us-west-2:1111111111:function:MyFunction"),
					},
				},
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("SnsTopicSink"),
					SnsTopicSinkConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementSnsTopicSinkConfigurationArgs{
						InsightsTarget: pulumi.String("arn:aws:sns:us-west-2:1111111111:topic/MyTopic"),
					},
				},
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("SqsQueueSink"),
					SqsQueueSinkConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementSqsQueueSinkConfigurationArgs{
						InsightsTarget: pulumi.String("arn:aws:sqs:us-west-2:1111111111:queue/MyQueue"),
					},
				},
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("KinesisDataStreamSink"),
					KinesisDataStreamSinkConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs{
						InsightsTarget: pulumi.Any(test.Arn),
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
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration;
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementSnsTopicSinkConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementSqsQueueSinkConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs;
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
        var myConfiguration = new MediaInsightsPipelineConfiguration("myConfiguration", MediaInsightsPipelineConfigurationArgs.builder()
            .name("MyVoiceAnalyticsConfiguration")
            .resourceAccessRoleArn(example.arn())
            .elements(            
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("VoiceAnalyticsProcessor")
                    .voiceAnalyticsProcessorConfiguration(MediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfigurationArgs.builder()
                        .speakerSearchStatus("Enabled")
                        .voiceToneAnalysisStatus("Enabled")
                        .build())
                    .build(),
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("LambdaFunctionSink")
                    .lambdaFunctionSinkConfiguration(MediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfigurationArgs.builder()
                        .insightsTarget("arn:aws:lambda:us-west-2:1111111111:function:MyFunction")
                        .build())
                    .build(),
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("SnsTopicSink")
                    .snsTopicSinkConfiguration(MediaInsightsPipelineConfigurationElementSnsTopicSinkConfigurationArgs.builder()
                        .insightsTarget("arn:aws:sns:us-west-2:1111111111:topic/MyTopic")
                        .build())
                    .build(),
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("SqsQueueSink")
                    .sqsQueueSinkConfiguration(MediaInsightsPipelineConfigurationElementSqsQueueSinkConfigurationArgs.builder()
                        .insightsTarget("arn:aws:sqs:us-west-2:1111111111:queue/MyQueue")
                        .build())
                    .build(),
                MediaInsightsPipelineConfigurationElementArgs.builder()
                    .type("KinesisDataStreamSink")
                    .kinesisDataStreamSinkConfiguration(MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationArgs.builder()
                        .insightsTarget(test.arn())
                        .build())
                    .build())
            .build());

    }
}
```
```yaml
resources:
  myConfiguration:
    type: aws:chimesdkmediapipelines:MediaInsightsPipelineConfiguration
    name: my_configuration
    properties:
      name: MyVoiceAnalyticsConfiguration
      resourceAccessRoleArn: ${example.arn}
      elements:
        - type: VoiceAnalyticsProcessor
          voiceAnalyticsProcessorConfiguration:
            speakerSearchStatus: Enabled
            voiceToneAnalysisStatus: Enabled
        - type: LambdaFunctionSink
          lambdaFunctionSinkConfiguration:
            insightsTarget: arn:aws:lambda:us-west-2:1111111111:function:MyFunction
        - type: SnsTopicSink
          snsTopicSinkConfiguration:
            insightsTarget: arn:aws:sns:us-west-2:1111111111:topic/MyTopic
        - type: SqsQueueSink
          sqsQueueSinkConfiguration:
            insightsTarget: arn:aws:sqs:us-west-2:1111111111:queue/MyQueue
        - type: KinesisDataStreamSink
          kinesisDataStreamSinkConfiguration:
            insightsTarget: ${test.arn}
```
<!--End PulumiCodeChooser -->

### S3 Recording sink usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const myConfiguration = new aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("my_configuration", {
    name: "MyS3RecordingConfiguration",
    resourceAccessRoleArn: example.arn,
    elements: [{
        type: "S3RecordingSink",
        s3RecordingSinkConfiguration: {
            destination: "arn:aws:s3:::MyBucket",
        },
    }],
});
```
```python
import pulumi
import pulumi_aws as aws

my_configuration = aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration("my_configuration",
    name="MyS3RecordingConfiguration",
    resource_access_role_arn=example["arn"],
    elements=[{
        "type": "S3RecordingSink",
        "s3_recording_sink_configuration": {
            "destination": "arn:aws:s3:::MyBucket",
        },
    }])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var myConfiguration = new Aws.ChimeSDKMediaPipelines.MediaInsightsPipelineConfiguration("my_configuration", new()
    {
        Name = "MyS3RecordingConfiguration",
        ResourceAccessRoleArn = example.Arn,
        Elements = new[]
        {
            new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementArgs
            {
                Type = "S3RecordingSink",
                S3RecordingSinkConfiguration = new Aws.ChimeSDKMediaPipelines.Inputs.MediaInsightsPipelineConfigurationElementS3RecordingSinkConfigurationArgs
                {
                    Destination = "arn:aws:s3:::MyBucket",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chimesdkmediapipelines"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chimesdkmediapipelines.NewMediaInsightsPipelineConfiguration(ctx, "my_configuration", &chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs{
			Name:                  pulumi.String("MyS3RecordingConfiguration"),
			ResourceAccessRoleArn: pulumi.Any(example.Arn),
			Elements: chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArray{
				&chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementArgs{
					Type: pulumi.String("S3RecordingSink"),
					S3RecordingSinkConfiguration: &chimesdkmediapipelines.MediaInsightsPipelineConfigurationElementS3RecordingSinkConfigurationArgs{
						Destination: pulumi.String("arn:aws:s3:::MyBucket"),
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
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfiguration;
import com.pulumi.aws.chimesdkmediapipelines.MediaInsightsPipelineConfigurationArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementArgs;
import com.pulumi.aws.chimesdkmediapipelines.inputs.MediaInsightsPipelineConfigurationElementS3RecordingSinkConfigurationArgs;
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
        var myConfiguration = new MediaInsightsPipelineConfiguration("myConfiguration", MediaInsightsPipelineConfigurationArgs.builder()
            .name("MyS3RecordingConfiguration")
            .resourceAccessRoleArn(example.arn())
            .elements(MediaInsightsPipelineConfigurationElementArgs.builder()
                .type("S3RecordingSink")
                .s3RecordingSinkConfiguration(MediaInsightsPipelineConfigurationElementS3RecordingSinkConfigurationArgs.builder()
                    .destination("arn:aws:s3:::MyBucket")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  myConfiguration:
    type: aws:chimesdkmediapipelines:MediaInsightsPipelineConfiguration
    name: my_configuration
    properties:
      name: MyS3RecordingConfiguration
      resourceAccessRoleArn: ${example.arn}
      elements:
        - type: S3RecordingSink
          s3RecordingSinkConfiguration:
            destination: arn:aws:s3:::MyBucket
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Chime SDK Media Pipelines Media Insights Pipeline Configuration using the `id`. For example:

```sh
$ pulumi import aws:chimesdkmediapipelines/mediaInsightsPipelineConfiguration:MediaInsightsPipelineConfiguration example abcdef123456
```
ì
elementsº*π:∂
≥
chimesdkmediapipelines)MediaInsightsPipelineConfigurationElementnaws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElement:MediaInsightsPipelineConfigurationElementHCollection of processors and sinks to transform media and deliver data.
"
nameB" Configuration name.
Ç
realTimeAlertConfigurationˆBÛ:
Ì
chimesdkmediapipelines<MediaInsightsPipelineConfigurationRealTimeAlertConfigurationîaws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationRealTimeAlertConfiguration:MediaInsightsPipelineConfigurationRealTimeAlertConfigurationkConfiguration for real-time alert rules to send EventBridge notifications when certain conditions are met.
Å
resourceAccessRoleArn" dARN of IAM Role used by service to invoke processors and sinks specified by configuration elements.
8
tagsB2" (Key-value map of tags for the resource.
"=
arn" 2ARN of the Media Insights Pipeline Configuration.
"ì
elementsº*π:∂
≥
chimesdkmediapipelines)MediaInsightsPipelineConfigurationElementnaws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElement:MediaInsightsPipelineConfigurationElementHCollection of processors and sinks to transform media and deliver data.
" 
name" Configuration name.
"Ç
realTimeAlertConfigurationˆBÛ:
Ì
chimesdkmediapipelines<MediaInsightsPipelineConfigurationRealTimeAlertConfigurationîaws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationRealTimeAlertConfiguration:MediaInsightsPipelineConfigurationRealTimeAlertConfigurationkConfiguration for real-time alert rules to send EventBridge notifications when certain conditions are met.
"Å
resourceAccessRoleArn" dARN of IAM Role used by service to invoke processors and sinks specified by configuration elements.
"8
tagsB2" (Key-value map of tags for the resource.
"
tagsAll2" *âe
G

cleanroomsCollaboration*aws:cleanrooms/collaboration:CollaborationΩ6Provides a AWS Clean Rooms collaboration.  All members included in the definition will be invited to
join the collaboration and can create memberships.

## Example Usage

### Collaboration with tags

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const testCollaboration = new aws.cleanrooms.Collaboration("test_collaboration", {
    name: "pulumi-example-collaboration",
    creatorMemberAbilities: [
        "CAN_QUERY",
        "CAN_RECEIVE_RESULTS",
    ],
    creatorDisplayName: "Creator ",
    description: "I made this collaboration with Pulumi!",
    queryLogStatus: "DISABLED",
    dataEncryptionMetadata: {
        allowClearText: true,
        allowDuplicates: true,
        allowJoinsOnColumnsWithDifferentNames: true,
        preserveNulls: false,
    },
    members: [{
        accountId: "123456789012",
        displayName: "Other member",
        memberAbilities: [],
    }],
    tags: {
        Project: "Pulumi",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test_collaboration = aws.cleanrooms.Collaboration("test_collaboration",
    name="pulumi-example-collaboration",
    creator_member_abilities=[
        "CAN_QUERY",
        "CAN_RECEIVE_RESULTS",
    ],
    creator_display_name="Creator ",
    description="I made this collaboration with Pulumi!",
    query_log_status="DISABLED",
    data_encryption_metadata={
        "allow_clear_text": True,
        "allow_duplicates": True,
        "allow_joins_on_columns_with_different_names": True,
        "preserve_nulls": False,
    },
    members=[{
        "account_id": "123456789012",
        "display_name": "Other member",
        "member_abilities": [],
    }],
    tags={
        "Project": "Pulumi",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var testCollaboration = new Aws.CleanRooms.Collaboration("test_collaboration", new()
    {
        Name = "pulumi-example-collaboration",
        CreatorMemberAbilities = new[]
        {
            "CAN_QUERY",
            "CAN_RECEIVE_RESULTS",
        },
        CreatorDisplayName = "Creator ",
        Description = "I made this collaboration with Pulumi!",
        QueryLogStatus = "DISABLED",
        DataEncryptionMetadata = new Aws.CleanRooms.Inputs.CollaborationDataEncryptionMetadataArgs
        {
            AllowClearText = true,
            AllowDuplicates = true,
            AllowJoinsOnColumnsWithDifferentNames = true,
            PreserveNulls = false,
        },
        Members = new[]
        {
            new Aws.CleanRooms.Inputs.CollaborationMemberArgs
            {
                AccountId = "123456789012",
                DisplayName = "Other member",
                MemberAbilities = new() { },
            },
        },
        Tags = 
        {
            { "Project", "Pulumi" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cleanrooms"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cleanrooms.NewCollaboration(ctx, "test_collaboration", &cleanrooms.CollaborationArgs{
			Name: pulumi.String("pulumi-example-collaboration"),
			CreatorMemberAbilities: pulumi.StringArray{
				pulumi.String("CAN_QUERY"),
				pulumi.String("CAN_RECEIVE_RESULTS"),
			},
			CreatorDisplayName: pulumi.String("Creator "),
			Description:        pulumi.String("I made this collaboration with Pulumi!"),
			QueryLogStatus:     pulumi.String("DISABLED"),
			DataEncryptionMetadata: &cleanrooms.CollaborationDataEncryptionMetadataArgs{
				AllowClearText:                        pulumi.Bool(true),
				AllowDuplicates:                       pulumi.Bool(true),
				AllowJoinsOnColumnsWithDifferentNames: pulumi.Bool(true),
				PreserveNulls:                         pulumi.Bool(false),
			},
			Members: cleanrooms.CollaborationMemberArray{
				&cleanrooms.CollaborationMemberArgs{
					AccountId:       pulumi.String("123456789012"),
					DisplayName:     pulumi.String("Other member"),
					MemberAbilities: pulumi.StringArray{},
				},
			},
			Tags: pulumi.StringMap{
				"Project": pulumi.String("Pulumi"),
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
import com.pulumi.aws.cleanrooms.Collaboration;
import com.pulumi.aws.cleanrooms.CollaborationArgs;
import com.pulumi.aws.cleanrooms.inputs.CollaborationDataEncryptionMetadataArgs;
import com.pulumi.aws.cleanrooms.inputs.CollaborationMemberArgs;
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
        var testCollaboration = new Collaboration("testCollaboration", CollaborationArgs.builder()
            .name("pulumi-example-collaboration")
            .creatorMemberAbilities(            
                "CAN_QUERY",
                "CAN_RECEIVE_RESULTS")
            .creatorDisplayName("Creator ")
            .description("I made this collaboration with Pulumi!")
            .queryLogStatus("DISABLED")
            .dataEncryptionMetadata(CollaborationDataEncryptionMetadataArgs.builder()
                .allowClearText(true)
                .allowDuplicates(true)
                .allowJoinsOnColumnsWithDifferentNames(true)
                .preserveNulls(false)
                .build())
            .members(CollaborationMemberArgs.builder()
                .accountId(123456789012)
                .displayName("Other member")
                .memberAbilities()
                .build())
            .tags(Map.of("Project", "Pulumi"))
            .build());

    }
}
```
```yaml
resources:
  testCollaboration:
    type: aws:cleanrooms:Collaboration
    name: test_collaboration
    properties:
      name: pulumi-example-collaboration
      creatorMemberAbilities:
        - CAN_QUERY
        - CAN_RECEIVE_RESULTS
      creatorDisplayName: 'Creator '
      description: I made this collaboration with Pulumi!
      queryLogStatus: DISABLED
      dataEncryptionMetadata:
        allowClearText: true
        allowDuplicates: true
        allowJoinsOnColumnsWithDifferentNames: true
        preserveNulls: false
      members:
        - accountId: 1.23456789012e+11
          displayName: Other member
          memberAbilities: []
      tags:
        Project: Pulumi
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_cleanrooms_collaboration` using the `id`. For example:

```sh
$ pulumi import aws:cleanrooms/collaboration:Collaboration collaboration 1234abcd-12ab-34cd-56ef-1234567890ab
```
X
creatorDisplayName" >The name for the member record for the collaboration creator.
ò
creatorMemberAbilities*" ˜The list of member abilities for the creator of the collaboration.  Valid values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_CreateCollaboration.html#API-CreateCollaboration-request-creatorMemberAbilities).
ã	
dataEncryptionMetadataíBè:å
â

cleanrooms#CollaborationDataEncryptionMetadataVaws:cleanrooms/CollaborationDataEncryptionMetadata:CollaborationDataEncryptionMetadata€a collection of settings which determine how the [c3r client](https://docs.aws.amazon.com/clean-rooms/latest/userguide/crypto-computing.html) will encrypt data for use within this collaboration.
* `data_encryption_metadata.allow_clear_text` - (Required - Forces new resource) - Indicates whether encrypted tables can contain cleartext data. This is a boolea
field.
* `data_encryption_metadata.allow_duplicates` - (Required - Forces new resource ) - Indicates whether Fingerprint columns can contain duplicate entries. This is a
boolean field.
* `data_encryption_metadata.allow_joins_on_columns_with_different_names` - (Required - Forces new resource) - Indicates whether Fingerprint columns can be joined
n any other Fingerprint column with a different name. This is a boolean field.
* `data_encryption_metadata.preserve_nulls` - (Required - Forces new resource) - Indicates whether NULL values are to be copied as NULL to encrypted tables (true)
or cryptographically processed (false).
6
description" #A description for a collaboration.
≤
membersaB_*]:[
Y

cleanroomsCollaborationMember6aws:cleanrooms/CollaborationMember:CollaborationMember√Additional members of the collaboration which will be invited to join the collaboration.
* `member.account_id` - (Required - Forces new resource) - The account id for the invited member.
* `member.display_name` - (Required - Forces new resource) - The display name for the invited member.
* `member.member_abilities` - (Required - Forces new resource) - The list of abilities for the invited member. Valid values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_CreateCollaboration.html#API-CreateCollaboration-request-creatorMemberAbilities).
\
nameB" NThe name of the collaboration.  Collaboration names do not need to be unique.
°
queryLogStatus" äDetermines if members of the collaboration can enable query logs within their own.
emberships. Valid values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_CreateCollaboration.html#API-CreateCollaboration-request-queryLogStatus).
=
tagsB2" -Key value pairs which tag the collaboration.
")
arn" The arn of the collaboration.
"œ

createTime" ºThe date and time the collaboration was created.
* `member status` - For each member included in the collaboration an additional computed attribute of status is added. These values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_MemberSummary.html#API-Type-MemberSummary-status).
"X
creatorDisplayName" >The name for the member record for the collaboration creator.
"ò
creatorMemberAbilities*" ˜The list of member abilities for the creator of the collaboration.  Valid values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_CreateCollaboration.html#API-CreateCollaboration-request-creatorMemberAbilities).
"ã	
dataEncryptionMetadataíBè:å
â

cleanrooms#CollaborationDataEncryptionMetadataVaws:cleanrooms/CollaborationDataEncryptionMetadata:CollaborationDataEncryptionMetadata€a collection of settings which determine how the [c3r client](https://docs.aws.amazon.com/clean-rooms/latest/userguide/crypto-computing.html) will encrypt data for use within this collaboration.
* `data_encryption_metadata.allow_clear_text` - (Required - Forces new resource) - Indicates whether encrypted tables can contain cleartext data. This is a boolea
field.
* `data_encryption_metadata.allow_duplicates` - (Required - Forces new resource ) - Indicates whether Fingerprint columns can contain duplicate entries. This is a
boolean field.
* `data_encryption_metadata.allow_joins_on_columns_with_different_names` - (Required - Forces new resource) - Indicates whether Fingerprint columns can be joined
n any other Fingerprint column with a different name. This is a boolean field.
* `data_encryption_metadata.preserve_nulls` - (Required - Forces new resource) - Indicates whether NULL values are to be copied as NULL to encrypted tables (true)
or cryptographically processed (false).
"6
description" #A description for a collaboration.
"≤
membersaB_*]:[
Y

cleanroomsCollaborationMember6aws:cleanrooms/CollaborationMember:CollaborationMember√Additional members of the collaboration which will be invited to join the collaboration.
* `member.account_id` - (Required - Forces new resource) - The account id for the invited member.
* `member.display_name` - (Required - Forces new resource) - The display name for the invited member.
* `member.member_abilities` - (Required - Forces new resource) - The list of abilities for the invited member. Valid values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_CreateCollaboration.html#API-CreateCollaboration-request-creatorMemberAbilities).
"Z
name" NThe name of the collaboration.  Collaboration names do not need to be unique.
"°
queryLogStatus" äDetermines if members of the collaboration can enable query logs within their own.
emberships. Valid values [may be found here](https://docs.aws.amazon.com/clean-rooms/latest/apireference/API_CreateCollaboration.html#API-CreateCollaboration-request-queryLogStatus).
"=
tagsB2" -Key value pairs which tag the collaboration.
"
tagsAll2" "

updateTime" *©7
M

cleanroomsConfiguredTable.aws:cleanrooms/configuredTable:ConfiguredTable—&Provides a AWS Clean Rooms configured table. Configured tables are used to represent references to existing tables in the AWS Glue Data Catalog.

## Example Usage

### Configured table with tags

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const testConfiguredTable = new aws.cleanrooms.ConfiguredTable("test_configured_table", {
    name: "pulumi-example-table",
    description: "I made this table with Pulumi!",
    analysisMethod: "DIRECT_QUERY",
    allowedColumns: [
        "column1",
        "column2",
        "column3",
    ],
    tableReference: {
        databaseName: "example_database",
        tableName: "example_table",
    },
    tags: {
        Project: "Pulumi",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test_configured_table = aws.cleanrooms.ConfiguredTable("test_configured_table",
    name="pulumi-example-table",
    description="I made this table with Pulumi!",
    analysis_method="DIRECT_QUERY",
    allowed_columns=[
        "column1",
        "column2",
        "column3",
    ],
    table_reference={
        "database_name": "example_database",
        "table_name": "example_table",
    },
    tags={
        "Project": "Pulumi",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var testConfiguredTable = new Aws.CleanRooms.ConfiguredTable("test_configured_table", new()
    {
        Name = "pulumi-example-table",
        Description = "I made this table with Pulumi!",
        AnalysisMethod = "DIRECT_QUERY",
        AllowedColumns = new[]
        {
            "column1",
            "column2",
            "column3",
        },
        TableReference = new Aws.CleanRooms.Inputs.ConfiguredTableTableReferenceArgs
        {
            DatabaseName = "example_database",
            TableName = "example_table",
        },
        Tags = 
        {
            { "Project", "Pulumi" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cleanrooms"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cleanrooms.NewConfiguredTable(ctx, "test_configured_table", &cleanrooms.ConfiguredTableArgs{
			Name:           pulumi.String("pulumi-example-table"),
			Description:    pulumi.String("I made this table with Pulumi!"),
			AnalysisMethod: pulumi.String("DIRECT_QUERY"),
			AllowedColumns: pulumi.StringArray{
				pulumi.String("column1"),
				pulumi.String("column2"),
				pulumi.String("column3"),
			},
			TableReference: &cleanrooms.ConfiguredTableTableReferenceArgs{
				DatabaseName: pulumi.String("example_database"),
				TableName:    pulumi.String("example_table"),
			},
			Tags: pulumi.StringMap{
				"Project": pulumi.String("Pulumi"),
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
import com.pulumi.aws.cleanrooms.ConfiguredTable;
import com.pulumi.aws.cleanrooms.ConfiguredTableArgs;
import com.pulumi.aws.cleanrooms.inputs.ConfiguredTableTableReferenceArgs;
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
        var testConfiguredTable = new ConfiguredTable("testConfiguredTable", ConfiguredTableArgs.builder()
            .name("pulumi-example-table")
            .description("I made this table with Pulumi!")
            .analysisMethod("DIRECT_QUERY")
            .allowedColumns(            
                "column1",
                "column2",
                "column3")
            .tableReference(ConfiguredTableTableReferenceArgs.builder()
                .databaseName("example_database")
                .tableName("example_table")
                .build())
            .tags(Map.of("Project", "Pulumi"))
            .build());

    }
}
```
```yaml
resources:
  testConfiguredTable:
    type: aws:cleanrooms:ConfiguredTable
    name: test_configured_table
    properties:
      name: pulumi-example-table
      description: I made this table with Pulumi!
      analysisMethod: DIRECT_QUERY
      allowedColumns:
        - column1
        - column2
        - column3
      tableReference:
        databaseName: example_database
        tableName: example_table
      tags:
        Project: Pulumi
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import `aws_cleanrooms_configured_table` using the `id`. For example:

```sh
$ pulumi import aws:cleanrooms/configuredTable:ConfiguredTable table 1234abcd-12ab-34cd-56ef-1234567890ab
```
l
allowedColumns*" TThe columns of the references table which will be included in the configured table.
v
analysisMethod" `The analysis method for the configured table. The only valid value is currently `DIRECT_QUERY`.
=
descriptionB" (A description for the configured table.
0
nameB" "The name of the configured table.
˛
tableReference{:y
w

cleanroomsConfiguredTableTableReferenceJaws:cleanrooms/ConfiguredTableTableReference:ConfiguredTableTableReferenceÓA reference to the AWS Glue table which will be used to create the configured table.
* `table_reference.database_name` - (Required - Forces new resource) - The name of the AWS Glue database which contains the table.
* `table_reference.table_name` - (Required - Forces new resource) - The name of the AWS Glue table which will be used to create the configured table.
@
tagsB2" 0Key value pairs which tag the configured table.
"l
allowedColumns*" TThe columns of the references table which will be included in the configured table.
"v
analysisMethod" `The analysis method for the configured table. The only valid value is currently `DIRECT_QUERY`.
",
arn" !The ARN of the configured table.
"F

createTime" 4The date and time the configured table was created.
"=
descriptionB" (A description for the configured table.
".
name" "The name of the configured table.
"˛
tableReference{:y
w

cleanroomsConfiguredTableTableReferenceJaws:cleanrooms/ConfiguredTableTableReference:ConfiguredTableTableReferenceÓA reference to the AWS Glue table which will be used to create the configured table.
* `table_reference.database_name` - (Required - Forces new resource) - The name of the AWS Glue database which contains the table.
* `table_reference.table_name` - (Required - Forces new resource) - The name of the AWS Glue table which will be used to create the configured table.
"@
tagsB2" 0Key value pairs which tag the configured table.
"
tagsAll2" "K

updateTime" 9The date and time the configured table was last updated.
*∞u
B
cloud9EnvironmentEC2(aws:cloud9/environmentEC2:EnvironmentEC2 ZProvides a Cloud9 EC2 Development Environment.

## Example Usage

Basic usage:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloud9.EnvironmentEC2("example", {
    instanceType: "t2.micro",
    name: "example-env",
    imageId: "amazonlinux-2023-x86_64",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloud9.EnvironmentEC2("example",
    instance_type="t2.micro",
    name="example-env",
    image_id="amazonlinux-2023-x86_64")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cloud9.EnvironmentEC2("example", new()
    {
        InstanceType = "t2.micro",
        Name = "example-env",
        ImageId = "amazonlinux-2023-x86_64",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloud9"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloud9.NewEnvironmentEC2(ctx, "example", &cloud9.EnvironmentEC2Args{
			InstanceType: pulumi.String("t2.micro"),
			Name:         pulumi.String("example-env"),
			ImageId:      pulumi.String("amazonlinux-2023-x86_64"),
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
import com.pulumi.aws.cloud9.EnvironmentEC2;
import com.pulumi.aws.cloud9.EnvironmentEC2Args;
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
        var example = new EnvironmentEC2("example", EnvironmentEC2Args.builder()
            .instanceType("t2.micro")
            .name("example-env")
            .imageId("amazonlinux-2023-x86_64")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloud9:EnvironmentEC2
    properties:
      instanceType: t2.micro
      name: example-env
      imageId: amazonlinux-2023-x86_64
```
<!--End PulumiCodeChooser -->

Get the URL of the Cloud9 environment after creation:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloud9.EnvironmentEC2("example", {instanceType: "t2.micro"});
const cloud9Instance = aws.ec2.getInstanceOutput({
    filters: [{
        name: "tag:aws:cloud9:environment",
        values: [example.id],
    }],
});
export const cloud9Url = pulumi.interpolate`https://${region}.console.aws.amazon.com/cloud9/ide/${example.id}`;
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloud9.EnvironmentEC2("example", instance_type="t2.micro")
cloud9_instance = aws.ec2.get_instance_output(filters=[{
    "name": "tag:aws:cloud9:environment",
    "values": [example.id],
}])
pulumi.export("cloud9Url", example.id.apply(lambda id: f"https://{region}.console.aws.amazon.com/cloud9/ide/{id}"))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cloud9.EnvironmentEC2("example", new()
    {
        InstanceType = "t2.micro",
    });

    var cloud9Instance = Aws.Ec2.GetInstance.Invoke(new()
    {
        Filters = new[]
        {
            new Aws.Ec2.Inputs.GetInstanceFilterInputArgs
            {
                Name = "tag:aws:cloud9:environment",
                Values = new[]
                {
                    example.Id,
                },
            },
        },
    });

    return new Dictionary<string, object?>
    {
        ["cloud9Url"] = example.Id.Apply(id => $"https://{region}.console.aws.amazon.com/cloud9/ide/{id}"),
    };
});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloud9"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := cloud9.NewEnvironmentEC2(ctx, "example", &cloud9.EnvironmentEC2Args{
			InstanceType: pulumi.String("t2.micro"),
		})
		if err != nil {
			return err
		}
		_ = ec2.LookupInstanceOutput(ctx, ec2.GetInstanceOutputArgs{
			Filters: ec2.GetInstanceFilterArray{
				&ec2.GetInstanceFilterArgs{
					Name: pulumi.String("tag:aws:cloud9:environment"),
					Values: pulumi.StringArray{
						example.ID(),
					},
				},
			},
		}, nil)
		ctx.Export("cloud9Url", example.ID().ApplyT(func(id string) (string, error) {
			return fmt.Sprintf("https://%v.console.aws.amazon.com/cloud9/ide/%v", region, id), nil
		}).(pulumi.StringOutput))
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cloud9.EnvironmentEC2;
import com.pulumi.aws.cloud9.EnvironmentEC2Args;
import com.pulumi.aws.ec2.Ec2Functions;
import com.pulumi.aws.ec2.inputs.GetInstanceArgs;
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
        var example = new EnvironmentEC2("example", EnvironmentEC2Args.builder()
            .instanceType("t2.micro")
            .build());

        final var cloud9Instance = Ec2Functions.getInstance(GetInstanceArgs.builder()
            .filters(GetInstanceFilterArgs.builder()
                .name("tag:aws:cloud9:environment")
                .values(example.id())
                .build())
            .build());

        ctx.export("cloud9Url", example.id().applyValue(id -> String.format("https://%s.console.aws.amazon.com/cloud9/ide/%s", region,id)));
    }
}
```
```yaml
resources:
  example:
    type: aws:cloud9:EnvironmentEC2
    properties:
      instanceType: t2.micro
variables:
  cloud9Instance:
    fn::invoke:
      function: aws:ec2:getInstance
      arguments:
        filters:
          - name: tag:aws:cloud9:environment
            values:
              - ${example.id}
outputs:
  cloud9Url: https://${region}.console.aws.amazon.com/cloud9/ide/${example.id}
```
<!--End PulumiCodeChooser -->

Allocate a static IP to the Cloud9 environment:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloud9.EnvironmentEC2("example", {instanceType: "t2.micro"});
const cloud9Instance = aws.ec2.getInstanceOutput({
    filters: [{
        name: "tag:aws:cloud9:environment",
        values: [example.id],
    }],
});
const cloud9Eip = new aws.ec2.Eip("cloud9_eip", {
    instance: cloud9Instance.apply(cloud9Instance => cloud9Instance.id),
    domain: "vpc",
});
export const cloud9PublicIp = cloud9Eip.publicIp;
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloud9.EnvironmentEC2("example", instance_type="t2.micro")
cloud9_instance = aws.ec2.get_instance_output(filters=[{
    "name": "tag:aws:cloud9:environment",
    "values": [example.id],
}])
cloud9_eip = aws.ec2.Eip("cloud9_eip",
    instance=cloud9_instance.id,
    domain="vpc")
pulumi.export("cloud9PublicIp", cloud9_eip.public_ip)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.Cloud9.EnvironmentEC2("example", new()
    {
        InstanceType = "t2.micro",
    });

    var cloud9Instance = Aws.Ec2.GetInstance.Invoke(new()
    {
        Filters = new[]
        {
            new Aws.Ec2.Inputs.GetInstanceFilterInputArgs
            {
                Name = "tag:aws:cloud9:environment",
                Values = new[]
                {
                    example.Id,
                },
            },
        },
    });

    var cloud9Eip = new Aws.Ec2.Eip("cloud9_eip", new()
    {
        Instance = cloud9Instance.Apply(getInstanceResult => getInstanceResult.Id),
        Domain = "vpc",
    });

    return new Dictionary<string, object?>
    {
        ["cloud9PublicIp"] = cloud9Eip.PublicIp,
    };
});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloud9"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := cloud9.NewEnvironmentEC2(ctx, "example", &cloud9.EnvironmentEC2Args{
			InstanceType: pulumi.String("t2.micro"),
		})
		if err != nil {
			return err
		}
		cloud9Instance := ec2.LookupInstanceOutput(ctx, ec2.GetInstanceOutputArgs{
			Filters: ec2.GetInstanceFilterArray{
				&ec2.GetInstanceFilterArgs{
					Name: pulumi.String("tag:aws:cloud9:environment"),
					Values: pulumi.StringArray{
						example.ID(),
					},
				},
			},
		}, nil)
		cloud9Eip, err := ec2.NewEip(ctx, "cloud9_eip", &ec2.EipArgs{
			Instance: pulumi.String(cloud9Instance.ApplyT(func(cloud9Instance ec2.GetInstanceResult) (*string, error) {
				return &cloud9Instance.Id, nil
			}).(pulumi.StringPtrOutput)),
			Domain: pulumi.String("vpc"),
		})
		if err != nil {
			return err
		}
		ctx.Export("cloud9PublicIp", cloud9Eip.PublicIp)
		return nil
	})
}
```
```java
package generated_program;

import com.pulumi.Context;
import com.pulumi.Pulumi;
import com.pulumi.core.Output;
import com.pulumi.aws.cloud9.EnvironmentEC2;
import com.pulumi.aws.cloud9.EnvironmentEC2Args;
import com.pulumi.aws.ec2.Ec2Functions;
import com.pulumi.aws.ec2.inputs.GetInstanceArgs;
import com.pulumi.aws.ec2.Eip;
import com.pulumi.aws.ec2.EipArgs;
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
        var example = new EnvironmentEC2("example", EnvironmentEC2Args.builder()
            .instanceType("t2.micro")
            .build());

        final var cloud9Instance = Ec2Functions.getInstance(GetInstanceArgs.builder()
            .filters(GetInstanceFilterArgs.builder()
                .name("tag:aws:cloud9:environment")
                .values(example.id())
                .build())
            .build());

        var cloud9Eip = new Eip("cloud9Eip", EipArgs.builder()
            .instance(cloud9Instance.applyValue(getInstanceResult -> getInstanceResult).applyValue(cloud9Instance -> cloud9Instance.applyValue(getInstanceResult -> getInstanceResult.id())))
            .domain("vpc")
            .build());

        ctx.export("cloud9PublicIp", cloud9Eip.publicIp());
    }
}
```
```yaml
resources:
  example:
    type: aws:cloud9:EnvironmentEC2
    properties:
      instanceType: t2.micro
  cloud9Eip:
    type: aws:ec2:Eip
    name: cloud9_eip
    properties:
      instance: ${cloud9Instance.id}
      domain: vpc
variables:
  cloud9Instance:
    fn::invoke:
      function: aws:ec2:getInstance
      arguments:
        filters:
          - name: tag:aws:cloud9:environment
            values:
              - ${example.id}
outputs:
  cloud9PublicIp: ${cloud9Eip.publicIp}
```
<!--End PulumiCodeChooser -->
ä
automaticStopTimeMinutesB hThe number of minutes until the running instance is shut down after the environment has last been used.
ñ
connectionTypeB" ˝The connection type used for connecting to an Amazon EC2 environment. Valid values are `CONNECT_SSH` and `CONNECT_SSM`. For more information please refer [AWS documentation for Cloud9](https://docs.aws.amazon.com/cloud9/latest/user-guide/ec2-ssm.html).
9
descriptionB" $The description of the environment.
Ÿ
imageId" …The identifier for the Amazon Machine Image (AMI) that's used to create the EC2 instance. Valid values are
* `amazonlinux-2-x86_64`
* `amazonlinux-2023-x86_64`
* `ubuntu-18.04-x86_64`
* `ubuntu-22.04-x86_64`
* `resolve:ssm:/aws/service/cloud9/amis/amazonlinux-2-x86_64`
* `resolve:ssm:/aws/service/cloud9/amis/amazonlinux-2023-x86_64`
* `resolve:ssm:/aws/service/cloud9/amis/ubuntu-18.04-x86_64`
* `resolve:ssm:/aws/service/cloud9/amis/ubuntu-22.04-x86_64`
Z
instanceType" FThe type of instance to connect to the environment, e.g., `t2.micro`.
+
nameB" The name of the environment.
Ö
ownerArnB" sThe ARN of the environment owner. This can be ARN of any AWS IAM principal. Defaults to the environment's creator.
{
subnetIdB" iThe ID of the subnet in Amazon VPC that AWS Cloud9 will use to communicate with the Amazon EC2 instance.
»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"'
arn" The ARN of the environment.
"ä
automaticStopTimeMinutesB hThe number of minutes until the running instance is shut down after the environment has last been used.
"ñ
connectionTypeB" ˝The connection type used for connecting to an Amazon EC2 environment. Valid values are `CONNECT_SSH` and `CONNECT_SSM`. For more information please refer [AWS documentation for Cloud9](https://docs.aws.amazon.com/cloud9/latest/user-guide/ec2-ssm.html).
"9
descriptionB" $The description of the environment.
"Ÿ
imageId" …The identifier for the Amazon Machine Image (AMI) that's used to create the EC2 instance. Valid values are
* `amazonlinux-2-x86_64`
* `amazonlinux-2023-x86_64`
* `ubuntu-18.04-x86_64`
* `ubuntu-22.04-x86_64`
* `resolve:ssm:/aws/service/cloud9/amis/amazonlinux-2-x86_64`
* `resolve:ssm:/aws/service/cloud9/amis/amazonlinux-2023-x86_64`
* `resolve:ssm:/aws/service/cloud9/amis/ubuntu-18.04-x86_64`
* `resolve:ssm:/aws/service/cloud9/amis/ubuntu-22.04-x86_64`
"Z
instanceType" FThe type of instance to connect to the environment, e.g., `t2.micro`.
")
name" The name of the environment.
"É
ownerArn" sThe ARN of the environment owner. This can be ARN of any AWS IAM principal. Defaults to the environment's creator.
"{
subnetIdB" iThe ID of the subnet in Amazon VPC that AWS Cloud9 will use to communicate with the Amazon EC2 instance.
"»
tagsB2" ∑Key-value map of resource tags. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"@
type" 4The type of the environment (e.g., `ssh` or `ec2`).
*–(
W
cloud9EnvironmentMembership6aws:cloud9/environmentMembership:EnvironmentMembershipπ!Provides an environment member to an AWS Cloud9 development environment.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.cloud9.EnvironmentEC2("test", {
    instanceType: "t2.micro",
    name: "some-env",
});
const testUser = new aws.iam.User("test", {name: "some-user"});
const testEnvironmentMembership = new aws.cloud9.EnvironmentMembership("test", {
    environmentId: test.id,
    permissions: "read-only",
    userArn: testUser.arn,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.cloud9.EnvironmentEC2("test",
    instance_type="t2.micro",
    name="some-env")
test_user = aws.iam.User("test", name="some-user")
test_environment_membership = aws.cloud9.EnvironmentMembership("test",
    environment_id=test.id,
    permissions="read-only",
    user_arn=test_user.arn)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = new Aws.Cloud9.EnvironmentEC2("test", new()
    {
        InstanceType = "t2.micro",
        Name = "some-env",
    });

    var testUser = new Aws.Iam.User("test", new()
    {
        Name = "some-user",
    });

    var testEnvironmentMembership = new Aws.Cloud9.EnvironmentMembership("test", new()
    {
        EnvironmentId = test.Id,
        Permissions = "read-only",
        UserArn = testUser.Arn,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloud9"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		test, err := cloud9.NewEnvironmentEC2(ctx, "test", &cloud9.EnvironmentEC2Args{
			InstanceType: pulumi.String("t2.micro"),
			Name:         pulumi.String("some-env"),
		})
		if err != nil {
			return err
		}
		testUser, err := iam.NewUser(ctx, "test", &iam.UserArgs{
			Name: pulumi.String("some-user"),
		})
		if err != nil {
			return err
		}
		_, err = cloud9.NewEnvironmentMembership(ctx, "test", &cloud9.EnvironmentMembershipArgs{
			EnvironmentId: test.ID(),
			Permissions:   pulumi.String("read-only"),
			UserArn:       testUser.Arn,
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
import com.pulumi.aws.cloud9.EnvironmentEC2;
import com.pulumi.aws.cloud9.EnvironmentEC2Args;
import com.pulumi.aws.iam.User;
import com.pulumi.aws.iam.UserArgs;
import com.pulumi.aws.cloud9.EnvironmentMembership;
import com.pulumi.aws.cloud9.EnvironmentMembershipArgs;
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
        var test = new EnvironmentEC2("test", EnvironmentEC2Args.builder()
            .instanceType("t2.micro")
            .name("some-env")
            .build());

        var testUser = new User("testUser", UserArgs.builder()
            .name("some-user")
            .build());

        var testEnvironmentMembership = new EnvironmentMembership("testEnvironmentMembership", EnvironmentMembershipArgs.builder()
            .environmentId(test.id())
            .permissions("read-only")
            .userArn(testUser.arn())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:cloud9:EnvironmentEC2
    properties:
      instanceType: t2.micro
      name: some-env
  testUser:
    type: aws:iam:User
    name: test
    properties:
      name: some-user
  testEnvironmentMembership:
    type: aws:cloud9:EnvironmentMembership
    name: test
    properties:
      environmentId: ${test.id}
      permissions: read-only
      userArn: ${testUser.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cloud9 environment membership using the `environment-id#user-arn`. For example:

```sh
$ pulumi import aws:cloud9/environmentMembership:EnvironmentMembership test environment-id#user-arn
```
e
environmentId" PThe ID of the environment that contains the environment member you want to add.
•
permissions" ëThe type of environment member permissions you want to associate with this environment member. Allowed values are `read-only` and `read-write` .
Y
userArn" JThe Amazon Resource Name (ARN) of the environment member you want to add.
"e
environmentId" PThe ID of the environment that contains the environment member you want to add.
"•
permissions" ëThe type of environment member permissions you want to associate with this environment member. Allowed values are `read-only` and `read-write` .
"Y
userArn" JThe Amazon Resource Name (ARN) of the environment member you want to add.
"e
userId" WThe user ID in AWS Identity and Access Management (AWS IAM) of the environment member.
*¶-
<
cloudcontrolResource"aws:cloudcontrol/resource:ResourceµManages a Cloud Control API Resource. The configuration and lifecycle handling of these resources is proxied through Cloud Control API handlers to the backend service.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudcontrol.Resource("example", {
    typeName: "AWS::ECS::Cluster",
    desiredState: JSON.stringify({
        ClusterName: "example",
        Tags: [{
            Key: "CostCenter",
            Value: "IT",
        }],
    }),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

example = aws.cloudcontrol.Resource("example",
    type_name="AWS::ECS::Cluster",
    desired_state=json.dumps({
        "ClusterName": "example",
        "Tags": [{
            "Key": "CostCenter",
            "Value": "IT",
        }],
    }))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudControl.Resource("example", new()
    {
        TypeName = "AWS::ECS::Cluster",
        DesiredState = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["ClusterName"] = "example",
            ["Tags"] = new[]
            {
                new Dictionary<string, object?>
                {
                    ["Key"] = "CostCenter",
                    ["Value"] = "IT",
                },
            },
        }),
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudcontrol"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"ClusterName": "example",
			"Tags": []map[string]interface{}{
				map[string]interface{}{
					"Key":   "CostCenter",
					"Value": "IT",
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = cloudcontrol.NewResource(ctx, "example", &cloudcontrol.ResourceArgs{
			TypeName:     pulumi.String("AWS::ECS::Cluster"),
			DesiredState: pulumi.String(json0),
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
import com.pulumi.aws.cloudcontrol.Resource;
import com.pulumi.aws.cloudcontrol.ResourceArgs;
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
        var example = new Resource("example", ResourceArgs.builder()
            .typeName("AWS::ECS::Cluster")
            .desiredState(serializeJson(
                jsonObject(
                    jsonProperty("ClusterName", "example"),
                    jsonProperty("Tags", jsonArray(jsonObject(
                        jsonProperty("Key", "CostCenter"),
                        jsonProperty("Value", "IT")
                    )))
                )))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudcontrol:Resource
    properties:
      typeName: AWS::ECS::Cluster
      desiredState:
        fn::toJSON:
          ClusterName: example
          Tags:
            - Key: CostCenter
              Value: IT
```
<!--End PulumiCodeChooser -->
m
desiredState" YJSON string matching the CloudFormation resource type schema with desired configuration.
V
roleArnB" EAmazon Resource Name (ARN) of the IAM Role to assume for operations.
˙
schemaB" ÈJSON string of the CloudFormation resource type schema which is used for plan time validation where possible. Automatically fetched if not provided. In large scale environments with multiple resources using the same `type_name`, it is recommended to fetch the schema once via the `aws.cloudformation.CloudFormationType` data source and use this argument to reduce `DescribeType` API operation throttling. This value is marked sensitive only to prevent large plan differences from showing.
x
typeName" hCloudFormation resource type name. For example, `AWS::EC2::VPC`.

The following arguments are optional:
O
typeVersionIdB" 8Identifier of the CloudFormation resource type version.
"m
desiredState" YJSON string matching the CloudFormation resource type schema with desired configuration.
"ë

properties" ˛JSON string matching the CloudFormation resource type schema with current configuration. Underlying attributes can be referenced via the `jsondecode()` function, for example, `jsondecode(data.aws_cloudcontrolapi_resource.example.properties)["example"]`.
"V
roleArnB" EAmazon Resource Name (ARN) of the IAM Role to assume for operations.
"¯
schema" ÈJSON string of the CloudFormation resource type schema which is used for plan time validation where possible. Automatically fetched if not provided. In large scale environments with multiple resources using the same `type_name`, it is recommended to fetch the schema once via the `aws.cloudformation.CloudFormationType` data source and use this argument to reduce `DescribeType` API operation throttling. This value is marked sensitive only to prevent large plan differences from showing.
"x
typeName" hCloudFormation resource type name. For example, `AWS::EC2::VPC`.

The following arguments are optional:
"O
typeVersionIdB" 8Identifier of the CloudFormation resource type version.
*¬
^
cloudformationCloudFormationType8aws:cloudformation/cloudFormationType:CloudFormationTypeˆManages a version of a CloudFormation Type.



## Import

Using `pulumi import`, import `aws_cloudformation_type` using the type version Amazon Resource Name (ARN). For example:

```sh
$ pulumi import aws:cloudformation/cloudFormationType:CloudFormationType example arn:aws:cloudformation:us-east-1:123456789012:type/resource/ExampleCompany-ExampleService-ExampleType/1
```
œ
executionRoleArnB" ¥Amazon Resource Name (ARN) of the IAM Role for CloudFormation to assume when invoking the extension. If your extension calls AWS APIs in any of its handlers, you must create an IAM execution role that includes the necessary permissions to call those AWS APIs, and provision that execution role in your account. When CloudFormation needs to invoke the extension handler, CloudFormation assumes this execution role to create a temporary session token, which it then passes to the extension handler, thereby supplying your extension with the appropriate credentials.
ÿ
loggingConfigéBã:à
Ö
cloudformationCloudFormationTypeLoggingConfigRaws:cloudformation/CloudFormationTypeLoggingConfig:CloudFormationTypeLoggingConfig6Configuration block containing logging configuration.
ˇ
schemaHandlerPackage" ‚URL to the S3 bucket containing the extension project package that contains the necessary files for the extension you want to register. Must begin with `s3://` or `https://`. For example, `s3://example-bucket/example-object`.
Q
typeB" CCloudFormation Registry Type. For example, `RESOURCE` or `MODULE`.
j
typeName" ZCloudFormation Type name. For example, `ExampleCompany::ExampleService::ExampleResource`.
"j
arn" _(Optional) Amazon Resource Name (ARN) of the CloudFormation Type version. See also `type_arn`.
"O
defaultVersionId" 7Identifier of the CloudFormation Type default version.
";
deprecatedStatus" #Deprecation status of the version.
"/
description" Description of the version.
"N
documentationUrl" 6URL of the documentation for the CloudFormation Type.
"œ
executionRoleArnB" ¥Amazon Resource Name (ARN) of the IAM Role for CloudFormation to assume when invoking the extension. If your extension calls AWS APIs in any of its handlers, you must create an IAM execution role that includes the necessary permissions to call those AWS APIs, and provision that execution role in your account. When CloudFormation needs to invoke the extension handler, CloudFormation assumes this execution role to create a temporary session token, which it then passes to the extension handler, thereby supplying your extension with the appropriate credentials.
"X
isDefaultVersion
 @Whether the CloudFormation Type version is the default version.
"ÿ
loggingConfigéBã:à
Ö
cloudformationCloudFormationTypeLoggingConfigRaws:cloudformation/CloudFormationTypeLoggingConfig:CloudFormationTypeLoggingConfig6Configuration block containing logging configuration.
"J
provisioningType" 2Provisioning behavior of the CloudFormation Type.
"?
schema" 1JSON document of the CloudFormation Type schema.
"ˇ
schemaHandlerPackage" ‚URL to the S3 bucket containing the extension project package that contains the necessary files for the extension you want to register. Must begin with `s3://` or `https://`. For example, `s3://example-bucket/example-object`.
"E
	sourceUrl" 4URL of the source code for the CloudFormation Type.
"O
type" CCloudFormation Registry Type. For example, `RESOURCE` or `MODULE`.
"a
typeArn" R(Optional) Amazon Resource Name (ARN) of the CloudFormation Type. See also `arn`.
"j
typeName" ZCloudFormation Type name. For example, `ExampleCompany::ExampleService::ExampleResource`.
"K
	versionId" :(Optional) Identifier of the CloudFormation Type version.
"4

visibility" "Scope of the CloudFormation Type.
*÷X
7
cloudformationStackaws:cloudformation/stack:Stack–<Provides a CloudFormation Stack resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const network = new aws.cloudformation.Stack("network", {
    name: "networking-stack",
    parameters: {
        VPCCidr: "10.0.0.0/16",
    },
    templateBody: JSON.stringify({
        Parameters: {
            VPCCidr: {
                Type: "String",
                Default: "10.0.0.0/16",
                Description: "Enter the CIDR block for the VPC. Default is 10.0.0.0/16.",
            },
        },
        Resources: {
            myVpc: {
                Type: "AWS::EC2::VPC",
                Properties: {
                    CidrBlock: {
                        Ref: "VPCCidr",
                    },
                    Tags: [{
                        Key: "Name",
                        Value: "Primary_CF_VPC",
                    }],
                },
            },
        },
    }),
});
```
```python
import pulumi
import json
import pulumi_aws as aws

network = aws.cloudformation.Stack("network",
    name="networking-stack",
    parameters={
        "VPCCidr": "10.0.0.0/16",
    },
    template_body=json.dumps({
        "Parameters": {
            "VPCCidr": {
                "Type": "String",
                "Default": "10.0.0.0/16",
                "Description": "Enter the CIDR block for the VPC. Default is 10.0.0.0/16.",
            },
        },
        "Resources": {
            "myVpc": {
                "Type": "AWS::EC2::VPC",
                "Properties": {
                    "CidrBlock": {
                        "Ref": "VPCCidr",
                    },
                    "Tags": [{
                        "Key": "Name",
                        "Value": "Primary_CF_VPC",
                    }],
                },
            },
        },
    }))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var network = new Aws.CloudFormation.Stack("network", new()
    {
        Name = "networking-stack",
        Parameters = 
        {
            { "VPCCidr", "10.0.0.0/16" },
        },
        TemplateBody = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Parameters"] = new Dictionary<string, object?>
            {
                ["VPCCidr"] = new Dictionary<string, object?>
                {
                    ["Type"] = "String",
                    ["Default"] = "10.0.0.0/16",
                    ["Description"] = "Enter the CIDR block for the VPC. Default is 10.0.0.0/16.",
                },
            },
            ["Resources"] = new Dictionary<string, object?>
            {
                ["myVpc"] = new Dictionary<string, object?>
                {
                    ["Type"] = "AWS::EC2::VPC",
                    ["Properties"] = new Dictionary<string, object?>
                    {
                        ["CidrBlock"] = new Dictionary<string, object?>
                        {
                            ["Ref"] = "VPCCidr",
                        },
                        ["Tags"] = new[]
                        {
                            new Dictionary<string, object?>
                            {
                                ["Key"] = "Name",
                                ["Value"] = "Primary_CF_VPC",
                            },
                        },
                    },
                },
            },
        }),
    });

});
```
```go
package main

import (
	"encoding/json"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudformation"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Parameters": map[string]interface{}{
				"VPCCidr": map[string]interface{}{
					"Type":        "String",
					"Default":     "10.0.0.0/16",
					"Description": "Enter the CIDR block for the VPC. Default is 10.0.0.0/16.",
				},
			},
			"Resources": map[string]interface{}{
				"myVpc": map[string]interface{}{
					"Type": "AWS::EC2::VPC",
					"Properties": map[string]interface{}{
						"CidrBlock": map[string]interface{}{
							"Ref": "VPCCidr",
						},
						"Tags": []map[string]interface{}{
							map[string]interface{}{
								"Key":   "Name",
								"Value": "Primary_CF_VPC",
							},
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		_, err = cloudformation.NewStack(ctx, "network", &cloudformation.StackArgs{
			Name: pulumi.String("networking-stack"),
			Parameters: pulumi.StringMap{
				"VPCCidr": pulumi.String("10.0.0.0/16"),
			},
			TemplateBody: pulumi.String(json0),
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
import com.pulumi.aws.cloudformation.Stack;
import com.pulumi.aws.cloudformation.StackArgs;
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
        var network = new Stack("network", StackArgs.builder()
            .name("networking-stack")
            .parameters(Map.of("VPCCidr", "10.0.0.0/16"))
            .templateBody(serializeJson(
                jsonObject(
                    jsonProperty("Parameters", jsonObject(
                        jsonProperty("VPCCidr", jsonObject(
                            jsonProperty("Type", "String"),
                            jsonProperty("Default", "10.0.0.0/16"),
                            jsonProperty("Description", "Enter the CIDR block for the VPC. Default is 10.0.0.0/16.")
                        ))
                    )),
                    jsonProperty("Resources", jsonObject(
                        jsonProperty("myVpc", jsonObject(
                            jsonProperty("Type", "AWS::EC2::VPC"),
                            jsonProperty("Properties", jsonObject(
                                jsonProperty("CidrBlock", jsonObject(
                                    jsonProperty("Ref", "VPCCidr")
                                )),
                                jsonProperty("Tags", jsonArray(jsonObject(
                                    jsonProperty("Key", "Name"),
                                    jsonProperty("Value", "Primary_CF_VPC")
                                )))
                            ))
                        ))
                    ))
                )))
            .build());

    }
}
```
```yaml
resources:
  network:
    type: aws:cloudformation:Stack
    properties:
      name: networking-stack
      parameters:
        VPCCidr: 10.0.0.0/16
      templateBody:
        fn::toJSON:
          Parameters:
            VPCCidr:
              Type: String
              Default: 10.0.0.0/16
              Description: Enter the CIDR block for the VPC. Default is 10.0.0.0/16.
          Resources:
            myVpc:
              Type: AWS::EC2::VPC
              Properties:
                CidrBlock:
                  Ref: VPCCidr
                Tags:
                  - Key: Name
                    Value: Primary_CF_VPC
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cloudformation Stacks using the `name`. For example:

```sh
$ pulumi import aws:cloudformation/stack:Stack stack networking-stack
```
Ñ
capabilitiesB*" lA list of capabilities.
Valid values: `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, or `CAPABILITY_AUTO_EXPAND`
}
disableRollbackB
 dSet to true to disable rollback of the stack if stack creation failed.
Conflicts with `on_failure`.
¬

iamRoleArnB" ≠The ARN of an IAM role that AWS CloudFormation assumes to create the stack. If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.

nameB" Stack name.
V
notificationArnsB*" :A list of SNS topic ARNs to publish stack related events.
ü
	onFailureB" ãAction to be taken if stack creation fails. This must be
one of: `DO_NOTHING`, `ROLLBACK`, or `DELETE`. Conflicts with `disable_rollback`.
a

parametersB2" KA map of Parameter structures that specify input parameters for the stack.
[

policyBodyB" GStructure containing the stack policy body.
Conflicts w/ `policy_url`.
_
	policyUrlB" LLocation of a file containing the stack policy.
Conflicts w/ `policy_body`.
€
tagsB2"  Map of resource tags to associate with this stack. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
W
templateBodyB" AStructure containing the template body (max size: 51,200 bytes).
`
templateUrlB" KLocation of a file containing the template body (max size: 460,800 bytes).
l
timeoutInMinutesB RThe amount of time that can pass before the stack status becomes `CREATE_FAILED`.
"Ñ
capabilitiesB*" lA list of capabilities.
Valid values: `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, or `CAPABILITY_AUTO_EXPAND`
"}
disableRollbackB
 dSet to true to disable rollback of the stack if stack creation failed.
Conflicts with `on_failure`.
"¬

iamRoleArnB" ≠The ARN of an IAM role that AWS CloudFormation assumes to create the stack. If you don't specify a value, AWS CloudFormation uses the role that was previously associated with the stack. If no role is available, AWS CloudFormation uses a temporary session that is generated from your user credentials.
"
name" Stack name.
"V
notificationArnsB*" :A list of SNS topic ARNs to publish stack related events.
"ü
	onFailureB" ãAction to be taken if stack creation fails. This must be
one of: `DO_NOTHING`, `ROLLBACK`, or `DELETE`. Conflicts with `disable_rollback`.
"2
outputs2" !A map of outputs from the stack.
"_

parameters2" KA map of Parameter structures that specify input parameters for the stack.
"Y

policyBody" GStructure containing the stack policy body.
Conflicts w/ `policy_url`.
"_
	policyUrlB" LLocation of a file containing the stack policy.
Conflicts w/ `policy_body`.
"€
tagsB2"  Map of resource tags to associate with this stack. .If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"U
templateBody" AStructure containing the template body (max size: 51,200 bytes).
"`
templateUrlB" KLocation of a file containing the template body (max size: 460,800 bytes).
"l
timeoutInMinutesB RThe amount of time that can pass before the stack status becomes `CREATE_FAILED`.
*àª
R
cloudformationStackInstances0aws:cloudformation/stackInstances:StackInstances«ó## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudformation.StackInstances("example", {
    accounts: [
        "123456789012",
        "234567890123",
    ],
    regions: [
        "us-east-1",
        "us-west-2",
    ],
    stackSetName: exampleAwsCloudformationStackSet.name,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudformation.StackInstances("example",
    accounts=[
        "123456789012",
        "234567890123",
    ],
    regions=[
        "us-east-1",
        "us-west-2",
    ],
    stack_set_name=example_aws_cloudformation_stack_set["name"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFormation.StackInstances("example", new()
    {
        Accounts = new[]
        {
            "123456789012",
            "234567890123",
        },
        Regions = new[]
        {
            "us-east-1",
            "us-west-2",
        },
        StackSetName = exampleAwsCloudformationStackSet.Name,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudformation"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudformation.NewStackInstances(ctx, "example", &cloudformation.StackInstancesArgs{
			Accounts: pulumi.StringArray{
				pulumi.String("123456789012"),
				pulumi.String("234567890123"),
			},
			Regions: pulumi.StringArray{
				pulumi.String("us-east-1"),
				pulumi.String("us-west-2"),
			},
			StackSetName: pulumi.Any(exampleAwsCloudformationStackSet.Name),
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
import com.pulumi.aws.cloudformation.StackInstances;
import com.pulumi.aws.cloudformation.StackInstancesArgs;
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
        var example = new StackInstances("example", StackInstancesArgs.builder()
            .accounts(            
                "123456789012",
                "234567890123")
            .regions(            
                "us-east-1",
                "us-west-2")
            .stackSetName(exampleAwsCloudformationStackSet.name())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudformation:StackInstances
    properties:
      accounts:
        - '123456789012'
        - '234567890123'
      regions:
        - us-east-1
        - us-west-2
      stackSetName: ${exampleAwsCloudformationStackSet.name}
```
<!--End PulumiCodeChooser -->

### Example IAM Setup in Target Account

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy = aws.iam.getPolicyDocument({
    statements: [{
        actions: ["sts:AssumeRole"],
        effect: "Allow",
        principals: [{
            identifiers: [aWSCloudFormationStackSetAdministrationRole.arn],
            type: "AWS",
        }],
    }],
});
const aWSCloudFormationStackSetExecutionRole = new aws.iam.Role("AWSCloudFormationStackSetExecutionRole", {
    assumeRolePolicy: aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.then(aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy => aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.json),
    name: "AWSCloudFormationStackSetExecutionRole",
});
// Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
// Additional IAM permissions necessary depend on the resources defined in the StackSet template
const aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy = aws.iam.getPolicyDocument({
    statements: [{
        actions: [
            "cloudformation:*",
            "s3:*",
            "sns:*",
        ],
        effect: "Allow",
        resources: ["*"],
    }],
});
const aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy = new aws.iam.RolePolicy("AWSCloudFormationStackSetExecutionRole_MinimumExecutionPolicy", {
    name: "MinimumExecutionPolicy",
    policy: aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.then(aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy => aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.json),
    role: aWSCloudFormationStackSetExecutionRole.name,
});
```
```python
import pulumi
import pulumi_aws as aws

a_ws_cloud_formation_stack_set_execution_role_assume_role_policy = aws.iam.get_policy_document(statements=[{
    "actions": ["sts:AssumeRole"],
    "effect": "Allow",
    "principals": [{
        "identifiers": [a_ws_cloud_formation_stack_set_administration_role["arn"]],
        "type": "AWS",
    }],
}])
a_ws_cloud_formation_stack_set_execution_role = aws.iam.Role("AWSCloudFormationStackSetExecutionRole",
    assume_role_policy=a_ws_cloud_formation_stack_set_execution_role_assume_role_policy.json,
    name="AWSCloudFormationStackSetExecutionRole")
# Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
# Additional IAM permissions necessary depend on the resources defined in the StackSet template
a_ws_cloud_formation_stack_set_execution_role_minimum_execution_policy = aws.iam.get_policy_document(statements=[{
    "actions": [
        "cloudformation:*",
        "s3:*",
        "sns:*",
    ],
    "effect": "Allow",
    "resources": ["*"],
}])
a_ws_cloud_formation_stack_set_execution_role_minimum_execution_policy_role_policy = aws.iam.RolePolicy("AWSCloudFormationStackSetExecutionRole_MinimumExecutionPolicy",
    name="MinimumExecutionPolicy",
    policy=a_ws_cloud_formation_stack_set_execution_role_minimum_execution_policy.json,
    role=a_ws_cloud_formation_stack_set_execution_role.name)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Identifiers = new[]
                        {
                            aWSCloudFormationStackSetAdministrationRole.Arn,
                        },
                        Type = "AWS",
                    },
                },
            },
        },
    });

    var aWSCloudFormationStackSetExecutionRole = new Aws.Iam.Role("AWSCloudFormationStackSetExecutionRole", new()
    {
        AssumeRolePolicy = aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
        Name = "AWSCloudFormationStackSetExecutionRole",
    });

    // Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
    // Additional IAM permissions necessary depend on the resources defined in the StackSet template
    var aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "cloudformation:*",
                    "s3:*",
                    "sns:*",
                },
                Effect = "Allow",
                Resources = new[]
                {
                    "*",
                },
            },
        },
    });

    var aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy = new Aws.Iam.RolePolicy("AWSCloudFormationStackSetExecutionRole_MinimumExecutionPolicy", new()
    {
        Name = "MinimumExecutionPolicy",
        Policy = aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
        Role = aWSCloudFormationStackSetExecutionRole.Name,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
Statements: []iam.GetPolicyDocumentStatement{
{
Actions: []string{
"sts:AssumeRole",
},
Effect: pulumi.StringRef("Allow"),
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Identifiers: interface{}{
aWSCloudFormationStackSetAdministrationRole.Arn,
},
Type: "AWS",
},
},
},
},
}, nil);
if err != nil {
return err
}
aWSCloudFormationStackSetExecutionRole, err := iam.NewRole(ctx, "AWSCloudFormationStackSetExecutionRole", &iam.RoleArgs{
AssumeRolePolicy: pulumi.String(aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.Json),
Name: pulumi.String("AWSCloudFormationStackSetExecutionRole"),
})
if err != nil {
return err
}
// Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
// Additional IAM permissions necessary depend on the resources defined in the StackSet template
aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
Statements: []iam.GetPolicyDocumentStatement{
{
Actions: []string{
"cloudformation:*",
"s3:*",
"sns:*",
},
Effect: pulumi.StringRef("Allow"),
Resources: []string{
"*",
},
},
},
}, nil);
if err != nil {
return err
}
_, err = iam.NewRolePolicy(ctx, "AWSCloudFormationStackSetExecutionRole_MinimumExecutionPolicy", &iam.RolePolicyArgs{
Name: pulumi.String("MinimumExecutionPolicy"),
Policy: pulumi.String(aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.Json),
Role: aWSCloudFormationStackSetExecutionRole.Name,
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
        final var aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions("sts:AssumeRole")
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .identifiers(aWSCloudFormationStackSetAdministrationRole.arn())
                    .type("AWS")
                    .build())
                .build())
            .build());

        var aWSCloudFormationStackSetExecutionRole = new Role("aWSCloudFormationStackSetExecutionRole", RoleArgs.builder()
            .assumeRolePolicy(aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .name("AWSCloudFormationStackSetExecutionRole")
            .build());

        // Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
        // Additional IAM permissions necessary depend on the resources defined in the StackSet template
        final var aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions(                
                    "cloudformation:*",
                    "s3:*",
                    "sns:*")
                .effect("Allow")
                .resources("*")
                .build())
            .build());

        var aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy = new RolePolicy("aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy", RolePolicyArgs.builder()
            .name("MinimumExecutionPolicy")
            .policy(aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .role(aWSCloudFormationStackSetExecutionRole.name())
            .build());

    }
}
```
```yaml
resources:
  aWSCloudFormationStackSetExecutionRole:
    type: aws:iam:Role
    name: AWSCloudFormationStackSetExecutionRole
    properties:
      assumeRolePolicy: ${aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.json}
      name: AWSCloudFormationStackSetExecutionRole
  aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy:
    type: aws:iam:RolePolicy
    name: AWSCloudFormationStackSetExecutionRole_MinimumExecutionPolicy
    properties:
      name: MinimumExecutionPolicy
      policy: ${aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.json}
      role: ${aWSCloudFormationStackSetExecutionRole.name}
variables:
  aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - actions:
              - sts:AssumeRole
            effect: Allow
            principals:
              - identifiers:
                  - ${aWSCloudFormationStackSetAdministrationRole.arn}
                type: AWS
  # Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
  # Additional IAM permissions necessary depend on the resources defined in the StackSet template
  aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - actions:
              - cloudformation:*
              - s3:*
              - sns:*
            effect: Allow
            resources:
              - '*'
```
<!--End PulumiCodeChooser -->

### Example Deployment across Organizations account

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudformation.StackInstances("example", {
    deploymentTargets: {
        organizationalUnitIds: [exampleAwsOrganizationsOrganization.roots[0].id],
    },
    regions: [
        "us-west-2",
        "us-east-1",
    ],
    stackSetName: exampleAwsCloudformationStackSet.name,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudformation.StackInstances("example",
    deployment_targets={
        "organizational_unit_ids": [example_aws_organizations_organization["roots"][0]["id"]],
    },
    regions=[
        "us-west-2",
        "us-east-1",
    ],
    stack_set_name=example_aws_cloudformation_stack_set["name"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFormation.StackInstances("example", new()
    {
        DeploymentTargets = new Aws.CloudFormation.Inputs.StackInstancesDeploymentTargetsArgs
        {
            OrganizationalUnitIds = new[]
            {
                exampleAwsOrganizationsOrganization.Roots[0].Id,
            },
        },
        Regions = new[]
        {
            "us-west-2",
            "us-east-1",
        },
        StackSetName = exampleAwsCloudformationStackSet.Name,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudformation"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudformation.NewStackInstances(ctx, "example", &cloudformation.StackInstancesArgs{
			DeploymentTargets: &cloudformation.StackInstancesDeploymentTargetsArgs{
				OrganizationalUnitIds: pulumi.StringArray{
					exampleAwsOrganizationsOrganization.Roots[0].Id,
				},
			},
			Regions: pulumi.StringArray{
				pulumi.String("us-west-2"),
				pulumi.String("us-east-1"),
			},
			StackSetName: pulumi.Any(exampleAwsCloudformationStackSet.Name),
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
import com.pulumi.aws.cloudformation.StackInstances;
import com.pulumi.aws.cloudformation.StackInstancesArgs;
import com.pulumi.aws.cloudformation.inputs.StackInstancesDeploymentTargetsArgs;
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
        var example = new StackInstances("example", StackInstancesArgs.builder()
            .deploymentTargets(StackInstancesDeploymentTargetsArgs.builder()
                .organizationalUnitIds(exampleAwsOrganizationsOrganization.roots()[0].id())
                .build())
            .regions(            
                "us-west-2",
                "us-east-1")
            .stackSetName(exampleAwsCloudformationStackSet.name())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudformation:StackInstances
    properties:
      deploymentTargets:
        organizationalUnitIds:
          - ${exampleAwsOrganizationsOrganization.roots[0].id}
      regions:
        - us-west-2
        - us-east-1
      stackSetName: ${exampleAwsCloudformationStackSet.name}
```
<!--End PulumiCodeChooser -->

## Import

Import CloudFormation stack instances that target OUs, using the stack set name, `call_as`, and "OU" separated by commas (`,`). For example:

Using `pulumi import`, import CloudFormation stack instances using the stack set name and `call_as` separated by commas (`,`). If you are importing a stack instance targeting OUs, see the example below. For example:

```sh
$ pulumi import aws:cloudformation/stackInstances:StackInstances example example,SELF
```
Using `pulumi import`, Import CloudFormation stack instances that target OUs, using the stack set name, `call_as`, and "OU" separated by commas (`,`). For example:

```sh
$ pulumi import aws:cloudformation/stackInstances:StackInstances example example,SELF,OU
```
¨
accountsB*" óAccounts where you want to create stack instances in the specified `regions`. You can specify either `accounts` or `deployment_targets`, but not both.
’
callAsB" ƒWhether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
â
deploymentTargetséBã:à
Ö
cloudformationStackInstancesDeploymentTargetsRaws:cloudformation/StackInstancesDeploymentTargets:StackInstancesDeploymentTargets‚AWS Organizations accounts for which to create stack instances in the `regions`. stack sets doesn't deploy stack instances to the organization management account, even if the organization management account is in your organization or in an OU in your organization. Drift detection is not possible for most of this argument. See deployment_targets below.
ö
operationPreferencesóBî:ë
é
cloudformation"StackInstancesOperationPreferencesXaws:cloudformation/StackInstancesOperationPreferences:StackInstancesOperationPreferenceshPreferences for how AWS CloudFormation performs a stack set operation. See operation_preferences below.

parameterOverridesB2" —Key-value map of input parameters to override from the stack set for these instances. This argument's drift detection is limited to the first account and region since each instance can have unique parameters.
a
regionsB*" NRegions where you want to create stack instances in the specified `accounts`.
‡
retainStacksB
 …Whether to remove the stack instances from the stack set, but not delete the stacks. You can't reassociate a retained stack or add an existing, saved stack to a new stack set. To retain the stack, ensure `retain_stacks = true` has been successfully applied _before_ an apply that would destroy the resource. Defaults to `false`.
R
stackSetName" >Name of the stack set.

The following arguments are optional:
"™
accounts*" óAccounts where you want to create stack instances in the specified `regions`. You can specify either `accounts` or `deployment_targets`, but not both.
"’
callAsB" ƒWhether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
"â
deploymentTargetséBã:à
Ö
cloudformationStackInstancesDeploymentTargetsRaws:cloudformation/StackInstancesDeploymentTargets:StackInstancesDeploymentTargets‚AWS Organizations accounts for which to create stack instances in the `regions`. stack sets doesn't deploy stack instances to the organization management account, even if the organization management account is in your organization or in an OU in your organization. Drift detection is not possible for most of this argument. See deployment_targets below.
"ö
operationPreferencesóBî:ë
é
cloudformation"StackInstancesOperationPreferencesXaws:cloudformation/StackInstancesOperationPreferences:StackInstancesOperationPreferenceshPreferences for how AWS CloudFormation performs a stack set operation. See operation_preferences below.
"
parameterOverridesB2" —Key-value map of input parameters to override from the stack set for these instances. This argument's drift detection is limited to the first account and region since each instance can have unique parameters.
"_
regions*" NRegions where you want to create stack instances in the specified `accounts`.
"‡
retainStacksB
 …Whether to remove the stack instances from the stack set, but not delete the stacks. You can't reassociate a retained stack or add an existing, saved stack to a new stack set. To retain the stack, ensure `retain_stacks = true` has been successfully applied _before_ an apply that would destroy the resource. Defaults to `false`.
"ç
stackInstanceSummariesó*î:ë
é
cloudformation"StackInstancesStackInstanceSummaryXaws:cloudformation/StackInstancesStackInstanceSummary:StackInstancesStackInstanceSummaryÿList of stack instances created from an organizational unit deployment target. This may not always be set depending on whether CloudFormation returns summaries for your configuration. See `stack_instance_summaries`.
"a

stackSetId" OName or unique ID of the stack set that the stack instance is associated with.
"R
stackSetName" >Name of the stack set.

The following arguments are optional:
*˙“
@
cloudformationStackSet$aws:cloudformation/stackSet:StackSetùManages a CloudFormation StackSet. StackSets allow CloudFormation templates to be easily deployed across multiple accounts and regions via StackSet Instances (`aws.cloudformation.StackSetInstance` resource). Additional information about StackSets can be found in the [AWS CloudFormation User Guide](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/what-is-cfnstacksets.html).

> **NOTE:** All template parameters, including those with a `Default`, must be configured or ignored with the `lifecycle` configuration block `ignore_changes` argument.

> **NOTE:** All `NoEcho` template parameters must be ignored with the `lifecycle` configuration block `ignore_changes` argument.

> **NOTE:** When using a delegated administrator account, ensure that your IAM User or Role has the `organizations:ListDelegatedAdministrators` permission. Otherwise, you may get an error like `ValidationError: Account used is not a delegated administrator`.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy = aws.iam.getPolicyDocument({
    statements: [{
        actions: ["sts:AssumeRole"],
        effect: "Allow",
        principals: [{
            identifiers: ["cloudformation.amazonaws.com"],
            type: "Service",
        }],
    }],
});
const aWSCloudFormationStackSetAdministrationRole = new aws.iam.Role("AWSCloudFormationStackSetAdministrationRole", {
    assumeRolePolicy: aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy.then(aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy => aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy.json),
    name: "AWSCloudFormationStackSetAdministrationRole",
});
const example = new aws.cloudformation.StackSet("example", {
    administrationRoleArn: aWSCloudFormationStackSetAdministrationRole.arn,
    name: "example",
    parameters: {
        VPCCidr: "10.0.0.0/16",
    },
    templateBody: JSON.stringify({
        Parameters: {
            VPCCidr: {
                Type: "String",
                Default: "10.0.0.0/16",
                Description: "Enter the CIDR block for the VPC. Default is 10.0.0.0/16.",
            },
        },
        Resources: {
            myVpc: {
                Type: "AWS::EC2::VPC",
                Properties: {
                    CidrBlock: {
                        Ref: "VPCCidr",
                    },
                    Tags: [{
                        Key: "Name",
                        Value: "Primary_CF_VPC",
                    }],
                },
            },
        },
    }),
});
const aWSCloudFormationStackSetAdministrationRoleExecutionPolicy = aws.iam.getPolicyDocumentOutput({
    statements: [{
        actions: ["sts:AssumeRole"],
        effect: "Allow",
        resources: [pulumi.interpolate`arn:aws:iam::*:role/${example.executionRoleName}`],
    }],
});
const aWSCloudFormationStackSetAdministrationRoleExecutionPolicyRolePolicy = new aws.iam.RolePolicy("AWSCloudFormationStackSetAdministrationRole_ExecutionPolicy", {
    name: "ExecutionPolicy",
    policy: aWSCloudFormationStackSetAdministrationRoleExecutionPolicy.apply(aWSCloudFormationStackSetAdministrationRoleExecutionPolicy => aWSCloudFormationStackSetAdministrationRoleExecutionPolicy.json),
    role: aWSCloudFormationStackSetAdministrationRole.name,
});
```
```python
import pulumi
import json
import pulumi_aws as aws

a_ws_cloud_formation_stack_set_administration_role_assume_role_policy = aws.iam.get_policy_document(statements=[{
    "actions": ["sts:AssumeRole"],
    "effect": "Allow",
    "principals": [{
        "identifiers": ["cloudformation.amazonaws.com"],
        "type": "Service",
    }],
}])
a_ws_cloud_formation_stack_set_administration_role = aws.iam.Role("AWSCloudFormationStackSetAdministrationRole",
    assume_role_policy=a_ws_cloud_formation_stack_set_administration_role_assume_role_policy.json,
    name="AWSCloudFormationStackSetAdministrationRole")
example = aws.cloudformation.StackSet("example",
    administration_role_arn=a_ws_cloud_formation_stack_set_administration_role.arn,
    name="example",
    parameters={
        "VPCCidr": "10.0.0.0/16",
    },
    template_body=json.dumps({
        "Parameters": {
            "VPCCidr": {
                "Type": "String",
                "Default": "10.0.0.0/16",
                "Description": "Enter the CIDR block for the VPC. Default is 10.0.0.0/16.",
            },
        },
        "Resources": {
            "myVpc": {
                "Type": "AWS::EC2::VPC",
                "Properties": {
                    "CidrBlock": {
                        "Ref": "VPCCidr",
                    },
                    "Tags": [{
                        "Key": "Name",
                        "Value": "Primary_CF_VPC",
                    }],
                },
            },
        },
    }))
a_ws_cloud_formation_stack_set_administration_role_execution_policy = aws.iam.get_policy_document_output(statements=[{
    "actions": ["sts:AssumeRole"],
    "effect": "Allow",
    "resources": [example.execution_role_name.apply(lambda execution_role_name: f"arn:aws:iam::*:role/{execution_role_name}")],
}])
a_ws_cloud_formation_stack_set_administration_role_execution_policy_role_policy = aws.iam.RolePolicy("AWSCloudFormationStackSetAdministrationRole_ExecutionPolicy",
    name="ExecutionPolicy",
    policy=a_ws_cloud_formation_stack_set_administration_role_execution_policy.json,
    role=a_ws_cloud_formation_stack_set_administration_role.name)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using System.Text.Json;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Identifiers = new[]
                        {
                            "cloudformation.amazonaws.com",
                        },
                        Type = "Service",
                    },
                },
            },
        },
    });

    var aWSCloudFormationStackSetAdministrationRole = new Aws.Iam.Role("AWSCloudFormationStackSetAdministrationRole", new()
    {
        AssumeRolePolicy = aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
        Name = "AWSCloudFormationStackSetAdministrationRole",
    });

    var example = new Aws.CloudFormation.StackSet("example", new()
    {
        AdministrationRoleArn = aWSCloudFormationStackSetAdministrationRole.Arn,
        Name = "example",
        Parameters = 
        {
            { "VPCCidr", "10.0.0.0/16" },
        },
        TemplateBody = JsonSerializer.Serialize(new Dictionary<string, object?>
        {
            ["Parameters"] = new Dictionary<string, object?>
            {
                ["VPCCidr"] = new Dictionary<string, object?>
                {
                    ["Type"] = "String",
                    ["Default"] = "10.0.0.0/16",
                    ["Description"] = "Enter the CIDR block for the VPC. Default is 10.0.0.0/16.",
                },
            },
            ["Resources"] = new Dictionary<string, object?>
            {
                ["myVpc"] = new Dictionary<string, object?>
                {
                    ["Type"] = "AWS::EC2::VPC",
                    ["Properties"] = new Dictionary<string, object?>
                    {
                        ["CidrBlock"] = new Dictionary<string, object?>
                        {
                            ["Ref"] = "VPCCidr",
                        },
                        ["Tags"] = new[]
                        {
                            new Dictionary<string, object?>
                            {
                                ["Key"] = "Name",
                                ["Value"] = "Primary_CF_VPC",
                            },
                        },
                    },
                },
            },
        }),
    });

    var aWSCloudFormationStackSetAdministrationRoleExecutionPolicy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
                Effect = "Allow",
                Resources = new[]
                {
                    $"arn:aws:iam::*:role/{example.ExecutionRoleName}",
                },
            },
        },
    });

    var aWSCloudFormationStackSetAdministrationRoleExecutionPolicyRolePolicy = new Aws.Iam.RolePolicy("AWSCloudFormationStackSetAdministrationRole_ExecutionPolicy", new()
    {
        Name = "ExecutionPolicy",
        Policy = aWSCloudFormationStackSetAdministrationRoleExecutionPolicy.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
        Role = aWSCloudFormationStackSetAdministrationRole.Name,
    });

});
```
```go
package main

import (
	"encoding/json"
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudformation"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
			Statements: []iam.GetPolicyDocumentStatement{
				{
					Actions: []string{
						"sts:AssumeRole",
					},
					Effect: pulumi.StringRef("Allow"),
					Principals: []iam.GetPolicyDocumentStatementPrincipal{
						{
							Identifiers: []string{
								"cloudformation.amazonaws.com",
							},
							Type: "Service",
						},
					},
				},
			},
		}, nil)
		if err != nil {
			return err
		}
		aWSCloudFormationStackSetAdministrationRole, err := iam.NewRole(ctx, "AWSCloudFormationStackSetAdministrationRole", &iam.RoleArgs{
			AssumeRolePolicy: pulumi.String(aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy.Json),
			Name:             pulumi.String("AWSCloudFormationStackSetAdministrationRole"),
		})
		if err != nil {
			return err
		}
		tmpJSON0, err := json.Marshal(map[string]interface{}{
			"Parameters": map[string]interface{}{
				"VPCCidr": map[string]interface{}{
					"Type":        "String",
					"Default":     "10.0.0.0/16",
					"Description": "Enter the CIDR block for the VPC. Default is 10.0.0.0/16.",
				},
			},
			"Resources": map[string]interface{}{
				"myVpc": map[string]interface{}{
					"Type": "AWS::EC2::VPC",
					"Properties": map[string]interface{}{
						"CidrBlock": map[string]interface{}{
							"Ref": "VPCCidr",
						},
						"Tags": []map[string]interface{}{
							map[string]interface{}{
								"Key":   "Name",
								"Value": "Primary_CF_VPC",
							},
						},
					},
				},
			},
		})
		if err != nil {
			return err
		}
		json0 := string(tmpJSON0)
		example, err := cloudformation.NewStackSet(ctx, "example", &cloudformation.StackSetArgs{
			AdministrationRoleArn: aWSCloudFormationStackSetAdministrationRole.Arn,
			Name:                  pulumi.String("example"),
			Parameters: pulumi.StringMap{
				"VPCCidr": pulumi.String("10.0.0.0/16"),
			},
			TemplateBody: pulumi.String(json0),
		})
		if err != nil {
			return err
		}
		aWSCloudFormationStackSetAdministrationRoleExecutionPolicy := iam.GetPolicyDocumentOutput(ctx, iam.GetPolicyDocumentOutputArgs{
			Statements: iam.GetPolicyDocumentStatementArray{
				&iam.GetPolicyDocumentStatementArgs{
					Actions: pulumi.StringArray{
						pulumi.String("sts:AssumeRole"),
					},
					Effect: pulumi.String("Allow"),
					Resources: pulumi.StringArray{
						example.ExecutionRoleName.ApplyT(func(executionRoleName string) (string, error) {
							return fmt.Sprintf("arn:aws:iam::*:role/%v", executionRoleName), nil
						}).(pulumi.StringOutput),
					},
				},
			},
		}, nil)
		_, err = iam.NewRolePolicy(ctx, "AWSCloudFormationStackSetAdministrationRole_ExecutionPolicy", &iam.RolePolicyArgs{
			Name: pulumi.String("ExecutionPolicy"),
			Policy: pulumi.String(aWSCloudFormationStackSetAdministrationRoleExecutionPolicy.ApplyT(func(aWSCloudFormationStackSetAdministrationRoleExecutionPolicy iam.GetPolicyDocumentResult) (*string, error) {
				return &aWSCloudFormationStackSetAdministrationRoleExecutionPolicy.Json, nil
			}).(pulumi.StringPtrOutput)),
			Role: aWSCloudFormationStackSetAdministrationRole.Name,
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
import com.pulumi.aws.cloudformation.StackSet;
import com.pulumi.aws.cloudformation.StackSetArgs;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
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
        final var aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions("sts:AssumeRole")
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .identifiers("cloudformation.amazonaws.com")
                    .type("Service")
                    .build())
                .build())
            .build());

        var aWSCloudFormationStackSetAdministrationRole = new Role("aWSCloudFormationStackSetAdministrationRole", RoleArgs.builder()
            .assumeRolePolicy(aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .name("AWSCloudFormationStackSetAdministrationRole")
            .build());

        var example = new StackSet("example", StackSetArgs.builder()
            .administrationRoleArn(aWSCloudFormationStackSetAdministrationRole.arn())
            .name("example")
            .parameters(Map.of("VPCCidr", "10.0.0.0/16"))
            .templateBody(serializeJson(
                jsonObject(
                    jsonProperty("Parameters", jsonObject(
                        jsonProperty("VPCCidr", jsonObject(
                            jsonProperty("Type", "String"),
                            jsonProperty("Default", "10.0.0.0/16"),
                            jsonProperty("Description", "Enter the CIDR block for the VPC. Default is 10.0.0.0/16.")
                        ))
                    )),
                    jsonProperty("Resources", jsonObject(
                        jsonProperty("myVpc", jsonObject(
                            jsonProperty("Type", "AWS::EC2::VPC"),
                            jsonProperty("Properties", jsonObject(
                                jsonProperty("CidrBlock", jsonObject(
                                    jsonProperty("Ref", "VPCCidr")
                                )),
                                jsonProperty("Tags", jsonArray(jsonObject(
                                    jsonProperty("Key", "Name"),
                                    jsonProperty("Value", "Primary_CF_VPC")
                                )))
                            ))
                        ))
                    ))
                )))
            .build());

        final var aWSCloudFormationStackSetAdministrationRoleExecutionPolicy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions("sts:AssumeRole")
                .effect("Allow")
                .resources(example.executionRoleName().applyValue(executionRoleName -> String.format("arn:aws:iam::*:role/%s", executionRoleName)))
                .build())
            .build());

        var aWSCloudFormationStackSetAdministrationRoleExecutionPolicyRolePolicy = new RolePolicy("aWSCloudFormationStackSetAdministrationRoleExecutionPolicyRolePolicy", RolePolicyArgs.builder()
            .name("ExecutionPolicy")
            .policy(aWSCloudFormationStackSetAdministrationRoleExecutionPolicy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult).applyValue(aWSCloudFormationStackSetAdministrationRoleExecutionPolicy -> aWSCloudFormationStackSetAdministrationRoleExecutionPolicy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json())))
            .role(aWSCloudFormationStackSetAdministrationRole.name())
            .build());

    }
}
```
```yaml
resources:
  aWSCloudFormationStackSetAdministrationRole:
    type: aws:iam:Role
    name: AWSCloudFormationStackSetAdministrationRole
    properties:
      assumeRolePolicy: ${aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy.json}
      name: AWSCloudFormationStackSetAdministrationRole
  example:
    type: aws:cloudformation:StackSet
    properties:
      administrationRoleArn: ${aWSCloudFormationStackSetAdministrationRole.arn}
      name: example
      parameters:
        VPCCidr: 10.0.0.0/16
      templateBody:
        fn::toJSON:
          Parameters:
            VPCCidr:
              Type: String
              Default: 10.0.0.0/16
              Description: Enter the CIDR block for the VPC. Default is 10.0.0.0/16.
          Resources:
            myVpc:
              Type: AWS::EC2::VPC
              Properties:
                CidrBlock:
                  Ref: VPCCidr
                Tags:
                  - Key: Name
                    Value: Primary_CF_VPC
  aWSCloudFormationStackSetAdministrationRoleExecutionPolicyRolePolicy:
    type: aws:iam:RolePolicy
    name: AWSCloudFormationStackSetAdministrationRole_ExecutionPolicy
    properties:
      name: ExecutionPolicy
      policy: ${aWSCloudFormationStackSetAdministrationRoleExecutionPolicy.json}
      role: ${aWSCloudFormationStackSetAdministrationRole.name}
variables:
  aWSCloudFormationStackSetAdministrationRoleAssumeRolePolicy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - actions:
              - sts:AssumeRole
            effect: Allow
            principals:
              - identifiers:
                  - cloudformation.amazonaws.com
                type: Service
  aWSCloudFormationStackSetAdministrationRoleExecutionPolicy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - actions:
              - sts:AssumeRole
            effect: Allow
            resources:
              - arn:aws:iam::*:role/${example.executionRoleName}
```
<!--End PulumiCodeChooser -->

## Import

Import CloudFormation StackSets when acting a delegated administrator in a member account using the `name` and `call_as` values separated by a comma (`,`). For example:

Using `pulumi import`, import CloudFormation StackSets using the `name`. For example:

```sh
$ pulumi import aws:cloudformation/stackSet:StackSet example example
```
Using `pulumi import`, import CloudFormation StackSets when acting a delegated administrator in a member account using the `name` and `call_as` values separated by a comma (`,`). For example:

```sh
$ pulumi import aws:cloudformation/stackSet:StackSet example example,DELEGATED_ADMIN
```
∞
administrationRoleArnB" êAmazon Resource Number (ARN) of the IAM Role in the administrator account. This must be defined when using the `SELF_MANAGED` permission model.
ù
autoDeploymentpBn:l
j
cloudformationStackSetAutoDeployment@aws:cloudformation/StackSetAutoDeployment:StackSetAutoDeploymentòConfiguration block containing the auto-deployment model for your StackSet. This can only be defined when using the `SERVICE_MANAGED` permission model.
ﬂ
callAsB" ŒSpecifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
Ç
capabilitiesB*" jA list of capabilities. Valid values: `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, `CAPABILITY_AUTO_EXPAND`.
2
descriptionB" Description of the StackSet.
î
executionRoleNameB" ¯Name of the IAM Role in all target accounts for StackSet operations. Defaults to `AWSCloudFormationStackSetExecutionRole` when using the `SELF_MANAGED` permission model. This should not be defined when using the `SERVICE_MANAGED` permission model.
â
managedExecutionvBt:r
p
cloudformationStackSetManagedExecutionDaws:cloudformation/StackSetManagedExecution:StackSetManagedExecution}Configuration block to allow StackSets to perform non-conflicting operations concurrently and queues conflicting operations.
ê
nameB" ÅName of the StackSet. The name must be unique in the region where you create your StackSet. The name can contain only alphanumeric characters (case-sensitive) and hyphens. It must start with an alphabetic character and cannot be longer than 128 characters.
‚
operationPreferencesÉBÄ:~
|
cloudformationStackSetOperationPreferencesLaws:cloudformation/StackSetOperationPreferences:StackSetOperationPreferencesDPreferences for how AWS CloudFormation performs a stack set update.
‚

parametersB2" ÀKey-value map of input parameters for the StackSet template. All template parameters, including those with a `Default`, must be configured or ignored with `lifecycle` configuration block `ignore_changes` argument. All `NoEcho` template parameters must be ignored with the `lifecycle` configuration block `ignore_changes` argument.
ò
permissionModelB" Describes how the IAM roles required for your StackSet are created. Valid values: `SELF_MANAGED` (default), `SERVICE_MANAGED`.
ê
tagsB2" ˇKey-value map of tags to associate with this StackSet and the Stacks created from it. AWS CloudFormation also propagates these tags to supported resources that are created in the Stacks. A maximum number of 50 tags can be specified. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
Ö
templateBodyB" oString containing the CloudFormation template body. Maximum size: 51,200 bytes. Conflicts with `template_url`.
Ä
templateUrlB" ÍString containing the location of a file containing the CloudFormation template body. The URL must point to a template that is located in an Amazon S3 bucket. Maximum location file size: 460,800 bytes. Conflicts with `template_body`.
"∞
administrationRoleArnB" êAmazon Resource Number (ARN) of the IAM Role in the administrator account. This must be defined when using the `SELF_MANAGED` permission model.
"7
arn" ,Amazon Resource Name (ARN) of the StackSet.
"ù
autoDeploymentpBn:l
j
cloudformationStackSetAutoDeployment@aws:cloudformation/StackSetAutoDeployment:StackSetAutoDeploymentòConfiguration block containing the auto-deployment model for your StackSet. This can only be defined when using the `SERVICE_MANAGED` permission model.
"ﬂ
callAsB" ŒSpecifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
"Ç
capabilitiesB*" jA list of capabilities. Valid values: `CAPABILITY_IAM`, `CAPABILITY_NAMED_IAM`, `CAPABILITY_AUTO_EXPAND`.
"2
descriptionB" Description of the StackSet.
"í
executionRoleName" ¯Name of the IAM Role in all target accounts for StackSet operations. Defaults to `AWSCloudFormationStackSetExecutionRole` when using the `SELF_MANAGED` permission model. This should not be defined when using the `SERVICE_MANAGED` permission model.
"â
managedExecutionvBt:r
p
cloudformationStackSetManagedExecutionDaws:cloudformation/StackSetManagedExecution:StackSetManagedExecution}Configuration block to allow StackSets to perform non-conflicting operations concurrently and queues conflicting operations.
"é
name" ÅName of the StackSet. The name must be unique in the region where you create your StackSet. The name can contain only alphanumeric characters (case-sensitive) and hyphens. It must start with an alphabetic character and cannot be longer than 128 characters.
"‚
operationPreferencesÉBÄ:~
|
cloudformationStackSetOperationPreferencesLaws:cloudformation/StackSetOperationPreferences:StackSetOperationPreferencesDPreferences for how AWS CloudFormation performs a stack set update.
"‚

parametersB2" ÀKey-value map of input parameters for the StackSet template. All template parameters, including those with a `Default`, must be configured or ignored with `lifecycle` configuration block `ignore_changes` argument. All `NoEcho` template parameters must be ignored with the `lifecycle` configuration block `ignore_changes` argument.
"ò
permissionModelB" Describes how the IAM roles required for your StackSet are created. Valid values: `SELF_MANAGED` (default), `SERVICE_MANAGED`.
"5

stackSetId" #Unique identifier of the StackSet.
"ê
tagsB2" ˇKey-value map of tags to associate with this StackSet and the Stacks created from it. AWS CloudFormation also propagates these tags to supported resources that are created in the Stacks. A maximum number of 50 tags can be specified. If configured with a provider `default_tags` configuration block present, tags with matching keys will overwrite those defined at the provider-level.
"â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"É
templateBody" oString containing the CloudFormation template body. Maximum size: 51,200 bytes. Conflicts with `template_url`.
"Ä
templateUrlB" ÍString containing the location of a file containing the CloudFormation template body. The URL must point to a template that is located in an Amazon S3 bucket. Maximum location file size: 460,800 bytes. Conflicts with `template_body`.
*í¿
X
cloudformationStackSetInstance4aws:cloudformation/stackSetInstance:StackSetInstance¡°Manages a CloudFormation StackSet Instance. Instances are managed in the account and region of the StackSet after the target account permissions have been configured. Additional information about StackSets can be found in the [AWS CloudFormation User Guide](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/what-is-cfnstacksets.html).

> **NOTE:** All target accounts must have an IAM Role created that matches the name of the execution role configured in the StackSet (the `execution_role_name` argument in the `aws.cloudformation.StackSet` resource) in a trust relationship with the administrative account or administration IAM Role. The execution role must have appropriate permissions to manage resources defined in the template along with those required for StackSets to operate. See the [AWS CloudFormation User Guide](https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html) for more details.

> **NOTE:** To retain the Stack during resource destroy, ensure `retain_stack` has been set to `true` in the state first. This must be completed _before_ a deployment that would destroy the resource.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudformation.StackSetInstance("example", {
    accountId: "123456789012",
    region: "us-east-1",
    stackSetName: exampleAwsCloudformationStackSet.name,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudformation.StackSetInstance("example",
    account_id="123456789012",
    region="us-east-1",
    stack_set_name=example_aws_cloudformation_stack_set["name"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFormation.StackSetInstance("example", new()
    {
        AccountId = "123456789012",
        Region = "us-east-1",
        StackSetName = exampleAwsCloudformationStackSet.Name,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudformation"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudformation.NewStackSetInstance(ctx, "example", &cloudformation.StackSetInstanceArgs{
			AccountId:    pulumi.String("123456789012"),
			Region:       pulumi.String("us-east-1"),
			StackSetName: pulumi.Any(exampleAwsCloudformationStackSet.Name),
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
import com.pulumi.aws.cloudformation.StackSetInstance;
import com.pulumi.aws.cloudformation.StackSetInstanceArgs;
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
        var example = new StackSetInstance("example", StackSetInstanceArgs.builder()
            .accountId("123456789012")
            .region("us-east-1")
            .stackSetName(exampleAwsCloudformationStackSet.name())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudformation:StackSetInstance
    properties:
      accountId: '123456789012'
      region: us-east-1
      stackSetName: ${exampleAwsCloudformationStackSet.name}
```
<!--End PulumiCodeChooser -->

### Example IAM Setup in Target Account

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy = aws.iam.getPolicyDocument({
    statements: [{
        actions: ["sts:AssumeRole"],
        effect: "Allow",
        principals: [{
            identifiers: [aWSCloudFormationStackSetAdministrationRole.arn],
            type: "AWS",
        }],
    }],
});
const aWSCloudFormationStackSetExecutionRole = new aws.iam.Role("AWSCloudFormationStackSetExecutionRole", {
    assumeRolePolicy: aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.then(aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy => aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.json),
    name: "AWSCloudFormationStackSetExecutionRole",
});
// Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
// Additional IAM permissions necessary depend on the resources defined in the StackSet template
const aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy = aws.iam.getPolicyDocument({
    statements: [{
        actions: [
            "cloudformation:*",
            "s3:*",
            "sns:*",
        ],
        effect: "Allow",
        resources: ["*"],
    }],
});
const aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy = new aws.iam.RolePolicy("AWSCloudFormationStackSetExecutionRole_MinimumExecutionPolicy", {
    name: "MinimumExecutionPolicy",
    policy: aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.then(aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy => aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.json),
    role: aWSCloudFormationStackSetExecutionRole.name,
});
```
```python
import pulumi
import pulumi_aws as aws

a_ws_cloud_formation_stack_set_execution_role_assume_role_policy = aws.iam.get_policy_document(statements=[{
    "actions": ["sts:AssumeRole"],
    "effect": "Allow",
    "principals": [{
        "identifiers": [a_ws_cloud_formation_stack_set_administration_role["arn"]],
        "type": "AWS",
    }],
}])
a_ws_cloud_formation_stack_set_execution_role = aws.iam.Role("AWSCloudFormationStackSetExecutionRole",
    assume_role_policy=a_ws_cloud_formation_stack_set_execution_role_assume_role_policy.json,
    name="AWSCloudFormationStackSetExecutionRole")
# Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
# Additional IAM permissions necessary depend on the resources defined in the StackSet template
a_ws_cloud_formation_stack_set_execution_role_minimum_execution_policy = aws.iam.get_policy_document(statements=[{
    "actions": [
        "cloudformation:*",
        "s3:*",
        "sns:*",
    ],
    "effect": "Allow",
    "resources": ["*"],
}])
a_ws_cloud_formation_stack_set_execution_role_minimum_execution_policy_role_policy = aws.iam.RolePolicy("AWSCloudFormationStackSetExecutionRole_MinimumExecutionPolicy",
    name="MinimumExecutionPolicy",
    policy=a_ws_cloud_formation_stack_set_execution_role_minimum_execution_policy.json,
    role=a_ws_cloud_formation_stack_set_execution_role.name)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "sts:AssumeRole",
                },
                Effect = "Allow",
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Identifiers = new[]
                        {
                            aWSCloudFormationStackSetAdministrationRole.Arn,
                        },
                        Type = "AWS",
                    },
                },
            },
        },
    });

    var aWSCloudFormationStackSetExecutionRole = new Aws.Iam.Role("AWSCloudFormationStackSetExecutionRole", new()
    {
        AssumeRolePolicy = aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
        Name = "AWSCloudFormationStackSetExecutionRole",
    });

    // Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
    // Additional IAM permissions necessary depend on the resources defined in the StackSet template
    var aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "cloudformation:*",
                    "s3:*",
                    "sns:*",
                },
                Effect = "Allow",
                Resources = new[]
                {
                    "*",
                },
            },
        },
    });

    var aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy = new Aws.Iam.RolePolicy("AWSCloudFormationStackSetExecutionRole_MinimumExecutionPolicy", new()
    {
        Name = "MinimumExecutionPolicy",
        Policy = aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
        Role = aWSCloudFormationStackSetExecutionRole.Name,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
Statements: []iam.GetPolicyDocumentStatement{
{
Actions: []string{
"sts:AssumeRole",
},
Effect: pulumi.StringRef("Allow"),
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Identifiers: interface{}{
aWSCloudFormationStackSetAdministrationRole.Arn,
},
Type: "AWS",
},
},
},
},
}, nil);
if err != nil {
return err
}
aWSCloudFormationStackSetExecutionRole, err := iam.NewRole(ctx, "AWSCloudFormationStackSetExecutionRole", &iam.RoleArgs{
AssumeRolePolicy: pulumi.String(aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.Json),
Name: pulumi.String("AWSCloudFormationStackSetExecutionRole"),
})
if err != nil {
return err
}
// Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
// Additional IAM permissions necessary depend on the resources defined in the StackSet template
aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
Statements: []iam.GetPolicyDocumentStatement{
{
Actions: []string{
"cloudformation:*",
"s3:*",
"sns:*",
},
Effect: pulumi.StringRef("Allow"),
Resources: []string{
"*",
},
},
},
}, nil);
if err != nil {
return err
}
_, err = iam.NewRolePolicy(ctx, "AWSCloudFormationStackSetExecutionRole_MinimumExecutionPolicy", &iam.RolePolicyArgs{
Name: pulumi.String("MinimumExecutionPolicy"),
Policy: pulumi.String(aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.Json),
Role: aWSCloudFormationStackSetExecutionRole.Name,
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
        final var aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions("sts:AssumeRole")
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .identifiers(aWSCloudFormationStackSetAdministrationRole.arn())
                    .type("AWS")
                    .build())
                .build())
            .build());

        var aWSCloudFormationStackSetExecutionRole = new Role("aWSCloudFormationStackSetExecutionRole", RoleArgs.builder()
            .assumeRolePolicy(aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .name("AWSCloudFormationStackSetExecutionRole")
            .build());

        // Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
        // Additional IAM permissions necessary depend on the resources defined in the StackSet template
        final var aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions(                
                    "cloudformation:*",
                    "s3:*",
                    "sns:*")
                .effect("Allow")
                .resources("*")
                .build())
            .build());

        var aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy = new RolePolicy("aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy", RolePolicyArgs.builder()
            .name("MinimumExecutionPolicy")
            .policy(aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .role(aWSCloudFormationStackSetExecutionRole.name())
            .build());

    }
}
```
```yaml
resources:
  aWSCloudFormationStackSetExecutionRole:
    type: aws:iam:Role
    name: AWSCloudFormationStackSetExecutionRole
    properties:
      assumeRolePolicy: ${aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy.json}
      name: AWSCloudFormationStackSetExecutionRole
  aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicyRolePolicy:
    type: aws:iam:RolePolicy
    name: AWSCloudFormationStackSetExecutionRole_MinimumExecutionPolicy
    properties:
      name: MinimumExecutionPolicy
      policy: ${aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy.json}
      role: ${aWSCloudFormationStackSetExecutionRole.name}
variables:
  aWSCloudFormationStackSetExecutionRoleAssumeRolePolicy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - actions:
              - sts:AssumeRole
            effect: Allow
            principals:
              - identifiers:
                  - ${aWSCloudFormationStackSetAdministrationRole.arn}
                type: AWS
  # Documentation: https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/stacksets-prereqs.html
  # Additional IAM permissions necessary depend on the resources defined in the StackSet template
  aWSCloudFormationStackSetExecutionRoleMinimumExecutionPolicy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - actions:
              - cloudformation:*
              - s3:*
              - sns:*
            effect: Allow
            resources:
              - '*'
```
<!--End PulumiCodeChooser -->

### Example Deployment across Organizations account

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudformation.StackSetInstance("example", {
    deploymentTargets: {
        organizationalUnitIds: [exampleAwsOrganizationsOrganization.roots[0].id],
    },
    region: "us-east-1",
    stackSetName: exampleAwsCloudformationStackSet.name,
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudformation.StackSetInstance("example",
    deployment_targets={
        "organizational_unit_ids": [example_aws_organizations_organization["roots"][0]["id"]],
    },
    region="us-east-1",
    stack_set_name=example_aws_cloudformation_stack_set["name"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFormation.StackSetInstance("example", new()
    {
        DeploymentTargets = new Aws.CloudFormation.Inputs.StackSetInstanceDeploymentTargetsArgs
        {
            OrganizationalUnitIds = new[]
            {
                exampleAwsOrganizationsOrganization.Roots[0].Id,
            },
        },
        Region = "us-east-1",
        StackSetName = exampleAwsCloudformationStackSet.Name,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudformation"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudformation.NewStackSetInstance(ctx, "example", &cloudformation.StackSetInstanceArgs{
			DeploymentTargets: &cloudformation.StackSetInstanceDeploymentTargetsArgs{
				OrganizationalUnitIds: pulumi.StringArray{
					exampleAwsOrganizationsOrganization.Roots[0].Id,
				},
			},
			Region:       pulumi.String("us-east-1"),
			StackSetName: pulumi.Any(exampleAwsCloudformationStackSet.Name),
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
import com.pulumi.aws.cloudformation.StackSetInstance;
import com.pulumi.aws.cloudformation.StackSetInstanceArgs;
import com.pulumi.aws.cloudformation.inputs.StackSetInstanceDeploymentTargetsArgs;
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
        var example = new StackSetInstance("example", StackSetInstanceArgs.builder()
            .deploymentTargets(StackSetInstanceDeploymentTargetsArgs.builder()
                .organizationalUnitIds(exampleAwsOrganizationsOrganization.roots()[0].id())
                .build())
            .region("us-east-1")
            .stackSetName(exampleAwsCloudformationStackSet.name())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudformation:StackSetInstance
    properties:
      deploymentTargets:
        organizationalUnitIds:
          - ${exampleAwsOrganizationsOrganization.roots[0].id}
      region: us-east-1
      stackSetName: ${exampleAwsCloudformationStackSet.name}
```
<!--End PulumiCodeChooser -->

## Import

Import CloudFormation StackSet Instances that target AWS Organizational Units using the StackSet name, a slash (`/`) separated list of organizational unit IDs, and target AWS Region separated by commas (`,`). For example:

Import CloudFormation StackSet Instances when acting a delegated administrator in a member account using the StackSet name, target AWS account ID or slash (`/`) separated list of organizational unit IDs, target AWS Region and `call_as` value separated by commas (`,`). For example:

Using `pulumi import`, import CloudFormation StackSet Instances that target an AWS Account ID using the StackSet name, target AWS account ID, and target AWS Region separated by commas (`,`). For example:

```sh
$ pulumi import aws:cloudformation/stackSetInstance:StackSetInstance example example,123456789012,us-east-1
```
Using `pulumi import`, import CloudFormation StackSet Instances that target AWS Organizational Units using the StackSet name, a slash (`/`) separated list of organizational unit IDs, and target AWS Region separated by commas (`,`). For example:

```sh
$ pulumi import aws:cloudformation/stackSetInstance:StackSetInstance example example,ou-sdas-123123123/ou-sdas-789789789,us-east-1
```
Using `pulumi import`, import CloudFormation StackSet Instances when acting a delegated administrator in a member account using the StackSet name, target AWS account ID or slash (`/`) separated list of organizational unit IDs, target AWS Region and `call_as` value separated by commas (`,`). For example:

```sh
$ pulumi import aws:cloudformation/stackSetInstance:StackSetInstance example example,ou-sdas-123123123/ou-sdas-789789789,us-east-1,DELEGATED_ADMIN
```
o
	accountIdB" \Target AWS Account ID to create a Stack based on the StackSet. Defaults to current account.
ﬂ
callAsB" ŒSpecifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
Ï
deploymentTargetsîBë:é
ã
cloudformation!StackSetInstanceDeploymentTargetsVaws:cloudformation/StackSetInstanceDeploymentTargets:StackSetInstanceDeploymentTargetsøAWS Organizations accounts to which StackSets deploys. StackSets doesn't deploy stack instances to the organization management account, even if the organization management account is in your organization or in an OU in your organization. Drift detection is not possible for this argument. See deployment_targets below.
ˇ
operationPreferencesùBö:ó
î
cloudformation$StackSetInstanceOperationPreferences\aws:cloudformation/StackSetInstanceOperationPreferences:StackSetInstanceOperationPreferencesGPreferences for how AWS CloudFormation performs a stack set operation.
q
parameterOverridesB2" SKey-value map of input parameters to override from the StackSet for this Instance.
g
regionB" WTarget AWS Region to create a Stack based on the StackSet. Defaults to current region.
ª
retainStackB
 •During resource destroy, remove Instance from StackSet while keeping the Stack and its associated resources. Must be enabled in the state _before_ destroy operation to take effect. You cannot reassociate a retained Stack or add an existing, saved Stack to a new StackSet. Defaults to `false`.
*
stackSetName" Name of the StackSet.
"m
	accountId" \Target AWS Account ID to create a Stack based on the StackSet. Defaults to current account.
"ﬂ
callAsB" ŒSpecifies whether you are acting as an account administrator in the organization's management account or as a delegated administrator in a member account. Valid values: `SELF` (default), `DELEGATED_ADMIN`.
"Ï
deploymentTargetsîBë:é
ã
cloudformation!StackSetInstanceDeploymentTargetsVaws:cloudformation/StackSetInstanceDeploymentTargets:StackSetInstanceDeploymentTargetsøAWS Organizations accounts to which StackSets deploys. StackSets doesn't deploy stack instances to the organization management account, even if the organization management account is in your organization or in an OU in your organization. Drift detection is not possible for this argument. See deployment_targets below.
"ˇ
operationPreferencesùBö:ó
î
cloudformation$StackSetInstanceOperationPreferences\aws:cloudformation/StackSetInstanceOperationPreferences:StackSetInstanceOperationPreferencesGPreferences for how AWS CloudFormation performs a stack set operation.
"S
organizationalUnitId" 7Organizational unit ID in which the stack is deployed.
"q
parameterOverridesB2" SKey-value map of input parameters to override from the StackSet for this Instance.
"e
region" WTarget AWS Region to create a Stack based on the StackSet. Defaults to current region.
"ª
retainStackB
 •During resource destroy, remove Instance from StackSet while keeping the Stack and its associated resources. Must be enabled in the state _before_ destroy operation to take effect. You cannot reassociate a retained Stack or add an existing, saved Stack to a new StackSet. Defaults to `false`.
"!
stackId" Stack identifier.
"Ë
stackInstanceSummariesù*ö:ó
î
cloudformation$StackSetInstanceStackInstanceSummary\aws:cloudformation/StackSetInstanceStackInstanceSummary:StackSetInstanceStackInstanceSummary≠List of stack instances created from an organizational unit deployment target. This will only be populated when `deployment_targets` is set. See `stack_instance_summaries`.
"*
stackSetName" Name of the StackSet.
*ÔY
A

cloudfrontCachePolicy&aws:cloudfront/cachePolicy:CachePolicy¬G## Example Usage

Use the `aws.cloudfront.CachePolicy` resource to create a cache policy for CloudFront.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudfront.CachePolicy("example", {
    name: "example-policy",
    comment: "test comment",
    defaultTtl: 50,
    maxTtl: 100,
    minTtl: 1,
    parametersInCacheKeyAndForwardedToOrigin: {
        cookiesConfig: {
            cookieBehavior: "whitelist",
            cookies: {
                items: ["example"],
            },
        },
        headersConfig: {
            headerBehavior: "whitelist",
            headers: {
                items: ["example"],
            },
        },
        queryStringsConfig: {
            queryStringBehavior: "whitelist",
            queryStrings: {
                items: ["example"],
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.CachePolicy("example",
    name="example-policy",
    comment="test comment",
    default_ttl=50,
    max_ttl=100,
    min_ttl=1,
    parameters_in_cache_key_and_forwarded_to_origin={
        "cookies_config": {
            "cookie_behavior": "whitelist",
            "cookies": {
                "items": ["example"],
            },
        },
        "headers_config": {
            "header_behavior": "whitelist",
            "headers": {
                "items": ["example"],
            },
        },
        "query_strings_config": {
            "query_string_behavior": "whitelist",
            "query_strings": {
                "items": ["example"],
            },
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
    var example = new Aws.CloudFront.CachePolicy("example", new()
    {
        Name = "example-policy",
        Comment = "test comment",
        DefaultTtl = 50,
        MaxTtl = 100,
        MinTtl = 1,
        ParametersInCacheKeyAndForwardedToOrigin = new Aws.CloudFront.Inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginArgs
        {
            CookiesConfig = new Aws.CloudFront.Inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigArgs
            {
                CookieBehavior = "whitelist",
                Cookies = new Aws.CloudFront.Inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookiesArgs
                {
                    Items = new[]
                    {
                        "example",
                    },
                },
            },
            HeadersConfig = new Aws.CloudFront.Inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigArgs
            {
                HeaderBehavior = "whitelist",
                Headers = new Aws.CloudFront.Inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeadersArgs
                {
                    Items = new[]
                    {
                        "example",
                    },
                },
            },
            QueryStringsConfig = new Aws.CloudFront.Inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigArgs
            {
                QueryStringBehavior = "whitelist",
                QueryStrings = new Aws.CloudFront.Inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStringsArgs
                {
                    Items = new[]
                    {
                        "example",
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
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewCachePolicy(ctx, "example", &cloudfront.CachePolicyArgs{
			Name:       pulumi.String("example-policy"),
			Comment:    pulumi.String("test comment"),
			DefaultTtl: pulumi.Int(50),
			MaxTtl:     pulumi.Int(100),
			MinTtl:     pulumi.Int(1),
			ParametersInCacheKeyAndForwardedToOrigin: &cloudfront.CachePolicyParametersInCacheKeyAndForwardedToOriginArgs{
				CookiesConfig: &cloudfront.CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigArgs{
					CookieBehavior: pulumi.String("whitelist"),
					Cookies: &cloudfront.CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookiesArgs{
						Items: pulumi.StringArray{
							pulumi.String("example"),
						},
					},
				},
				HeadersConfig: &cloudfront.CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigArgs{
					HeaderBehavior: pulumi.String("whitelist"),
					Headers: &cloudfront.CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeadersArgs{
						Items: pulumi.StringArray{
							pulumi.String("example"),
						},
					},
				},
				QueryStringsConfig: &cloudfront.CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigArgs{
					QueryStringBehavior: pulumi.String("whitelist"),
					QueryStrings: &cloudfront.CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStringsArgs{
						Items: pulumi.StringArray{
							pulumi.String("example"),
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
import com.pulumi.aws.cloudfront.CachePolicy;
import com.pulumi.aws.cloudfront.CachePolicyArgs;
import com.pulumi.aws.cloudfront.inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginArgs;
import com.pulumi.aws.cloudfront.inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigArgs;
import com.pulumi.aws.cloudfront.inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookiesArgs;
import com.pulumi.aws.cloudfront.inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigArgs;
import com.pulumi.aws.cloudfront.inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeadersArgs;
import com.pulumi.aws.cloudfront.inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigArgs;
import com.pulumi.aws.cloudfront.inputs.CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStringsArgs;
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
        var example = new CachePolicy("example", CachePolicyArgs.builder()
            .name("example-policy")
            .comment("test comment")
            .defaultTtl(50)
            .maxTtl(100)
            .minTtl(1)
            .parametersInCacheKeyAndForwardedToOrigin(CachePolicyParametersInCacheKeyAndForwardedToOriginArgs.builder()
                .cookiesConfig(CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigArgs.builder()
                    .cookieBehavior("whitelist")
                    .cookies(CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookiesArgs.builder()
                        .items("example")
                        .build())
                    .build())
                .headersConfig(CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigArgs.builder()
                    .headerBehavior("whitelist")
                    .headers(CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeadersArgs.builder()
                        .items("example")
                        .build())
                    .build())
                .queryStringsConfig(CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigArgs.builder()
                    .queryStringBehavior("whitelist")
                    .queryStrings(CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStringsArgs.builder()
                        .items("example")
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:CachePolicy
    properties:
      name: example-policy
      comment: test comment
      defaultTtl: 50
      maxTtl: 100
      minTtl: 1
      parametersInCacheKeyAndForwardedToOrigin:
        cookiesConfig:
          cookieBehavior: whitelist
          cookies:
            items:
              - example
        headersConfig:
          headerBehavior: whitelist
          headers:
            items:
              - example
        queryStringsConfig:
          queryStringBehavior: whitelist
          queryStrings:
            items:
              - example
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CloudFront cache policies using the `id` of the cache policy. For example:

```sh
$ pulumi import aws:cloudfront/cachePolicy:CachePolicy policy 658327ea-f89d-4fab-a63d-7e88639e58f6
```
3
commentB" "Description for the cache policy.
–

defaultTtlB ªAmount of time, in seconds, that objects are allowed to remain in the CloudFront cache before CloudFront sends a new request to the origin server to check if the object has been updated.
º
maxTtlB ´Maximum amount of time, in seconds, that objects stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated.
©
minTtlB òMinimum amount of time, in seconds, that objects should remain in the CloudFront cache before a new request is sent to the origin to check for updates.
=
nameB" /Unique name used to identify the cache policy.
¶
(parametersInCacheKeyAndForwardedToOriginø:º
π

cloudfront3CachePolicyParametersInCacheKeyAndForwardedToOriginvaws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOrigin:CachePolicyParametersInCacheKeyAndForwardedToOrigin∑Configuration for including HTTP headers, cookies, and URL query strings in the cache key. For more information, refer to the Parameters In Cache Key And Forwarded To Origin section.
"3
commentB" "Description for the cache policy.
"–

defaultTtlB ªAmount of time, in seconds, that objects are allowed to remain in the CloudFront cache before CloudFront sends a new request to the origin server to check if the object has been updated.
"1
etag" %Current version of the cache policy.
"º
maxTtlB ´Maximum amount of time, in seconds, that objects stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated.
"©
minTtlB òMinimum amount of time, in seconds, that objects should remain in the CloudFront cache before a new request is sent to the origin to check for updates.
";
name" /Unique name used to identify the cache policy.
"¶
(parametersInCacheKeyAndForwardedToOriginø:º
π

cloudfront3CachePolicyParametersInCacheKeyAndForwardedToOriginvaws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOrigin:CachePolicyParametersInCacheKeyAndForwardedToOrigin∑Configuration for including HTTP headers, cookies, and URL query strings in the cache key. For more information, refer to the Parameters In Cache Key And Forwarded To Origin section.
*Æì
n

cloudfrontContinuousDeploymentPolicyDaws:cloudfront/continuousDeploymentPolicy:ContinuousDeploymentPolicy≈ÜResource for managing an AWS CloudFront Continuous Deployment Policy.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const staging = new aws.cloudfront.Distribution("staging", {
    enabled: true,
    staging: true,
});
const example = new aws.cloudfront.ContinuousDeploymentPolicy("example", {
    enabled: true,
    stagingDistributionDnsNames: {
        items: [staging.domainName],
        quantity: 1,
    },
    trafficConfig: {
        type: "SingleWeight",
        singleWeightConfig: {
            weight: 0.01,
        },
    },
});
const production = new aws.cloudfront.Distribution("production", {
    enabled: true,
    continuousDeploymentPolicyId: example.id,
});
```
```python
import pulumi
import pulumi_aws as aws

staging = aws.cloudfront.Distribution("staging",
    enabled=True,
    staging=True)
example = aws.cloudfront.ContinuousDeploymentPolicy("example",
    enabled=True,
    staging_distribution_dns_names={
        "items": [staging.domain_name],
        "quantity": 1,
    },
    traffic_config={
        "type": "SingleWeight",
        "single_weight_config": {
            "weight": 0.01,
        },
    })
production = aws.cloudfront.Distribution("production",
    enabled=True,
    continuous_deployment_policy_id=example.id)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var staging = new Aws.CloudFront.Distribution("staging", new()
    {
        Enabled = true,
        Staging = true,
    });

    var example = new Aws.CloudFront.ContinuousDeploymentPolicy("example", new()
    {
        Enabled = true,
        StagingDistributionDnsNames = new Aws.CloudFront.Inputs.ContinuousDeploymentPolicyStagingDistributionDnsNamesArgs
        {
            Items = new[]
            {
                staging.DomainName,
            },
            Quantity = 1,
        },
        TrafficConfig = new Aws.CloudFront.Inputs.ContinuousDeploymentPolicyTrafficConfigArgs
        {
            Type = "SingleWeight",
            SingleWeightConfig = new Aws.CloudFront.Inputs.ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigArgs
            {
                Weight = 0.01,
            },
        },
    });

    var production = new Aws.CloudFront.Distribution("production", new()
    {
        Enabled = true,
        ContinuousDeploymentPolicyId = example.Id,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		staging, err := cloudfront.NewDistribution(ctx, "staging", &cloudfront.DistributionArgs{
			Enabled: pulumi.Bool(true),
			Staging: pulumi.Bool(true),
		})
		if err != nil {
			return err
		}
		example, err := cloudfront.NewContinuousDeploymentPolicy(ctx, "example", &cloudfront.ContinuousDeploymentPolicyArgs{
			Enabled: pulumi.Bool(true),
			StagingDistributionDnsNames: &cloudfront.ContinuousDeploymentPolicyStagingDistributionDnsNamesArgs{
				Items: pulumi.StringArray{
					staging.DomainName,
				},
				Quantity: pulumi.Int(1),
			},
			TrafficConfig: &cloudfront.ContinuousDeploymentPolicyTrafficConfigArgs{
				Type: pulumi.String("SingleWeight"),
				SingleWeightConfig: &cloudfront.ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigArgs{
					Weight: pulumi.Float64(0.01),
				},
			},
		})
		if err != nil {
			return err
		}
		_, err = cloudfront.NewDistribution(ctx, "production", &cloudfront.DistributionArgs{
			Enabled:                      pulumi.Bool(true),
			ContinuousDeploymentPolicyId: example.ID(),
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
import com.pulumi.aws.cloudfront.Distribution;
import com.pulumi.aws.cloudfront.DistributionArgs;
import com.pulumi.aws.cloudfront.ContinuousDeploymentPolicy;
import com.pulumi.aws.cloudfront.ContinuousDeploymentPolicyArgs;
import com.pulumi.aws.cloudfront.inputs.ContinuousDeploymentPolicyStagingDistributionDnsNamesArgs;
import com.pulumi.aws.cloudfront.inputs.ContinuousDeploymentPolicyTrafficConfigArgs;
import com.pulumi.aws.cloudfront.inputs.ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigArgs;
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
        var staging = new Distribution("staging", DistributionArgs.builder()
            .enabled(true)
            .staging(true)
            .build());

        var example = new ContinuousDeploymentPolicy("example", ContinuousDeploymentPolicyArgs.builder()
            .enabled(true)
            .stagingDistributionDnsNames(ContinuousDeploymentPolicyStagingDistributionDnsNamesArgs.builder()
                .items(staging.domainName())
                .quantity(1)
                .build())
            .trafficConfig(ContinuousDeploymentPolicyTrafficConfigArgs.builder()
                .type("SingleWeight")
                .singleWeightConfig(ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigArgs.builder()
                    .weight("0.01")
                    .build())
                .build())
            .build());

        var production = new Distribution("production", DistributionArgs.builder()
            .enabled(true)
            .continuousDeploymentPolicyId(example.id())
            .build());

    }
}
```
```yaml
resources:
  staging:
    type: aws:cloudfront:Distribution
    properties:
      enabled: true
      staging: true # ... other configuration ...
  example:
    type: aws:cloudfront:ContinuousDeploymentPolicy
    properties:
      enabled: true
      stagingDistributionDnsNames:
        items:
          - ${staging.domainName}
        quantity: 1
      trafficConfig:
        type: SingleWeight
        singleWeightConfig:
          weight: '0.01'
  production:
    type: aws:cloudfront:Distribution
    properties:
      enabled: true # NOTE: A continuous deployment policy cannot be associated to distribution
      #   # on creation. Set this argument once the resource exists.
      continuousDeploymentPolicyId: ${example.id}
```
<!--End PulumiCodeChooser -->

### Single Weight Config with Session Stickiness

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudfront.ContinuousDeploymentPolicy("example", {
    enabled: true,
    stagingDistributionDnsNames: {
        items: [staging.domainName],
        quantity: 1,
    },
    trafficConfig: {
        type: "SingleWeight",
        singleWeightConfig: {
            weight: 0.01,
            sessionStickinessConfig: {
                idleTtl: 300,
                maximumTtl: 600,
            },
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.ContinuousDeploymentPolicy("example",
    enabled=True,
    staging_distribution_dns_names={
        "items": [staging["domainName"]],
        "quantity": 1,
    },
    traffic_config={
        "type": "SingleWeight",
        "single_weight_config": {
            "weight": 0.01,
            "session_stickiness_config": {
                "idle_ttl": 300,
                "maximum_ttl": 600,
            },
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
    var example = new Aws.CloudFront.ContinuousDeploymentPolicy("example", new()
    {
        Enabled = true,
        StagingDistributionDnsNames = new Aws.CloudFront.Inputs.ContinuousDeploymentPolicyStagingDistributionDnsNamesArgs
        {
            Items = new[]
            {
                staging.DomainName,
            },
            Quantity = 1,
        },
        TrafficConfig = new Aws.CloudFront.Inputs.ContinuousDeploymentPolicyTrafficConfigArgs
        {
            Type = "SingleWeight",
            SingleWeightConfig = new Aws.CloudFront.Inputs.ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigArgs
            {
                Weight = 0.01,
                SessionStickinessConfig = new Aws.CloudFront.Inputs.ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfigArgs
                {
                    IdleTtl = 300,
                    MaximumTtl = 600,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewContinuousDeploymentPolicy(ctx, "example", &cloudfront.ContinuousDeploymentPolicyArgs{
			Enabled: pulumi.Bool(true),
			StagingDistributionDnsNames: &cloudfront.ContinuousDeploymentPolicyStagingDistributionDnsNamesArgs{
				Items: pulumi.StringArray{
					staging.DomainName,
				},
				Quantity: pulumi.Int(1),
			},
			TrafficConfig: &cloudfront.ContinuousDeploymentPolicyTrafficConfigArgs{
				Type: pulumi.String("SingleWeight"),
				SingleWeightConfig: &cloudfront.ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigArgs{
					Weight: pulumi.Float64(0.01),
					SessionStickinessConfig: &cloudfront.ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfigArgs{
						IdleTtl:    pulumi.Int(300),
						MaximumTtl: pulumi.Int(600),
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
import com.pulumi.aws.cloudfront.ContinuousDeploymentPolicy;
import com.pulumi.aws.cloudfront.ContinuousDeploymentPolicyArgs;
import com.pulumi.aws.cloudfront.inputs.ContinuousDeploymentPolicyStagingDistributionDnsNamesArgs;
import com.pulumi.aws.cloudfront.inputs.ContinuousDeploymentPolicyTrafficConfigArgs;
import com.pulumi.aws.cloudfront.inputs.ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigArgs;
import com.pulumi.aws.cloudfront.inputs.ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfigArgs;
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
        var example = new ContinuousDeploymentPolicy("example", ContinuousDeploymentPolicyArgs.builder()
            .enabled(true)
            .stagingDistributionDnsNames(ContinuousDeploymentPolicyStagingDistributionDnsNamesArgs.builder()
                .items(staging.domainName())
                .quantity(1)
                .build())
            .trafficConfig(ContinuousDeploymentPolicyTrafficConfigArgs.builder()
                .type("SingleWeight")
                .singleWeightConfig(ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigArgs.builder()
                    .weight("0.01")
                    .sessionStickinessConfig(ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfigArgs.builder()
                        .idleTtl(300)
                        .maximumTtl(600)
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:ContinuousDeploymentPolicy
    properties:
      enabled: true
      stagingDistributionDnsNames:
        items:
          - ${staging.domainName}
        quantity: 1
      trafficConfig:
        type: SingleWeight
        singleWeightConfig:
          weight: '0.01'
          sessionStickinessConfig:
            idleTtl: 300
            maximumTtl: 600
```
<!--End PulumiCodeChooser -->

### Single Header Config

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudfront.ContinuousDeploymentPolicy("example", {
    enabled: true,
    stagingDistributionDnsNames: {
        items: [staging.domainName],
        quantity: 1,
    },
    trafficConfig: {
        type: "SingleHeader",
        singleHeaderConfig: {
            header: "aws-cf-cd-example",
            value: "example",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.ContinuousDeploymentPolicy("example",
    enabled=True,
    staging_distribution_dns_names={
        "items": [staging["domainName"]],
        "quantity": 1,
    },
    traffic_config={
        "type": "SingleHeader",
        "single_header_config": {
            "header": "aws-cf-cd-example",
            "value": "example",
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
    var example = new Aws.CloudFront.ContinuousDeploymentPolicy("example", new()
    {
        Enabled = true,
        StagingDistributionDnsNames = new Aws.CloudFront.Inputs.ContinuousDeploymentPolicyStagingDistributionDnsNamesArgs
        {
            Items = new[]
            {
                staging.DomainName,
            },
            Quantity = 1,
        },
        TrafficConfig = new Aws.CloudFront.Inputs.ContinuousDeploymentPolicyTrafficConfigArgs
        {
            Type = "SingleHeader",
            SingleHeaderConfig = new Aws.CloudFront.Inputs.ContinuousDeploymentPolicyTrafficConfigSingleHeaderConfigArgs
            {
                Header = "aws-cf-cd-example",
                Value = "example",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewContinuousDeploymentPolicy(ctx, "example", &cloudfront.ContinuousDeploymentPolicyArgs{
			Enabled: pulumi.Bool(true),
			StagingDistributionDnsNames: &cloudfront.ContinuousDeploymentPolicyStagingDistributionDnsNamesArgs{
				Items: pulumi.StringArray{
					staging.DomainName,
				},
				Quantity: pulumi.Int(1),
			},
			TrafficConfig: &cloudfront.ContinuousDeploymentPolicyTrafficConfigArgs{
				Type: pulumi.String("SingleHeader"),
				SingleHeaderConfig: &cloudfront.ContinuousDeploymentPolicyTrafficConfigSingleHeaderConfigArgs{
					Header: pulumi.String("aws-cf-cd-example"),
					Value:  pulumi.String("example"),
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
import com.pulumi.aws.cloudfront.ContinuousDeploymentPolicy;
import com.pulumi.aws.cloudfront.ContinuousDeploymentPolicyArgs;
import com.pulumi.aws.cloudfront.inputs.ContinuousDeploymentPolicyStagingDistributionDnsNamesArgs;
import com.pulumi.aws.cloudfront.inputs.ContinuousDeploymentPolicyTrafficConfigArgs;
import com.pulumi.aws.cloudfront.inputs.ContinuousDeploymentPolicyTrafficConfigSingleHeaderConfigArgs;
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
        var example = new ContinuousDeploymentPolicy("example", ContinuousDeploymentPolicyArgs.builder()
            .enabled(true)
            .stagingDistributionDnsNames(ContinuousDeploymentPolicyStagingDistributionDnsNamesArgs.builder()
                .items(staging.domainName())
                .quantity(1)
                .build())
            .trafficConfig(ContinuousDeploymentPolicyTrafficConfigArgs.builder()
                .type("SingleHeader")
                .singleHeaderConfig(ContinuousDeploymentPolicyTrafficConfigSingleHeaderConfigArgs.builder()
                    .header("aws-cf-cd-example")
                    .value("example")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:ContinuousDeploymentPolicy
    properties:
      enabled: true
      stagingDistributionDnsNames:
        items:
          - ${staging.domainName}
        quantity: 1
      trafficConfig:
        type: SingleHeader
        singleHeaderConfig:
          header: aws-cf-cd-example
          value: example
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CloudFront Continuous Deployment Policy using the `id`. For example:

```sh
$ pulumi import aws:cloudfront/continuousDeploymentPolicy:ContinuousDeploymentPolicy example abcd-1234
```
E
enabled
 6Whether this continuous deployment policy is enabled.
ƒ
stagingDistributionDnsNames»B≈:¬
ø

cloudfront5ContinuousDeploymentPolicyStagingDistributionDnsNameszaws:cloudfront/ContinuousDeploymentPolicyStagingDistributionDnsNames:ContinuousDeploymentPolicyStagingDistributionDnsNamesZCloudFront domain name of the staging distribution. See `staging_distribution_dns_names`.
ô
trafficConfigûBõ:ò
ï

cloudfront'ContinuousDeploymentPolicyTrafficConfig^aws:cloudfront/ContinuousDeploymentPolicyTrafficConfig:ContinuousDeploymentPolicyTrafficConfiggParameters for routing production traffic from primary to staging distributions. See `traffic_config`.
"E
enabled
 6Whether this continuous deployment policy is enabled.
"C
etag" 7Current version of the continuous distribution policy.
"Z
lastModifiedTime" BDate and time the continuous deployment policy was last modified.
"ƒ
stagingDistributionDnsNames»B≈:¬
ø

cloudfront5ContinuousDeploymentPolicyStagingDistributionDnsNameszaws:cloudfront/ContinuousDeploymentPolicyStagingDistributionDnsNames:ContinuousDeploymentPolicyStagingDistributionDnsNamesZCloudFront domain name of the staging distribution. See `staging_distribution_dns_names`.
"ô
trafficConfigûBõ:ò
ï

cloudfront'ContinuousDeploymentPolicyTrafficConfig^aws:cloudfront/ContinuousDeploymentPolicyTrafficConfig:ContinuousDeploymentPolicyTrafficConfiggParameters for routing production traffic from primary to staging distributions. See `traffic_config`.
*Ãï
D

cloudfrontDistribution(aws:cloudfront/distribution:Distribution‡ÔCreates an Amazon CloudFront web distribution.

For information about CloudFront distributions, see the [Amazon CloudFront Developer Guide](http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Introduction.html). For specific information about creating CloudFront web distributions, see the [POST Distribution](https://docs.aws.amazon.com/cloudfront/latest/APIReference/API_CreateDistribution.html) page in the Amazon CloudFront API Reference.

> **NOTE:** CloudFront distributions take about 15 minutes to reach a deployed state after creation or modification. During this time, deletes to resources will be blocked. If you need to delete a distribution that is enabled and you do not want to wait, you need to use the `retain_on_delete` flag.

## Example Usage

### S3 Origin

The example below creates a CloudFront distribution with an S3 origin.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const b = new aws.s3.BucketV2("b", {
    bucket: "mybucket",
    tags: {
        Name: "My bucket",
    },
});
const bAcl = new aws.s3.BucketAclV2("b_acl", {
    bucket: b.id,
    acl: "private",
});
const s3OriginId = "myS3Origin";
const s3Distribution = new aws.cloudfront.Distribution("s3_distribution", {
    origins: [{
        domainName: b.bucketRegionalDomainName,
        originAccessControlId: _default.id,
        originId: s3OriginId,
    }],
    enabled: true,
    isIpv6Enabled: true,
    comment: "Some comment",
    defaultRootObject: "index.html",
    loggingConfig: {
        includeCookies: false,
        bucket: "mylogs.s3.amazonaws.com",
        prefix: "myprefix",
    },
    aliases: [
        "mysite.example.com",
        "yoursite.example.com",
    ],
    defaultCacheBehavior: {
        allowedMethods: [
            "DELETE",
            "GET",
            "HEAD",
            "OPTIONS",
            "PATCH",
            "POST",
            "PUT",
        ],
        cachedMethods: [
            "GET",
            "HEAD",
        ],
        targetOriginId: s3OriginId,
        forwardedValues: {
            queryString: false,
            cookies: {
                forward: "none",
            },
        },
        viewerProtocolPolicy: "allow-all",
        minTtl: 0,
        defaultTtl: 3600,
        maxTtl: 86400,
    },
    orderedCacheBehaviors: [
        {
            pathPattern: "/content/immutable/*",
            allowedMethods: [
                "GET",
                "HEAD",
                "OPTIONS",
            ],
            cachedMethods: [
                "GET",
                "HEAD",
                "OPTIONS",
            ],
            targetOriginId: s3OriginId,
            forwardedValues: {
                queryString: false,
                headers: ["Origin"],
                cookies: {
                    forward: "none",
                },
            },
            minTtl: 0,
            defaultTtl: 86400,
            maxTtl: 31536000,
            compress: true,
            viewerProtocolPolicy: "redirect-to-https",
        },
        {
            pathPattern: "/content/*",
            allowedMethods: [
                "GET",
                "HEAD",
                "OPTIONS",
            ],
            cachedMethods: [
                "GET",
                "HEAD",
            ],
            targetOriginId: s3OriginId,
            forwardedValues: {
                queryString: false,
                cookies: {
                    forward: "none",
                },
            },
            minTtl: 0,
            defaultTtl: 3600,
            maxTtl: 86400,
            compress: true,
            viewerProtocolPolicy: "redirect-to-https",
        },
    ],
    priceClass: "PriceClass_200",
    restrictions: {
        geoRestriction: {
            restrictionType: "whitelist",
            locations: [
                "US",
                "CA",
                "GB",
                "DE",
            ],
        },
    },
    tags: {
        Environment: "production",
    },
    viewerCertificate: {
        cloudfrontDefaultCertificate: true,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

b = aws.s3.BucketV2("b",
    bucket="mybucket",
    tags={
        "Name": "My bucket",
    })
b_acl = aws.s3.BucketAclV2("b_acl",
    bucket=b.id,
    acl="private")
s3_origin_id = "myS3Origin"
s3_distribution = aws.cloudfront.Distribution("s3_distribution",
    origins=[{
        "domain_name": b.bucket_regional_domain_name,
        "origin_access_control_id": default["id"],
        "origin_id": s3_origin_id,
    }],
    enabled=True,
    is_ipv6_enabled=True,
    comment="Some comment",
    default_root_object="index.html",
    logging_config={
        "include_cookies": False,
        "bucket": "mylogs.s3.amazonaws.com",
        "prefix": "myprefix",
    },
    aliases=[
        "mysite.example.com",
        "yoursite.example.com",
    ],
    default_cache_behavior={
        "allowed_methods": [
            "DELETE",
            "GET",
            "HEAD",
            "OPTIONS",
            "PATCH",
            "POST",
            "PUT",
        ],
        "cached_methods": [
            "GET",
            "HEAD",
        ],
        "target_origin_id": s3_origin_id,
        "forwarded_values": {
            "query_string": False,
            "cookies": {
                "forward": "none",
            },
        },
        "viewer_protocol_policy": "allow-all",
        "min_ttl": 0,
        "default_ttl": 3600,
        "max_ttl": 86400,
    },
    ordered_cache_behaviors=[
        {
            "path_pattern": "/content/immutable/*",
            "allowed_methods": [
                "GET",
                "HEAD",
                "OPTIONS",
            ],
            "cached_methods": [
                "GET",
                "HEAD",
                "OPTIONS",
            ],
            "target_origin_id": s3_origin_id,
            "forwarded_values": {
                "query_string": False,
                "headers": ["Origin"],
                "cookies": {
                    "forward": "none",
                },
            },
            "min_ttl": 0,
            "default_ttl": 86400,
            "max_ttl": 31536000,
            "compress": True,
            "viewer_protocol_policy": "redirect-to-https",
        },
        {
            "path_pattern": "/content/*",
            "allowed_methods": [
                "GET",
                "HEAD",
                "OPTIONS",
            ],
            "cached_methods": [
                "GET",
                "HEAD",
            ],
            "target_origin_id": s3_origin_id,
            "forwarded_values": {
                "query_string": False,
                "cookies": {
                    "forward": "none",
                },
            },
            "min_ttl": 0,
            "default_ttl": 3600,
            "max_ttl": 86400,
            "compress": True,
            "viewer_protocol_policy": "redirect-to-https",
        },
    ],
    price_class="PriceClass_200",
    restrictions={
        "geo_restriction": {
            "restriction_type": "whitelist",
            "locations": [
                "US",
                "CA",
                "GB",
                "DE",
            ],
        },
    },
    tags={
        "Environment": "production",
    },
    viewer_certificate={
        "cloudfront_default_certificate": True,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var b = new Aws.S3.BucketV2("b", new()
    {
        Bucket = "mybucket",
        Tags = 
        {
            { "Name", "My bucket" },
        },
    });

    var bAcl = new Aws.S3.BucketAclV2("b_acl", new()
    {
        Bucket = b.Id,
        Acl = "private",
    });

    var s3OriginId = "myS3Origin";

    var s3Distribution = new Aws.CloudFront.Distribution("s3_distribution", new()
    {
        Origins = new[]
        {
            new Aws.CloudFront.Inputs.DistributionOriginArgs
            {
                DomainName = b.BucketRegionalDomainName,
                OriginAccessControlId = @default.Id,
                OriginId = s3OriginId,
            },
        },
        Enabled = true,
        IsIpv6Enabled = true,
        Comment = "Some comment",
        DefaultRootObject = "index.html",
        LoggingConfig = new Aws.CloudFront.Inputs.DistributionLoggingConfigArgs
        {
            IncludeCookies = false,
            Bucket = "mylogs.s3.amazonaws.com",
            Prefix = "myprefix",
        },
        Aliases = new[]
        {
            "mysite.example.com",
            "yoursite.example.com",
        },
        DefaultCacheBehavior = new Aws.CloudFront.Inputs.DistributionDefaultCacheBehaviorArgs
        {
            AllowedMethods = new[]
            {
                "DELETE",
                "GET",
                "HEAD",
                "OPTIONS",
                "PATCH",
                "POST",
                "PUT",
            },
            CachedMethods = new[]
            {
                "GET",
                "HEAD",
            },
            TargetOriginId = s3OriginId,
            ForwardedValues = new Aws.CloudFront.Inputs.DistributionDefaultCacheBehaviorForwardedValuesArgs
            {
                QueryString = false,
                Cookies = new Aws.CloudFront.Inputs.DistributionDefaultCacheBehaviorForwardedValuesCookiesArgs
                {
                    Forward = "none",
                },
            },
            ViewerProtocolPolicy = "allow-all",
            MinTtl = 0,
            DefaultTtl = 3600,
            MaxTtl = 86400,
        },
        OrderedCacheBehaviors = new[]
        {
            new Aws.CloudFront.Inputs.DistributionOrderedCacheBehaviorArgs
            {
                PathPattern = "/content/immutable/*",
                AllowedMethods = new[]
                {
                    "GET",
                    "HEAD",
                    "OPTIONS",
                },
                CachedMethods = new[]
                {
                    "GET",
                    "HEAD",
                    "OPTIONS",
                },
                TargetOriginId = s3OriginId,
                ForwardedValues = new Aws.CloudFront.Inputs.DistributionOrderedCacheBehaviorForwardedValuesArgs
                {
                    QueryString = false,
                    Headers = new[]
                    {
                        "Origin",
                    },
                    Cookies = new Aws.CloudFront.Inputs.DistributionOrderedCacheBehaviorForwardedValuesCookiesArgs
                    {
                        Forward = "none",
                    },
                },
                MinTtl = 0,
                DefaultTtl = 86400,
                MaxTtl = 31536000,
                Compress = true,
                ViewerProtocolPolicy = "redirect-to-https",
            },
            new Aws.CloudFront.Inputs.DistributionOrderedCacheBehaviorArgs
            {
                PathPattern = "/content/*",
                AllowedMethods = new[]
                {
                    "GET",
                    "HEAD",
                    "OPTIONS",
                },
                CachedMethods = new[]
                {
                    "GET",
                    "HEAD",
                },
                TargetOriginId = s3OriginId,
                ForwardedValues = new Aws.CloudFront.Inputs.DistributionOrderedCacheBehaviorForwardedValuesArgs
                {
                    QueryString = false,
                    Cookies = new Aws.CloudFront.Inputs.DistributionOrderedCacheBehaviorForwardedValuesCookiesArgs
                    {
                        Forward = "none",
                    },
                },
                MinTtl = 0,
                DefaultTtl = 3600,
                MaxTtl = 86400,
                Compress = true,
                ViewerProtocolPolicy = "redirect-to-https",
            },
        },
        PriceClass = "PriceClass_200",
        Restrictions = new Aws.CloudFront.Inputs.DistributionRestrictionsArgs
        {
            GeoRestriction = new Aws.CloudFront.Inputs.DistributionRestrictionsGeoRestrictionArgs
            {
                RestrictionType = "whitelist",
                Locations = new[]
                {
                    "US",
                    "CA",
                    "GB",
                    "DE",
                },
            },
        },
        Tags = 
        {
            { "Environment", "production" },
        },
        ViewerCertificate = new Aws.CloudFront.Inputs.DistributionViewerCertificateArgs
        {
            CloudfrontDefaultCertificate = true,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		b, err := s3.NewBucketV2(ctx, "b", &s3.BucketV2Args{
			Bucket: pulumi.String("mybucket"),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("My bucket"),
			},
		})
		if err != nil {
			return err
		}
		_, err = s3.NewBucketAclV2(ctx, "b_acl", &s3.BucketAclV2Args{
			Bucket: b.ID(),
			Acl:    pulumi.String("private"),
		})
		if err != nil {
			return err
		}
		s3OriginId := "myS3Origin"
		_, err = cloudfront.NewDistribution(ctx, "s3_distribution", &cloudfront.DistributionArgs{
			Origins: cloudfront.DistributionOriginArray{
				&cloudfront.DistributionOriginArgs{
					DomainName:            b.BucketRegionalDomainName,
					OriginAccessControlId: pulumi.Any(_default.Id),
					OriginId:              pulumi.String(s3OriginId),
				},
			},
			Enabled:           pulumi.Bool(true),
			IsIpv6Enabled:     pulumi.Bool(true),
			Comment:           pulumi.String("Some comment"),
			DefaultRootObject: pulumi.String("index.html"),
			LoggingConfig: &cloudfront.DistributionLoggingConfigArgs{
				IncludeCookies: pulumi.Bool(false),
				Bucket:         pulumi.String("mylogs.s3.amazonaws.com"),
				Prefix:         pulumi.String("myprefix"),
			},
			Aliases: pulumi.StringArray{
				pulumi.String("mysite.example.com"),
				pulumi.String("yoursite.example.com"),
			},
			DefaultCacheBehavior: &cloudfront.DistributionDefaultCacheBehaviorArgs{
				AllowedMethods: pulumi.StringArray{
					pulumi.String("DELETE"),
					pulumi.String("GET"),
					pulumi.String("HEAD"),
					pulumi.String("OPTIONS"),
					pulumi.String("PATCH"),
					pulumi.String("POST"),
					pulumi.String("PUT"),
				},
				CachedMethods: pulumi.StringArray{
					pulumi.String("GET"),
					pulumi.String("HEAD"),
				},
				TargetOriginId: pulumi.String(s3OriginId),
				ForwardedValues: &cloudfront.DistributionDefaultCacheBehaviorForwardedValuesArgs{
					QueryString: pulumi.Bool(false),
					Cookies: &cloudfront.DistributionDefaultCacheBehaviorForwardedValuesCookiesArgs{
						Forward: pulumi.String("none"),
					},
				},
				ViewerProtocolPolicy: pulumi.String("allow-all"),
				MinTtl:               pulumi.Int(0),
				DefaultTtl:           pulumi.Int(3600),
				MaxTtl:               pulumi.Int(86400),
			},
			OrderedCacheBehaviors: cloudfront.DistributionOrderedCacheBehaviorArray{
				&cloudfront.DistributionOrderedCacheBehaviorArgs{
					PathPattern: pulumi.String("/content/immutable/*"),
					AllowedMethods: pulumi.StringArray{
						pulumi.String("GET"),
						pulumi.String("HEAD"),
						pulumi.String("OPTIONS"),
					},
					CachedMethods: pulumi.StringArray{
						pulumi.String("GET"),
						pulumi.String("HEAD"),
						pulumi.String("OPTIONS"),
					},
					TargetOriginId: pulumi.String(s3OriginId),
					ForwardedValues: &cloudfront.DistributionOrderedCacheBehaviorForwardedValuesArgs{
						QueryString: pulumi.Bool(false),
						Headers: pulumi.StringArray{
							pulumi.String("Origin"),
						},
						Cookies: &cloudfront.DistributionOrderedCacheBehaviorForwardedValuesCookiesArgs{
							Forward: pulumi.String("none"),
						},
					},
					MinTtl:               pulumi.Int(0),
					DefaultTtl:           pulumi.Int(86400),
					MaxTtl:               pulumi.Int(31536000),
					Compress:             pulumi.Bool(true),
					ViewerProtocolPolicy: pulumi.String("redirect-to-https"),
				},
				&cloudfront.DistributionOrderedCacheBehaviorArgs{
					PathPattern: pulumi.String("/content/*"),
					AllowedMethods: pulumi.StringArray{
						pulumi.String("GET"),
						pulumi.String("HEAD"),
						pulumi.String("OPTIONS"),
					},
					CachedMethods: pulumi.StringArray{
						pulumi.String("GET"),
						pulumi.String("HEAD"),
					},
					TargetOriginId: pulumi.String(s3OriginId),
					ForwardedValues: &cloudfront.DistributionOrderedCacheBehaviorForwardedValuesArgs{
						QueryString: pulumi.Bool(false),
						Cookies: &cloudfront.DistributionOrderedCacheBehaviorForwardedValuesCookiesArgs{
							Forward: pulumi.String("none"),
						},
					},
					MinTtl:               pulumi.Int(0),
					DefaultTtl:           pulumi.Int(3600),
					MaxTtl:               pulumi.Int(86400),
					Compress:             pulumi.Bool(true),
					ViewerProtocolPolicy: pulumi.String("redirect-to-https"),
				},
			},
			PriceClass: pulumi.String("PriceClass_200"),
			Restrictions: &cloudfront.DistributionRestrictionsArgs{
				GeoRestriction: &cloudfront.DistributionRestrictionsGeoRestrictionArgs{
					RestrictionType: pulumi.String("whitelist"),
					Locations: pulumi.StringArray{
						pulumi.String("US"),
						pulumi.String("CA"),
						pulumi.String("GB"),
						pulumi.String("DE"),
					},
				},
			},
			Tags: pulumi.StringMap{
				"Environment": pulumi.String("production"),
			},
			ViewerCertificate: &cloudfront.DistributionViewerCertificateArgs{
				CloudfrontDefaultCertificate: pulumi.Bool(true),
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
import com.pulumi.aws.s3.BucketAclV2;
import com.pulumi.aws.s3.BucketAclV2Args;
import com.pulumi.aws.cloudfront.Distribution;
import com.pulumi.aws.cloudfront.DistributionArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionOriginArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionLoggingConfigArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionDefaultCacheBehaviorArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionDefaultCacheBehaviorForwardedValuesArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionDefaultCacheBehaviorForwardedValuesCookiesArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionOrderedCacheBehaviorArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionOrderedCacheBehaviorForwardedValuesArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionOrderedCacheBehaviorForwardedValuesCookiesArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionRestrictionsArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionRestrictionsGeoRestrictionArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionViewerCertificateArgs;
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
        var b = new BucketV2("b", BucketV2Args.builder()
            .bucket("mybucket")
            .tags(Map.of("Name", "My bucket"))
            .build());

        var bAcl = new BucketAclV2("bAcl", BucketAclV2Args.builder()
            .bucket(b.id())
            .acl("private")
            .build());

        final var s3OriginId = "myS3Origin";

        var s3Distribution = new Distribution("s3Distribution", DistributionArgs.builder()
            .origins(DistributionOriginArgs.builder()
                .domainName(b.bucketRegionalDomainName())
                .originAccessControlId(default_.id())
                .originId(s3OriginId)
                .build())
            .enabled(true)
            .isIpv6Enabled(true)
            .comment("Some comment")
            .defaultRootObject("index.html")
            .loggingConfig(DistributionLoggingConfigArgs.builder()
                .includeCookies(false)
                .bucket("mylogs.s3.amazonaws.com")
                .prefix("myprefix")
                .build())
            .aliases(            
                "mysite.example.com",
                "yoursite.example.com")
            .defaultCacheBehavior(DistributionDefaultCacheBehaviorArgs.builder()
                .allowedMethods(                
                    "DELETE",
                    "GET",
                    "HEAD",
                    "OPTIONS",
                    "PATCH",
                    "POST",
                    "PUT")
                .cachedMethods(                
                    "GET",
                    "HEAD")
                .targetOriginId(s3OriginId)
                .forwardedValues(DistributionDefaultCacheBehaviorForwardedValuesArgs.builder()
                    .queryString(false)
                    .cookies(DistributionDefaultCacheBehaviorForwardedValuesCookiesArgs.builder()
                        .forward("none")
                        .build())
                    .build())
                .viewerProtocolPolicy("allow-all")
                .minTtl(0)
                .defaultTtl(3600)
                .maxTtl(86400)
                .build())
            .orderedCacheBehaviors(            
                DistributionOrderedCacheBehaviorArgs.builder()
                    .pathPattern("/content/immutable/*")
                    .allowedMethods(                    
                        "GET",
                        "HEAD",
                        "OPTIONS")
                    .cachedMethods(                    
                        "GET",
                        "HEAD",
                        "OPTIONS")
                    .targetOriginId(s3OriginId)
                    .forwardedValues(DistributionOrderedCacheBehaviorForwardedValuesArgs.builder()
                        .queryString(false)
                        .headers("Origin")
                        .cookies(DistributionOrderedCacheBehaviorForwardedValuesCookiesArgs.builder()
                            .forward("none")
                            .build())
                        .build())
                    .minTtl(0)
                    .defaultTtl(86400)
                    .maxTtl(31536000)
                    .compress(true)
                    .viewerProtocolPolicy("redirect-to-https")
                    .build(),
                DistributionOrderedCacheBehaviorArgs.builder()
                    .pathPattern("/content/*")
                    .allowedMethods(                    
                        "GET",
                        "HEAD",
                        "OPTIONS")
                    .cachedMethods(                    
                        "GET",
                        "HEAD")
                    .targetOriginId(s3OriginId)
                    .forwardedValues(DistributionOrderedCacheBehaviorForwardedValuesArgs.builder()
                        .queryString(false)
                        .cookies(DistributionOrderedCacheBehaviorForwardedValuesCookiesArgs.builder()
                            .forward("none")
                            .build())
                        .build())
                    .minTtl(0)
                    .defaultTtl(3600)
                    .maxTtl(86400)
                    .compress(true)
                    .viewerProtocolPolicy("redirect-to-https")
                    .build())
            .priceClass("PriceClass_200")
            .restrictions(DistributionRestrictionsArgs.builder()
                .geoRestriction(DistributionRestrictionsGeoRestrictionArgs.builder()
                    .restrictionType("whitelist")
                    .locations(                    
                        "US",
                        "CA",
                        "GB",
                        "DE")
                    .build())
                .build())
            .tags(Map.of("Environment", "production"))
            .viewerCertificate(DistributionViewerCertificateArgs.builder()
                .cloudfrontDefaultCertificate(true)
                .build())
            .build());

    }
}
```
```yaml
resources:
  b:
    type: aws:s3:BucketV2
    properties:
      bucket: mybucket
      tags:
        Name: My bucket
  bAcl:
    type: aws:s3:BucketAclV2
    name: b_acl
    properties:
      bucket: ${b.id}
      acl: private
  s3Distribution:
    type: aws:cloudfront:Distribution
    name: s3_distribution
    properties:
      origins:
        - domainName: ${b.bucketRegionalDomainName}
          originAccessControlId: ${default.id}
          originId: ${s3OriginId}
      enabled: true
      isIpv6Enabled: true
      comment: Some comment
      defaultRootObject: index.html
      loggingConfig:
        includeCookies: false
        bucket: mylogs.s3.amazonaws.com
        prefix: myprefix
      aliases:
        - mysite.example.com
        - yoursite.example.com
      defaultCacheBehavior:
        allowedMethods:
          - DELETE
          - GET
          - HEAD
          - OPTIONS
          - PATCH
          - POST
          - PUT
        cachedMethods:
          - GET
          - HEAD
        targetOriginId: ${s3OriginId}
        forwardedValues:
          queryString: false
          cookies:
            forward: none
        viewerProtocolPolicy: allow-all
        minTtl: 0
        defaultTtl: 3600
        maxTtl: 86400
      orderedCacheBehaviors:
        - pathPattern: /content/immutable/*
          allowedMethods:
            - GET
            - HEAD
            - OPTIONS
          cachedMethods:
            - GET
            - HEAD
            - OPTIONS
          targetOriginId: ${s3OriginId}
          forwardedValues:
            queryString: false
            headers:
              - Origin
            cookies:
              forward: none
          minTtl: 0
          defaultTtl: 86400
          maxTtl: 3.1536e+07
          compress: true
          viewerProtocolPolicy: redirect-to-https
        - pathPattern: /content/*
          allowedMethods:
            - GET
            - HEAD
            - OPTIONS
          cachedMethods:
            - GET
            - HEAD
          targetOriginId: ${s3OriginId}
          forwardedValues:
            queryString: false
            cookies:
              forward: none
          minTtl: 0
          defaultTtl: 3600
          maxTtl: 86400
          compress: true
          viewerProtocolPolicy: redirect-to-https
      priceClass: PriceClass_200
      restrictions:
        geoRestriction:
          restrictionType: whitelist
          locations:
            - US
            - CA
            - GB
            - DE
      tags:
        Environment: production
      viewerCertificate:
        cloudfrontDefaultCertificate: true
variables:
  s3OriginId: myS3Origin
```
<!--End PulumiCodeChooser -->

### With Failover Routing

The example below creates a CloudFront distribution with an origin group for failover routing.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const s3Distribution = new aws.cloudfront.Distribution("s3_distribution", {
    originGroups: [{
        originId: "groupS3",
        failoverCriteria: {
            statusCodes: [
                403,
                404,
                500,
                502,
            ],
        },
        members: [
            {
                originId: "primaryS3",
            },
            {
                originId: "failoverS3",
            },
        ],
    }],
    origins: [
        {
            domainName: primary.bucketRegionalDomainName,
            originId: "primaryS3",
            s3OriginConfig: {
                originAccessIdentity: _default.cloudfrontAccessIdentityPath,
            },
        },
        {
            domainName: failover.bucketRegionalDomainName,
            originId: "failoverS3",
            s3OriginConfig: {
                originAccessIdentity: _default.cloudfrontAccessIdentityPath,
            },
        },
    ],
    defaultCacheBehavior: {
        targetOriginId: "groupS3",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

s3_distribution = aws.cloudfront.Distribution("s3_distribution",
    origin_groups=[{
        "origin_id": "groupS3",
        "failover_criteria": {
            "status_codes": [
                403,
                404,
                500,
                502,
            ],
        },
        "members": [
            {
                "origin_id": "primaryS3",
            },
            {
                "origin_id": "failoverS3",
            },
        ],
    }],
    origins=[
        {
            "domain_name": primary["bucketRegionalDomainName"],
            "origin_id": "primaryS3",
            "s3_origin_config": {
                "origin_access_identity": default["cloudfrontAccessIdentityPath"],
            },
        },
        {
            "domain_name": failover["bucketRegionalDomainName"],
            "origin_id": "failoverS3",
            "s3_origin_config": {
                "origin_access_identity": default["cloudfrontAccessIdentityPath"],
            },
        },
    ],
    default_cache_behavior={
        "target_origin_id": "groupS3",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var s3Distribution = new Aws.CloudFront.Distribution("s3_distribution", new()
    {
        OriginGroups = new[]
        {
            new Aws.CloudFront.Inputs.DistributionOriginGroupArgs
            {
                OriginId = "groupS3",
                FailoverCriteria = new Aws.CloudFront.Inputs.DistributionOriginGroupFailoverCriteriaArgs
                {
                    StatusCodes = new[]
                    {
                        403,
                        404,
                        500,
                        502,
                    },
                },
                Members = new[]
                {
                    new Aws.CloudFront.Inputs.DistributionOriginGroupMemberArgs
                    {
                        OriginId = "primaryS3",
                    },
                    new Aws.CloudFront.Inputs.DistributionOriginGroupMemberArgs
                    {
                        OriginId = "failoverS3",
                    },
                },
            },
        },
        Origins = new[]
        {
            new Aws.CloudFront.Inputs.DistributionOriginArgs
            {
                DomainName = primary.BucketRegionalDomainName,
                OriginId = "primaryS3",
                S3OriginConfig = new Aws.CloudFront.Inputs.DistributionOriginS3OriginConfigArgs
                {
                    OriginAccessIdentity = @default.CloudfrontAccessIdentityPath,
                },
            },
            new Aws.CloudFront.Inputs.DistributionOriginArgs
            {
                DomainName = failover.BucketRegionalDomainName,
                OriginId = "failoverS3",
                S3OriginConfig = new Aws.CloudFront.Inputs.DistributionOriginS3OriginConfigArgs
                {
                    OriginAccessIdentity = @default.CloudfrontAccessIdentityPath,
                },
            },
        },
        DefaultCacheBehavior = new Aws.CloudFront.Inputs.DistributionDefaultCacheBehaviorArgs
        {
            TargetOriginId = "groupS3",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewDistribution(ctx, "s3_distribution", &cloudfront.DistributionArgs{
			OriginGroups: cloudfront.DistributionOriginGroupArray{
				&cloudfront.DistributionOriginGroupArgs{
					OriginId: pulumi.String("groupS3"),
					FailoverCriteria: &cloudfront.DistributionOriginGroupFailoverCriteriaArgs{
						StatusCodes: pulumi.IntArray{
							pulumi.Int(403),
							pulumi.Int(404),
							pulumi.Int(500),
							pulumi.Int(502),
						},
					},
					Members: cloudfront.DistributionOriginGroupMemberArray{
						&cloudfront.DistributionOriginGroupMemberArgs{
							OriginId: pulumi.String("primaryS3"),
						},
						&cloudfront.DistributionOriginGroupMemberArgs{
							OriginId: pulumi.String("failoverS3"),
						},
					},
				},
			},
			Origins: cloudfront.DistributionOriginArray{
				&cloudfront.DistributionOriginArgs{
					DomainName: pulumi.Any(primary.BucketRegionalDomainName),
					OriginId:   pulumi.String("primaryS3"),
					S3OriginConfig: &cloudfront.DistributionOriginS3OriginConfigArgs{
						OriginAccessIdentity: pulumi.Any(_default.CloudfrontAccessIdentityPath),
					},
				},
				&cloudfront.DistributionOriginArgs{
					DomainName: pulumi.Any(failover.BucketRegionalDomainName),
					OriginId:   pulumi.String("failoverS3"),
					S3OriginConfig: &cloudfront.DistributionOriginS3OriginConfigArgs{
						OriginAccessIdentity: pulumi.Any(_default.CloudfrontAccessIdentityPath),
					},
				},
			},
			DefaultCacheBehavior: &cloudfront.DistributionDefaultCacheBehaviorArgs{
				TargetOriginId: pulumi.String("groupS3"),
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
import com.pulumi.aws.cloudfront.Distribution;
import com.pulumi.aws.cloudfront.DistributionArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionOriginGroupArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionOriginGroupFailoverCriteriaArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionOriginArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionOriginS3OriginConfigArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionDefaultCacheBehaviorArgs;
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
        var s3Distribution = new Distribution("s3Distribution", DistributionArgs.builder()
            .originGroups(DistributionOriginGroupArgs.builder()
                .originId("groupS3")
                .failoverCriteria(DistributionOriginGroupFailoverCriteriaArgs.builder()
                    .statusCodes(                    
                        403,
                        404,
                        500,
                        502)
                    .build())
                .members(                
                    DistributionOriginGroupMemberArgs.builder()
                        .originId("primaryS3")
                        .build(),
                    DistributionOriginGroupMemberArgs.builder()
                        .originId("failoverS3")
                        .build())
                .build())
            .origins(            
                DistributionOriginArgs.builder()
                    .domainName(primary.bucketRegionalDomainName())
                    .originId("primaryS3")
                    .s3OriginConfig(DistributionOriginS3OriginConfigArgs.builder()
                        .originAccessIdentity(default_.cloudfrontAccessIdentityPath())
                        .build())
                    .build(),
                DistributionOriginArgs.builder()
                    .domainName(failover.bucketRegionalDomainName())
                    .originId("failoverS3")
                    .s3OriginConfig(DistributionOriginS3OriginConfigArgs.builder()
                        .originAccessIdentity(default_.cloudfrontAccessIdentityPath())
                        .build())
                    .build())
            .defaultCacheBehavior(DistributionDefaultCacheBehaviorArgs.builder()
                .targetOriginId("groupS3")
                .build())
            .build());

    }
}
```
```yaml
resources:
  s3Distribution:
    type: aws:cloudfront:Distribution
    name: s3_distribution
    properties:
      originGroups:
        - originId: groupS3
          failoverCriteria:
            statusCodes:
              - 403
              - 404
              - 500
              - 502
          members:
            - originId: primaryS3
            - originId: failoverS3
      origins:
        - domainName: ${primary.bucketRegionalDomainName}
          originId: primaryS3
          s3OriginConfig:
            originAccessIdentity: ${default.cloudfrontAccessIdentityPath}
        - domainName: ${failover.bucketRegionalDomainName}
          originId: failoverS3
          s3OriginConfig:
            originAccessIdentity: ${default.cloudfrontAccessIdentityPath}
      defaultCacheBehavior:
        targetOriginId: groupS3
```
<!--End PulumiCodeChooser -->

### With Managed Caching Policy

The example below creates a CloudFront distribution with an [AWS managed caching policy](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/using-managed-cache-policies.html).

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const s3OriginId = "myS3Origin";
const s3Distribution = new aws.cloudfront.Distribution("s3_distribution", {
    origins: [{
        domainName: primary.bucketRegionalDomainName,
        originId: "myS3Origin",
        s3OriginConfig: {
            originAccessIdentity: _default.cloudfrontAccessIdentityPath,
        },
    }],
    enabled: true,
    isIpv6Enabled: true,
    comment: "Some comment",
    defaultRootObject: "index.html",
    defaultCacheBehavior: {
        cachePolicyId: "4135ea2d-6df8-44a3-9df3-4b5a84be39ad",
        allowedMethods: [
            "GET",
            "HEAD",
            "OPTIONS",
        ],
        targetOriginId: s3OriginId,
    },
    restrictions: {
        geoRestriction: {
            restrictionType: "whitelist",
            locations: [
                "US",
                "CA",
                "GB",
                "DE",
            ],
        },
    },
    viewerCertificate: {
        cloudfrontDefaultCertificate: true,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

s3_origin_id = "myS3Origin"
s3_distribution = aws.cloudfront.Distribution("s3_distribution",
    origins=[{
        "domain_name": primary["bucketRegionalDomainName"],
        "origin_id": "myS3Origin",
        "s3_origin_config": {
            "origin_access_identity": default["cloudfrontAccessIdentityPath"],
        },
    }],
    enabled=True,
    is_ipv6_enabled=True,
    comment="Some comment",
    default_root_object="index.html",
    default_cache_behavior={
        "cache_policy_id": "4135ea2d-6df8-44a3-9df3-4b5a84be39ad",
        "allowed_methods": [
            "GET",
            "HEAD",
            "OPTIONS",
        ],
        "target_origin_id": s3_origin_id,
    },
    restrictions={
        "geo_restriction": {
            "restriction_type": "whitelist",
            "locations": [
                "US",
                "CA",
                "GB",
                "DE",
            ],
        },
    },
    viewer_certificate={
        "cloudfront_default_certificate": True,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var s3OriginId = "myS3Origin";

    var s3Distribution = new Aws.CloudFront.Distribution("s3_distribution", new()
    {
        Origins = new[]
        {
            new Aws.CloudFront.Inputs.DistributionOriginArgs
            {
                DomainName = primary.BucketRegionalDomainName,
                OriginId = "myS3Origin",
                S3OriginConfig = new Aws.CloudFront.Inputs.DistributionOriginS3OriginConfigArgs
                {
                    OriginAccessIdentity = @default.CloudfrontAccessIdentityPath,
                },
            },
        },
        Enabled = true,
        IsIpv6Enabled = true,
        Comment = "Some comment",
        DefaultRootObject = "index.html",
        DefaultCacheBehavior = new Aws.CloudFront.Inputs.DistributionDefaultCacheBehaviorArgs
        {
            CachePolicyId = "4135ea2d-6df8-44a3-9df3-4b5a84be39ad",
            AllowedMethods = new[]
            {
                "GET",
                "HEAD",
                "OPTIONS",
            },
            TargetOriginId = s3OriginId,
        },
        Restrictions = new Aws.CloudFront.Inputs.DistributionRestrictionsArgs
        {
            GeoRestriction = new Aws.CloudFront.Inputs.DistributionRestrictionsGeoRestrictionArgs
            {
                RestrictionType = "whitelist",
                Locations = new[]
                {
                    "US",
                    "CA",
                    "GB",
                    "DE",
                },
            },
        },
        ViewerCertificate = new Aws.CloudFront.Inputs.DistributionViewerCertificateArgs
        {
            CloudfrontDefaultCertificate = true,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		s3OriginId := "myS3Origin"
		_, err := cloudfront.NewDistribution(ctx, "s3_distribution", &cloudfront.DistributionArgs{
			Origins: cloudfront.DistributionOriginArray{
				&cloudfront.DistributionOriginArgs{
					DomainName: pulumi.Any(primary.BucketRegionalDomainName),
					OriginId:   pulumi.String("myS3Origin"),
					S3OriginConfig: &cloudfront.DistributionOriginS3OriginConfigArgs{
						OriginAccessIdentity: pulumi.Any(_default.CloudfrontAccessIdentityPath),
					},
				},
			},
			Enabled:           pulumi.Bool(true),
			IsIpv6Enabled:     pulumi.Bool(true),
			Comment:           pulumi.String("Some comment"),
			DefaultRootObject: pulumi.String("index.html"),
			DefaultCacheBehavior: &cloudfront.DistributionDefaultCacheBehaviorArgs{
				CachePolicyId: pulumi.String("4135ea2d-6df8-44a3-9df3-4b5a84be39ad"),
				AllowedMethods: pulumi.StringArray{
					pulumi.String("GET"),
					pulumi.String("HEAD"),
					pulumi.String("OPTIONS"),
				},
				TargetOriginId: pulumi.String(s3OriginId),
			},
			Restrictions: &cloudfront.DistributionRestrictionsArgs{
				GeoRestriction: &cloudfront.DistributionRestrictionsGeoRestrictionArgs{
					RestrictionType: pulumi.String("whitelist"),
					Locations: pulumi.StringArray{
						pulumi.String("US"),
						pulumi.String("CA"),
						pulumi.String("GB"),
						pulumi.String("DE"),
					},
				},
			},
			ViewerCertificate: &cloudfront.DistributionViewerCertificateArgs{
				CloudfrontDefaultCertificate: pulumi.Bool(true),
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
import com.pulumi.aws.cloudfront.Distribution;
import com.pulumi.aws.cloudfront.DistributionArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionOriginArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionOriginS3OriginConfigArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionDefaultCacheBehaviorArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionRestrictionsArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionRestrictionsGeoRestrictionArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionViewerCertificateArgs;
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
        final var s3OriginId = "myS3Origin";

        var s3Distribution = new Distribution("s3Distribution", DistributionArgs.builder()
            .origins(DistributionOriginArgs.builder()
                .domainName(primary.bucketRegionalDomainName())
                .originId("myS3Origin")
                .s3OriginConfig(DistributionOriginS3OriginConfigArgs.builder()
                    .originAccessIdentity(default_.cloudfrontAccessIdentityPath())
                    .build())
                .build())
            .enabled(true)
            .isIpv6Enabled(true)
            .comment("Some comment")
            .defaultRootObject("index.html")
            .defaultCacheBehavior(DistributionDefaultCacheBehaviorArgs.builder()
                .cachePolicyId("4135ea2d-6df8-44a3-9df3-4b5a84be39ad")
                .allowedMethods(                
                    "GET",
                    "HEAD",
                    "OPTIONS")
                .targetOriginId(s3OriginId)
                .build())
            .restrictions(DistributionRestrictionsArgs.builder()
                .geoRestriction(DistributionRestrictionsGeoRestrictionArgs.builder()
                    .restrictionType("whitelist")
                    .locations(                    
                        "US",
                        "CA",
                        "GB",
                        "DE")
                    .build())
                .build())
            .viewerCertificate(DistributionViewerCertificateArgs.builder()
                .cloudfrontDefaultCertificate(true)
                .build())
            .build());

    }
}
```
```yaml
resources:
  s3Distribution:
    type: aws:cloudfront:Distribution
    name: s3_distribution
    properties:
      origins:
        - domainName: ${primary.bucketRegionalDomainName}
          originId: myS3Origin
          s3OriginConfig:
            originAccessIdentity: ${default.cloudfrontAccessIdentityPath}
      enabled: true
      isIpv6Enabled: true
      comment: Some comment
      defaultRootObject: index.html
      defaultCacheBehavior:
        cachePolicyId: 4135ea2d-6df8-44a3-9df3-4b5a84be39ad
        allowedMethods:
          - GET
          - HEAD
          - OPTIONS
        targetOriginId: ${s3OriginId}
      restrictions:
        geoRestriction:
          restrictionType: whitelist
          locations:
            - US
            - CA
            - GB
            - DE
      viewerCertificate:
        cloudfrontDefaultCertificate: true
variables:
  s3OriginId: myS3Origin
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CloudFront Distributions using the `id`. For example:

```sh
$ pulumi import aws:cloudfront/distribution:Distribution distribution E74FTE3EXAMPLE
```

aliasesB*" 
commentB" $
continuousDeploymentPolicyIdB" †
customErrorResponsesáBÑ*Å:
}

cloudfrontDistributionCustomErrorResponseNaws:cloudfront/DistributionCustomErrorResponse:DistributionCustomErrorResponseü
defaultCacheBehaviorÜ:É
Ä

cloudfront DistributionDefaultCacheBehaviorPaws:cloudfront/DistributionDefaultCacheBehavior:DistributionDefaultCacheBehavior
defaultRootObjectB" l
enabled
 ]`true` if any of the AWS accounts listed as trusted signers have active CloudFront key pairs

httpVersionB" 
isIpv6EnabledB
 Ç
loggingConfigqBo:m
k

cloudfrontDistributionLoggingConfigBaws:cloudfront/DistributionLoggingConfig:DistributionLoggingConfig¶
orderedCacheBehaviorsåBâ*Ü:É
Ä

cloudfront DistributionOrderedCacheBehaviorPaws:cloudfront/DistributionOrderedCacheBehavior:DistributionOrderedCacheBehavior}
originGroupsmBk*i:g
e

cloudfrontDistributionOriginGroup>aws:cloudfront/DistributionOriginGroup:DistributionOriginGroupg
origins\*Z:X
V

cloudfrontDistributionOrigin4aws:cloudfront/DistributionOrigin:DistributionOrigin

priceClassB" |
restrictionsl:j
h

cloudfrontDistributionRestrictions@aws:cloudfront/DistributionRestrictions:DistributionRestrictions
retainOnDeleteB
 
stagingB
 
tagsB2" ê
viewerCertificate{:y
w

cloudfrontDistributionViewerCertificateJaws:cloudfront/DistributionViewerCertificate:DistributionViewerCertificate
waitForDeploymentB
 
webAclIdB" "
aliasesB*" "°
arn" ïARN for the distribution. For example: `arn:aws:cloudfront::123456789012:distribution/EDFDVBD632BHDS5`, where `123456789012` is your AWS account ID.
"t
callerReference" ]Internal value used by CloudFront to allow future updates to the distribution configuration.
"
commentB" ""
continuousDeploymentPolicyId" "†
customErrorResponsesáBÑ*Å:
}

cloudfrontDistributionCustomErrorResponseNaws:cloudfront/DistributionCustomErrorResponse:DistributionCustomErrorResponse"ü
defaultCacheBehaviorÜ:É
Ä

cloudfront DistributionDefaultCacheBehaviorPaws:cloudfront/DistributionDefaultCacheBehavior:DistributionDefaultCacheBehavior"
defaultRootObjectB" "o

domainName" ]Domain name corresponding to the distribution. For example: `d604721fxaaqy9.cloudfront.net`.
"l
enabled
 ]`true` if any of the AWS accounts listed as trusted signers have active CloudFront key pairs
"^
etag" RCurrent version of the distribution's information. For example: `E2QWRUHAPOMQZL`.
"Å
hostedZoneId" ÏCloudFront Route 53 zone ID that can be used to route an [Alias Resource Record Set](http://docs.aws.amazon.com/Route53/latest/APIReference/CreateAliasRRSAPI.html) to. This attribute is simply an alias for the zone ID `Z2FDTNDATAQYW2`.
"
httpVersionB" "Y
inProgressValidationBatches 6Number of invalidation batches currently in progress.
"
isIpv6EnabledB
 "J
lastModifiedTime" 2Date and time the distribution was last modified.
"Ç
loggingConfigqBo:m
k

cloudfrontDistributionLoggingConfigBaws:cloudfront/DistributionLoggingConfig:DistributionLoggingConfig"¶
orderedCacheBehaviorsåBâ*Ü:É
Ä

cloudfront DistributionOrderedCacheBehaviorPaws:cloudfront/DistributionOrderedCacheBehavior:DistributionOrderedCacheBehavior"}
originGroupsmBk*i:g
e

cloudfrontDistributionOriginGroup>aws:cloudfront/DistributionOriginGroup:DistributionOriginGroup"g
origins\*Z:X
V

cloudfrontDistributionOrigin4aws:cloudfront/DistributionOrigin:DistributionOrigin"

priceClassB" "|
restrictionsl:j
h

cloudfrontDistributionRestrictions@aws:cloudfront/DistributionRestrictions:DistributionRestrictions"
retainOnDeleteB
 "
stagingB
 "ù
status" éCurrent status of the distribution. `Deployed` if the distribution's information is fully propagated throughout the Amazon CloudFront system.
"
tagsB2" "á
tagsAll2" vMap of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"ê
trustedKeyGroupsw*u:s
q

cloudfrontDistributionTrustedKeyGroupFaws:cloudfront/DistributionTrustedKeyGroup:DistributionTrustedKeyGroupÇList of nested attributes for active trusted key groups, if the distribution is set up to serve private content with signed URLs.
"Ñ
trustedSignersq*o:m
k

cloudfrontDistributionTrustedSignerBaws:cloudfront/DistributionTrustedSigner:DistributionTrustedSignerList of nested attributes for active trusted signers, if the distribution is set up to serve private content with signed URLs.
"ê
viewerCertificate{:y
w

cloudfrontDistributionViewerCertificateJaws:cloudfront/DistributionViewerCertificate:DistributionViewerCertificate"
waitForDeploymentB
 "
webAclIdB" *öO
n

cloudfrontFieldLevelEncryptionConfigDaws:cloudfront/fieldLevelEncryptionConfig:FieldLevelEncryptionConfig˜>Provides a CloudFront Field-level Encryption Config resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = new aws.cloudfront.FieldLevelEncryptionConfig("test", {
    comment: "test comment",
    contentTypeProfileConfig: {
        forwardWhenContentTypeIsUnknown: true,
        contentTypeProfiles: {
            items: [{
                contentType: "application/x-www-form-urlencoded",
                format: "URLEncoded",
            }],
        },
    },
    queryArgProfileConfig: {
        forwardWhenQueryArgProfileIsUnknown: true,
        queryArgProfiles: {
            items: [{
                profileId: testAwsCloudfrontFieldLevelEncryptionProfile.id,
                queryArg: "Arg1",
            }],
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.cloudfront.FieldLevelEncryptionConfig("test",
    comment="test comment",
    content_type_profile_config={
        "forward_when_content_type_is_unknown": True,
        "content_type_profiles": {
            "items": [{
                "content_type": "application/x-www-form-urlencoded",
                "format": "URLEncoded",
            }],
        },
    },
    query_arg_profile_config={
        "forward_when_query_arg_profile_is_unknown": True,
        "query_arg_profiles": {
            "items": [{
                "profile_id": test_aws_cloudfront_field_level_encryption_profile["id"],
                "query_arg": "Arg1",
            }],
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
    var test = new Aws.CloudFront.FieldLevelEncryptionConfig("test", new()
    {
        Comment = "test comment",
        ContentTypeProfileConfig = new Aws.CloudFront.Inputs.FieldLevelEncryptionConfigContentTypeProfileConfigArgs
        {
            ForwardWhenContentTypeIsUnknown = true,
            ContentTypeProfiles = new Aws.CloudFront.Inputs.FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesArgs
            {
                Items = new[]
                {
                    new Aws.CloudFront.Inputs.FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesItemArgs
                    {
                        ContentType = "application/x-www-form-urlencoded",
                        Format = "URLEncoded",
                    },
                },
            },
        },
        QueryArgProfileConfig = new Aws.CloudFront.Inputs.FieldLevelEncryptionConfigQueryArgProfileConfigArgs
        {
            ForwardWhenQueryArgProfileIsUnknown = true,
            QueryArgProfiles = new Aws.CloudFront.Inputs.FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesArgs
            {
                Items = new[]
                {
                    new Aws.CloudFront.Inputs.FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesItemArgs
                    {
                        ProfileId = testAwsCloudfrontFieldLevelEncryptionProfile.Id,
                        QueryArg = "Arg1",
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
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewFieldLevelEncryptionConfig(ctx, "test", &cloudfront.FieldLevelEncryptionConfigArgs{
			Comment: pulumi.String("test comment"),
			ContentTypeProfileConfig: &cloudfront.FieldLevelEncryptionConfigContentTypeProfileConfigArgs{
				ForwardWhenContentTypeIsUnknown: pulumi.Bool(true),
				ContentTypeProfiles: &cloudfront.FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesArgs{
					Items: cloudfront.FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesItemArray{
						&cloudfront.FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesItemArgs{
							ContentType: pulumi.String("application/x-www-form-urlencoded"),
							Format:      pulumi.String("URLEncoded"),
						},
					},
				},
			},
			QueryArgProfileConfig: &cloudfront.FieldLevelEncryptionConfigQueryArgProfileConfigArgs{
				ForwardWhenQueryArgProfileIsUnknown: pulumi.Bool(true),
				QueryArgProfiles: &cloudfront.FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesArgs{
					Items: cloudfront.FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesItemArray{
						&cloudfront.FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesItemArgs{
							ProfileId: pulumi.Any(testAwsCloudfrontFieldLevelEncryptionProfile.Id),
							QueryArg:  pulumi.String("Arg1"),
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
import com.pulumi.aws.cloudfront.FieldLevelEncryptionConfig;
import com.pulumi.aws.cloudfront.FieldLevelEncryptionConfigArgs;
import com.pulumi.aws.cloudfront.inputs.FieldLevelEncryptionConfigContentTypeProfileConfigArgs;
import com.pulumi.aws.cloudfront.inputs.FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesArgs;
import com.pulumi.aws.cloudfront.inputs.FieldLevelEncryptionConfigQueryArgProfileConfigArgs;
import com.pulumi.aws.cloudfront.inputs.FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesArgs;
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
        var test = new FieldLevelEncryptionConfig("test", FieldLevelEncryptionConfigArgs.builder()
            .comment("test comment")
            .contentTypeProfileConfig(FieldLevelEncryptionConfigContentTypeProfileConfigArgs.builder()
                .forwardWhenContentTypeIsUnknown(true)
                .contentTypeProfiles(FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesArgs.builder()
                    .items(FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesItemArgs.builder()
                        .contentType("application/x-www-form-urlencoded")
                        .format("URLEncoded")
                        .build())
                    .build())
                .build())
            .queryArgProfileConfig(FieldLevelEncryptionConfigQueryArgProfileConfigArgs.builder()
                .forwardWhenQueryArgProfileIsUnknown(true)
                .queryArgProfiles(FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesArgs.builder()
                    .items(FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesItemArgs.builder()
                        .profileId(testAwsCloudfrontFieldLevelEncryptionProfile.id())
                        .queryArg("Arg1")
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  test:
    type: aws:cloudfront:FieldLevelEncryptionConfig
    properties:
      comment: test comment
      contentTypeProfileConfig:
        forwardWhenContentTypeIsUnknown: true
        contentTypeProfiles:
          items:
            - contentType: application/x-www-form-urlencoded
              format: URLEncoded
      queryArgProfileConfig:
        forwardWhenQueryArgProfileIsUnknown: true
        queryArgProfiles:
          items:
            - profileId: ${testAwsCloudfrontFieldLevelEncryptionProfile.id}
              queryArg: Arg1
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cloudfront Field Level Encryption Config using the `id`. For example:

```sh
$ pulumi import aws:cloudfront/fieldLevelEncryptionConfig:FieldLevelEncryptionConfig config E74FTE3AEXAMPLE
```
N
commentB" =An optional comment about the Field Level Encryption Config.
¢
contentTypeProfileConfigº:π
∂

cloudfront2FieldLevelEncryptionConfigContentTypeProfileConfigtaws:cloudfront/FieldLevelEncryptionConfigContentTypeProfileConfig:FieldLevelEncryptionConfigContentTypeProfileConfig∆Content Type Profile Config specifies when to forward content if a content type isn't recognized and profiles to use as by default in a request if a query argument doesn't specify a profile to use.

queryArgProfileConfig≥:∞
≠

cloudfront/FieldLevelEncryptionConfigQueryArgProfileConfignaws:cloudfront/FieldLevelEncryptionConfigQueryArgProfileConfig:FieldLevelEncryptionConfigQueryArgProfileConfig†Query Arg Profile Config that specifies when to forward content if a profile isn't found and the profile that can be provided as a query argument in a request.
"w
callerReference" `Internal value used by CloudFront to allow future updates to the Field Level Encryption Config.
"N
commentB" =An optional comment about the Field Level Encryption Config.
"¢
contentTypeProfileConfigº:π
∂

cloudfront2FieldLevelEncryptionConfigContentTypeProfileConfigtaws:cloudfront/FieldLevelEncryptionConfigContentTypeProfileConfig:FieldLevelEncryptionConfigContentTypeProfileConfig∆Content Type Profile Config specifies when to forward content if a content type isn't recognized and profiles to use as by default in a request if a query argument doesn't specify a profile to use.
"e
etag" YThe current version of the Field Level Encryption Config. For example: `E2QWRUHAPOMQZL`.
"
queryArgProfileConfig≥:∞
≠

cloudfront/FieldLevelEncryptionConfigQueryArgProfileConfignaws:cloudfront/FieldLevelEncryptionConfigQueryArgProfileConfig:FieldLevelEncryptionConfigQueryArgProfileConfig†Query Arg Profile Config that specifies when to forward content if a profile isn't found and the profile that can be provided as a query argument in a request.
*Ï?
q

cloudfrontFieldLevelEncryptionProfileFaws:cloudfront/fieldLevelEncryptionProfile:FieldLevelEncryptionProfileÄ5Provides a CloudFront Field-level Encryption Profile resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const example = new aws.cloudfront.PublicKey("example", {
    comment: "test public key",
    encodedKey: std.file({
        input: "public_key.pem",
    }).then(invoke => invoke.result),
    name: "test_key",
});
const test = new aws.cloudfront.FieldLevelEncryptionProfile("test", {
    comment: "test comment",
    name: "test profile",
    encryptionEntities: {
        items: [{
            publicKeyId: example.id,
            providerId: "test provider",
            fieldPatterns: {
                items: ["DateOfBirth"],
            },
        }],
    },
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

example = aws.cloudfront.PublicKey("example",
    comment="test public key",
    encoded_key=std.file(input="public_key.pem").result,
    name="test_key")
test = aws.cloudfront.FieldLevelEncryptionProfile("test",
    comment="test comment",
    name="test profile",
    encryption_entities={
        "items": [{
            "public_key_id": example.id,
            "provider_id": "test provider",
            "field_patterns": {
                "items": ["DateOfBirth"],
            },
        }],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFront.PublicKey("example", new()
    {
        Comment = "test public key",
        EncodedKey = Std.File.Invoke(new()
        {
            Input = "public_key.pem",
        }).Apply(invoke => invoke.Result),
        Name = "test_key",
    });

    var test = new Aws.CloudFront.FieldLevelEncryptionProfile("test", new()
    {
        Comment = "test comment",
        Name = "test profile",
        EncryptionEntities = new Aws.CloudFront.Inputs.FieldLevelEncryptionProfileEncryptionEntitiesArgs
        {
            Items = new[]
            {
                new Aws.CloudFront.Inputs.FieldLevelEncryptionProfileEncryptionEntitiesItemArgs
                {
                    PublicKeyId = example.Id,
                    ProviderId = "test provider",
                    FieldPatterns = new Aws.CloudFront.Inputs.FieldLevelEncryptionProfileEncryptionEntitiesItemFieldPatternsArgs
                    {
                        Items = new[]
                        {
                            "DateOfBirth",
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
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		invokeFile, err := std.File(ctx, &std.FileArgs{
			Input: "public_key.pem",
		}, nil)
		if err != nil {
			return err
		}
		example, err := cloudfront.NewPublicKey(ctx, "example", &cloudfront.PublicKeyArgs{
			Comment:    pulumi.String("test public key"),
			EncodedKey: pulumi.String(invokeFile.Result),
			Name:       pulumi.String("test_key"),
		})
		if err != nil {
			return err
		}
		_, err = cloudfront.NewFieldLevelEncryptionProfile(ctx, "test", &cloudfront.FieldLevelEncryptionProfileArgs{
			Comment: pulumi.String("test comment"),
			Name:    pulumi.String("test profile"),
			EncryptionEntities: &cloudfront.FieldLevelEncryptionProfileEncryptionEntitiesArgs{
				Items: cloudfront.FieldLevelEncryptionProfileEncryptionEntitiesItemArray{
					&cloudfront.FieldLevelEncryptionProfileEncryptionEntitiesItemArgs{
						PublicKeyId: example.ID(),
						ProviderId:  pulumi.String("test provider"),
						FieldPatterns: &cloudfront.FieldLevelEncryptionProfileEncryptionEntitiesItemFieldPatternsArgs{
							Items: pulumi.StringArray{
								pulumi.String("DateOfBirth"),
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
import com.pulumi.aws.cloudfront.PublicKey;
import com.pulumi.aws.cloudfront.PublicKeyArgs;
import com.pulumi.aws.cloudfront.FieldLevelEncryptionProfile;
import com.pulumi.aws.cloudfront.FieldLevelEncryptionProfileArgs;
import com.pulumi.aws.cloudfront.inputs.FieldLevelEncryptionProfileEncryptionEntitiesArgs;
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
        var example = new PublicKey("example", PublicKeyArgs.builder()
            .comment("test public key")
            .encodedKey(StdFunctions.file(FileArgs.builder()
                .input("public_key.pem")
                .build()).result())
            .name("test_key")
            .build());

        var test = new FieldLevelEncryptionProfile("test", FieldLevelEncryptionProfileArgs.builder()
            .comment("test comment")
            .name("test profile")
            .encryptionEntities(FieldLevelEncryptionProfileEncryptionEntitiesArgs.builder()
                .items(FieldLevelEncryptionProfileEncryptionEntitiesItemArgs.builder()
                    .publicKeyId(example.id())
                    .providerId("test provider")
                    .fieldPatterns(FieldLevelEncryptionProfileEncryptionEntitiesItemFieldPatternsArgs.builder()
                        .items("DateOfBirth")
                        .build())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:PublicKey
    properties:
      comment: test public key
      encodedKey:
        fn::invoke:
          function: std:file
          arguments:
            input: public_key.pem
          return: result
      name: test_key
  test:
    type: aws:cloudfront:FieldLevelEncryptionProfile
    properties:
      comment: test comment
      name: test profile
      encryptionEntities:
        items:
          - publicKeyId: ${example.id}
            providerId: test provider
            fieldPatterns:
              items:
                - DateOfBirth
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cloudfront Field Level Encryption Profile using the `id`. For example:

```sh
$ pulumi import aws:cloudfront/fieldLevelEncryptionProfile:FieldLevelEncryptionProfile profile K3D5EWEUDCCXON
```
O
commentB" >An optional comment about the Field Level Encryption Profile.
˜
encryptionEntities≠:™
ß

cloudfront-FieldLevelEncryptionProfileEncryptionEntitiesjaws:cloudfront/FieldLevelEncryptionProfileEncryptionEntities:FieldLevelEncryptionProfileEncryptionEntities∞The encryption entities config block for field-level encryption profiles that contains an attribute `items` which includes the encryption key and field pattern specifications.
>
nameB" 0The name of the Field Level Encryption Profile.
"x
callerReference" aInternal value used by CloudFront to allow future updates to the Field Level Encryption Profile.
"O
commentB" >An optional comment about the Field Level Encryption Profile.
"˜
encryptionEntities≠:™
ß

cloudfront-FieldLevelEncryptionProfileEncryptionEntitiesjaws:cloudfront/FieldLevelEncryptionProfileEncryptionEntities:FieldLevelEncryptionProfileEncryptionEntities∞The encryption entities config block for field-level encryption profiles that contains an attribute `items` which includes the encryption key and field pattern specifications.
"f
etag" ZThe current version of the Field Level Encryption Profile. For example: `E2QWRUHAPOMQZL`.
"<
name" 0The name of the Field Level Encryption Profile.
*‡
8

cloudfrontFunction aws:cloudfront/function:FunctionÀProvides a CloudFront Function resource. With CloudFront Functions in Amazon CloudFront, you can write lightweight functions in JavaScript for high-scale, latency-sensitive CDN customizations.

See [CloudFront Functions](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/cloudfront-functions.html)

> **NOTE:** You cannot delete a function if it‚Äôs associated with a cache behavior. First, update your distributions to remove the function association from all cache behaviors, then delete the function.

## Example Usage

## Import

Using `pulumi import`, import CloudFront Functions using the `name`. For example:

```sh
$ pulumi import aws:cloudfront/function:Function test my_test_function
```
(
code" Source code of the function

commentB" 	Comment.
∞
keyValueStoreAssociationsB*" äList of `aws.cloudfront.KeyValueStore` ARNs to be associated to the function. AWS limits associations to on key value store per function.
8
nameB" *Unique name for your CloudFront Function.
m
publishB
 \Whether to publish creation/change as Live CloudFront Function Version. Defaults to `true`.
õ
runtime" ãIdentifier of the function's runtime. Valid values are `cloudfront-js-1.0` and `cloudfront-js-2.0`.

The following arguments are optional:
"L
arn" AAmazon Resource Name (ARN) identifying your CloudFront Function.
"(
code" Source code of the function
"
commentB" 	Comment.
"f
etag" ZETag hash of the function. This is the value for the `DEVELOPMENT` stage of the function.
"∞
keyValueStoreAssociationsB*" äList of `aws.cloudfront.KeyValueStore` ARNs to be associated to the function. AWS limits associations to on key value store per function.
"D
liveStageEtag" /ETag hash of any `LIVE` stage of the function.
"6
name" *Unique name for your CloudFront Function.
"m
publishB
 \Whether to publish creation/change as Live CloudFront Function Version. Defaults to `true`.
"õ
runtime" ãIdentifier of the function's runtime. Valid values are `cloudfront-js-1.0` and `cloudfront-js-2.0`.

The following arguments are optional:
"\
status" NStatus of the function. Can be `UNPUBLISHED`, `UNASSOCIATED` or `ASSOCIATED`.
*›&
8

cloudfrontKeyGroup aws:cloudfront/keyGroup:KeyGroupÏ"## Example Usage

The following example below creates a CloudFront key group.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const example = new aws.cloudfront.PublicKey("example", {
    comment: "example public key",
    encodedKey: std.file({
        input: "public_key.pem",
    }).then(invoke => invoke.result),
    name: "example-key",
});
const exampleKeyGroup = new aws.cloudfront.KeyGroup("example", {
    comment: "example key group",
    items: [example.id],
    name: "example-key-group",
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

example = aws.cloudfront.PublicKey("example",
    comment="example public key",
    encoded_key=std.file(input="public_key.pem").result,
    name="example-key")
example_key_group = aws.cloudfront.KeyGroup("example",
    comment="example key group",
    items=[example.id],
    name="example-key-group")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFront.PublicKey("example", new()
    {
        Comment = "example public key",
        EncodedKey = Std.File.Invoke(new()
        {
            Input = "public_key.pem",
        }).Apply(invoke => invoke.Result),
        Name = "example-key",
    });

    var exampleKeyGroup = new Aws.CloudFront.KeyGroup("example", new()
    {
        Comment = "example key group",
        Items = new[]
        {
            example.Id,
        },
        Name = "example-key-group",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		invokeFile, err := std.File(ctx, &std.FileArgs{
			Input: "public_key.pem",
		}, nil)
		if err != nil {
			return err
		}
		example, err := cloudfront.NewPublicKey(ctx, "example", &cloudfront.PublicKeyArgs{
			Comment:    pulumi.String("example public key"),
			EncodedKey: pulumi.String(invokeFile.Result),
			Name:       pulumi.String("example-key"),
		})
		if err != nil {
			return err
		}
		_, err = cloudfront.NewKeyGroup(ctx, "example", &cloudfront.KeyGroupArgs{
			Comment: pulumi.String("example key group"),
			Items: pulumi.StringArray{
				example.ID(),
			},
			Name: pulumi.String("example-key-group"),
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
import com.pulumi.aws.cloudfront.PublicKey;
import com.pulumi.aws.cloudfront.PublicKeyArgs;
import com.pulumi.aws.cloudfront.KeyGroup;
import com.pulumi.aws.cloudfront.KeyGroupArgs;
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
        var example = new PublicKey("example", PublicKeyArgs.builder()
            .comment("example public key")
            .encodedKey(StdFunctions.file(FileArgs.builder()
                .input("public_key.pem")
                .build()).result())
            .name("example-key")
            .build());

        var exampleKeyGroup = new KeyGroup("exampleKeyGroup", KeyGroupArgs.builder()
            .comment("example key group")
            .items(example.id())
            .name("example-key-group")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:PublicKey
    properties:
      comment: example public key
      encodedKey:
        fn::invoke:
          function: std:file
          arguments:
            input: public_key.pem
          return: result
      name: example-key
  exampleKeyGroup:
    type: aws:cloudfront:KeyGroup
    name: example
    properties:
      comment: example key group
      items:
        - ${example.id}
      name: example-key-group
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CloudFront Key Group using the `id`. For example:

```sh
$ pulumi import aws:cloudfront/keyGroup:KeyGroup example 4b4f2r1c-315d-5c2e-f093-216t50jed10f
```
7
commentB" &A comment to describe the key group..
N
items*" ?A list of the identifiers of the public keys in the key group.
0
nameB" "A name to identify the key group.
"7
commentB" &A comment to describe the key group..
">
etag" 2The identifier for this version of the key group.
"N
items*" ?A list of the identifiers of the public keys in the key group.
".
name" "A name to identify the key group.
*¸
G

cloudfrontKeyValueStore*aws:cloudfront/keyValueStore:KeyValueStoreØResource for managing an AWS CloudFront Key Value Store.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudfront.KeyValueStore("example", {
    name: "ExampleKeyValueStore",
    comment: "This is an example key value store",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.KeyValueStore("example",
    name="ExampleKeyValueStore",
    comment="This is an example key value store")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFront.KeyValueStore("example", new()
    {
        Name = "ExampleKeyValueStore",
        Comment = "This is an example key value store",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewKeyValueStore(ctx, "example", &cloudfront.KeyValueStoreArgs{
			Name:    pulumi.String("ExampleKeyValueStore"),
			Comment: pulumi.String("This is an example key value store"),
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
import com.pulumi.aws.cloudfront.KeyValueStore;
import com.pulumi.aws.cloudfront.KeyValueStoreArgs;
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
        var example = new KeyValueStore("example", KeyValueStoreArgs.builder()
            .name("ExampleKeyValueStore")
            .comment("This is an example key value store")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:KeyValueStore
    properties:
      name: ExampleKeyValueStore
      comment: This is an example key value store
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CloudFront Key Value Store using the `name`. For example:

```sh
$ pulumi import aws:cloudfront/keyValueStore:KeyValueStore example example_store
```

commentB" 	Comment.
d
nameB" VUnique name for your CloudFront KeyValueStore.

The following arguments are optional:
q
timeoutseBc:a
_

cloudfrontKeyValueStoreTimeouts:aws:cloudfront/KeyValueStoreTimeouts:KeyValueStoreTimeouts"Q
arn" FAmazon Resource Name (ARN) identifying your CloudFront KeyValueStore.
"
commentB" 	Comment.
",
etag"  ETag hash of the KeyValueStore.
"
lastModifiedTime" "b
name" VUnique name for your CloudFront KeyValueStore.

The following arguments are optional:
"q
timeoutseBc:a
_

cloudfrontKeyValueStoreTimeouts:aws:cloudfront/KeyValueStoreTimeouts:KeyValueStoreTimeouts*◊!
P

cloudfrontKeyvaluestoreKey0aws:cloudfront/keyvaluestoreKey:KeyvaluestoreKey∂Resource for managing an AWS CloudFront KeyValueStore Key.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudfront.KeyValueStore("example", {
    name: "ExampleKeyValueStore",
    comment: "This is an example key value store",
});
const exampleKeyvaluestoreKey = new aws.cloudfront.KeyvaluestoreKey("example", {
    keyValueStoreArn: example.arn,
    key: "Test Key",
    value: "Test Value",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.KeyValueStore("example",
    name="ExampleKeyValueStore",
    comment="This is an example key value store")
example_keyvaluestore_key = aws.cloudfront.KeyvaluestoreKey("example",
    key_value_store_arn=example.arn,
    key="Test Key",
    value="Test Value")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFront.KeyValueStore("example", new()
    {
        Name = "ExampleKeyValueStore",
        Comment = "This is an example key value store",
    });

    var exampleKeyvaluestoreKey = new Aws.CloudFront.KeyvaluestoreKey("example", new()
    {
        KeyValueStoreArn = example.Arn,
        Key = "Test Key",
        Value = "Test Value",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		example, err := cloudfront.NewKeyValueStore(ctx, "example", &cloudfront.KeyValueStoreArgs{
			Name:    pulumi.String("ExampleKeyValueStore"),
			Comment: pulumi.String("This is an example key value store"),
		})
		if err != nil {
			return err
		}
		_, err = cloudfront.NewKeyvaluestoreKey(ctx, "example", &cloudfront.KeyvaluestoreKeyArgs{
			KeyValueStoreArn: example.Arn,
			Key:              pulumi.String("Test Key"),
			Value:            pulumi.String("Test Value"),
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
import com.pulumi.aws.cloudfront.KeyValueStore;
import com.pulumi.aws.cloudfront.KeyValueStoreArgs;
import com.pulumi.aws.cloudfront.KeyvaluestoreKey;
import com.pulumi.aws.cloudfront.KeyvaluestoreKeyArgs;
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
        var example = new KeyValueStore("example", KeyValueStoreArgs.builder()
            .name("ExampleKeyValueStore")
            .comment("This is an example key value store")
            .build());

        var exampleKeyvaluestoreKey = new KeyvaluestoreKey("exampleKeyvaluestoreKey", KeyvaluestoreKeyArgs.builder()
            .keyValueStoreArn(example.arn())
            .key("Test Key")
            .value("Test Value")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:KeyValueStore
    properties:
      name: ExampleKeyValueStore
      comment: This is an example key value store
  exampleKeyvaluestoreKey:
    type: aws:cloudfront:KeyvaluestoreKey
    name: example
    properties:
      keyValueStoreArn: ${example.arn}
      key: Test Key
      value: Test Value
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CloudFront KeyValueStore Key using the `id`. For example:

```sh
$ pulumi import aws:cloudfront/keyvaluestoreKey:KeyvaluestoreKey example arn:aws:cloudfront::111111111111:key-value-store/8562g61f-caba-2845-9d99-b97diwae5d3c,someKey
```

key" Key to put.
K
keyValueStoreArn" 3Amazon Resource Name (ARN) of the Key Value Store.

value" Value to put.
"
key" Key to put.
"K
keyValueStoreArn" 3Amazon Resource Name (ARN) of the Key Value Store.
"D
totalSizeInBytes ,Total size of the Key Value Store in bytes.
"
value" Value to put.
*€'
b

cloudfrontMonitoringSubscription<aws:cloudfront/monitoringSubscription:MonitoringSubscription˙Provides a CloudFront real-time log configuration resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudfront.MonitoringSubscription("example", {
    distributionId: exampleAwsCloudfrontDistribution.id,
    monitoringSubscription: {
        realtimeMetricsSubscriptionConfig: {
            realtimeMetricsSubscriptionStatus: "Enabled",
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.MonitoringSubscription("example",
    distribution_id=example_aws_cloudfront_distribution["id"],
    monitoring_subscription={
        "realtime_metrics_subscription_config": {
            "realtime_metrics_subscription_status": "Enabled",
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
    var example = new Aws.CloudFront.MonitoringSubscription("example", new()
    {
        DistributionId = exampleAwsCloudfrontDistribution.Id,
        MonitoringSubscriptionDetails = new Aws.CloudFront.Inputs.MonitoringSubscriptionMonitoringSubscriptionArgs
        {
            RealtimeMetricsSubscriptionConfig = new Aws.CloudFront.Inputs.MonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfigArgs
            {
                RealtimeMetricsSubscriptionStatus = "Enabled",
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewMonitoringSubscription(ctx, "example", &cloudfront.MonitoringSubscriptionArgs{
			DistributionId: pulumi.Any(exampleAwsCloudfrontDistribution.Id),
			MonitoringSubscription: &cloudfront.MonitoringSubscriptionMonitoringSubscriptionArgs{
				RealtimeMetricsSubscriptionConfig: &cloudfront.MonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfigArgs{
					RealtimeMetricsSubscriptionStatus: pulumi.String("Enabled"),
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
import com.pulumi.aws.cloudfront.MonitoringSubscription;
import com.pulumi.aws.cloudfront.MonitoringSubscriptionArgs;
import com.pulumi.aws.cloudfront.inputs.MonitoringSubscriptionMonitoringSubscriptionArgs;
import com.pulumi.aws.cloudfront.inputs.MonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfigArgs;
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
        var example = new MonitoringSubscription("example", MonitoringSubscriptionArgs.builder()
            .distributionId(exampleAwsCloudfrontDistribution.id())
            .monitoringSubscription(MonitoringSubscriptionMonitoringSubscriptionArgs.builder()
                .realtimeMetricsSubscriptionConfig(MonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfigArgs.builder()
                    .realtimeMetricsSubscriptionStatus("Enabled")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:MonitoringSubscription
    properties:
      distributionId: ${exampleAwsCloudfrontDistribution.id}
      monitoringSubscription:
        realtimeMetricsSubscriptionConfig:
          realtimeMetricsSubscriptionStatus: Enabled
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CloudFront monitoring subscription using the id. For example:

```sh
$ pulumi import aws:cloudfront/monitoringSubscription:MonitoringSubscription example E3QYSUHO4VYRGB
```
T
distributionId" >The ID of the distribution that you are enabling metrics for.
‰
monitoringSubscription™:ß
§

cloudfront,MonitoringSubscriptionMonitoringSubscriptionhaws:cloudfront/MonitoringSubscriptionMonitoringSubscription:MonitoringSubscriptionMonitoringSubscriptionúA monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.
"T
distributionId" >The ID of the distribution that you are enabling metrics for.
"‰
monitoringSubscription™:ß
§

cloudfront,MonitoringSubscriptionMonitoringSubscriptionhaws:cloudfront/MonitoringSubscriptionMonitoringSubscription:MonitoringSubscriptionMonitoringSubscriptionúA monitoring subscription. This structure contains information about whether additional CloudWatch metrics are enabled for a given CloudFront distribution.
*‚%
Y

cloudfrontOriginAccessControl6aws:cloudfront/originAccessControl:OriginAccessControlœManages an AWS CloudFront Origin Access Control, which is used by CloudFront Distributions with an Amazon S3 bucket as the origin.

Read more about Origin Access Control in the [CloudFront Developer Guide](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-restricting-access-to-s3.html).

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudfront.OriginAccessControl("example", {
    name: "example",
    description: "Example Policy",
    originAccessControlOriginType: "s3",
    signingBehavior: "always",
    signingProtocol: "sigv4",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.OriginAccessControl("example",
    name="example",
    description="Example Policy",
    origin_access_control_origin_type="s3",
    signing_behavior="always",
    signing_protocol="sigv4")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFront.OriginAccessControl("example", new()
    {
        Name = "example",
        Description = "Example Policy",
        OriginAccessControlOriginType = "s3",
        SigningBehavior = "always",
        SigningProtocol = "sigv4",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewOriginAccessControl(ctx, "example", &cloudfront.OriginAccessControlArgs{
			Name:                          pulumi.String("example"),
			Description:                   pulumi.String("Example Policy"),
			OriginAccessControlOriginType: pulumi.String("s3"),
			SigningBehavior:               pulumi.String("always"),
			SigningProtocol:               pulumi.String("sigv4"),
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
import com.pulumi.aws.cloudfront.OriginAccessControl;
import com.pulumi.aws.cloudfront.OriginAccessControlArgs;
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
        var example = new OriginAccessControl("example", OriginAccessControlArgs.builder()
            .name("example")
            .description("Example Policy")
            .originAccessControlOriginType("s3")
            .signingBehavior("always")
            .signingProtocol("sigv4")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:OriginAccessControl
    properties:
      name: example
      description: Example Policy
      originAccessControlOriginType: s3
      signingBehavior: always
      signingProtocol: sigv4
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CloudFront Origin Access Control using the `id`. For example:

```sh
$ pulumi import aws:cloudfront/originAccessControl:OriginAccessControl example E327GJI25M56DG
```
o
descriptionB" ZThe description of the Origin Access Control. Defaults to "Managed by Pulumi" if omitted.
@
nameB" 2A name that identifies the Origin Access Control.
¶
originAccessControlOriginType" ÄThe type of origin that this Origin Access Control is for. Valid values are `lambda`, `mediapackagev2`, `mediastore`, and `s3`.
®
signingBehavior" êSpecifies which requests CloudFront signs. Specify `always` for the most common use case. Allowed values: `always`, `never`, and `no-override`.
r
signingProtocol" [Determines how CloudFront signs (authenticates) requests. The only valid value is `sigv4`.
"o
descriptionB" ZThe description of the Origin Access Control. Defaults to "Managed by Pulumi" if omitted.
"?
etag" 3The current version of this Origin Access Control.
">
name" 2A name that identifies the Origin Access Control.
"¶
originAccessControlOriginType" ÄThe type of origin that this Origin Access Control is for. Valid values are `lambda`, `mediapackagev2`, `mediastore`, and `s3`.
"®
signingBehavior" êSpecifies which requests CloudFront signs. Specify `always` for the most common use case. Allowed values: `always`, `never`, and `no-override`.
"r
signingProtocol" [Determines how CloudFront signs (authenticates) requests. The only valid value is `sigv4`.
*Ûb
\

cloudfrontOriginAccessIdentity8aws:cloudfront/originAccessIdentity:OriginAccessIdentityπ[Creates an Amazon CloudFront origin access identity.

For information about CloudFront distributions, see the
[Amazon CloudFront Developer Guide](http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Introduction.html). For more information on generating
origin access identities, see
[Using an Origin Access Identity to Restrict Access to Your Amazon S3 Content][2].

## Example Usage

The following example below creates a CloudFront origin access identity.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudfront.OriginAccessIdentity("example", {comment: "Some comment"});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.OriginAccessIdentity("example", comment="Some comment")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFront.OriginAccessIdentity("example", new()
    {
        Comment = "Some comment",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewOriginAccessIdentity(ctx, "example", &cloudfront.OriginAccessIdentityArgs{
			Comment: pulumi.String("Some comment"),
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
import com.pulumi.aws.cloudfront.OriginAccessIdentity;
import com.pulumi.aws.cloudfront.OriginAccessIdentityArgs;
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
        var example = new OriginAccessIdentity("example", OriginAccessIdentityArgs.builder()
            .comment("Some comment")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:OriginAccessIdentity
    properties:
      comment: Some comment
```
<!--End PulumiCodeChooser -->

## Using With CloudFront

Normally, when referencing an origin access identity in CloudFront, you need to
prefix the ID with the `origin-access-identity/cloudfront/` special path.
The `cloudfront_access_identity_path` allows this to be circumvented.
The below snippet demonstrates use with the `s3_origin_config` structure for the
`aws.cloudfront.Distribution` resource:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudfront.Distribution("example", {origins: [{
    s3OriginConfig: {
        originAccessIdentity: exampleAwsCloudfrontOriginAccessIdentity.cloudfrontAccessIdentityPath,
    },
}]});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.Distribution("example", origins=[{
    "s3_origin_config": {
        "origin_access_identity": example_aws_cloudfront_origin_access_identity["cloudfrontAccessIdentityPath"],
    },
}])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFront.Distribution("example", new()
    {
        Origins = new[]
        {
            new Aws.CloudFront.Inputs.DistributionOriginArgs
            {
                S3OriginConfig = new Aws.CloudFront.Inputs.DistributionOriginS3OriginConfigArgs
                {
                    OriginAccessIdentity = exampleAwsCloudfrontOriginAccessIdentity.CloudfrontAccessIdentityPath,
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewDistribution(ctx, "example", &cloudfront.DistributionArgs{
			Origins: cloudfront.DistributionOriginArray{
				&cloudfront.DistributionOriginArgs{
					S3OriginConfig: &cloudfront.DistributionOriginS3OriginConfigArgs{
						OriginAccessIdentity: pulumi.Any(exampleAwsCloudfrontOriginAccessIdentity.CloudfrontAccessIdentityPath),
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
import com.pulumi.aws.cloudfront.Distribution;
import com.pulumi.aws.cloudfront.DistributionArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionOriginArgs;
import com.pulumi.aws.cloudfront.inputs.DistributionOriginS3OriginConfigArgs;
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
        var example = new Distribution("example", DistributionArgs.builder()
            .origins(DistributionOriginArgs.builder()
                .s3OriginConfig(DistributionOriginS3OriginConfigArgs.builder()
                    .originAccessIdentity(exampleAwsCloudfrontOriginAccessIdentity.cloudfrontAccessIdentityPath())
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:Distribution
    properties:
      origins:
        - s3OriginConfig:
            originAccessIdentity: ${exampleAwsCloudfrontOriginAccessIdentity.cloudfrontAccessIdentityPath}
```
<!--End PulumiCodeChooser -->

### Updating your bucket policy

Note that the AWS API may translate the `s3_canonical_user_id` `CanonicalUser`
principal into an `AWS` IAM ARN principal when supplied in an
`aws.s3.BucketV2` bucket policy, causing spurious diffs. If
you see this behavior, use the `iam_arn` instead:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const s3Policy = aws.iam.getPolicyDocument({
    statements: [{
        actions: ["s3:GetObject"],
        resources: [`${exampleAwsS3Bucket.arn}/*`],
        principals: [{
            type: "AWS",
            identifiers: [exampleAwsCloudfrontOriginAccessIdentity.iamArn],
        }],
    }],
});
const example = new aws.s3.BucketPolicy("example", {
    bucket: exampleAwsS3Bucket.id,
    policy: s3Policy.then(s3Policy => s3Policy.json),
});
```
```python
import pulumi
import pulumi_aws as aws

s3_policy = aws.iam.get_policy_document(statements=[{
    "actions": ["s3:GetObject"],
    "resources": [f"{example_aws_s3_bucket['arn']}/*"],
    "principals": [{
        "type": "AWS",
        "identifiers": [example_aws_cloudfront_origin_access_identity["iamArn"]],
    }],
}])
example = aws.s3.BucketPolicy("example",
    bucket=example_aws_s3_bucket["id"],
    policy=s3_policy.json)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var s3Policy = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Actions = new[]
                {
                    "s3:GetObject",
                },
                Resources = new[]
                {
                    $"{exampleAwsS3Bucket.Arn}/*",
                },
                Principals = new[]
                {
                    new Aws.Iam.Inputs.GetPolicyDocumentStatementPrincipalInputArgs
                    {
                        Type = "AWS",
                        Identifiers = new[]
                        {
                            exampleAwsCloudfrontOriginAccessIdentity.IamArn,
                        },
                    },
                },
            },
        },
    });

    var example = new Aws.S3.BucketPolicy("example", new()
    {
        Bucket = exampleAwsS3Bucket.Id,
        Policy = s3Policy.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

});
```
```go
package main

import (
	"fmt"

	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
s3Policy, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
Statements: []iam.GetPolicyDocumentStatement{
{
Actions: []string{
"s3:GetObject",
},
Resources: []string{
fmt.Sprintf("%v/*", exampleAwsS3Bucket.Arn),
},
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "AWS",
Identifiers: interface{}{
exampleAwsCloudfrontOriginAccessIdentity.IamArn,
},
},
},
},
},
}, nil);
if err != nil {
return err
}
_, err = s3.NewBucketPolicy(ctx, "example", &s3.BucketPolicyArgs{
Bucket: pulumi.Any(exampleAwsS3Bucket.Id),
Policy: pulumi.String(s3Policy.Json),
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
        final var s3Policy = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .actions("s3:GetObject")
                .resources(String.format("%s/*", exampleAwsS3Bucket.arn()))
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("AWS")
                    .identifiers(exampleAwsCloudfrontOriginAccessIdentity.iamArn())
                    .build())
                .build())
            .build());

        var example = new BucketPolicy("example", BucketPolicyArgs.builder()
            .bucket(exampleAwsS3Bucket.id())
            .policy(s3Policy.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:s3:BucketPolicy
    properties:
      bucket: ${exampleAwsS3Bucket.id}
      policy: ${s3Policy.json}
variables:
  s3Policy:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - actions:
              - s3:GetObject
            resources:
              - ${exampleAwsS3Bucket.arn}/*
            principals:
              - type: AWS
                identifiers:
                  - ${exampleAwsCloudfrontOriginAccessIdentity.iamArn}
```
<!--End PulumiCodeChooser -->

[1]: http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/Introduction.html
[2]: http://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/private-content-restricting-access-to-s3.html

## Import

Using `pulumi import`, import Cloudfront Origin Access Identities using the `id`. For example:

```sh
$ pulumi import aws:cloudfront/originAccessIdentity:OriginAccessIdentity origin_access E74FTE3AEXAMPLE
```
E
commentB" 4An optional comment for the origin access identity.
"p
callerReference" YInternal value used by CloudFront to allow future
updates to the origin access identity.
"Ä
cloudfrontAccessIdentityPath" \A shortcut to the full path for the
origin access identity to use in CloudFront, see below.
"E
commentB" 4An optional comment for the origin access identity.
"l
etag" `The current version of the origin access identity's information.
For example: `E2QWRUHAPOMQZL`.
"®
iamArn" ôA pre-generated ARN for use in S3 bucket policies (see below).
Example: `arn:aws:iam::cloudfront:user/CloudFront Origin Access Identity
E2QWRUHAPOMQZL`.
"∫
s3CanonicalUserId" †The Amazon S3 canonical user ID for the origin
access identity, which you use when giving the origin access identity read
permission to an object in Amazon S3.
*ìM
Y

cloudfrontOriginRequestPolicy6aws:cloudfront/originRequestPolicy:OriginRequestPolicy≈6## Example Usage

The following example below creates a CloudFront origin request policy.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudfront.OriginRequestPolicy("example", {
    name: "example-policy",
    comment: "example comment",
    cookiesConfig: {
        cookieBehavior: "whitelist",
        cookies: {
            items: ["example"],
        },
    },
    headersConfig: {
        headerBehavior: "whitelist",
        headers: {
            items: ["example"],
        },
    },
    queryStringsConfig: {
        queryStringBehavior: "whitelist",
        queryStrings: {
            items: ["example"],
        },
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.OriginRequestPolicy("example",
    name="example-policy",
    comment="example comment",
    cookies_config={
        "cookie_behavior": "whitelist",
        "cookies": {
            "items": ["example"],
        },
    },
    headers_config={
        "header_behavior": "whitelist",
        "headers": {
            "items": ["example"],
        },
    },
    query_strings_config={
        "query_string_behavior": "whitelist",
        "query_strings": {
            "items": ["example"],
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
    var example = new Aws.CloudFront.OriginRequestPolicy("example", new()
    {
        Name = "example-policy",
        Comment = "example comment",
        CookiesConfig = new Aws.CloudFront.Inputs.OriginRequestPolicyCookiesConfigArgs
        {
            CookieBehavior = "whitelist",
            Cookies = new Aws.CloudFront.Inputs.OriginRequestPolicyCookiesConfigCookiesArgs
            {
                Items = new[]
                {
                    "example",
                },
            },
        },
        HeadersConfig = new Aws.CloudFront.Inputs.OriginRequestPolicyHeadersConfigArgs
        {
            HeaderBehavior = "whitelist",
            Headers = new Aws.CloudFront.Inputs.OriginRequestPolicyHeadersConfigHeadersArgs
            {
                Items = new[]
                {
                    "example",
                },
            },
        },
        QueryStringsConfig = new Aws.CloudFront.Inputs.OriginRequestPolicyQueryStringsConfigArgs
        {
            QueryStringBehavior = "whitelist",
            QueryStrings = new Aws.CloudFront.Inputs.OriginRequestPolicyQueryStringsConfigQueryStringsArgs
            {
                Items = new[]
                {
                    "example",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewOriginRequestPolicy(ctx, "example", &cloudfront.OriginRequestPolicyArgs{
			Name:    pulumi.String("example-policy"),
			Comment: pulumi.String("example comment"),
			CookiesConfig: &cloudfront.OriginRequestPolicyCookiesConfigArgs{
				CookieBehavior: pulumi.String("whitelist"),
				Cookies: &cloudfront.OriginRequestPolicyCookiesConfigCookiesArgs{
					Items: pulumi.StringArray{
						pulumi.String("example"),
					},
				},
			},
			HeadersConfig: &cloudfront.OriginRequestPolicyHeadersConfigArgs{
				HeaderBehavior: pulumi.String("whitelist"),
				Headers: &cloudfront.OriginRequestPolicyHeadersConfigHeadersArgs{
					Items: pulumi.StringArray{
						pulumi.String("example"),
					},
				},
			},
			QueryStringsConfig: &cloudfront.OriginRequestPolicyQueryStringsConfigArgs{
				QueryStringBehavior: pulumi.String("whitelist"),
				QueryStrings: &cloudfront.OriginRequestPolicyQueryStringsConfigQueryStringsArgs{
					Items: pulumi.StringArray{
						pulumi.String("example"),
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
import com.pulumi.aws.cloudfront.OriginRequestPolicy;
import com.pulumi.aws.cloudfront.OriginRequestPolicyArgs;
import com.pulumi.aws.cloudfront.inputs.OriginRequestPolicyCookiesConfigArgs;
import com.pulumi.aws.cloudfront.inputs.OriginRequestPolicyCookiesConfigCookiesArgs;
import com.pulumi.aws.cloudfront.inputs.OriginRequestPolicyHeadersConfigArgs;
import com.pulumi.aws.cloudfront.inputs.OriginRequestPolicyHeadersConfigHeadersArgs;
import com.pulumi.aws.cloudfront.inputs.OriginRequestPolicyQueryStringsConfigArgs;
import com.pulumi.aws.cloudfront.inputs.OriginRequestPolicyQueryStringsConfigQueryStringsArgs;
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
        var example = new OriginRequestPolicy("example", OriginRequestPolicyArgs.builder()
            .name("example-policy")
            .comment("example comment")
            .cookiesConfig(OriginRequestPolicyCookiesConfigArgs.builder()
                .cookieBehavior("whitelist")
                .cookies(OriginRequestPolicyCookiesConfigCookiesArgs.builder()
                    .items("example")
                    .build())
                .build())
            .headersConfig(OriginRequestPolicyHeadersConfigArgs.builder()
                .headerBehavior("whitelist")
                .headers(OriginRequestPolicyHeadersConfigHeadersArgs.builder()
                    .items("example")
                    .build())
                .build())
            .queryStringsConfig(OriginRequestPolicyQueryStringsConfigArgs.builder()
                .queryStringBehavior("whitelist")
                .queryStrings(OriginRequestPolicyQueryStringsConfigQueryStringsArgs.builder()
                    .items("example")
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:OriginRequestPolicy
    properties:
      name: example-policy
      comment: example comment
      cookiesConfig:
        cookieBehavior: whitelist
        cookies:
          items:
            - example
      headersConfig:
        headerBehavior: whitelist
        headers:
          items:
            - example
      queryStringsConfig:
        queryStringBehavior: whitelist
        queryStrings:
          items:
            - example
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cloudfront Origin Request Policies using the `id`. For example:

```sh
$ pulumi import aws:cloudfront/originRequestPolicy:OriginRequestPolicy policy ccca32ef-dce3-4df3-80df-1bd3000bc4d3
```
@
commentB" /Comment to describe the origin request policy.
ê
cookiesConfigÜ:É
Ä

cloudfront OriginRequestPolicyCookiesConfigPaws:cloudfront/OriginRequestPolicyCookiesConfig:OriginRequestPolicyCookiesConfigıObject that determines whether any cookies in viewer requests (and if so, which cookies) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
Ç
headersConfigÜ:É
Ä

cloudfront OriginRequestPolicyHeadersConfigPaws:cloudfront/OriginRequestPolicyHeadersConfig:OriginRequestPolicyHeadersConfigÁObject that determines whether any HTTP headers (and if so, which headers) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
A
nameB" 3Unique name to identify the origin request policy.
π
queryStringsConfigï:í
è

cloudfront%OriginRequestPolicyQueryStringsConfigZaws:cloudfront/OriginRequestPolicyQueryStringsConfig:OriginRequestPolicyQueryStringsConfigäObject that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Query String Config for more information.
"@
commentB" /Comment to describe the origin request policy.
"ê
cookiesConfigÜ:É
Ä

cloudfront OriginRequestPolicyCookiesConfigPaws:cloudfront/OriginRequestPolicyCookiesConfig:OriginRequestPolicyCookiesConfigıObject that determines whether any cookies in viewer requests (and if so, which cookies) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
">
etag" 2The current version of the origin request policy.
"Ç
headersConfigÜ:É
Ä

cloudfront OriginRequestPolicyHeadersConfigPaws:cloudfront/OriginRequestPolicyHeadersConfig:OriginRequestPolicyHeadersConfigÁObject that determines whether any HTTP headers (and if so, which headers) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
"?
name" 3Unique name to identify the origin request policy.
"π
queryStringsConfigï:í
è

cloudfront%OriginRequestPolicyQueryStringsConfigZaws:cloudfront/OriginRequestPolicyQueryStringsConfig:OriginRequestPolicyQueryStringsConfigäObject that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Query String Config for more information.
*»'
;

cloudfront	PublicKey"aws:cloudfront/publicKey:PublicKey¢## Example Usage

The following example below creates a CloudFront public key.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";
import * as std from "@pulumi/std";

const example = new aws.cloudfront.PublicKey("example", {
    comment: "test public key",
    encodedKey: std.file({
        input: "public_key.pem",
    }).then(invoke => invoke.result),
    name: "test_key",
});
```
```python
import pulumi
import pulumi_aws as aws
import pulumi_std as std

example = aws.cloudfront.PublicKey("example",
    comment="test public key",
    encoded_key=std.file(input="public_key.pem").result,
    name="test_key")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;
using Std = Pulumi.Std;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFront.PublicKey("example", new()
    {
        Comment = "test public key",
        EncodedKey = Std.File.Invoke(new()
        {
            Input = "public_key.pem",
        }).Apply(invoke => invoke.Result),
        Name = "test_key",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi-std/sdk/go/std"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		invokeFile, err := std.File(ctx, &std.FileArgs{
			Input: "public_key.pem",
		}, nil)
		if err != nil {
			return err
		}
		_, err = cloudfront.NewPublicKey(ctx, "example", &cloudfront.PublicKeyArgs{
			Comment:    pulumi.String("test public key"),
			EncodedKey: pulumi.String(invokeFile.Result),
			Name:       pulumi.String("test_key"),
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
import com.pulumi.aws.cloudfront.PublicKey;
import com.pulumi.aws.cloudfront.PublicKeyArgs;
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
        var example = new PublicKey("example", PublicKeyArgs.builder()
            .comment("test public key")
            .encodedKey(StdFunctions.file(FileArgs.builder()
                .input("public_key.pem")
                .build()).result())
            .name("test_key")
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:PublicKey
    properties:
      comment: test public key
      encodedKey:
        fn::invoke:
          function: std:file
          arguments:
            input: public_key.pem
          return: result
      name: test_key
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CloudFront Public Key using the `id`. For example:

```sh
$ pulumi import aws:cloudfront/publicKey:PublicKey example K3D5EWEUDCCXON
```
;
commentB" *An optional comment about the public key.
~

encodedKey" lThe encoded public key that you want to add to CloudFront to use with features like field-level encryption.
ä
nameB" ˚The name for the public key. By default generated by this provider. Note: Do not set if using the key's id in another resource (e.g. KeyGroup) since it will result in a dependency error from AWS. Instead, it is recommended to use Pulumi autonaming by leaving this property unset (default behavior) or set the `namePrefix` property to allow the provider to autoname the resource.
Ñ

namePrefixB" ÔThe name for the public key. Conflicts with `name`.

**NOTE:** When setting `encoded_key` value, there needs a newline at the end of string. Otherwise, multiple runs of pulumi will want to recreate the `aws.cloudfront.PublicKey` resource.
"r
callerReference" [Internal value used by CloudFront to allow future updates to the public key configuration.
";
commentB" *An optional comment about the public key.
"~

encodedKey" lThe encoded public key that you want to add to CloudFront to use with features like field-level encryption.
"R
etag" FThe current version of the public key. For example: `E2QWRUHAPOMQZL`.
"à
name" ˚The name for the public key. By default generated by this provider. Note: Do not set if using the key's id in another resource (e.g. KeyGroup) since it will result in a dependency error from AWS. Instead, it is recommended to use Pulumi autonaming by leaving this property unset (default behavior) or set the `namePrefix` property to allow the provider to autoname the resource.
"Ç

namePrefix" ÔThe name for the public key. Conflicts with `name`.

**NOTE:** When setting `encoded_key` value, there needs a newline at the end of string. Otherwise, multiple runs of pulumi will want to recreate the `aws.cloudfront.PublicKey` resource.
*Œq
S

cloudfrontRealtimeLogConfig2aws:cloudfront/realtimeLogConfig:RealtimeLogConfig•dProvides a CloudFront real-time log configuration resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const assumeRole = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        principals: [{
            type: "Service",
            identifiers: ["cloudfront.amazonaws.com"],
        }],
        actions: ["sts:AssumeRole"],
    }],
});
const exampleRole = new aws.iam.Role("example", {
    name: "cloudfront-realtime-log-config-example",
    assumeRolePolicy: assumeRole.then(assumeRole => assumeRole.json),
});
const example = aws.iam.getPolicyDocument({
    statements: [{
        effect: "Allow",
        actions: [
            "kinesis:DescribeStreamSummary",
            "kinesis:DescribeStream",
            "kinesis:PutRecord",
            "kinesis:PutRecords",
        ],
        resources: [exampleAwsKinesisStream.arn],
    }],
});
const exampleRolePolicy = new aws.iam.RolePolicy("example", {
    name: "cloudfront-realtime-log-config-example",
    role: exampleRole.id,
    policy: example.then(example => example.json),
});
const exampleRealtimeLogConfig = new aws.cloudfront.RealtimeLogConfig("example", {
    name: "example",
    samplingRate: 75,
    fields: [
        "timestamp",
        "c-ip",
    ],
    endpoint: {
        streamType: "Kinesis",
        kinesisStreamConfig: {
            roleArn: exampleRole.arn,
            streamArn: exampleAwsKinesisStream.arn,
        },
    },
}, {
    dependsOn: [exampleRolePolicy],
});
```
```python
import pulumi
import pulumi_aws as aws

assume_role = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "principals": [{
        "type": "Service",
        "identifiers": ["cloudfront.amazonaws.com"],
    }],
    "actions": ["sts:AssumeRole"],
}])
example_role = aws.iam.Role("example",
    name="cloudfront-realtime-log-config-example",
    assume_role_policy=assume_role.json)
example = aws.iam.get_policy_document(statements=[{
    "effect": "Allow",
    "actions": [
        "kinesis:DescribeStreamSummary",
        "kinesis:DescribeStream",
        "kinesis:PutRecord",
        "kinesis:PutRecords",
    ],
    "resources": [example_aws_kinesis_stream["arn"]],
}])
example_role_policy = aws.iam.RolePolicy("example",
    name="cloudfront-realtime-log-config-example",
    role=example_role.id,
    policy=example.json)
example_realtime_log_config = aws.cloudfront.RealtimeLogConfig("example",
    name="example",
    sampling_rate=75,
    fields=[
        "timestamp",
        "c-ip",
    ],
    endpoint={
        "stream_type": "Kinesis",
        "kinesis_stream_config": {
            "role_arn": example_role.arn,
            "stream_arn": example_aws_kinesis_stream["arn"],
        },
    },
    opts = pulumi.ResourceOptions(depends_on=[example_role_policy]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
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
                            "cloudfront.amazonaws.com",
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
        Name = "cloudfront-realtime-log-config-example",
        AssumeRolePolicy = assumeRole.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var example = Aws.Iam.GetPolicyDocument.Invoke(new()
    {
        Statements = new[]
        {
            new Aws.Iam.Inputs.GetPolicyDocumentStatementInputArgs
            {
                Effect = "Allow",
                Actions = new[]
                {
                    "kinesis:DescribeStreamSummary",
                    "kinesis:DescribeStream",
                    "kinesis:PutRecord",
                    "kinesis:PutRecords",
                },
                Resources = new[]
                {
                    exampleAwsKinesisStream.Arn,
                },
            },
        },
    });

    var exampleRolePolicy = new Aws.Iam.RolePolicy("example", new()
    {
        Name = "cloudfront-realtime-log-config-example",
        Role = exampleRole.Id,
        Policy = example.Apply(getPolicyDocumentResult => getPolicyDocumentResult.Json),
    });

    var exampleRealtimeLogConfig = new Aws.CloudFront.RealtimeLogConfig("example", new()
    {
        Name = "example",
        SamplingRate = 75,
        Fields = new[]
        {
            "timestamp",
            "c-ip",
        },
        Endpoint = new Aws.CloudFront.Inputs.RealtimeLogConfigEndpointArgs
        {
            StreamType = "Kinesis",
            KinesisStreamConfig = new Aws.CloudFront.Inputs.RealtimeLogConfigEndpointKinesisStreamConfigArgs
            {
                RoleArn = exampleRole.Arn,
                StreamArn = exampleAwsKinesisStream.Arn,
            },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleRolePolicy,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/iam"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)
func main() {
pulumi.Run(func(ctx *pulumi.Context) error {
assumeRole, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
Statements: []iam.GetPolicyDocumentStatement{
{
Effect: pulumi.StringRef("Allow"),
Principals: []iam.GetPolicyDocumentStatementPrincipal{
{
Type: "Service",
Identifiers: []string{
"cloudfront.amazonaws.com",
},
},
},
Actions: []string{
"sts:AssumeRole",
},
},
},
}, nil);
if err != nil {
return err
}
exampleRole, err := iam.NewRole(ctx, "example", &iam.RoleArgs{
Name: pulumi.String("cloudfront-realtime-log-config-example"),
AssumeRolePolicy: pulumi.String(assumeRole.Json),
})
if err != nil {
return err
}
example, err := iam.GetPolicyDocument(ctx, &iam.GetPolicyDocumentArgs{
Statements: []iam.GetPolicyDocumentStatement{
{
Effect: pulumi.StringRef("Allow"),
Actions: []string{
"kinesis:DescribeStreamSummary",
"kinesis:DescribeStream",
"kinesis:PutRecord",
"kinesis:PutRecords",
},
Resources: interface{}{
exampleAwsKinesisStream.Arn,
},
},
},
}, nil);
if err != nil {
return err
}
exampleRolePolicy, err := iam.NewRolePolicy(ctx, "example", &iam.RolePolicyArgs{
Name: pulumi.String("cloudfront-realtime-log-config-example"),
Role: exampleRole.ID(),
Policy: pulumi.String(example.Json),
})
if err != nil {
return err
}
_, err = cloudfront.NewRealtimeLogConfig(ctx, "example", &cloudfront.RealtimeLogConfigArgs{
Name: pulumi.String("example"),
SamplingRate: pulumi.Int(75),
Fields: pulumi.StringArray{
pulumi.String("timestamp"),
pulumi.String("c-ip"),
},
Endpoint: &cloudfront.RealtimeLogConfigEndpointArgs{
StreamType: pulumi.String("Kinesis"),
KinesisStreamConfig: &cloudfront.RealtimeLogConfigEndpointKinesisStreamConfigArgs{
RoleArn: exampleRole.Arn,
StreamArn: pulumi.Any(exampleAwsKinesisStream.Arn),
},
},
}, pulumi.DependsOn([]pulumi.Resource{
exampleRolePolicy,
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
import com.pulumi.aws.iam.IamFunctions;
import com.pulumi.aws.iam.inputs.GetPolicyDocumentArgs;
import com.pulumi.aws.iam.Role;
import com.pulumi.aws.iam.RoleArgs;
import com.pulumi.aws.iam.RolePolicy;
import com.pulumi.aws.iam.RolePolicyArgs;
import com.pulumi.aws.cloudfront.RealtimeLogConfig;
import com.pulumi.aws.cloudfront.RealtimeLogConfigArgs;
import com.pulumi.aws.cloudfront.inputs.RealtimeLogConfigEndpointArgs;
import com.pulumi.aws.cloudfront.inputs.RealtimeLogConfigEndpointKinesisStreamConfigArgs;
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
        final var assumeRole = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .principals(GetPolicyDocumentStatementPrincipalArgs.builder()
                    .type("Service")
                    .identifiers("cloudfront.amazonaws.com")
                    .build())
                .actions("sts:AssumeRole")
                .build())
            .build());

        var exampleRole = new Role("exampleRole", RoleArgs.builder()
            .name("cloudfront-realtime-log-config-example")
            .assumeRolePolicy(assumeRole.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        final var example = IamFunctions.getPolicyDocument(GetPolicyDocumentArgs.builder()
            .statements(GetPolicyDocumentStatementArgs.builder()
                .effect("Allow")
                .actions(                
                    "kinesis:DescribeStreamSummary",
                    "kinesis:DescribeStream",
                    "kinesis:PutRecord",
                    "kinesis:PutRecords")
                .resources(exampleAwsKinesisStream.arn())
                .build())
            .build());

        var exampleRolePolicy = new RolePolicy("exampleRolePolicy", RolePolicyArgs.builder()
            .name("cloudfront-realtime-log-config-example")
            .role(exampleRole.id())
            .policy(example.applyValue(getPolicyDocumentResult -> getPolicyDocumentResult.json()))
            .build());

        var exampleRealtimeLogConfig = new RealtimeLogConfig("exampleRealtimeLogConfig", RealtimeLogConfigArgs.builder()
            .name("example")
            .samplingRate(75)
            .fields(            
                "timestamp",
                "c-ip")
            .endpoint(RealtimeLogConfigEndpointArgs.builder()
                .streamType("Kinesis")
                .kinesisStreamConfig(RealtimeLogConfigEndpointKinesisStreamConfigArgs.builder()
                    .roleArn(exampleRole.arn())
                    .streamArn(exampleAwsKinesisStream.arn())
                    .build())
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleRolePolicy)
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
      name: cloudfront-realtime-log-config-example
      assumeRolePolicy: ${assumeRole.json}
  exampleRolePolicy:
    type: aws:iam:RolePolicy
    name: example
    properties:
      name: cloudfront-realtime-log-config-example
      role: ${exampleRole.id}
      policy: ${example.json}
  exampleRealtimeLogConfig:
    type: aws:cloudfront:RealtimeLogConfig
    name: example
    properties:
      name: example
      samplingRate: 75
      fields:
        - timestamp
        - c-ip
      endpoint:
        streamType: Kinesis
        kinesisStreamConfig:
          roleArn: ${exampleRole.arn}
          streamArn: ${exampleAwsKinesisStream.arn}
    options:
      dependsOn:
        - ${exampleRolePolicy}
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
                  - cloudfront.amazonaws.com
            actions:
              - sts:AssumeRole
  example:
    fn::invoke:
      function: aws:iam:getPolicyDocument
      arguments:
        statements:
          - effect: Allow
            actions:
              - kinesis:DescribeStreamSummary
              - kinesis:DescribeStream
              - kinesis:PutRecord
              - kinesis:PutRecords
            resources:
              - ${exampleAwsKinesisStream.arn}
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import CloudFront real-time log configurations using the ARN. For example:

```sh
$ pulumi import aws:cloudfront/realtimeLogConfig:RealtimeLogConfig example arn:aws:cloudfront::111122223333:realtime-log-config/ExampleNameForRealtimeLogConfig
```
ø
endpointo:m
k

cloudfrontRealtimeLogConfigEndpointBaws:cloudfront/RealtimeLogConfigEndpoint:RealtimeLogConfigEndpointBThe Amazon Kinesis data streams where real-time log data is sent.
˝
fields*" ÏThe fields that are included in each real-time log record. See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields) for supported values.
L
nameB" >The unique name to identify this real-time log configuration.
È
samplingRate ‘The sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. An integer between `1` and `100`, inclusive.
"Y
arn" NThe ARN (Amazon Resource Name) of the CloudFront real-time log configuration.
"ø
endpointo:m
k

cloudfrontRealtimeLogConfigEndpointBaws:cloudfront/RealtimeLogConfigEndpoint:RealtimeLogConfigEndpointBThe Amazon Kinesis data streams where real-time log data is sent.
"˝
fields*" ÏThe fields that are included in each real-time log record. See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields) for supported values.
"J
name" >The unique name to identify this real-time log configuration.
"È
samplingRate ‘The sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. An integer between `1` and `100`, inclusive.
*—©
_

cloudfrontResponseHeadersPolicy:aws:cloudfront/responseHeadersPolicy:ResponseHeadersPolicyÿäProvides a CloudFront response headers policy resource.
A response headers policy contains information about a set of HTTP response headers and their values.
After you create a response headers policy, you can use its ID to attach it to one or more cache behaviors in a CloudFront distribution.
When it‚Äôs attached to a cache behavior, CloudFront adds the headers in the policy to every response that it sends for requests that match the cache behavior.

## Example Usage

The example below creates a CloudFront response headers policy.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudfront.ResponseHeadersPolicy("example", {
    name: "example-policy",
    comment: "test comment",
    corsConfig: {
        accessControlAllowCredentials: true,
        accessControlAllowHeaders: {
            items: ["test"],
        },
        accessControlAllowMethods: {
            items: ["GET"],
        },
        accessControlAllowOrigins: {
            items: ["test.example.comtest"],
        },
        originOverride: true,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.ResponseHeadersPolicy("example",
    name="example-policy",
    comment="test comment",
    cors_config={
        "access_control_allow_credentials": True,
        "access_control_allow_headers": {
            "items": ["test"],
        },
        "access_control_allow_methods": {
            "items": ["GET"],
        },
        "access_control_allow_origins": {
            "items": ["test.example.comtest"],
        },
        "origin_override": True,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFront.ResponseHeadersPolicy("example", new()
    {
        Name = "example-policy",
        Comment = "test comment",
        CorsConfig = new Aws.CloudFront.Inputs.ResponseHeadersPolicyCorsConfigArgs
        {
            AccessControlAllowCredentials = true,
            AccessControlAllowHeaders = new Aws.CloudFront.Inputs.ResponseHeadersPolicyCorsConfigAccessControlAllowHeadersArgs
            {
                Items = new[]
                {
                    "test",
                },
            },
            AccessControlAllowMethods = new Aws.CloudFront.Inputs.ResponseHeadersPolicyCorsConfigAccessControlAllowMethodsArgs
            {
                Items = new[]
                {
                    "GET",
                },
            },
            AccessControlAllowOrigins = new Aws.CloudFront.Inputs.ResponseHeadersPolicyCorsConfigAccessControlAllowOriginsArgs
            {
                Items = new[]
                {
                    "test.example.comtest",
                },
            },
            OriginOverride = true,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewResponseHeadersPolicy(ctx, "example", &cloudfront.ResponseHeadersPolicyArgs{
			Name:    pulumi.String("example-policy"),
			Comment: pulumi.String("test comment"),
			CorsConfig: &cloudfront.ResponseHeadersPolicyCorsConfigArgs{
				AccessControlAllowCredentials: pulumi.Bool(true),
				AccessControlAllowHeaders: &cloudfront.ResponseHeadersPolicyCorsConfigAccessControlAllowHeadersArgs{
					Items: pulumi.StringArray{
						pulumi.String("test"),
					},
				},
				AccessControlAllowMethods: &cloudfront.ResponseHeadersPolicyCorsConfigAccessControlAllowMethodsArgs{
					Items: pulumi.StringArray{
						pulumi.String("GET"),
					},
				},
				AccessControlAllowOrigins: &cloudfront.ResponseHeadersPolicyCorsConfigAccessControlAllowOriginsArgs{
					Items: pulumi.StringArray{
						pulumi.String("test.example.comtest"),
					},
				},
				OriginOverride: pulumi.Bool(true),
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
import com.pulumi.aws.cloudfront.ResponseHeadersPolicy;
import com.pulumi.aws.cloudfront.ResponseHeadersPolicyArgs;
import com.pulumi.aws.cloudfront.inputs.ResponseHeadersPolicyCorsConfigArgs;
import com.pulumi.aws.cloudfront.inputs.ResponseHeadersPolicyCorsConfigAccessControlAllowHeadersArgs;
import com.pulumi.aws.cloudfront.inputs.ResponseHeadersPolicyCorsConfigAccessControlAllowMethodsArgs;
import com.pulumi.aws.cloudfront.inputs.ResponseHeadersPolicyCorsConfigAccessControlAllowOriginsArgs;
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
        var example = new ResponseHeadersPolicy("example", ResponseHeadersPolicyArgs.builder()
            .name("example-policy")
            .comment("test comment")
            .corsConfig(ResponseHeadersPolicyCorsConfigArgs.builder()
                .accessControlAllowCredentials(true)
                .accessControlAllowHeaders(ResponseHeadersPolicyCorsConfigAccessControlAllowHeadersArgs.builder()
                    .items("test")
                    .build())
                .accessControlAllowMethods(ResponseHeadersPolicyCorsConfigAccessControlAllowMethodsArgs.builder()
                    .items("GET")
                    .build())
                .accessControlAllowOrigins(ResponseHeadersPolicyCorsConfigAccessControlAllowOriginsArgs.builder()
                    .items("test.example.comtest")
                    .build())
                .originOverride(true)
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:ResponseHeadersPolicy
    properties:
      name: example-policy
      comment: test comment
      corsConfig:
        accessControlAllowCredentials: true
        accessControlAllowHeaders:
          items:
            - test
        accessControlAllowMethods:
          items:
            - GET
        accessControlAllowOrigins:
          items:
            - test.example.comtest
        originOverride: true
```
<!--End PulumiCodeChooser -->

The example below creates a CloudFront response headers policy with a custom headers config.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudfront.ResponseHeadersPolicy("example", {
    name: "example-headers-policy",
    customHeadersConfig: {
        items: [
            {
                header: "X-Permitted-Cross-Domain-Policies",
                override: true,
                value: "none",
            },
            {
                header: "X-Test",
                override: true,
                value: "none",
            },
        ],
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.ResponseHeadersPolicy("example",
    name="example-headers-policy",
    custom_headers_config={
        "items": [
            {
                "header": "X-Permitted-Cross-Domain-Policies",
                "override": True,
                "value": "none",
            },
            {
                "header": "X-Test",
                "override": True,
                "value": "none",
            },
        ],
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFront.ResponseHeadersPolicy("example", new()
    {
        Name = "example-headers-policy",
        CustomHeadersConfig = new Aws.CloudFront.Inputs.ResponseHeadersPolicyCustomHeadersConfigArgs
        {
            Items = new[]
            {
                new Aws.CloudFront.Inputs.ResponseHeadersPolicyCustomHeadersConfigItemArgs
                {
                    Header = "X-Permitted-Cross-Domain-Policies",
                    Override = true,
                    Value = "none",
                },
                new Aws.CloudFront.Inputs.ResponseHeadersPolicyCustomHeadersConfigItemArgs
                {
                    Header = "X-Test",
                    Override = true,
                    Value = "none",
                },
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewResponseHeadersPolicy(ctx, "example", &cloudfront.ResponseHeadersPolicyArgs{
			Name: pulumi.String("example-headers-policy"),
			CustomHeadersConfig: &cloudfront.ResponseHeadersPolicyCustomHeadersConfigArgs{
				Items: cloudfront.ResponseHeadersPolicyCustomHeadersConfigItemArray{
					&cloudfront.ResponseHeadersPolicyCustomHeadersConfigItemArgs{
						Header:   pulumi.String("X-Permitted-Cross-Domain-Policies"),
						Override: pulumi.Bool(true),
						Value:    pulumi.String("none"),
					},
					&cloudfront.ResponseHeadersPolicyCustomHeadersConfigItemArgs{
						Header:   pulumi.String("X-Test"),
						Override: pulumi.Bool(true),
						Value:    pulumi.String("none"),
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
import com.pulumi.aws.cloudfront.ResponseHeadersPolicy;
import com.pulumi.aws.cloudfront.ResponseHeadersPolicyArgs;
import com.pulumi.aws.cloudfront.inputs.ResponseHeadersPolicyCustomHeadersConfigArgs;
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
        var example = new ResponseHeadersPolicy("example", ResponseHeadersPolicyArgs.builder()
            .name("example-headers-policy")
            .customHeadersConfig(ResponseHeadersPolicyCustomHeadersConfigArgs.builder()
                .items(                
                    ResponseHeadersPolicyCustomHeadersConfigItemArgs.builder()
                        .header("X-Permitted-Cross-Domain-Policies")
                        .override(true)
                        .value("none")
                        .build(),
                    ResponseHeadersPolicyCustomHeadersConfigItemArgs.builder()
                        .header("X-Test")
                        .override(true)
                        .value("none")
                        .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:ResponseHeadersPolicy
    properties:
      name: example-headers-policy
      customHeadersConfig:
        items:
          - header: X-Permitted-Cross-Domain-Policies
            override: true
            value: none
          - header: X-Test
            override: true
            value: none
```
<!--End PulumiCodeChooser -->

The example below creates a CloudFront response headers policy with a custom headers config, remove headers config and server timing headers config.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = new aws.cloudfront.ResponseHeadersPolicy("example", {
    name: "example-headers-policy",
    customHeadersConfig: {
        items: [{
            header: "X-Permitted-Cross-Domain-Policies",
            override: true,
            value: "none",
        }],
    },
    removeHeadersConfig: {
        items: [{
            header: "Set-Cookie",
        }],
    },
    serverTimingHeadersConfig: {
        enabled: true,
        samplingRate: 50,
    },
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.ResponseHeadersPolicy("example",
    name="example-headers-policy",
    custom_headers_config={
        "items": [{
            "header": "X-Permitted-Cross-Domain-Policies",
            "override": True,
            "value": "none",
        }],
    },
    remove_headers_config={
        "items": [{
            "header": "Set-Cookie",
        }],
    },
    server_timing_headers_config={
        "enabled": True,
        "sampling_rate": 50,
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = new Aws.CloudFront.ResponseHeadersPolicy("example", new()
    {
        Name = "example-headers-policy",
        CustomHeadersConfig = new Aws.CloudFront.Inputs.ResponseHeadersPolicyCustomHeadersConfigArgs
        {
            Items = new[]
            {
                new Aws.CloudFront.Inputs.ResponseHeadersPolicyCustomHeadersConfigItemArgs
                {
                    Header = "X-Permitted-Cross-Domain-Policies",
                    Override = true,
                    Value = "none",
                },
            },
        },
        RemoveHeadersConfig = new Aws.CloudFront.Inputs.ResponseHeadersPolicyRemoveHeadersConfigArgs
        {
            Items = new[]
            {
                new Aws.CloudFront.Inputs.ResponseHeadersPolicyRemoveHeadersConfigItemArgs
                {
                    Header = "Set-Cookie",
                },
            },
        },
        ServerTimingHeadersConfig = new Aws.CloudFront.Inputs.ResponseHeadersPolicyServerTimingHeadersConfigArgs
        {
            Enabled = true,
            SamplingRate = 50,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewResponseHeadersPolicy(ctx, "example", &cloudfront.ResponseHeadersPolicyArgs{
			Name: pulumi.String("example-headers-policy"),
			CustomHeadersConfig: &cloudfront.ResponseHeadersPolicyCustomHeadersConfigArgs{
				Items: cloudfront.ResponseHeadersPolicyCustomHeadersConfigItemArray{
					&cloudfront.ResponseHeadersPolicyCustomHeadersConfigItemArgs{
						Header:   pulumi.String("X-Permitted-Cross-Domain-Policies"),
						Override: pulumi.Bool(true),
						Value:    pulumi.String("none"),
					},
				},
			},
			RemoveHeadersConfig: &cloudfront.ResponseHeadersPolicyRemoveHeadersConfigArgs{
				Items: cloudfront.ResponseHeadersPolicyRemoveHeadersConfigItemArray{
					&cloudfront.ResponseHeadersPolicyRemoveHeadersConfigItemArgs{
						Header: pulumi.String("Set-Cookie"),
					},
				},
			},
			ServerTimingHeadersConfig: &cloudfront.ResponseHeadersPolicyServerTimingHeadersConfigArgs{
				Enabled:      pulumi.Bool(true),
				SamplingRate: pulumi.Float64(50),
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
import com.pulumi.aws.cloudfront.ResponseHeadersPolicy;
import com.pulumi.aws.cloudfront.ResponseHeadersPolicyArgs;
import com.pulumi.aws.cloudfront.inputs.ResponseHeadersPolicyCustomHeadersConfigArgs;
import com.pulumi.aws.cloudfront.inputs.ResponseHeadersPolicyRemoveHeadersConfigArgs;
import com.pulumi.aws.cloudfront.inputs.ResponseHeadersPolicyServerTimingHeadersConfigArgs;
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
        var example = new ResponseHeadersPolicy("example", ResponseHeadersPolicyArgs.builder()
            .name("example-headers-policy")
            .customHeadersConfig(ResponseHeadersPolicyCustomHeadersConfigArgs.builder()
                .items(ResponseHeadersPolicyCustomHeadersConfigItemArgs.builder()
                    .header("X-Permitted-Cross-Domain-Policies")
                    .override(true)
                    .value("none")
                    .build())
                .build())
            .removeHeadersConfig(ResponseHeadersPolicyRemoveHeadersConfigArgs.builder()
                .items(ResponseHeadersPolicyRemoveHeadersConfigItemArgs.builder()
                    .header("Set-Cookie")
                    .build())
                .build())
            .serverTimingHeadersConfig(ResponseHeadersPolicyServerTimingHeadersConfigArgs.builder()
                .enabled(true)
                .samplingRate(50)
                .build())
            .build());

    }
}
```
```yaml
resources:
  example:
    type: aws:cloudfront:ResponseHeadersPolicy
    properties:
      name: example-headers-policy
      customHeadersConfig:
        items:
          - header: X-Permitted-Cross-Domain-Policies
            override: true
            value: none
      removeHeadersConfig:
        items:
          - header: Set-Cookie
      serverTimingHeadersConfig:
        enabled: true
        samplingRate: 50
```
<!--End PulumiCodeChooser -->

## Import

Using `pulumi import`, import Cloudfront Response Headers Policies using the `id`. For example:

```sh
$ pulumi import aws:cloudfront/responseHeadersPolicy:ResponseHeadersPolicy policy 658327ea-f89d-4fab-a63d-7e88639e58f9
```
v
commentB" eA comment to describe the response headers policy. The comment cannot be longer than 128 characters.
ß

corsConfigÑBÅ:
}

cloudfrontResponseHeadersPolicyCorsConfigNaws:cloudfront/ResponseHeadersPolicyCorsConfig:ResponseHeadersPolicyCorsConfigëA configuration for a set of HTTP response headers that are used for Cross-Origin Resource Sharing (CORS). See Cors Config for more information.
µ
customHeadersConfig°Bû:õ
ò

cloudfront(ResponseHeadersPolicyCustomHeadersConfig`aws:cloudfront/ResponseHeadersPolicyCustomHeadersConfig:ResponseHeadersPolicyCustomHeadersConfigzObject that contains an attribute `items` that contains a list of custom headers. See Custom Header for more information.
B
etagB" 4The current version of the response headers policy.
E
nameB" 7A unique name to identify the response headers policy.
˚
removeHeadersConfig°Bû:õ
ò

cloudfront(ResponseHeadersPolicyRemoveHeadersConfig`aws:cloudfront/ResponseHeadersPolicyRemoveHeadersConfig:ResponseHeadersPolicyRemoveHeadersConfigøA configuration for a set of HTTP headers to remove from the HTTP response. Object that contains an attribute `items` that contains a list of headers. See Remove Header for more information.
∫
securityHeadersConfigßB§:°
û

cloudfront*ResponseHeadersPolicySecurityHeadersConfigdaws:cloudfront/ResponseHeadersPolicySecurityHeadersConfig:ResponseHeadersPolicySecurityHeadersConfigwA configuration for a set of security-related HTTP response headers. See Security Headers Config for more information.
È
serverTimingHeadersConfig≥B∞:≠
™

cloudfront.ResponseHeadersPolicyServerTimingHeadersConfiglaws:cloudfront/ResponseHeadersPolicyServerTimingHeadersConfig:ResponseHeadersPolicyServerTimingHeadersConfigïA configuration for enabling the Server-Timing header in HTTP responses sent from CloudFront. See Server Timing Headers Config for more information.
"v
commentB" eA comment to describe the response headers policy. The comment cannot be longer than 128 characters.
"ß

corsConfigÑBÅ:
}

cloudfrontResponseHeadersPolicyCorsConfigNaws:cloudfront/ResponseHeadersPolicyCorsConfig:ResponseHeadersPolicyCorsConfigëA configuration for a set of HTTP response headers that are used for Cross-Origin Resource Sharing (CORS). See Cors Config for more information.
"µ
customHeadersConfig°Bû:õ
ò

cloudfront(ResponseHeadersPolicyCustomHeadersConfig`aws:cloudfront/ResponseHeadersPolicyCustomHeadersConfig:ResponseHeadersPolicyCustomHeadersConfigzObject that contains an attribute `items` that contains a list of custom headers. See Custom Header for more information.
"@
etag" 4The current version of the response headers policy.
"C
name" 7A unique name to identify the response headers policy.
"˚
removeHeadersConfig°Bû:õ
ò

cloudfront(ResponseHeadersPolicyRemoveHeadersConfig`aws:cloudfront/ResponseHeadersPolicyRemoveHeadersConfig:ResponseHeadersPolicyRemoveHeadersConfigøA configuration for a set of HTTP headers to remove from the HTTP response. Object that contains an attribute `items` that contains a list of headers. See Remove Header for more information.
"∫
securityHeadersConfigßB§:°
û

cloudfront*ResponseHeadersPolicySecurityHeadersConfigdaws:cloudfront/ResponseHeadersPolicySecurityHeadersConfig:ResponseHeadersPolicySecurityHeadersConfigwA configuration for a set of security-related HTTP response headers. See Security Headers Config for more information.
"È
serverTimingHeadersConfig≥B∞:≠
™

cloudfront.ResponseHeadersPolicyServerTimingHeadersConfiglaws:cloudfront/ResponseHeadersPolicyServerTimingHeadersConfig:ResponseHeadersPolicyServerTimingHeadersConfigïA configuration for enabling the Server-Timing header in HTTP responses sent from CloudFront. See Server Timing Headers Config for more information.
*∏+
;

cloudfront	VpcOrigin"aws:cloudfront/vpcOrigin:VpcOrigin‹$Creates an Amazon CloudFront VPC origin.

For information about CloudFront VPC origins, see
[Amazon CloudFront Developer Guide - Restrict access with VPC origins][1].

## Example Usage

### Application Load Balancer

The following example below creates a CloudFront VPC origin for a Application Load Balancer.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const alb = new aws.cloudfront.VpcOrigin("alb", {vpcOriginEndpointConfig: {
    name: "Example VPC Origin",
    arn: _this.arn,
    httpPort: 8080,
    httpsPort: 8443,
    originProtocolPolicy: "https-only",
    originSslProtocols: {
        items: ["TLSv1.2"],
        quantity: 1,
    },
}});
```
```python
import pulumi
import pulumi_aws as aws

alb = aws.cloudfront.VpcOrigin("alb", vpc_origin_endpoint_config={
    "name": "Example VPC Origin",
    "arn": this["arn"],
    "http_port": 8080,
    "https_port": 8443,
    "origin_protocol_policy": "https-only",
    "origin_ssl_protocols": {
        "items": ["TLSv1.2"],
        "quantity": 1,
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
    var alb = new Aws.CloudFront.VpcOrigin("alb", new()
    {
        VpcOriginEndpointConfig = new Aws.CloudFront.Inputs.VpcOriginVpcOriginEndpointConfigArgs
        {
            Name = "Example VPC Origin",
            Arn = @this.Arn,
            HttpPort = 8080,
            HttpsPort = 8443,
            OriginProtocolPolicy = "https-only",
            OriginSslProtocols = new Aws.CloudFront.Inputs.VpcOriginVpcOriginEndpointConfigOriginSslProtocolsArgs
            {
                Items = new[]
                {
                    "TLSv1.2",
                },
                Quantity = 1,
            },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.NewVpcOrigin(ctx, "alb", &cloudfront.VpcOriginArgs{
			VpcOriginEndpointConfig: &cloudfront.VpcOriginVpcOriginEndpointConfigArgs{
				Name:                 pulumi.String("Example VPC Origin"),
				Arn:                  pulumi.Any(this.Arn),
				HttpPort:             pulumi.Int(8080),
				HttpsPort:            pulumi.Int(8443),
				OriginProtocolPolicy: pulumi.String("https-only"),
				OriginSslProtocols: &cloudfront.VpcOriginVpcOriginEndpointConfigOriginSslProtocolsArgs{
					Items: pulumi.StringArray{
						pulumi.String("TLSv1.2"),
					},
					Quantity: pulumi.Int(1),
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
import com.pulumi.aws.cloudfront.VpcOrigin;
import com.pulumi.aws.cloudfront.VpcOriginArgs;
import com.pulumi.aws.cloudfront.inputs.VpcOriginVpcOriginEndpointConfigArgs;
import com.pulumi.aws.cloudfront.inputs.VpcOriginVpcOriginEndpointConfigOriginSslProtocolsArgs;
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
        var alb = new VpcOrigin("alb", VpcOriginArgs.builder()
            .vpcOriginEndpointConfig(VpcOriginVpcOriginEndpointConfigArgs.builder()
                .name("Example VPC Origin")
                .arn(this_.arn())
                .httpPort(8080)
                .httpsPort(8443)
                .originProtocolPolicy("https-only")
                .originSslProtocols(VpcOriginVpcOriginEndpointConfigOriginSslProtocolsArgs.builder()
                    .items("TLSv1.2")
                    .quantity(1)
                    .build())
                .build())
            .build());

    }
}
```
```yaml
resources:
  alb:
    type: aws:cloudfront:VpcOrigin
    properties:
      vpcOriginEndpointConfig:
        name: Example VPC Origin
        arn: ${this.arn}
        httpPort: 8080
        httpsPort: 8443
        originProtocolPolicy: https-only
        originSslProtocols:
          items:
            - TLSv1.2
          quantity: 1
```
<!--End PulumiCodeChooser -->

## Import

terraform

import {

  to = aws_cloudfront_vpc_origin.origin

  id = vo_JQEa410sssUFoY6wMkx69j

}

Using `pulumi import`, import Cloudfront VPC origins using the `id`. For example:

console

% pulumi import aws_cloudfront_vpc_origin vo_JQEa410sssUFoY6wMkx69j


tagsB2" e
timeoutsYBW:U
S

cloudfrontVpcOriginTimeouts2aws:cloudfront/VpcOriginTimeouts:VpcOriginTimeouts•
vpcOriginEndpointConfigâBÜ:É
Ä

cloudfront VpcOriginVpcOriginEndpointConfigPaws:cloudfront/VpcOriginVpcOriginEndpointConfig:VpcOriginVpcOriginEndpointConfig"
arn" The VPC origin ARN.
"/
etag" #The current version of the origin.
"
tagsB2" "â
tagsAll2" xA map of tags assigned to the resource, including those inherited from the provider `default_tags` configuration block.
"e
timeoutsYBW:U
S

cloudfrontVpcOriginTimeouts2aws:cloudfront/VpcOriginTimeouts:VpcOriginTimeouts"•
vpcOriginEndpointConfigâBÜ:É
Ä

cloudfront VpcOriginVpcOriginEndpointConfigPaws:cloudfront/VpcOriginVpcOriginEndpointConfig:VpcOriginVpcOriginEndpointConfig2¯
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
2˜'
5
budgets	getBudgetaws:budgets/getBudget:getBudgetÍData source for managing an AWS Web Services Budgets Budget.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.budgets.getBudget({
    name: testAwsBudgetsBudget.name,
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.budgets.get_budget(name=test_aws_budgets_budget["name"])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.Budgets.GetBudget.Invoke(new()
    {
        Name = testAwsBudgetsBudget.Name,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/budgets"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := budgets.LookupBudget(ctx, &budgets.LookupBudgetArgs{
			Name: testAwsBudgetsBudget.Name,
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
import com.pulumi.aws.budgets.BudgetsFunctions;
import com.pulumi.aws.budgets.inputs.GetBudgetArgs;
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
        final var test = BudgetsFunctions.getBudget(GetBudgetArgs.builder()
            .name(testAwsBudgetsBudget.name())
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:budgets:getBudget
      arguments:
        name: ${testAwsBudgetsBudget.name}
```
<!--End PulumiCodeChooser -->
v
	accountIdB" cThe ID of the target account for budget. Will use current user's account_id by default if omitted.
a
name" UThe name of a budget. Unique within accounts.

The following arguments are optional:
P

namePrefixB" <The prefix of the name of a budget. Unique within accounts.
6
tagsB2" &Map of tags assigned to the resource.
"
	accountId" "	
arn" "ﬁ
autoAdjustDatase*c:a
_
budgetsgetBudgetAutoAdjustData;aws:budgets/getBudgetAutoAdjustData:getBudgetAutoAdjustDatadObject containing [AutoAdjustData] which determines the budget amount for an auto-adjusting budget.
"P
budgetExceeded
 :Boolean indicating whether this budget has been exceeded.
"¢
budgetLimits\*Z:X
V
budgetsgetBudgetBudgetLimit5aws:budgets/getBudgetBudgetLimit:getBudgetBudgetLimit≥The total amount of cost, usage, RI utilization, RI coverage, Savings Plans utilization, or Savings Plans coverage that you want to track with your budget. Contains object Spend.
"E

budgetType" 3Whether this budget tracks monetary cost or usage.
"Å
calculatedSpendsh*f:d
b
budgetsgetBudgetCalculatedSpend=aws:budgets/getBudgetCalculatedSpend:getBudgetCalculatedSpendÇThe spend objects that are associated with this budget. The actualSpend tracks how much you've used, cost, usage, RI units, or Savings Plans units and the forecastedSpend tracks how much that you're predicted to spend based on your historical usage profile.
"§
costFiltersY*W:U
S
budgetsgetBudgetCostFilter3aws:budgets/getBudgetCostFilter:getBudgetCostFilter:A list of CostFilter name/values pair to apply to budget.
"≈
	costTypesS*Q:O
M
budgetsgetBudgetCostType/aws:budgets/getBudgetCostType:getBudgetCostTypecObject containing CostTypes The types of cost included in a budget, such as tax and subscriptions.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "

namePrefixB" "‚
notifications_*]:[
Y
budgetsgetBudgetNotification7aws:budgets/getBudgetNotification:getBudgetNotificationpObject containing Budget Notifications. Can be used multiple times to define more than one budget notification.
"õ
plannedLimits_*]:[
Y
budgetsgetBudgetPlannedLimit7aws:budgets/getBudgetPlannedLimit:getBudgetPlannedLimit®Object containing Planned Budget Limits. Can be used multiple times to plan more than one budget limit. See [PlannedBudgetLimits](https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_Budget.html#awscostmanagement-Type-budgets_Budget-PlannedBudgetLimits) documentation.
"4
tags2" &Map of tags assigned to the resource.
"é
timePeriodEnd" yThe end of the time period covered by the budget. There are no restrictions on the end date. Format: `2017-01-01_12:00`.
"Ô
timePeriodStart" ◊The start of the time period covered by the budget. If you don't specify a start date, AWS defaults to the start of your chosen time period. The start date must come before the end date. Format: `2017-01-01_12:00`.
"ö
timeUnit" âThe length of time until a budget resets the actual and forecasted spend. Valid values: `MONTHLY`, `QUARTERLY`, `ANNUALLY`, and `DAILY`.
2™
M
chatbotgetSlackWorkspace/aws:chatbot/getSlackWorkspace:getSlackWorkspaceÎData source for managing an AWS Chatbot Slack Workspace.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.chatbot.getSlackWorkspace({
    slackTeamName: "abc",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.chatbot.get_slack_workspace(slack_team_name="abc")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.Chatbot.GetSlackWorkspace.Invoke(new()
    {
        SlackTeamName = "abc",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/chatbot"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := chatbot.GetSlackWorkspace(ctx, &chatbot.GetSlackWorkspaceArgs{
			SlackTeamName: "abc",
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
import com.pulumi.aws.chatbot.ChatbotFunctions;
import com.pulumi.aws.chatbot.inputs.GetSlackWorkspaceArgs;
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
        final var example = ChatbotFunctions.getSlackWorkspace(GetSlackWorkspaceArgs.builder()
            .slackTeamName("abc")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:chatbot:getSlackWorkspace
      arguments:
        slackTeamName: abc
```
<!--End PulumiCodeChooser -->
G
slackTeamName" 2Slack workspace name configured with AWS Chatbot.
"E
id" ;The provider-assigned unique ID for this managed resource.
"F
slackTeamId" 3ID of the Slack Workspace assigned by AWS Chatbot.
"
slackTeamName" 2Ÿ
E
cloudcontrolgetResource(aws:cloudcontrol/getResource:getResource§Provides details for a Cloud Control API Resource. The reading of these resources is proxied through Cloud Control API handlers to the backend service.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cloudcontrol.getResource({
    identifier: "example",
    typeName: "AWS::ECS::Cluster",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudcontrol.get_resource(identifier="example",
    type_name="AWS::ECS::Cluster")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CloudControl.GetResource.Invoke(new()
    {
        Identifier = "example",
        TypeName = "AWS::ECS::Cluster",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudcontrol"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudcontrol.LookupResource(ctx, &cloudcontrol.LookupResourceArgs{
			Identifier: "example",
			TypeName:   "AWS::ECS::Cluster",
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
import com.pulumi.aws.cloudcontrol.CloudcontrolFunctions;
import com.pulumi.aws.cloudcontrol.inputs.GetResourceArgs;
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
        final var example = CloudcontrolFunctions.getResource(GetResourceArgs.builder()
            .identifier("example")
            .typeName("AWS::ECS::Cluster")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cloudcontrol:getResource
      arguments:
        identifier: example
        typeName: AWS::ECS::Cluster
```
<!--End PulumiCodeChooser -->
_

identifier" MIdentifier of the CloudFormation resource type. For example, `vpc-12345678`.
?
roleArnB" .ARN of the IAM Role to assume for operations.
x
typeName" hCloudFormation resource type name. For example, `AWS::EC2::VPC`.

The following arguments are optional:
O
typeVersionIdB" 8Identifier of the CloudFormation resource type version.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

identifier" "k

properties" YJSON string matching the CloudFormation resource type schema with current configuration.
"
roleArnB" "
typeName" "
typeVersionIdB" 2‰
g
cloudformationgetCloudFormationType>aws:cloudformation/getCloudFormationType:getCloudFormationType“Provides details about a CloudFormation Type.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cloudformation.getCloudFormationType({
    type: "RESOURCE",
    typeName: "AWS::Athena::WorkGroup",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudformation.get_cloud_formation_type(type="RESOURCE",
    type_name="AWS::Athena::WorkGroup")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CloudFormation.GetCloudFormationType.Invoke(new()
    {
        Type = "RESOURCE",
        TypeName = "AWS::Athena::WorkGroup",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudformation"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudformation.LookupCloudFormationType(ctx, &cloudformation.LookupCloudFormationTypeArgs{
			Type:     pulumi.StringRef("RESOURCE"),
			TypeName: pulumi.StringRef("AWS::Athena::WorkGroup"),
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
import com.pulumi.aws.cloudformation.CloudformationFunctions;
import com.pulumi.aws.cloudformation.inputs.GetCloudFormationTypeArgs;
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
        final var example = CloudformationFunctions.getCloudFormationType(GetCloudFormationTypeArgs.builder()
            .type("RESOURCE")
            .typeName("AWS::Athena::WorkGroup")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cloudformation:getCloudFormationType
      arguments:
        type: RESOURCE
        typeName: AWS::Athena::WorkGroup
```
<!--End PulumiCodeChooser -->
y
arnB" lARN of the CloudFormation Type. For example, `arn:aws:cloudformation:us-west-2::type/resource/AWS-EC2-VPC`.
E
typeB" 7CloudFormation Registry Type. For example, `RESOURCE`.
J
typeNameB" 8CloudFormation Type name. For example, `AWS::EC2::VPC`.
B
	versionIdB" /Identifier of the CloudFormation Type version.
"	
arn" "O
defaultVersionId" 7Identifier of the CloudFormation Type default version.
"G
deprecatedStatus" /Deprecation status of the CloudFormation Type.
";
description" (Description of the CloudFormation Type.
"N
documentationUrl" 6URL of the documentation for the CloudFormation Type.
"V
executionRoleArn" >ARN of the IAM Role used to register the CloudFormation Type.
"E
id" ;The provider-assigned unique ID for this managed resource.
"X
isDefaultVersion
 @Whether the CloudFormation Type version is the default version.
"ﬁ
loggingConfigsó*î:ë
é
cloudformation"getCloudFormationTypeLoggingConfigXaws:cloudformation/getCloudFormationTypeLoggingConfig:getCloudFormationTypeLoggingConfig2List of objects containing logging configuration.
"J
provisioningType" 2Provisioning behavior of the CloudFormation Type.
"?
schema" 1JSON document of the CloudFormation Type schema.
"E
	sourceUrl" 4URL of the source code for the CloudFormation Type.
"

type" "
typeArn" "
typeName" "
	versionIdB" "4

visibility" "Scope of the CloudFormation Type.
2Ù 
C
cloudformation	getExport&aws:cloudformation/getExport:getExportõThe CloudFormation Export data source allows access to stack
exports specified in the [Output](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/outputs-section-structure.html) section of the Cloudformation Template using the optional Export Property.

 > Note: If you are trying to use a value from a Cloudformation Stack in the same deployment please use normal interpolation or Cloudformation Outputs.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const subnetId = aws.cloudformation.getExport({
    name: "mySubnetIdExportName",
});
const web = new aws.ec2.Instance("web", {
    ami: "ami-abb07bcb",
    instanceType: aws.ec2.InstanceType.T2_Micro,
    subnetId: subnetId.then(subnetId => subnetId.value),
});
```
```python
import pulumi
import pulumi_aws as aws

subnet_id = aws.cloudformation.get_export(name="mySubnetIdExportName")
web = aws.ec2.Instance("web",
    ami="ami-abb07bcb",
    instance_type=aws.ec2.InstanceType.T2_MICRO,
    subnet_id=subnet_id.value)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var subnetId = Aws.CloudFormation.GetExport.Invoke(new()
    {
        Name = "mySubnetIdExportName",
    });

    var web = new Aws.Ec2.Instance("web", new()
    {
        Ami = "ami-abb07bcb",
        InstanceType = Aws.Ec2.InstanceType.T2_Micro,
        SubnetId = subnetId.Apply(getExportResult => getExportResult.Value),
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudformation"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		subnetId, err := cloudformation.GetExport(ctx, &cloudformation.GetExportArgs{
			Name: "mySubnetIdExportName",
		}, nil)
		if err != nil {
			return err
		}
		_, err = ec2.NewInstance(ctx, "web", &ec2.InstanceArgs{
			Ami:          pulumi.String("ami-abb07bcb"),
			InstanceType: pulumi.String(ec2.InstanceType_T2_Micro),
			SubnetId:     pulumi.String(subnetId.Value),
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
import com.pulumi.aws.cloudformation.CloudformationFunctions;
import com.pulumi.aws.cloudformation.inputs.GetExportArgs;
import com.pulumi.aws.ec2.Instance;
import com.pulumi.aws.ec2.InstanceArgs;
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
        final var subnetId = CloudformationFunctions.getExport(GetExportArgs.builder()
            .name("mySubnetIdExportName")
            .build());

        var web = new Instance("web", InstanceArgs.builder()
            .ami("ami-abb07bcb")
            .instanceType("t2.micro")
            .subnetId(subnetId.applyValue(getExportResult -> getExportResult.value()))
            .build());

    }
}
```
```yaml
resources:
  web:
    type: aws:ec2:Instance
    properties:
      ami: ami-abb07bcb
      instanceType: t2.micro
      subnetId: ${subnetId.value}
variables:
  subnetId:
    fn::invoke:
      function: aws:cloudformation:getExport
      arguments:
        name: mySubnetIdExportName
```
<!--End PulumiCodeChooser -->
¶
name" ôName of the export as it appears in the console or from [list-exports](http://docs.aws.amazon.com/cli/latest/reference/cloudformation/list-exports.html)
"W
exportingStackId" ?ARN of stack that contains the exported output name and value.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "π
value" ´Value from Cloudformation export identified by the export name found from [list-exports](http://docs.aws.amazon.com/cli/latest/reference/cloudformation/list-exports.html)
2˚#
@
cloudformationgetStack$aws:cloudformation/getStack:getStack‘The CloudFormation Stack data source allows access to stack
outputs and other useful data including the template body.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const network = aws.cloudformation.getStack({
    name: "my-network-stack",
});
const web = new aws.ec2.Instance("web", {
    ami: "ami-abb07bcb",
    instanceType: aws.ec2.InstanceType.T2_Micro,
    subnetId: network.then(network => network.outputs?.SubnetId),
    tags: {
        Name: "HelloWorld",
    },
});
```
```python
import pulumi
import pulumi_aws as aws

network = aws.cloudformation.get_stack(name="my-network-stack")
web = aws.ec2.Instance("web",
    ami="ami-abb07bcb",
    instance_type=aws.ec2.InstanceType.T2_MICRO,
    subnet_id=network.outputs["SubnetId"],
    tags={
        "Name": "HelloWorld",
    })
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var network = Aws.CloudFormation.GetStack.Invoke(new()
    {
        Name = "my-network-stack",
    });

    var web = new Aws.Ec2.Instance("web", new()
    {
        Ami = "ami-abb07bcb",
        InstanceType = Aws.Ec2.InstanceType.T2_Micro,
        SubnetId = network.Apply(getStackResult => getStackResult.Outputs?.SubnetId),
        Tags = 
        {
            { "Name", "HelloWorld" },
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudformation"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/ec2"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		network, err := cloudformation.LookupStack(ctx, &cloudformation.LookupStackArgs{
			Name: "my-network-stack",
		}, nil)
		if err != nil {
			return err
		}
		_, err = ec2.NewInstance(ctx, "web", &ec2.InstanceArgs{
			Ami:          pulumi.String("ami-abb07bcb"),
			InstanceType: pulumi.String(ec2.InstanceType_T2_Micro),
			SubnetId:     pulumi.String(network.Outputs.SubnetId),
			Tags: pulumi.StringMap{
				"Name": pulumi.String("HelloWorld"),
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
import com.pulumi.aws.cloudformation.CloudformationFunctions;
import com.pulumi.aws.cloudformation.inputs.GetStackArgs;
import com.pulumi.aws.ec2.Instance;
import com.pulumi.aws.ec2.InstanceArgs;
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
        final var network = CloudformationFunctions.getStack(GetStackArgs.builder()
            .name("my-network-stack")
            .build());

        var web = new Instance("web", InstanceArgs.builder()
            .ami("ami-abb07bcb")
            .instanceType("t2.micro")
            .subnetId(network.applyValue(getStackResult -> getStackResult.outputs().SubnetId()))
            .tags(Map.of("Name", "HelloWorld"))
            .build());

    }
}
```
```yaml
resources:
  web:
    type: aws:ec2:Instance
    properties:
      ami: ami-abb07bcb
      instanceType: t2.micro
      subnetId: ${network.outputs.SubnetId}
      tags:
        Name: HelloWorld
variables:
  network:
    fn::invoke:
      function: aws:cloudformation:getStack
      arguments:
        name: my-network-stack
```
<!--End PulumiCodeChooser -->

name" Name of the stack
8
tagsB2" (Map of tags associated with this stack.
"+
capabilities*" List of capabilities
",
description" Description of the stack
"_
disableRollback
 HWhether the rollback of the stack is disabled when stack creation fails
"@

iamRoleArn" .ARN of the IAM role used to create the stack.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "Q
notificationArns*" 7List of SNS topic ARNs to publish stack related events
"0
outputs2" Map of outputs from the stack.
"S

parameters2" ?Map of parameters that specify input parameters for the stack.
"6
tags2" (Map of tags associated with this stack.
"<
templateBody" (Structure containing the template body.
"e
timeoutInMinutes MAmount of time that can pass before the stack status becomes `CREATE_FAILED`
2ô)
J

cloudfrontgetCachePolicy,aws:cloudfront/getCachePolicy:getCachePolicyÛUse this data source to retrieve information about a CloudFront cache policy.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cloudfront.getCachePolicy({
    name: "example-policy",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.get_cache_policy(name="example-policy")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CloudFront.GetCachePolicy.Invoke(new()
    {
        Name = "example-policy",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.LookupCachePolicy(ctx, &cloudfront.LookupCachePolicyArgs{
			Name: pulumi.StringRef("example-policy"),
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
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetCachePolicyArgs;
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
        final var example = CloudfrontFunctions.getCachePolicy(GetCachePolicyArgs.builder()
            .name("example-policy")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cloudfront:getCachePolicy
      arguments:
        name: example-policy
```
<!--End PulumiCodeChooser -->

### AWS-Managed Policies

AWS managed cache policy names are prefixed with `Managed-`:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cloudfront.getCachePolicy({
    name: "Managed-CachingOptimized",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.get_cache_policy(name="Managed-CachingOptimized")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CloudFront.GetCachePolicy.Invoke(new()
    {
        Name = "Managed-CachingOptimized",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.LookupCachePolicy(ctx, &cloudfront.LookupCachePolicyArgs{
			Name: pulumi.StringRef("Managed-CachingOptimized"),
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
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetCachePolicyArgs;
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
        final var example = CloudfrontFunctions.getCachePolicy(GetCachePolicyArgs.builder()
            .name("Managed-CachingOptimized")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cloudfront:getCachePolicy
      arguments:
        name: Managed-CachingOptimized
```
<!--End PulumiCodeChooser -->
-
idB" !Identifier for the cache policy.
8
nameB" *Unique name to identify the cache policy.
"5
comment" &Comment to describe the cache policy.
" 

defaultTtl ∑Default amount of time, in seconds, that you want objects to stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated.
"1
etag" %Current version of the cache policy.
"

idB" "∫
maxTtl ´Maximum amount of time, in seconds, that objects stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated.
"∆
minTtl ∑Minimum amount of time, in seconds, that you want objects to stay in the CloudFront cache before CloudFront sends another request to the origin to see if the object has been updated.
"
nameB" "î
)parametersInCacheKeyAndForwardedToOriginsÀ*»:≈
¬

cloudfront6getCachePolicyParametersInCacheKeyAndForwardedToOrigin|aws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOrigin:getCachePolicyParametersInCacheKeyAndForwardedToOriginòThe HTTP headers, cookies, and URL query strings to include in the cache key. See Parameters In Cache Key And Forwarded To Origin for more information.
2Ÿ
M

cloudfrontgetDistribution.aws:cloudfront/getDistribution:getDistribution¯Use this data source to retrieve information about a CloudFront distribution.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const test = aws.cloudfront.getDistribution({
    id: "EDFDVBD632BHDS5",
});
```
```python
import pulumi
import pulumi_aws as aws

test = aws.cloudfront.get_distribution(id="EDFDVBD632BHDS5")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var test = Aws.CloudFront.GetDistribution.Invoke(new()
    {
        Id = "EDFDVBD632BHDS5",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.LookupDistribution(ctx, &cloudfront.LookupDistributionArgs{
			Id: "EDFDVBD632BHDS5",
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
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetDistributionArgs;
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
        final var test = CloudfrontFunctions.getDistribution(GetDistributionArgs.builder()
            .id("EDFDVBD632BHDS5")
            .build());

    }
}
```
```yaml
variables:
  test:
    fn::invoke:
      function: aws:cloudfront:getDistribution
      arguments:
        id: EDFDVBD632BHDS5
```
<!--End PulumiCodeChooser -->
K
id" AIdentifier for the distribution. For example: `EDFDVBD632BHDS5`.

tagsB2" "v
aliases*" eList that contains information about CNAMEs (alternate domain names), if any, for this distribution.
"¥
arn" ®ARN (Amazon Resource Name) for the distribution. For example: arn:aws:cloudfront::123456789012:distribution/EDFDVBD632BHDS5, where 123456789012 is your AWS account ID.
"o

domainName" ]Domain name corresponding to the distribution. For
example: `d604721fxaaqy9.cloudfront.net`.
"
enabled
 "^
etag" RCurrent version of the distribution's information. For example:
`E2QWRUHAPOMQZL`.
"µ
hostedZoneId" †CloudFront Route 53 zone ID that can be used to
route an [Alias Resource Record Set][7] to. This attribute is simply an
alias for the zone ID `Z2FDTNDATAQYW2`.
"K
id" AIdentifier for the distribution. For example: `EDFDVBD632BHDS5`.
"]
inProgressValidationBatches :The number of invalidation batches
currently in progress.
"J
lastModifiedTime" 2Date and time the distribution was last modified.
"ù
status" éCurrent status of the distribution. `Deployed` if the
distribution's information is fully propagated throughout the Amazon
CloudFront system.
"
tags2" "C
webAclId" 3AWS WAF web ACL associated with this distribution.
2…
A

cloudfrontgetFunction&aws:cloudfront/getFunction:getFunctionÃProvides information about a CloudFront Function.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const config = new pulumi.Config();
const functionName = config.require("functionName");
const existing = aws.cloudfront.getFunction({
    name: functionName,
});
```
```python
import pulumi
import pulumi_aws as aws

config = pulumi.Config()
function_name = config.require("functionName")
existing = aws.cloudfront.get_function(name=function_name)
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var config = new Config();
    var functionName = config.Require("functionName");
    var existing = Aws.CloudFront.GetFunction.Invoke(new()
    {
        Name = functionName,
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi/config"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		cfg := config.New(ctx, "")
		functionName := cfg.Require("functionName")
		_, err := cloudfront.LookupFunction(ctx, &cloudfront.LookupFunctionArgs{
			Name: functionName,
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
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetFunctionArgs;
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
        final var functionName = config.get("functionName");
        final var existing = CloudfrontFunctions.getFunction(GetFunctionArgs.builder()
            .name(functionName)
            .build());

    }
}
```
```yaml
configuration:
  functionName:
    type: string
variables:
  existing:
    fn::invoke:
      function: aws:cloudfront:getFunction
      arguments:
        name: ${functionName}
```
<!--End PulumiCodeChooser -->
-
name" !Name of the CloudFront function.
A
stage" 4Function‚Äôs stage, either `DEVELOPMENT` or `LIVE`.
"5
arn" *ARN identifying your CloudFront Function.
"(
code" Source code of the function
"
comment" 	Comment.
"&
etag" ETag hash of the function
"E
id" ;The provider-assigned unique ID for this managed resource.
"k
keyValueStoreAssociations*" HList of `aws.cloudfront.KeyValueStore` ARNs associated to the function.
">
lastModifiedTime" &When this resource was last modified.
"

name" "5
runtime" &Identifier of the function's runtime.
"
stage" "\
status" NStatus of the function. Can be `UNPUBLISHED`, `UNASSOCIATED` or `ASSOCIATED`.
2¢L
w

cloudfrontgetLogDeliveryCanonicalUserIdJaws:cloudfront/getLogDeliveryCanonicalUserId:getLogDeliveryCanonicalUserIdıIThe CloudFront Log Delivery Canonical User ID data source allows access to the [canonical user ID](http://docs.aws.amazon.com/general/latest/gr/acct-identifiers.html) of the AWS `awslogsdelivery` account for CloudFront bucket logging.
See the [Amazon CloudFront Developer Guide](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/AccessLogs.html) for more information.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const current = aws.s3.getCanonicalUserId({});
const example = aws.cloudfront.getLogDeliveryCanonicalUserId({});
const exampleBucketV2 = new aws.s3.BucketV2("example", {bucket: "example"});
const exampleBucketOwnershipControls = new aws.s3.BucketOwnershipControls("example", {
    bucket: exampleBucketV2.id,
    rule: {
        objectOwnership: "BucketOwnerPreferred",
    },
});
const exampleBucketAclV2 = new aws.s3.BucketAclV2("example", {
    bucket: exampleBucketV2.id,
    accessControlPolicy: {
        grants: [{
            grantee: {
                id: example.then(example => example.id),
                type: "CanonicalUser",
            },
            permission: "FULL_CONTROL",
        }],
        owner: {
            id: current.then(current => current.id),
        },
    },
}, {
    dependsOn: [exampleBucketOwnershipControls],
});
```
```python
import pulumi
import pulumi_aws as aws

current = aws.s3.get_canonical_user_id()
example = aws.cloudfront.get_log_delivery_canonical_user_id()
example_bucket_v2 = aws.s3.BucketV2("example", bucket="example")
example_bucket_ownership_controls = aws.s3.BucketOwnershipControls("example",
    bucket=example_bucket_v2.id,
    rule={
        "object_ownership": "BucketOwnerPreferred",
    })
example_bucket_acl_v2 = aws.s3.BucketAclV2("example",
    bucket=example_bucket_v2.id,
    access_control_policy={
        "grants": [{
            "grantee": {
                "id": example.id,
                "type": "CanonicalUser",
            },
            "permission": "FULL_CONTROL",
        }],
        "owner": {
            "id": current.id,
        },
    },
    opts = pulumi.ResourceOptions(depends_on=[example_bucket_ownership_controls]))
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var current = Aws.S3.GetCanonicalUserId.Invoke();

    var example = Aws.CloudFront.GetLogDeliveryCanonicalUserId.Invoke();

    var exampleBucketV2 = new Aws.S3.BucketV2("example", new()
    {
        Bucket = "example",
    });

    var exampleBucketOwnershipControls = new Aws.S3.BucketOwnershipControls("example", new()
    {
        Bucket = exampleBucketV2.Id,
        Rule = new Aws.S3.Inputs.BucketOwnershipControlsRuleArgs
        {
            ObjectOwnership = "BucketOwnerPreferred",
        },
    });

    var exampleBucketAclV2 = new Aws.S3.BucketAclV2("example", new()
    {
        Bucket = exampleBucketV2.Id,
        AccessControlPolicy = new Aws.S3.Inputs.BucketAclV2AccessControlPolicyArgs
        {
            Grants = new[]
            {
                new Aws.S3.Inputs.BucketAclV2AccessControlPolicyGrantArgs
                {
                    Grantee = new Aws.S3.Inputs.BucketAclV2AccessControlPolicyGrantGranteeArgs
                    {
                        Id = example.Apply(getLogDeliveryCanonicalUserIdResult => getLogDeliveryCanonicalUserIdResult.Id),
                        Type = "CanonicalUser",
                    },
                    Permission = "FULL_CONTROL",
                },
            },
            Owner = new Aws.S3.Inputs.BucketAclV2AccessControlPolicyOwnerArgs
            {
                Id = current.Apply(getCanonicalUserIdResult => getCanonicalUserIdResult.Id),
            },
        },
    }, new CustomResourceOptions
    {
        DependsOn =
        {
            exampleBucketOwnershipControls,
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		current, err := s3.GetCanonicalUserId(ctx, map[string]interface{}{}, nil)
		if err != nil {
			return err
		}
		example, err := cloudfront.GetLogDeliveryCanonicalUserId(ctx, &cloudfront.GetLogDeliveryCanonicalUserIdArgs{}, nil)
		if err != nil {
			return err
		}
		exampleBucketV2, err := s3.NewBucketV2(ctx, "example", &s3.BucketV2Args{
			Bucket: pulumi.String("example"),
		})
		if err != nil {
			return err
		}
		exampleBucketOwnershipControls, err := s3.NewBucketOwnershipControls(ctx, "example", &s3.BucketOwnershipControlsArgs{
			Bucket: exampleBucketV2.ID(),
			Rule: &s3.BucketOwnershipControlsRuleArgs{
				ObjectOwnership: pulumi.String("BucketOwnerPreferred"),
			},
		})
		if err != nil {
			return err
		}
		_, err = s3.NewBucketAclV2(ctx, "example", &s3.BucketAclV2Args{
			Bucket: exampleBucketV2.ID(),
			AccessControlPolicy: &s3.BucketAclV2AccessControlPolicyArgs{
				Grants: s3.BucketAclV2AccessControlPolicyGrantArray{
					&s3.BucketAclV2AccessControlPolicyGrantArgs{
						Grantee: &s3.BucketAclV2AccessControlPolicyGrantGranteeArgs{
							Id:   pulumi.String(example.Id),
							Type: pulumi.String("CanonicalUser"),
						},
						Permission: pulumi.String("FULL_CONTROL"),
					},
				},
				Owner: &s3.BucketAclV2AccessControlPolicyOwnerArgs{
					Id: pulumi.String(current.Id),
				},
			},
		}, pulumi.DependsOn([]pulumi.Resource{
			exampleBucketOwnershipControls,
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
import com.pulumi.aws.s3.S3Functions;
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetLogDeliveryCanonicalUserIdArgs;
import com.pulumi.aws.s3.BucketV2;
import com.pulumi.aws.s3.BucketV2Args;
import com.pulumi.aws.s3.BucketOwnershipControls;
import com.pulumi.aws.s3.BucketOwnershipControlsArgs;
import com.pulumi.aws.s3.inputs.BucketOwnershipControlsRuleArgs;
import com.pulumi.aws.s3.BucketAclV2;
import com.pulumi.aws.s3.BucketAclV2Args;
import com.pulumi.aws.s3.inputs.BucketAclV2AccessControlPolicyArgs;
import com.pulumi.aws.s3.inputs.BucketAclV2AccessControlPolicyOwnerArgs;
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
        final var current = S3Functions.getCanonicalUserId();

        final var example = CloudfrontFunctions.getLogDeliveryCanonicalUserId();

        var exampleBucketV2 = new BucketV2("exampleBucketV2", BucketV2Args.builder()
            .bucket("example")
            .build());

        var exampleBucketOwnershipControls = new BucketOwnershipControls("exampleBucketOwnershipControls", BucketOwnershipControlsArgs.builder()
            .bucket(exampleBucketV2.id())
            .rule(BucketOwnershipControlsRuleArgs.builder()
                .objectOwnership("BucketOwnerPreferred")
                .build())
            .build());

        var exampleBucketAclV2 = new BucketAclV2("exampleBucketAclV2", BucketAclV2Args.builder()
            .bucket(exampleBucketV2.id())
            .accessControlPolicy(BucketAclV2AccessControlPolicyArgs.builder()
                .grants(BucketAclV2AccessControlPolicyGrantArgs.builder()
                    .grantee(BucketAclV2AccessControlPolicyGrantGranteeArgs.builder()
                        .id(example.applyValue(getLogDeliveryCanonicalUserIdResult -> getLogDeliveryCanonicalUserIdResult.id()))
                        .type("CanonicalUser")
                        .build())
                    .permission("FULL_CONTROL")
                    .build())
                .owner(BucketAclV2AccessControlPolicyOwnerArgs.builder()
                    .id(current.applyValue(getCanonicalUserIdResult -> getCanonicalUserIdResult.id()))
                    .build())
                .build())
            .build(), CustomResourceOptions.builder()
                .dependsOn(exampleBucketOwnershipControls)
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
      bucket: example
  exampleBucketOwnershipControls:
    type: aws:s3:BucketOwnershipControls
    name: example
    properties:
      bucket: ${exampleBucketV2.id}
      rule:
        objectOwnership: BucketOwnerPreferred
  exampleBucketAclV2:
    type: aws:s3:BucketAclV2
    name: example
    properties:
      bucket: ${exampleBucketV2.id}
      accessControlPolicy:
        grants:
          - grantee:
              id: ${example.id}
              type: CanonicalUser
            permission: FULL_CONTROL
        owner:
          id: ${current.id}
    options:
      dependsOn:
        - ${exampleBucketOwnershipControls}
variables:
  current:
    fn::invoke:
      function: aws:s3:getCanonicalUserId
      arguments: {}
  example:
    fn::invoke:
      function: aws:cloudfront:getLogDeliveryCanonicalUserId
      arguments: {}
```
<!--End PulumiCodeChooser -->
X
regionB" HRegion you'd like the zone for. By default, fetches the current region.
"E
id" ;The provider-assigned unique ID for this managed resource.
"
regionB" 2å
b

cloudfrontgetOriginAccessControl<aws:cloudfront/getOriginAccessControl:getOriginAccessControl†Use this data source to retrieve information for an Amazon CloudFront origin access control config.

## Example Usage

The below example retrieves a CloudFront origin access control config.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cloudfront.getOriginAccessControl({
    id: "E2T5VTFBZJ3BJB",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.get_origin_access_control(id="E2T5VTFBZJ3BJB")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CloudFront.GetOriginAccessControl.Invoke(new()
    {
        Id = "E2T5VTFBZJ3BJB",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.LookupOriginAccessControl(ctx, &cloudfront.LookupOriginAccessControlArgs{
			Id: "E2T5VTFBZJ3BJB",
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
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetOriginAccessControlArgs;
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
        final var example = CloudfrontFunctions.getOriginAccessControl(GetOriginAccessControlArgs.builder()
            .id("E2T5VTFBZJ3BJB")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cloudfront:getOriginAccessControl
      arguments:
        id: E2T5VTFBZJ3BJB
```
<!--End PulumiCodeChooser -->
`
id" VThe identifier for the origin access control settings. For example: `E2T5VTFBZJ3BJB`.
"?
description" ,A description of the origin access control.
"g
etag" [Current version of the origin access control's information. For example: `E2QWRUHAPOMQZL`.
"
id" ":
name" .A name to identify the origin access control.
"`
originAccessControlOriginType" ;The type of origin that this origin access control is for.
"B
signingBehavior" +Specifies which requests CloudFront signs.
"ä
signingProtocol" sThe signing protocol of the origin access control, which determines how CloudFront signs (authenticates) requests.
2í%
k

cloudfrontgetOriginAccessIdentitiesBaws:cloudfront/getOriginAccessIdentities:getOriginAccessIdentitiesñ!Use this data source to get ARNs, ids and S3 canonical user IDs of Amazon CloudFront origin access identities.

## Example Usage

### All origin access identities in the account

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cloudfront.getOriginAccessIdentities({});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.get_origin_access_identities()
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CloudFront.GetOriginAccessIdentities.Invoke();

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.GetOriginAccessIdentities(ctx, &cloudfront.GetOriginAccessIdentitiesArgs{}, nil)
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
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetOriginAccessIdentitiesArgs;
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
        final var example = CloudfrontFunctions.getOriginAccessIdentities();

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cloudfront:getOriginAccessIdentities
      arguments: {}
```
<!--End PulumiCodeChooser -->

### Origin access identities filtered by comment/name

Origin access identities whose comments are `example-comment1`, `example-comment2`

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cloudfront.getOriginAccessIdentities({
    comments: [
        "example-comment1",
        "example-comment2",
    ],
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.get_origin_access_identities(comments=[
    "example-comment1",
    "example-comment2",
])
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CloudFront.GetOriginAccessIdentities.Invoke(new()
    {
        Comments = new[]
        {
            "example-comment1",
            "example-comment2",
        },
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.GetOriginAccessIdentities(ctx, &cloudfront.GetOriginAccessIdentitiesArgs{
			Comments: []string{
				"example-comment1",
				"example-comment2",
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
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetOriginAccessIdentitiesArgs;
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
        final var example = CloudfrontFunctions.getOriginAccessIdentities(GetOriginAccessIdentitiesArgs.builder()
            .comments(            
                "example-comment1",
                "example-comment2")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cloudfront:getOriginAccessIdentities
      arguments:
        comments:
          - example-comment1
          - example-comment2
```
<!--End PulumiCodeChooser -->
@
commentsB*" ,Filter origin access identities by comment.
"
commentsB*" "F
iamArns*" 5Set of ARNs of the matched origin access identities.
"E
id" ;The provider-assigned unique ID for this managed resource.
"A
ids*" 4Set of ids of the matched origin access identities.
"b
s3CanonicalUserIds*" FSet of S3 canonical user IDs of the matched origin access identities.
2˝
e

cloudfrontgetOriginAccessIdentity>aws:cloudfront/getOriginAccessIdentity:getOriginAccessIdentity•Use this data source to retrieve information for an Amazon CloudFront origin access identity.

## Example Usage

The following example below creates a CloudFront origin access identity.

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cloudfront.getOriginAccessIdentity({
    id: "E1ZAKK699EOLAL",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.get_origin_access_identity(id="E1ZAKK699EOLAL")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CloudFront.GetOriginAccessIdentity.Invoke(new()
    {
        Id = "E1ZAKK699EOLAL",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.LookupOriginAccessIdentity(ctx, &cloudfront.LookupOriginAccessIdentityArgs{
			Id: "E1ZAKK699EOLAL",
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
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetOriginAccessIdentityArgs;
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
        final var example = CloudfrontFunctions.getOriginAccessIdentity(GetOriginAccessIdentityArgs.builder()
            .id("E1ZAKK699EOLAL")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cloudfront:getOriginAccessIdentity
      arguments:
        id: E1ZAKK699EOLAL
```
<!--End PulumiCodeChooser -->
X
id" NThe identifier for the origin access identity. For example: `E1ZAKK699EOLAL`.
"p
callerReference" YInternal value used by CloudFront to allow future
updates to the origin access identity.
"Ä
cloudfrontAccessIdentityPath" \A shortcut to the full path for the
origin access identity to use in CloudFront, see below.
"C
comment" 4An optional comment for the origin access identity.
"h
etag" \Current version of the origin access identity's information.
For example: `E2QWRUHAPOMQZL`.
"¶
iamArn" óPre-generated ARN for use in S3 bucket policies (see below).
Example: `arn:aws:iam::cloudfront:user/CloudFront Origin Access Identity
E2QWRUHAPOMQZL`.
"
id" "∫
s3CanonicalUserId" †The Amazon S3 canonical user ID for the origin
access identity, which you use when giving the origin access identity read
permission to an object in Amazon S3.
2Á,
b

cloudfrontgetOriginRequestPolicy<aws:cloudfront/getOriginRequestPolicy:getOriginRequestPolicyÙ## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cloudfront.getOriginRequestPolicy({
    name: "example-policy",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.get_origin_request_policy(name="example-policy")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CloudFront.GetOriginRequestPolicy.Invoke(new()
    {
        Name = "example-policy",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.LookupOriginRequestPolicy(ctx, &cloudfront.LookupOriginRequestPolicyArgs{
			Name: pulumi.StringRef("example-policy"),
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
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetOriginRequestPolicyArgs;
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
        final var example = CloudfrontFunctions.getOriginRequestPolicy(GetOriginRequestPolicyArgs.builder()
            .name("example-policy")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cloudfront:getOriginRequestPolicy
      arguments:
        name: example-policy
```
<!--End PulumiCodeChooser -->

### AWS-Managed Policies

AWS managed origin request policy names are prefixed with `Managed-`:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const uaReferer = aws.cloudfront.getOriginRequestPolicy({
    name: "Managed-UserAgentRefererHeaders",
});
```
```python
import pulumi
import pulumi_aws as aws

ua_referer = aws.cloudfront.get_origin_request_policy(name="Managed-UserAgentRefererHeaders")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var uaReferer = Aws.CloudFront.GetOriginRequestPolicy.Invoke(new()
    {
        Name = "Managed-UserAgentRefererHeaders",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.LookupOriginRequestPolicy(ctx, &cloudfront.LookupOriginRequestPolicyArgs{
			Name: pulumi.StringRef("Managed-UserAgentRefererHeaders"),
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
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetOriginRequestPolicyArgs;
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
        final var uaReferer = CloudfrontFunctions.getOriginRequestPolicy(GetOriginRequestPolicyArgs.builder()
            .name("Managed-UserAgentRefererHeaders")
            .build());

    }
}
```
```yaml
variables:
  uaReferer:
    fn::invoke:
      function: aws:cloudfront:getOriginRequestPolicy
      arguments:
        name: Managed-UserAgentRefererHeaders
```
<!--End PulumiCodeChooser -->
6
idB" *Identifier for the origin request policy.
A
nameB" 3Unique name to identify the origin request policy.
">
comment" /Comment to describe the origin request policy.
"ù
cookiesConfigsí*è:å
â

cloudfront#getOriginRequestPolicyCookiesConfigVaws:cloudfront/getOriginRequestPolicyCookiesConfig:getOriginRequestPolicyCookiesConfigıObject that determines whether any cookies in viewer requests (and if so, which cookies) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
":
etag" .Current version of the origin request policy.
"è
headersConfigsí*è:å
â

cloudfront#getOriginRequestPolicyHeadersConfigVaws:cloudfront/getOriginRequestPolicyHeadersConfig:getOriginRequestPolicyHeadersConfigÁObject that determines whether any HTTP headers (and if so, which headers) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
"

idB" "
nameB" "∆
queryStringsConfigs°*û:õ
ò

cloudfront(getOriginRequestPolicyQueryStringsConfig`aws:cloudfront/getOriginRequestPolicyQueryStringsConfig:getOriginRequestPolicyQueryStringsConfigäObject that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the origin request key and automatically included in requests that CloudFront sends to the origin. See Query String Config for more information.
2¢
\

cloudfrontgetRealtimeLogConfig8aws:cloudfront/getRealtimeLogConfig:getRealtimeLogConfigÄProvides a CloudFront real-time log configuration resource.

## Example Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cloudfront.getRealtimeLogConfig({
    name: "example",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.get_realtime_log_config(name="example")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CloudFront.GetRealtimeLogConfig.Invoke(new()
    {
        Name = "example",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.LookupRealtimeLogConfig(ctx, &cloudfront.LookupRealtimeLogConfigArgs{
			Name: "example",
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
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetRealtimeLogConfigArgs;
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
        final var example = CloudfrontFunctions.getRealtimeLogConfig(GetRealtimeLogConfigArgs.builder()
            .name("example")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cloudfront:getRealtimeLogConfig
      arguments:
        name: example
```
<!--End PulumiCodeChooser -->
F
name" :Unique name to identify this real-time log configuration.
"U
arn" JARN (Amazon Resource Name) of the CloudFront real-time log configuration.
"“
	endpointsz*x:v
t

cloudfrontgetRealtimeLogConfigEndpointHaws:cloudfront/getRealtimeLogConfigEndpoint:getRealtimeLogConfigEndpointI(Required) Amazon Kinesis data streams where real-time log data is sent.
"Ñ
fields*" Û(Required) Fields that are included in each real-time log record. See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-fields) for supported values.
"E
id" ;The provider-assigned unique ID for this managed resource.
"

name" "
samplingRate €(Required) Sampling rate for this real-time log configuration. The sampling rate determines the percentage of viewer requests that are represented in the real-time log data. An integer between `1` and `100`, inclusive.
2¡0
h

cloudfrontgetResponseHeadersPolicy@aws:cloudfront/getResponseHeadersPolicy:getResponseHeadersPolicyè Use this data source to retrieve information about a CloudFront cache policy.

## Example Usage

### Basic Usage

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cloudfront.getResponseHeadersPolicy({
    name: "example-policy",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.get_response_headers_policy(name="example-policy")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CloudFront.GetResponseHeadersPolicy.Invoke(new()
    {
        Name = "example-policy",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.LookupResponseHeadersPolicy(ctx, &cloudfront.LookupResponseHeadersPolicyArgs{
			Name: pulumi.StringRef("example-policy"),
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
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetResponseHeadersPolicyArgs;
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
        final var example = CloudfrontFunctions.getResponseHeadersPolicy(GetResponseHeadersPolicyArgs.builder()
            .name("example-policy")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cloudfront:getResponseHeadersPolicy
      arguments:
        name: example-policy
```
<!--End PulumiCodeChooser -->

### AWS-Managed Policies

AWS managed response header policy names are prefixed with `Managed-`:

<!--Start PulumiCodeChooser -->
```typescript
import * as pulumi from "@pulumi/pulumi";
import * as aws from "@pulumi/aws";

const example = aws.cloudfront.getResponseHeadersPolicy({
    name: "Managed-SimpleCORS",
});
```
```python
import pulumi
import pulumi_aws as aws

example = aws.cloudfront.get_response_headers_policy(name="Managed-SimpleCORS")
```
```csharp
using System.Collections.Generic;
using System.Linq;
using Pulumi;
using Aws = Pulumi.Aws;

return await Deployment.RunAsync(() => 
{
    var example = Aws.CloudFront.GetResponseHeadersPolicy.Invoke(new()
    {
        Name = "Managed-SimpleCORS",
    });

});
```
```go
package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v6/go/aws/cloudfront"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
)

func main() {
	pulumi.Run(func(ctx *pulumi.Context) error {
		_, err := cloudfront.LookupResponseHeadersPolicy(ctx, &cloudfront.LookupResponseHeadersPolicyArgs{
			Name: pulumi.StringRef("Managed-SimpleCORS"),
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
import com.pulumi.aws.cloudfront.CloudfrontFunctions;
import com.pulumi.aws.cloudfront.inputs.GetResponseHeadersPolicyArgs;
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
        final var example = CloudfrontFunctions.getResponseHeadersPolicy(GetResponseHeadersPolicyArgs.builder()
            .name("Managed-SimpleCORS")
            .build());

    }
}
```
```yaml
variables:
  example:
    fn::invoke:
      function: aws:cloudfront:getResponseHeadersPolicy
      arguments:
        name: Managed-SimpleCORS
```
<!--End PulumiCodeChooser -->
8
idB" ,Identifier for the response headers policy.
C
nameB" 5Unique name to identify the response headers policy.
"r
comment" cComment to describe the response headers policy. The comment cannot be longer than 128 characters.
"±
corsConfigsè*å:â
Ü

cloudfront"getResponseHeadersPolicyCorsConfigTaws:cloudfront/getResponseHeadersPolicyCorsConfig:getResponseHeadersPolicyCorsConfigèConfiguration for a set of HTTP response headers that are used for Cross-Origin Resource Sharing (CORS). See Cors Config for more information.
"ø
customHeadersConfigs™*ß:§
°

cloudfront+getResponseHeadersPolicyCustomHeadersConfigfaws:cloudfront/getResponseHeadersPolicyCustomHeadersConfig:getResponseHeadersPolicyCustomHeadersConfigzObject that contains an attribute `items` that contains a list of Custom Headers. See Custom Header for more information.
"<
etag" 0Current version of the response headers policy.
"
id" "

name" "ø
removeHeadersConfigs™*ß:§
°

cloudfront+getResponseHeadersPolicyRemoveHeadersConfigfaws:cloudfront/getResponseHeadersPolicyRemoveHeadersConfig:getResponseHeadersPolicyRemoveHeadersConfigzObject that contains an attribute `items` that contains a list of Remove Headers. See Remove Header for more information.
"ƒ
securityHeadersConfigs∞*≠:™
ß

cloudfront-getResponseHeadersPolicySecurityHeadersConfigjaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfig:getResponseHeadersPolicySecurityHeadersConfigwA configuration for a set of security-related HTTP response headers. See Security Headers Config for more information.
"¸
serverTimingHeadersConfigsº*π:∂
≥

cloudfront1getResponseHeadersPolicyServerTimingHeadersConfigraws:cloudfront/getResponseHeadersPolicyServerTimingHeadersConfig:getResponseHeadersPolicyServerTimingHeadersConfigû(Optional) Configuration for enabling the Server-Timing header in HTTP responses sent from CloudFront. See Server Timing Headers Config for more information.
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
:¨
k
budgetsBudgetActionActionThresholdCaws:budgets/BudgetActionActionThreshold:BudgetActionActionThresholdº
πx
actionThresholdType" ]The type of threshold for a notification. Valid values are `PERCENTAGE` or `ABSOLUTE_VALUE`.
=
actionThresholdValue !The threshold of a notification.
:¶
\
budgetsBudgetActionDefinition9aws:budgets/BudgetActionDefinition:BudgetActionDefinition≈
¬õ
iamActionDefinitionûBõ:ò
ï
budgets)BudgetActionDefinitionIamActionDefinition_aws:budgets/BudgetActionDefinitionIamActionDefinition:BudgetActionDefinitionIamActionDefinitioncThe AWS Identity and Access Management (IAM) action definition details. See IAM Action Definition.
í
scpActionDefinitionûBõ:ò
ï
budgets)BudgetActionDefinitionScpActionDefinition_aws:budgets/BudgetActionDefinitionScpActionDefinition:BudgetActionDefinitionScpActionDefinitionZThe service control policies (SCPs) action definition details. See SCP Action Definition.
å
ssmActionDefinitionûBõ:ò
ï
budgets)BudgetActionDefinitionSsmActionDefinition_aws:budgets/BudgetActionDefinitionSsmActionDefinition:BudgetActionDefinitionSsmActionDefinitionTThe AWS Systems Manager (SSM) action definition details. See SSM Action Definition.
:Ì
ï
budgets)BudgetActionDefinitionIamActionDefinition_aws:budgets/BudgetActionDefinitionIamActionDefinition:BudgetActionDefinitionIamActionDefinition“
œU
groupsB*" CA list of groups to be attached. There must be at least one group.
N
	policyArn" =The Amazon Resource Name (ARN) of the policy to be attached.
R
rolesB*" AA list of roles to be attached. There must be at least one role.
R
usersB*" AA list of users to be attached. There must be at least one user.
:Ò
ï
budgets)BudgetActionDefinitionScpActionDefinition_aws:budgets/BudgetActionDefinitionScpActionDefinition:BudgetActionDefinitionScpActionDefinitionW
U(
policyId" The policy ID attached.
)
	targetIds*" A list of target IDs.
:Ò
ï
budgets)BudgetActionDefinitionSsmActionDefinition_aws:budgets/BudgetActionDefinitionSsmActionDefinition:BudgetActionDefinitionSsmActionDefinition÷
”h
actionSubType" SThe action subType. Valid values are `STOP_EC2_INSTANCES` or `STOP_RDS_INSTANCES`.
3
instanceIds*" The EC2 and RDS instance IDs.
2
region" $The Region to run the SSM document.
:¿
\
budgetsBudgetActionSubscriber9aws:budgets/BudgetActionSubscriber:BudgetActionSubscriberﬂ
‹d
address" UThe address that AWS sends budget notifications to, either an SNS topic or an email.
t
subscriptionType" \The type of notification that AWS sends to a subscriber. Valid values are `SNS` or `EMAIL`.
:·
V
budgetsBudgetAutoAdjustData5aws:budgets/BudgetAutoAdjustData:BudgetAutoAdjustDataÜ
É´
autoAdjustType" î(Required) - The string that defines whether your budget auto-adjusts based on historical or forecasted data. Valid values: `FORECAST`,`HISTORICAL`
ı
historicalOptionsíBè:å
â
budgets%BudgetAutoAdjustDataHistoricalOptionsWaws:budgets/BudgetAutoAdjustDataHistoricalOptions:BudgetAutoAdjustDataHistoricalOptions (Optional) - Configuration block of Historical Options. Required for `auto_adjust_type` of `HISTORICAL` Configuration block that defines the historical data that your auto-adjusting budget is based on.
[
lastAutoAdjustTimeB" ?(Optional) - The last time that your budget was auto-adjusted.
:¡
â
budgets%BudgetAutoAdjustDataHistoricalOptionsWaws:budgets/BudgetAutoAdjustDataHistoricalOptions:BudgetAutoAdjustDataHistoricalOptions≤
Ø¶
budgetAdjustmentPeriod á(Required) - The number of budget periods included in the moving-average calculation that determines your auto-adjusted budget amount.
É
lookbackAvailablePeriodsB ‡(Optional) - The integer that describes how many budget periods in your BudgetAdjustmentPeriod are included in the calculation of your current budget limit. If the first budget period in your BudgetAdjustmentPeriod has no cost data, then that budget period isn‚Äôt included in the average that determines your budget limit. You can‚Äôt set your own LookBackAvailablePeriods. The value is automatically calculated from the `budget_adjustment_period` and your historical cost data.
:ú
J
budgetsBudgetCostFilter-aws:budgets/BudgetCostFilter:BudgetCostFilterN
L:
name" .The name of a budget. Unique within accounts.

values*" :ˆ	
G
budgetsBudgetCostTypes+aws:budgets/BudgetCostTypes:BudgetCostTypes™	
ß	i
includeCreditB
 RA boolean value whether to include credits in the cost budget. Defaults to `true`
Q
includeDiscountB
 8Whether a budget includes discounts. Defaults to `true`
Ö
includeOtherSubscriptionB
 cA boolean value whether to include other subscription costs in the cost budget. Defaults to `true`
t
includeRecurringB
 ZA boolean value whether to include recurring costs in the cost budget. Defaults to `true`
i
includeRefundB
 RA boolean value whether to include refunds in the cost budget. Defaults to `true`
u
includeSubscriptionB
 XA boolean value whether to include subscriptions in the cost budget. Defaults to `true`
p
includeSupportB
 XA boolean value whether to include support costs in the cost budget. Defaults to `true`
b

includeTaxB
 NA boolean value whether to include tax in the cost budget. Defaults to `true`
p
includeUpfrontB
 XA boolean value whether to include upfront costs in the cost budget. Defaults to `true`
T
useAmortizedB
 >Whether a budget uses the amortized rate. Defaults to `false`
i

useBlendedB
 UA boolean value whether to use blended costs in the cost budget. Defaults to `false`
:ï
P
budgetsBudgetNotification1aws:budgets/BudgetNotification:BudgetNotification¿
Ωç
comparisonOperator" s(Required) Comparison operator to use to evaluate the condition. Can be `LESS_THAN`, `EQUAL_TO` or `GREATER_THAN`.
k
notificationType" S(Required) What kind of budget value to notify on. Can be `ACTUAL` or `FORECASTED`
É
subscriberEmailAddressesB*" _(Optional) E-Mail addresses to notify. Either this or `subscriber_sns_topic_arns` is required.
|
subscriberSnsTopicArnsB*" Z(Optional) SNS topics to notify. Either this or `subscriber_email_addresses` is required.
L
	threshold ;(Required) Threshold when the notification should be sent.
l
thresholdType" W(Required) What kind of threshold is defined. Can be `PERCENTAGE` OR `ABSOLUTE_VALUE`.
:ƒ
P
budgetsBudgetPlannedLimit1aws:budgets/BudgetPlannedLimit:BudgetPlannedLimitÔ
ÏR
amount" D(Required) The amount of cost or usage being measured for a budget.
ù
	startTime" ã(Required) The start time of the budget limit. Format: `2017-01-01_12:00`. See [PlannedBudgetLimits](https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_Budget.html#awscostmanagement-Type-budgets_Budget-PlannedBudgetLimits) documentation.
ı
unit" Ë(Required) The unit of measurement used for the budget forecast, actual spend, or budget threshold, such as dollars or GB. See [Spend](http://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/data-type-spend.html) documentation.
:Ô
_
budgetsgetBudgetAutoAdjustData;aws:budgets/getBudgetAutoAdjustData:getBudgetAutoAdjustDataã
à¨
autoAdjustType" ï(Required) - The string that defines whether your budget auto-adjusts based on historical or forecasted data. Valid values: `FORECAST`,`HISTORICAL`.
˚
historicalOptionsò*ï:í
è
budgets'getBudgetAutoAdjustDataHistoricalOption[aws:budgets/getBudgetAutoAdjustDataHistoricalOption:getBudgetAutoAdjustDataHistoricalOption (Optional) - Configuration block of Historical Options. Required for `auto_adjust_type` of `HISTORICAL` Configuration block that defines the historical data that your auto-adjusting budget is based on.
Y
lastAutoAdjustTime" ?(Optional) - The last time that your budget was auto-adjusted.
:≈
è
budgets'getBudgetAutoAdjustDataHistoricalOption[aws:budgets/getBudgetAutoAdjustDataHistoricalOption:getBudgetAutoAdjustDataHistoricalOption∞
≠¶
budgetAdjustmentPeriod á(Required) - The number of budget periods included in the moving-average calculation that determines your auto-adjusted budget amount.
Å
lookbackAvailablePeriods ‡(Optional) - The integer that describes how many budget periods in your BudgetAdjustmentPeriod are included in the calculation of your current budget limit. If the first budget period in your BudgetAdjustmentPeriod has no cost data, then that budget period isn‚Äôt included in the average that determines your budget limit. You can‚Äôt set your own LookBackAvailablePeriods. The value is automatically calculated from the `budget_adjustment_period` and your historical cost data.
:
V
budgetsgetBudgetBudgetLimit5aws:budgets/getBudgetBudgetLimit:getBudgetBudgetLimitï
í¿
amount" ±The cost or usage amount that's associated with a budget forecast, actual spend, or budget threshold. Length Constraints: Minimum length of `1`. Maximum length of `2147483647`.
Ã
unit" øThe unit of measurement that's used for the budget forecast, actual spend, or budget threshold, such as USD or GBP. Length Constraints: Minimum length of `1`. Maximum length of `2147483647`.
:ä
b
budgetsgetBudgetCalculatedSpend=aws:budgets/getBudgetCalculatedSpend:getBudgetCalculatedSpend£
†ù
actualSpendså*â:Ü
É
budgets#getBudgetCalculatedSpendActualSpendSaws:budgets/getBudgetCalculatedSpendActualSpend:getBudgetCalculatedSpendActualSpend:û
É
budgets#getBudgetCalculatedSpendActualSpendSaws:budgets/getBudgetCalculatedSpendActualSpend:getBudgetCalculatedSpendActualSpendï
í¿
amount" ±The cost or usage amount that's associated with a budget forecast, actual spend, or budget threshold. Length Constraints: Minimum length of `1`. Maximum length of `2147483647`.
Ã
unit" øThe unit of measurement that's used for the budget forecast, actual spend, or budget threshold, such as USD or GBP. Length Constraints: Minimum length of `1`. Maximum length of `2147483647`.
:Ã
S
budgetsgetBudgetCostFilter3aws:budgets/getBudgetCostFilter:getBudgetCostFilteru
sa
name" UThe name of a budget. Unique within accounts.

The following arguments are optional:

values*" :Ò	
M
budgetsgetBudgetCostType/aws:budgets/getBudgetCostType:getBudgetCostTypeü	
ú	h
includeCredit
 SA boolean value whether to include credits in the cost budget. Defaults to `true`.
P
includeDiscount
 9Whether a budget includes discounts. Defaults to `true`.
Ñ
includeOtherSubscription
 dA boolean value whether to include other subscription costs in the cost budget. Defaults to `true`.
s
includeRecurring
 [A boolean value whether to include recurring costs in the cost budget. Defaults to `true`.
h
includeRefund
 SA boolean value whether to include refunds in the cost budget. Defaults to `true`.
t
includeSubscription
 YA boolean value whether to include subscriptions in the cost budget. Defaults to `true`.
o
includeSupport
 YA boolean value whether to include support costs in the cost budget. Defaults to `true`.
a

includeTax
 OA boolean value whether to include tax in the cost budget. Defaults to `true`.
o
includeUpfront
 YA boolean value whether to include upfront costs in the cost budget. Defaults to `true`.
S
useAmortized
 ?Whether a budget uses the amortized rate. Defaults to `false`.
h

useBlended
 VA boolean value whether to use blended costs in the cost budget. Defaults to `false`.
:õ
Y
budgetsgetBudgetNotification7aws:budgets/getBudgetNotification:getBudgetNotificationΩ
∫ç
comparisonOperator" s(Required) Comparison operator to use to evaluate the condition. Can be `LESS_THAN`, `EQUAL_TO` or `GREATER_THAN`.
l
notificationType" T(Required) What kind of budget value to notify on. Can be `ACTUAL` or `FORECASTED`.
Å
subscriberEmailAddresses*" _(Optional) E-Mail addresses to notify. Either this or `subscriber_sns_topic_arns` is required.
z
subscriberSnsTopicArns*" Z(Optional) SNS topics to notify. Either this or `subscriber_email_addresses` is required.
L
	threshold ;(Required) Threshold when the notification should be sent.
l
thresholdType" W(Required) What kind of threshold is defined. Can be `PERCENTAGE` OR `ABSOLUTE_VALUE`.
:ì
Y
budgetsgetBudgetPlannedLimit7aws:budgets/getBudgetPlannedLimit:getBudgetPlannedLimitµ
≤¿
amount" ±The cost or usage amount that's associated with a budget forecast, actual spend, or budget threshold. Length Constraints: Minimum length of `1`. Maximum length of `2147483647`.
ù
	startTime" ã(Required) The start time of the budget limit. Format: `2017-01-01_12:00`. See [PlannedBudgetLimits](https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_budgets_Budget.html#awscostmanagement-Type-budgets_Budget-PlannedBudgetLimits) documentation.
Ã
unit" øThe unit of measurement that's used for the budget forecast, actual spend, or budget threshold, such as USD or GBP. Length Constraints: Minimum length of `1`. Maximum length of `2147483647`.
:÷
ü
cfg/ConfigurationAggregatorAccountAggregationSourcegaws:cfg/ConfigurationAggregatorAccountAggregationSource:ConfigurationAggregatorAccountAggregationSource±
ÆU

accountIds*" AList of 12-digit account IDs of the account(s) being aggregated.
W

allRegionsB
 CIf true, aggregate existing AWS Config regions and future regions.
|
regionsB*" iList of source regions being aggregated.

Either `regions` or `all_regions` (as true) must be specified.
:á
Æ
cfg4ConfigurationAggregatorOrganizationAggregationSourceqaws:cfg/ConfigurationAggregatorOrganizationAggregationSource:ConfigurationAggregatorOrganizationAggregationSource”
–W

allRegionsB
 CIf true, aggregate existing AWS Config regions and future regions.
<
regionsB*" )List of source regions being aggregated.
∂
roleArn" ¶ARN of the IAM role used to retrieve AWS Organization details associated with the aggregator account.

Either `regions` or `all_regions` (as true) must be specified.
:æ
i
cfgConformancePackInputParameterCaws:cfg/ConformancePackInputParameter:ConformancePackInputParameterQ
O$
parameterName" The input key.
'
parameterValue" The input value.
:ÿ
ç
cfg)DeliveryChannelSnapshotDeliveryProperties[aws:cfg/DeliveryChannelSnapshotDeliveryProperties:DeliveryChannelSnapshotDeliveryProperties≈
¬ø
deliveryFrequencyB" £The frequency with which AWS Config recurringly delivers configuration snapshotsE.g., `One_Hour` or `Three_Hours`. Valid values are listed [here](https://docs.aws.amazon.com/config/latest/APIReference/API_ConfigSnapshotDeliveryProperties.html#API_ConfigSnapshotDeliveryProperties_Contents).
:„
ç
cfg)OrganizationConformancePackInputParameter[aws:cfg/OrganizationConformancePackInputParameter:OrganizationConformancePackInputParameterQ
O$
parameterName" The input key.
'
parameterValue" The input value.
:∞
T
cfgRecorderRecordingGroup5aws:cfg/RecorderRecordingGroup:RecorderRecordingGroup◊
‘˝
allSupportedB
 ÊSpecifies whether AWS Config records configuration changes for every supported type of regional resource (which includes any new type that will become supported in the future). Conflicts with `resource_types`. Defaults to `true`.
„
exclusionByResourceTypes•B¢*ü:ú
ô
cfg-RecorderRecordingGroupExclusionByResourceTypecaws:cfg/RecorderRecordingGroupExclusionByResourceType:RecorderRecordingGroupExclusionByResourceTypeûAn object that specifies how AWS Config excludes resource types from being recorded by the configuration recorder.To use this option, you must set the useOnly field of RecordingStrategy to `EXCLUSION_BY_RESOURCE_TYPES` Requires `all_supported = false`. Conflicts with `resource_types`.
€
includeGlobalResourceTypesB
 ∂Specifies whether AWS Config includes all supported types of _global resources_ with the resources that it records. Requires `all_supported = true`. Conflicts with `resource_types`.
—
recordingStrategiesìBê*ç:ä
á
cfg'RecorderRecordingGroupRecordingStrategyWaws:cfg/RecorderRecordingGroupRecordingStrategy:RecorderRecordingGroupRecordingStrategy$Recording Strategy. Detailed below.
π
resourceTypesB*" üA list that specifies the types of AWS resources for which AWS Config records configuration changes (for example, `AWS::EC2::Instance` or `AWS::CloudTrail::Trail`). See [relevant part of AWS Docs](http://docs.aws.amazon.com/config/latest/APIReference/API_ResourceIdentifier.html#config-Type-ResourceIdentifier-resourceType) for available types. In order to use this attribute, `all_supported` must be set to false.
:·
ô
cfg-RecorderRecordingGroupExclusionByResourceTypecaws:cfg/RecorderRecordingGroupExclusionByResourceType:RecorderRecordingGroupExclusionByResourceType¬
øº
resourceTypesB*" ¢A list that specifies the types of AWS resources for which AWS Config excludes records configuration changes. See [relevant part of AWS Docs](http://docs.aws.amazon.com/config/latest/APIReference/API_ResourceIdentifier.html#config-Type-ResourceIdentifier-resourceType) for available types.
:ü
á
cfg'RecorderRecordingGroupRecordingStrategyWaws:cfg/RecorderRecordingGroupRecordingStrategy:RecorderRecordingGroupRecordingStrategy

useOnlyB" :ê
Q
cfgRecorderRecordingMode3aws:cfg/RecorderRecordingMode:RecorderRecordingMode∫
∑S
recordingFrequencyB" 7Default reecording frequency. `CONTINUOUS` or `DAILY`.
ﬂ
recordingModeOverrideôBñ:ì
ê
cfg*RecorderRecordingModeRecordingModeOverride]aws:cfg/RecorderRecordingModeRecordingModeOverride:RecorderRecordingModeRecordingModeOverride*Recording mode overrides. Detailed below.
:ª
ê
cfg*RecorderRecordingModeRecordingModeOverride]aws:cfg/RecorderRecordingModeRecordingModeOverride:RecorderRecordingModeRecordingModeOverride•
¢@
descriptionB" +A description you provide of the override.
t
recordingFrequency" ZThe recording frequency for the resources in the override block. `CONTINUOUS` or `DAILY`.
Á
resourceTypes*" œA list that specifies the types of AWS resources for which the override applies to.  See [restrictions in the AWS Docs](https://docs.aws.amazon.com/config/latest/APIReference/API_RecordingModeOverride.html)
:ì
ç
cfg)RemediationConfigurationExecutionControls[aws:cfg/RemediationConfigurationExecutionControls:RemediationConfigurationExecutionControlsÄ
˝˙
ssmControls∑B¥:±
Æ
cfg4RemediationConfigurationExecutionControlsSsmControlsqaws:cfg/RemediationConfigurationExecutionControlsSsmControls:RemediationConfigurationExecutionControlsSsmControls1Configuration block for SSM controls. See below.
:´
Æ
cfg4RemediationConfigurationExecutionControlsSsmControlsqaws:cfg/RemediationConfigurationExecutionControlsSsmControls:RemediationConfigurationExecutionControlsSsmControls˜
Ù¬
!concurrentExecutionRatePercentageB ñMaximum percentage of remediation actions allowed to run in parallel on the non-compliant resources for that specific rule. The default value is 10%.
¨
errorPercentageB íPercentage of errors that are allowed before SSM stops running automations on non-compliant resources for that specific rule. The default is 50%.
:ﬁ
u
cfg!RemediationConfigurationParameterKaws:cfg/RemediationConfigurationParameter:RemediationConfigurationParameter‰
·#
name" Name of the attribute.
A
resourceValueB" *Value is dynamic and changes at run-time.
F
staticValueB" 1Value is static and does not change at run-time.
/
staticValuesB*" List of static values.
:y
H
cfgRuleEvaluationMode-aws:cfg/RuleEvaluationMode:RuleEvaluationMode-
+)
modeB" The mode of an evaluation.
:–
-
cfg	RuleScopeaws:cfg/RuleScope:RuleScopeû
õ◊
complianceResourceIdB" ∏The IDs of the only AWS resource that you want to trigger an evaluation for the rule. If you specify a resource ID, you must specify one resource type for `compliance_resource_types`.
∫
complianceResourceTypesB*" ñA list of resource types of only those AWS resources that you want to trigger an evaluation for the ruleE.g., `AWS::EC2::Instance`. You can only specify one type if you also specify a resource ID for `compliance_resource_id`. See [relevant part of AWS Docs](http://docs.aws.amazon.com/config/latest/APIReference/API_ResourceIdentifier.html#config-Type-ResourceIdentifier-resourceType) for available types.
Ü
tagKeyB" vThe tag key that is applied to only those AWS resources that you want you want to trigger an evaluation for the rule.
y
tagValueB" gThe tag value applied to only those AWS resources that you want to trigger an evaluation for the rule.
:
0
cfg
RuleSourceaws:cfg/RuleSource:RuleSourceª
∏Ø
customPolicyDetailsoBm:k
i
cfgRuleSourceCustomPolicyDetailsCaws:cfg/RuleSourceCustomPolicyDetails:RuleSourceCustomPolicyDetails¶Provides the runtime system, policy definition, and whether debug logging is enabled. Required when owner is set to `CUSTOM_POLICY`. See Custom Policy Details Below.
ñ
owner" àIndicates whether AWS or the customer owns and manages the AWS Config rule. Valid values are `AWS`, `CUSTOM_LAMBDA` or `CUSTOM_POLICY`. For more information about managed rules, see the [AWS Config Managed Rules documentation](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_use-managed-rules.html). For more information about custom rules, see the [AWS Config Custom Rules documentation](https://docs.aws.amazon.com/config/latest/developerguide/evaluate-config_develop-rules.html). Custom Lambda Functions require permissions to allow the AWS Config service to invoke them, e.g., via the `aws.lambda.Permission` resource.
ß
sourceDetails\BZ*X:V
T
cfgRuleSourceSourceDetail5aws:cfg/RuleSourceSourceDetail:RuleSourceSourceDetail∑Provides the source and type of the event that causes AWS Config to evaluate your AWS resources. Only valid if `owner` is `CUSTOM_LAMBDA` or `CUSTOM_POLICY`. See Source Detail Below.
¿
sourceIdentifierB" •For AWS Config managed rules, a predefined identifier, e.g `IAM_PASSWORD_POLICY`. For custom Lambda rules, the identifier is the ARN of the Lambda Function, such as `arn:aws:lambda:us-east-1:123456789012:function:custom_rule_name` or the `arn` attribute of the `aws.lambda.Function` resource.
:≤
i
cfgRuleSourceCustomPolicyDetailsCaws:cfg/RuleSourceCustomPolicyDetails:RuleSourceCustomPolicyDetailsƒ
¡î
enableDebugLogDeliveryB
 tThe boolean expression for enabling debug logging for your Config Custom Policy rule. The default value is `false`.
ƒ
policyRuntime" ÆThe runtime system for your Config Custom Policy rule. Guard is a policy-as-code language that allows you to write policies that are enforced by Config Custom Policy rules. For more information about Guard, see the [Guard GitHub Repository](https://github.com/aws-cloudformation/cloudformation-guard).
a

policyText" OThe policy definition containing the logic for your Config Custom Policy rule.
:ü

T
cfgRuleSourceSourceDetail5aws:cfg/RuleSourceSourceDetail:RuleSourceSourceDetail∆	
√	∫
eventSourceB" §The source of the event, such as an AWS service, that triggers AWS Config to evaluate your AWSresources. This defaults to `aws.config` and is the only valid value.
Õ
maximumExecutionFrequencyB" ©The frequency that you want AWS Config to run evaluations for a rule that istriggered periodically. If specified, requires `message_type` to be `ScheduledNotification`.
≥
messageTypeB" ùThe type of notification that triggers AWS Config to run an evaluation for a rule. You canspecify the following notification types:
* `ConfigurationItemChangeNotification` - Triggers an evaluation when AWS Config delivers a configuration item as a result of a resource change.
* `OversizedConfigurationItemChangeNotification` - Triggers an evaluation when AWS Config delivers an oversized configuration item. AWS Config may generate this notification type when a resource changes and the notification exceeds the maximum size allowed by Amazon SNS.
* `ScheduledNotification` - Triggers a periodic evaluation at the frequency specified for `maximum_execution_frequency`.
* `ConfigurationSnapshotDeliveryCompleted` - Triggers a periodic evaluation when AWS Config delivers a configuration snapshot.
:ƒ
}
chatbot!SlackChannelConfigurationTimeoutsOaws:chatbot/SlackChannelConfigurationTimeouts:SlackChannelConfigurationTimeouts¬
øÁ
createB" ÷A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
Ë
deleteB" ◊A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
Á
updateB" ÷A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:ƒ
}
chatbot!TeamsChannelConfigurationTimeoutsOaws:chatbot/TeamsChannelConfigurationTimeouts:TeamsChannelConfigurationTimeouts¬
øÁ
createB" ÷A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
Ë
deleteB" ◊A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
Á
updateB" ÷A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:„
Ç
chime$SdkvoiceGlobalSettingsVoiceConnectorSaws:chime/SdkvoiceGlobalSettingsVoiceConnector:SdkvoiceGlobalSettingsVoiceConnector\
ZX
	cdrBucketB" EThe S3 bucket that stores the Voice Connector's call detail records.
:æ
Ç
chime$SdkvoiceSipMediaApplicationEndpointsSaws:chime/SdkvoiceSipMediaApplicationEndpoints:SdkvoiceSipMediaApplicationEndpoints∂
≥∞
	lambdaArn" ûValid Amazon Resource Name (ARN) of the Lambda function, version, or alias. The function must be created in the same AWS Region as the SIP media application.
:ƒ
v
chime SdkvoiceSipRuleTargetApplicationKaws:chime/SdkvoiceSipRuleTargetApplication:SdkvoiceSipRuleTargetApplication…
∆;
	awsRegion" *The AWS Region of the target application.
J
priority :Priority of the SIP media application in the target list.
;
sipMediaApplicationId" The SIP media application ID.
:ö
»
chime;SdkvoiceVoiceProfileDomainServerSideEncryptionConfigurationÅaws:chime/SdkvoiceVoiceProfileDomainServerSideEncryptionConfiguration:SdkvoiceVoiceProfileDomainServerSideEncryptionConfigurationM
KI
	kmsKeyArn" 8ARN for KMS Key.

The following arguments are optional:
:Í
j
chimeVoiceConnectorGroupConnectorCaws:chime/VoiceConnectorGroupConnector:VoiceConnectorGroupConnector˚
¯∂
priority •The priority associated with the Amazon Chime Voice Connector, with 1 being the highest priority. Higher priority Amazon Chime Voice Connectors are attempted first.
=
voiceConnectorId" %The Amazon Chime Voice Connector ID.
:¡
s
chimeVoiceConnectorOrganizationRouteIaws:chime/VoiceConnectorOrganizationRoute:VoiceConnectorOrganizationRoute…
∆G
host" ;The FQDN or IP address to contact for origination traffic.
I
portB ;The designated origination route port. Defaults to `5060`.
Ö
priority uThe priority associated with the host, with 1 being the highest priority. Higher priority hosts are attempted first.
ç
protocol" }The protocol to use for the origination route. Encryption-enabled Amazon Chime Voice Connectors use TCP protocol by default.
ó
weight àThe weight associated with the host. If hosts are equal in priority, calls are redistributed among them based on their relative weight.
:É
©
chime1VoiceConnectorStreamingMediaInsightsConfigurationmaws:chime/VoiceConnectorStreamingMediaInsightsConfiguration:VoiceConnectorStreamingMediaInsightsConfiguration‘
—h
configurationArnB" NThe media insights configuration that will be invoked by the Voice Connector.
e
disabledB
 SWhen `true`, the media insights configuration is not enabled. Defaults to `false`.
:Õ
†
chime.VoiceConnectorTerminationCredentialsCredentialgaws:chime/VoiceConnectorTerminationCredentialsCredential:VoiceConnectorTerminationCredentialsCredentialß
§P
password" @RFC2617 compliant password associated with the SIP credentials.
P
username" @RFC2617 compliant username associated with the SIP credentials.
:ó
≥
chimesdkmediapipelines)MediaInsightsPipelineConfigurationElementnaws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElement:MediaInsightsPipelineConfigurationElementﬁ
€Œ
3amazonTranscribeCallAnalyticsProcessorConfiguration÷B”:–
Õ
chimesdkmediapipelines\MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration‘aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration:MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration>Configuration for Amazon Transcribe Call Analytics processor.
ã
&amazonTranscribeProcessorConfigurationØB¨:©
¶
chimesdkmediapipelinesOMediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration∫aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration:MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration/Configuration for Amazon Transcribe processor.
¯
"kinesisDataStreamSinkConfiguration£B†:ù
ö
chimesdkmediapipelinesKMediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfiguration≤aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfiguration:MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfiguration,Configuration for Kinesis Data Stream sink.
Ë
lambdaFunctionSinkConfigurationöBó:î
ë
chimesdkmediapipelinesHMediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfiguration¨aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfiguration:MediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfiguration(Configuration for Lambda Function sink.
Ÿ
s3RecordingSinkConfigurationëBé:ã
à
chimesdkmediapipelinesEMediaInsightsPipelineConfigurationElementS3RecordingSinkConfiguration¶aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementS3RecordingSinkConfiguration:MediaInsightsPipelineConfigurationElementS3RecordingSinkConfiguration%Configuration for S3 recording sink.
 
snsTopicSinkConfigurationàBÖ:Ç
ˇ
chimesdkmediapipelinesBMediaInsightsPipelineConfigurationElementSnsTopicSinkConfiguration†aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementSnsTopicSinkConfiguration:MediaInsightsPipelineConfigurationElementSnsTopicSinkConfiguration"Configuration for SNS Topic sink.
 
sqsQueueSinkConfigurationàBÖ:Ç
ˇ
chimesdkmediapipelinesBMediaInsightsPipelineConfigurationElementSqsQueueSinkConfiguration†aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementSqsQueueSinkConfiguration:MediaInsightsPipelineConfigurationElementSqsQueueSinkConfiguration"Configuration for SQS Queue sink.

type" Element type.
Å
$voiceAnalyticsProcessorConfiguration©B¶:£
†
chimesdkmediapipelinesMMediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfiguration∂aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfiguration:MediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfiguration-Configuration for Voice analytics processor.
:˝
Õ
chimesdkmediapipelines\MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration‘aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration:MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfiguration™
ßh
callAnalyticsStreamCategoriesB*" ?Filter for category events to be delivered to insights target.
x
contentIdentificationTypeB" ULabels all personally identifiable information (PII) identified in Utterance events.
t
contentRedactionTypeB" VRedacts all personally identifiable information (PII) identified in Utterance events.
e
!enablePartialResultsStabilizationB
 :Enables partial result stabilization in Utterance events.
e
filterPartialResultsB
 GFilters partial Utterance events from delivery to the insights target.
?
languageCode" +Language code for the transcription model.
L
languageModelNameB" 1Name of custom language model for transcription.
j
partialResultsStabilityB" ILevel of stability to use when partial results stabilization is enabled.
n
piiEntityTypesB" VTypes of personally identifiable information (PII) to redact from an Utterance event.
„
postCallAnalyticsSettings°Bû:õ
ò
chimesdkmediapipelinesuMediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettingsÜaws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettings:MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettings"Settings for post call analytics.
]
vocabularyFilterMethodB" =Method for applying a vocabulary filter to Utterance events.
l
vocabularyFilterNameB" NName of the custom vocabulary filter to use when processing Utterance events.
_
vocabularyNameB" GName of the custom vocabulary to use when processing Utterance events.
:é
ò
chimesdkmediapipelinesuMediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettingsÜaws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettings:MediaInsightsPipelineConfigurationElementAmazonTranscribeCallAnalyticsProcessorConfigurationPostCallAnalyticsSettings
Ì;
contentRedactionOutputB" Should output be redacted.
c
dataAccessRoleArn" JARN of the role used by AWS Transcribe to upload your post call analysis.
P
outputEncryptionKmsKeyIdB" .ID of the KMS key used to encrypt the output.
w
outputLocation" aThe Amazon S3 location where you want your Call Analytics post-call transcription output stored.
:Ó
¶
chimesdkmediapipelinesOMediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration∫aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration:MediaInsightsPipelineConfigurationElementAmazonTranscribeProcessorConfiguration¬	
ø	y
contentIdentificationTypeB" VLabels all personally identifiable information (PII) identified in Transcript events.
u
contentRedactionTypeB" WRedacts all personally identifiable information (PII) identified in Transcript events.
f
!enablePartialResultsStabilizationB
 ;Enables partial result stabilization in Transcript events.
e
filterPartialResultsB
 GFilters partial Utterance events from delivery to the insights target.
?
languageCode" +Language code for the transcription model.
L
languageModelNameB" 1Name of custom language model for transcription.
j
partialResultsStabilityB" ILevel of stability to use when partial results stabilization is enabled.
n
piiEntityTypesB" VTypes of personally identifiable information (PII) to redact from a Transcript event.
`
showSpeakerLabelB
 FEnables speaker partitioning (diarization) in your Transcript events.
^
vocabularyFilterMethodB" >Method for applying a vocabulary filter to Transcript events.
m
vocabularyFilterNameB" OName of the custom vocabulary filter to use when processing Transcript events.
`
vocabularyNameB" HName of the custom vocabulary to use when processing Transcript events.
:·
ö
chimesdkmediapipelinesKMediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfiguration≤aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfiguration:MediaInsightsPipelineConfigurationElementKinesisDataStreamSinkConfigurationB
@>
insightsTarget" (Kinesis Data Stream to deliver results.
:‘
ë
chimesdkmediapipelinesHMediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfiguration¨aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfiguration:MediaInsightsPipelineConfigurationElementLambdaFunctionSinkConfiguration>
<:
insightsTarget" $Lambda Function to deliver results.
:ƒ
à
chimesdkmediapipelinesEMediaInsightsPipelineConfigurationElementS3RecordingSinkConfiguration¶aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementS3RecordingSinkConfiguration:MediaInsightsPipelineConfigurationElementS3RecordingSinkConfiguration7
53
destinationB" S3 URI to deliver recordings.
:º
ˇ
chimesdkmediapipelinesBMediaInsightsPipelineConfigurationElementSnsTopicSinkConfiguration†aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementSnsTopicSinkConfiguration:MediaInsightsPipelineConfigurationElementSnsTopicSinkConfiguration8
64
insightsTarget" SNS topic to deliver results.
:º
ˇ
chimesdkmediapipelinesBMediaInsightsPipelineConfigurationElementSqsQueueSinkConfiguration†aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementSqsQueueSinkConfiguration:MediaInsightsPipelineConfigurationElementSqsQueueSinkConfiguration8
64
insightsTarget" SQS queue to deliver results.
:ò
†
chimesdkmediapipelinesMMediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfiguration∂aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfiguration:MediaInsightsPipelineConfigurationElementVoiceAnalyticsProcessorConfigurations
q2
speakerSearchStatus" Enable speaker search.
;
voiceToneAnalysisStatus" Enable voice tone analysis.
:ﬂ
Ì
chimesdkmediapipelines<MediaInsightsPipelineConfigurationRealTimeAlertConfigurationîaws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationRealTimeAlertConfiguration:MediaInsightsPipelineConfigurationRealTimeAlertConfigurationÏ
È2
disabledB
  Disables real time alert rules.
≤
rulesÇ*ˇ:¸
˘
chimesdkmediapipelines@MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleúaws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRule:MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRule$Collection of real time alert rules
:ﬁ
˘
chimesdkmediapipelines@MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleúaws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRule:MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleﬂ	
‹	†
issueDetectionConfiguration”B–:Õ
 
chimesdkmediapipelines[MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleIssueDetectionConfiguration“aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleIssueDetectionConfiguration:MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleIssueDetectionConfiguration+Configuration for an issue detection rule.
ï
keywordMatchConfigurationÕB :«
ƒ
chimesdkmediapipelinesYMediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfigurationŒaws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfiguration:MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfiguration(Configuration for a keyword match rule.
Ö
sentimentConfigurationƒB¡:æ
ª
chimesdkmediapipelinesVMediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfiguration»aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfiguration:MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfiguration$Configuration for a sentiment rule.

type" Rule type.
:Ó
 
chimesdkmediapipelines[MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleIssueDetectionConfiguration“aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleIssueDetectionConfiguration:MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleIssueDetectionConfiguration

ruleName" Rule name.
:¿
ƒ
chimesdkmediapipelinesYMediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfigurationŒaws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfiguration:MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleKeywordMatchConfigurationw
u3
keywords*" !Collection of keywords to match.
!
negateB
 Negate the rule.

ruleName" Rule name.
:∂
ª
chimesdkmediapipelinesVMediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfiguration»aws:chimesdkmediapipelines/MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfiguration:MediaInsightsPipelineConfigurationRealTimeAlertConfigurationRuleSentimentConfigurationv
t
ruleName" Rule name.
.
sentimentType" Sentiment type to match.
%

timePeriod Analysis interval.
:ˇ
â

cleanrooms#CollaborationDataEncryptionMetadataVaws:cleanrooms/CollaborationDataEncryptionMetadata:CollaborationDataEncryptionMetadataq
o
allowClearText
 
allowDuplicates
 +
%allowJoinsOnColumnsWithDifferentNames
 
preserveNulls
 :¨
Y

cleanroomsCollaborationMember6aws:cleanrooms/CollaborationMember:CollaborationMemberO
M
	accountId" 
displayName" 
memberAbilities*" 
statusB" :¢
w

cleanroomsConfiguredTableTableReferenceJaws:cleanrooms/ConfiguredTableTableReference:ConfiguredTableTableReference'
%
databaseName" 
	tableName" :∞
Ö
cloudformationCloudFormationTypeLoggingConfigRaws:cloudformation/CloudFormationTypeLoggingConfig:CloudFormationTypeLoggingConfig•
¢ç
logGroupName" yName of the CloudWatch Log Group where CloudFormation sends error logging information when invoking the type's handlers.
è

logRoleArn" }Amazon Resource Name (ARN) of the IAM Role CloudFormation assumes when sending error logging information to CloudWatch Logs.
:œ
Ö
cloudformationStackInstancesDeploymentTargetsRaws:cloudformation/StackInstancesDeploymentTargets:StackInstancesDeploymentTargetsƒ
¡ª
accountFilterTypeB" üLimit deployment targets to individual accounts or include additional accounts with provided OUs. Valid values: `INTERSECTION`, `DIFFERENCE`, `UNION`, `NONE`.
B
accountsB*" .List of accounts to deploy stack set updates.
I
accountsUrlB" 4S3 URL of the file containing the list of accounts.
r
organizationalUnitIdsB*" QOrganization root ID or organizational unit (OU) IDs to which stack sets deploy.
:Ñ	
é
cloudformation"StackInstancesOperationPreferencesXaws:cloudformation/StackInstancesOperationPreferences:StackInstancesOperationPreferences
Ì¶
concurrencyModeB" åHow the concurrency level behaves during the operation execution. Valid values are `STRICT_FAILURE_TOLERANCE` and `SOFT_FAILURE_TOLERANCE`.
õ
failureToleranceCountB |Number of accounts, per region, for which this operation can fail before CloudFormation stops the operation in that region.
´
failureTolerancePercentageB ÜPercentage of accounts, per region, for which this stack operation can fail before CloudFormation stops the operation in that region.
g
maxConcurrentCountB KMaximum number of accounts in which to perform this operation at one time.
p
maxConcurrentPercentageB OMaximum percentage of accounts in which to perform this operation at one time.
º
regionConcurrencyTypeB" úConcurrency type of deploying stack sets operations in regions, could be in parallel or one region at a time. Valid values are `SEQUENTIAL` and `PARALLEL`.
\
regionOrdersB*" DOrder of the regions where you want to perform the stack operation.
:Û	
é
cloudformation"StackInstancesStackInstanceSummaryXaws:cloudformation/StackInstancesStackInstanceSummary:StackInstancesStackInstanceSummaryﬂ
‹A
	accountIdB" .Account ID in which the instance is deployed.
≈
detailedStatusB" ¨Detailed status of the stack instance. Values include `PENDING`, `RUNNING`, `SUCCEEDED`, `FAILED`, `CANCELLED`, `INOPERABLE`, `SKIPPED_SUSPENDED_ACCOUNT`, `FAILED_IMPORT`.
Ì
driftStatusB" ◊Status of the stack instance's actual configuration compared to the expected template and parameter configuration of the stack set to which it belongs. Values include `DRIFTED`, `IN_SYNC`, `UNKNOWN`, `NOT_CHECKED`.
Ä
organizationalUnitIdB" bOrganization root ID or organizational unit (OU) IDs that you specified for `deployment_targets`.
C
regionB" 3Region that the stack instance is associated with.
+
stackIdB" ID of the stack instance.
c

stackSetIdB" OName or unique ID of the stack set that the stack instance is associated with.
¢
statusB" ëStatus of the stack instance, in terms of its synchronization with its associated stack set. Values include `CURRENT`, `OUTDATED`, `INOPERABLE`.
`
statusReasonB" JExplanation for the specific status code assigned to this stack instance.
:ï
j
cloudformationStackSetAutoDeployment@aws:cloudformation/StackSetAutoDeployment:StackSetAutoDeployment¶
£<
enabledB
 +Whether or not auto-deployment is enabled.
c
retainStacksOnAccountRemovalB
 =Whether or not to retain stacks when the account is removed.
:’
ã
cloudformation!StackSetInstanceDeploymentTargetsVaws:cloudformation/StackSetInstanceDeploymentTargets:StackSetInstanceDeploymentTargetsƒ
¡ª
accountFilterTypeB" üLimit deployment targets to individual accounts or include additional accounts with provided OUs. Valid values: `INTERSECTION`, `DIFFERENCE`, `UNION`, `NONE`.
B
accountsB*" .List of accounts to deploy stack set updates.
I
accountsUrlB" 4S3 URL of the file containing the list of accounts.
r
organizationalUnitIdsB*" QOrganization root ID or organizational unit (OU) IDs to which StackSets deploys.
:ü	
î
cloudformation$StackSetInstanceOperationPreferences\aws:cloudformation/StackSetInstanceOperationPreferences:StackSetInstanceOperationPreferencesÖ
Ç∞
concurrencyModeB" ñSpecifies how the concurrency level behaves during the operation execution. Valid values are `STRICT_FAILURE_TOLERANCE` and `SOFT_FAILURE_TOLERANCE`.
†
failureToleranceCountB ÄNumber of accounts, per Region, for which this operation can fail before AWS CloudFormation stops the operation in that Region.
Ø
failureTolerancePercentageB äPercentage of accounts, per Region, for which this stack operation can fail before AWS CloudFormation stops the operation in that Region.
g
maxConcurrentCountB KMaximum number of accounts in which to perform this operation at one time.
p
maxConcurrentPercentageB OMaximum percentage of accounts in which to perform this operation at one time.
ª
regionConcurrencyTypeB" õConcurrency type of deploying StackSets operations in Regions, could be in parallel or one Region at a time. Valid values are `SEQUENTIAL` and `PARALLEL`.
_
regionOrdersB*" GOrder of the Regions in where you want to perform the stack operation.
:ä
î
cloudformation$StackSetInstanceStackInstanceSummary\aws:cloudformation/StackSetInstanceStackInstanceSummary:StackSetInstanceStackInstanceSummary
Ìo
	accountIdB" \Target AWS Account ID to create a Stack based on the StackSet. Defaults to current account.
U
organizationalUnitIdB" 7Organizational unit ID in which the stack is deployed.
#
stackIdB" Stack identifier.
:Ë
p
cloudformationStackSetManagedExecutionDaws:cloudformation/StackSetManagedExecution:StackSetManagedExecutionÛ
Ì
activeB
 ‹When set to true, StackSets performs non-conflicting operations concurrently and queues conflicting operations. After conflicting operations finish, StackSets starts queued operations in request order. Default is false.
:º
|
cloudformationStackSetOperationPreferencesLaws:cloudformation/StackSetOperationPreferences:StackSetOperationPreferencesª
∏§
failureToleranceCountB ÑThe number of accounts, per Region, for which this operation can fail before AWS CloudFormation stops the operation in that Region.
≥
failureTolerancePercentageB éThe percentage of accounts, per Region, for which this stack operation can fail before AWS CloudFormation stops the operation in that Region.
k
maxConcurrentCountB OThe maximum number of accounts in which to perform this operation at one time.
t
maxConcurrentPercentageB SThe maximum percentage of accounts in which to perform this operation at one time.
ê
regionConcurrencyTypeB" qThe concurrency type of deploying StackSets operations in Regions, could be in parallel or one Region at a time.
c
regionOrdersB*" KThe order of the Regions in where you want to perform the stack operation.
:°
é
cloudformation"getCloudFormationTypeLoggingConfigXaws:cloudformation/getCloudFormationTypeLoggingConfig:getCloudFormationTypeLoggingConfigç
äç
logGroupName" yName of the CloudWatch Log Group where CloudFormation sends error logging information when invoking the type's handlers.
x

logRoleArn" fARN of the IAM Role CloudFormation assumes when sending error logging information to CloudWatch Logs.
:˘
π

cloudfront3CachePolicyParametersInCacheKeyAndForwardedToOriginvaws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOrigin:CachePolicyParametersInCacheKeyAndForwardedToOrigin∫
∑∂
cookiesConfigÁ:‰
·

cloudfront@CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigêaws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig:CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig∫Whether any cookies in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
Ø
enableAcceptEncodingBrotliB
 äFlag determines whether the Accept-Encoding HTTP header is included in the cache key and in requests that CloudFront sends to the origin.
ô
enableAcceptEncodingGzipB
 wWhether the Accept-Encoding HTTP header is included in the cache key and in requests sent to the origin by CloudFront.
®
headersConfigÁ:‰
·

cloudfront@CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigêaws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig:CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig¨Whether any HTTP headers are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
Ç
queryStringsConfigˆ:Û


cloudfrontECachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigöaws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig:CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigÚWhether any URL query strings in viewer requests are included in the cache key. It also automatically includes these query strings in requests that CloudFront sends to the origin. Please refer to the Query String Config for more information.
:∆
·

cloudfront@CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigêaws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig:CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigﬂ
‹¸
cookieBehavior" ÂWhether any cookies in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values for `cookie_behavior` are `none`, `whitelist`, `allExcept`, and `all`.
⁄
cookiesˇB¸:˘
ˆ

cloudfrontGCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookiesûaws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookies:CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookiesMObject that contains a list of cookie names. See Items for more information.
:–
ˆ

cloudfrontGCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookiesûaws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookies:CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookiesU
SQ
itemsB*" @List of item names, such as cookies, headers, or query strings.
:†
·

cloudfront@CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigêaws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig:CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigπ
∂€
headerBehaviorB" ¬Whether any HTTP headers are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values for `header_behavior` are `none` and `whitelist`.
’
headersˇB¸:˘
ˆ

cloudfrontGCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeadersûaws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeaders:CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeadersHObject contains a list of header names. See Items for more information.
:–
ˆ

cloudfrontGCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeadersûaws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeaders:CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeadersU
SQ
itemsB*" @List of item names, such as cookies, headers, or query strings.
:†


cloudfrontECachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigöaws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig:CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig™
ßç
queryStringBehavior" ÒWhether URL query strings in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values for `query_string_behavior` are `none`, `whitelist`, `allExcept`, and `all`.
î
queryStringsùBö:ó
î

cloudfrontQCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStrings≤aws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStrings:CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStringsdConfiguration parameter that contains a list of query string names. See Items for more information.
:Ó
î

cloudfrontQCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStrings≤aws:cloudfront/CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStrings:CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStringsU
SQ
itemsB*" @List of item names, such as cookies, headers, or query strings.
:Ï
ø

cloudfront5ContinuousDeploymentPolicyStagingDistributionDnsNameszaws:cloudfront/ContinuousDeploymentPolicyStagingDistributionDnsNames:ContinuousDeploymentPolicyStagingDistributionDnsNamesß
§Q
itemsB*" @A list of CloudFront domain names for the staging distribution.
O
quantity ?Number of CloudFront domain names in the staging distribution.
:¶
ï

cloudfront'ContinuousDeploymentPolicyTrafficConfig^aws:cloudfront/ContinuousDeploymentPolicyTrafficConfig:ContinuousDeploymentPolicyTrafficConfigã
àœ
singleHeaderConfig’B“:œ
Ã

cloudfront9ContinuousDeploymentPolicyTrafficConfigSingleHeaderConfigÇaws:cloudfront/ContinuousDeploymentPolicyTrafficConfigSingleHeaderConfig:ContinuousDeploymentPolicyTrafficConfigSingleHeaderConfigaDetermines which HTTP requests are sent to the staging distribution. See `single_header_config`.
“
singleWeightConfig’B“:œ
Ã

cloudfront9ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigÇaws:cloudfront/ContinuousDeploymentPolicyTrafficConfigSingleWeightConfig:ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigdContains the percentage of traffic to send to the staging distribution. See `single_weight_config`.
_
type" SType of traffic configuration. Valid values are `SingleWeight` and `SingleHeader`.
:Ù
Ã

cloudfront9ContinuousDeploymentPolicyTrafficConfigSingleHeaderConfigÇaws:cloudfront/ContinuousDeploymentPolicyTrafficConfigSingleHeaderConfig:ContinuousDeploymentPolicyTrafficConfigSingleHeaderConfig¢
üx
header" jRequest header name to send to the staging distribution. The header must contain the prefix `aws-cf-cd-`.
#
value" Request header value.
:Ç
Ã

cloudfront9ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigÇaws:cloudfront/ContinuousDeploymentPolicyTrafficConfigSingleWeightConfig:ContinuousDeploymentPolicyTrafficConfigSingleWeightConfig∞
≠ß
sessionStickinessConfigöBó:î
ë

cloudfrontPContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfig∞aws:cloudfront/ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfig:ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfigÓSession stickiness provides the ability to define multiple requests from a single viewer as a single session. This prevents the potentially inconsistent experience of sending some of a given user's requests to the staging distribution, while others are sent to the primary distribution. Define the session duration using TTL values. See `session_stickiness_config`.
Ä
weight rThe percentage of traffic to send to a staging distribution, expressed as a decimal number between `0` and `.15`.
:‚
ë

cloudfrontPContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfig∞aws:cloudfront/ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfig:ContinuousDeploymentPolicyTrafficConfigSingleWeightConfigSessionStickinessConfigÀ
»÷
idleTtl ∆The amount of time in seconds after which sessions will cease if no requests are received. Valid values are `300` ‚Äì `3600` (5‚Äì60 minutes). The value must be less than or equal to `maximum_ttl`.
Ï

maximumTtl ŸThe maximum amount of time in seconds to consider requests from the viewer as being part of the same session. Valid values are `300` ‚Äì `3600` (5‚Äì60 minutes). The value must be greater than or equal to `idle_ttl`.
:„
}

cloudfrontDistributionCustomErrorResponseNaws:cloudfront/DistributionCustomErrorResponse:DistributionCustomErrorResponse·
ﬁΩ
errorCachingMinTtlB †Minimum amount of time you want HTTP error codes to stay in CloudFront caches before CloudFront queries your origin to see whether the object has been updated.
I
	errorCode 84xx or 5xx HTTP status code that you want to customize.
t
responseCodeB ^HTTP status code that you want CloudFront to return with the custom error page to the viewer.
[
responsePagePathB" APath of the custom error page (for example, `/custom_404.html`).
:Ú
Ä

cloudfront DistributionDefaultCacheBehaviorPaws:cloudfront/DistributionDefaultCacheBehavior:DistributionDefaultCacheBehaviorÏ
ÈÜ
allowedMethods*" nControls which HTTP methods CloudFront processes and forwards to your Amazon S3 bucket or your custom origin.
Œ
cachePolicyIdB" ∂Unique identifier of the cache policy that is attached to the cache behavior. If configuring the `default_cache_behavior` either `cache_policy_id` or `forwarded_values` must be set.
u
cachedMethods*" ^Controls whether CloudFront caches the response to requests using the specified HTTP methods.
±
compressB
 ûWhether you want CloudFront to automatically compress content for web requests that include `Accept-Encoding: gzip` in the request header (default: `false`).
Œ

defaultTtlB πDefault amount of time (in seconds) that an object is in a CloudFront cache before CloudFront forwards another request in the absence of an `Cache-Control max-age` or `Expires` header.
I
fieldLevelEncryptionIdB" )Field level encryption configuration ID.
«
forwardedValues∂B≥:∞
≠

cloudfront/DistributionDefaultCacheBehaviorForwardedValuesnaws:cloudfront/DistributionDefaultCacheBehaviorForwardedValues:DistributionDefaultCacheBehaviorForwardedValues{The forwarded values configuration that specifies how CloudFront handles query strings, cookies and headers (maximum one).
∂
functionAssociations≈B¬*ø:º
π

cloudfront3DistributionDefaultCacheBehaviorFunctionAssociationvaws:cloudfront/DistributionDefaultCacheBehaviorFunctionAssociation:DistributionDefaultCacheBehaviorFunctionAssociationVA config block that triggers a cloudfront function with specific actions (maximum 2).
À
lambdaFunctionAssociationsÿB’*“:œ
Ã

cloudfront9DistributionDefaultCacheBehaviorLambdaFunctionAssociationÇaws:cloudfront/DistributionDefaultCacheBehaviorLambdaFunctionAssociation:DistributionDefaultCacheBehaviorLambdaFunctionAssociationRA config block that triggers a lambda function with specific actions (maximum 4).
µ
maxTtlB §Maximum amount of time (in seconds) that an object is in a CloudFront cache before CloudFront forwards another request to your origin to determine whether the object has been updated. Only effective in the presence of `Cache-Control max-age`, `Cache-Control s-maxage`, and `Expires` headers.
ƒ
minTtlB ≥Minimum amount of time that you want objects to stay in CloudFront caches before CloudFront queries your origin to see whether the object has been updated. Defaults to 0 seconds.
p
originRequestPolicyIdB" QUnique identifier of the origin request policy that is attached to the behavior.
n
realtimeLogConfigArnB" PARN of the real-time log configuration that is attached to this cache behavior.
K
responseHeadersPolicyIdB" *Identifier for a response headers policy.
≤
smoothStreamingB
 òIndicates whether you want to distribute media files in Microsoft Smooth Streaming format using the origin that is associated with this cache behavior.
«
targetOriginId" ∞Value of ID for the origin that you want CloudFront to route requests to when a request matches the path pattern either for a cache behavior or for the default cache behavior.
ü
trustedKeyGroupsB*" ÇList of nested attributes for active trusted key groups, if the distribution is set up to serve private content with signed URLs.
ô
trustedSignersB*" List of nested attributes for active trusted signers, if the distribution is set up to serve private content with signed URLs.
â
viewerProtocolPolicy" ÏUse this element to specify the protocol that users can use to access the files in the origin specified by TargetOriginId when a request matches the path pattern in PathPattern. One of `allow-all`, `https-only`, or `redirect-to-https`.
:™
≠

cloudfront/DistributionDefaultCacheBehaviorForwardedValuesnaws:cloudfront/DistributionDefaultCacheBehaviorForwardedValues:DistributionDefaultCacheBehaviorForwardedValues˜
Ù∞
cookies»:≈
¬

cloudfront6DistributionDefaultCacheBehaviorForwardedValuesCookies|aws:cloudfront/DistributionDefaultCacheBehaviorForwardedValuesCookies:DistributionDefaultCacheBehaviorForwardedValuesCookiesZThe forwarded values cookies that specifies how CloudFront handles cookies (maximum one).
á
headersB*" tHeaders, if any, that you want CloudFront to vary upon for this cache behavior. Specify `*` to include all headers.
ç
queryString
 zIndicates whether you want CloudFront to forward query strings to the origin that is associated with this cache behavior.
§
queryStringCacheKeysB*" ÉWhen specified, along with a value of `true` for `query_string`, all query strings are forwarded, however only the query string keys listed in this argument are cached. When omitted with a value of `true` for `query_string`, all query string keys are cached.
:÷
¬

cloudfront6DistributionDefaultCacheBehaviorForwardedValuesCookies|aws:cloudfront/DistributionDefaultCacheBehaviorForwardedValuesCookies:DistributionDefaultCacheBehaviorForwardedValuesCookiesé
ãÌ
forward" ›Whether you want CloudFront to forward cookies to the origin that is associated with this cache behavior. You can specify `all`, `none` or `whitelist`. If `whitelist`, you must include the subsequent `whitelisted_names`.
ò
whitelistedNamesB*" |If you have specified `whitelist` to `forward`, the whitelisted cookies that you want CloudFront to forward to your origin.
:Ë
π

cloudfront3DistributionDefaultCacheBehaviorFunctionAssociationvaws:cloudfront/DistributionDefaultCacheBehaviorFunctionAssociation:DistributionDefaultCacheBehaviorFunctionAssociation©
¶o
	eventType" ^Specific event to trigger this function. Valid values: `viewer-request` or `viewer-response`.
3
functionArn"  ARN of the CloudFront function.
:©
Ã

cloudfront9DistributionDefaultCacheBehaviorLambdaFunctionAssociationÇaws:cloudfront/DistributionDefaultCacheBehaviorLambdaFunctionAssociation:DistributionDefaultCacheBehaviorLambdaFunctionAssociation◊
‘ì
	eventType" ÅSpecific event to trigger this function. Valid values: `viewer-request`, `origin-request`, `viewer-response`, `origin-response`.
å
includeBodyB
 wWhen set to true it exposes the request body to the lambda function. Defaults to false. Valid values: `true`, `false`.
-
	lambdaArn" ARN of the Lambda function.
:ü
k

cloudfrontDistributionLoggingConfigBaws:cloudfront/DistributionLoggingConfig:DistributionLoggingConfigØ
¨l
bucket" ^Amazon S3 bucket to store the access logs in, for example, `myawslogbucket.s3.amazonaws.com`.
V
includeCookiesB
 >Whether to include cookies in access logs (default: `false`).
d
prefixB" TPrefix to the access log filenames for this distribution, for example, `myprefix/`.
:ˆ
Ä

cloudfront DistributionOrderedCacheBehaviorPaws:cloudfront/DistributionOrderedCacheBehavior:DistributionOrderedCacheBehavior
ÌÜ
allowedMethods*" nControls which HTTP methods CloudFront processes and forwards to your Amazon S3 bucket or your custom origin.
Œ
cachePolicyIdB" ∂Unique identifier of the cache policy that is attached to the cache behavior. If configuring the `default_cache_behavior` either `cache_policy_id` or `forwarded_values` must be set.
u
cachedMethods*" ^Controls whether CloudFront caches the response to requests using the specified HTTP methods.
±
compressB
 ûWhether you want CloudFront to automatically compress content for web requests that include `Accept-Encoding: gzip` in the request header (default: `false`).
Œ

defaultTtlB πDefault amount of time (in seconds) that an object is in a CloudFront cache before CloudFront forwards another request in the absence of an `Cache-Control max-age` or `Expires` header.
I
fieldLevelEncryptionIdB" )Field level encryption configuration ID.
«
forwardedValues∂B≥:∞
≠

cloudfront/DistributionOrderedCacheBehaviorForwardedValuesnaws:cloudfront/DistributionOrderedCacheBehaviorForwardedValues:DistributionOrderedCacheBehaviorForwardedValues{The forwarded values configuration that specifies how CloudFront handles query strings, cookies and headers (maximum one).
∂
functionAssociations≈B¬*ø:º
π

cloudfront3DistributionOrderedCacheBehaviorFunctionAssociationvaws:cloudfront/DistributionOrderedCacheBehaviorFunctionAssociation:DistributionOrderedCacheBehaviorFunctionAssociationVA config block that triggers a cloudfront function with specific actions (maximum 2).
À
lambdaFunctionAssociationsÿB’*“:œ
Ã

cloudfront9DistributionOrderedCacheBehaviorLambdaFunctionAssociationÇaws:cloudfront/DistributionOrderedCacheBehaviorLambdaFunctionAssociation:DistributionOrderedCacheBehaviorLambdaFunctionAssociationRA config block that triggers a lambda function with specific actions (maximum 4).
µ
maxTtlB §Maximum amount of time (in seconds) that an object is in a CloudFront cache before CloudFront forwards another request to your origin to determine whether the object has been updated. Only effective in the presence of `Cache-Control max-age`, `Cache-Control s-maxage`, and `Expires` headers.
ƒ
minTtlB ≥Minimum amount of time that you want objects to stay in CloudFront caches before CloudFront queries your origin to see whether the object has been updated. Defaults to 0 seconds.
p
originRequestPolicyIdB" QUnique identifier of the origin request policy that is attached to the behavior.
Å
pathPattern" nPattern (for example, `images/*.jpg`) that specifies which requests you want this cache behavior to apply to.
n
realtimeLogConfigArnB" PARN of the real-time log configuration that is attached to this cache behavior.
K
responseHeadersPolicyIdB" *Identifier for a response headers policy.
≤
smoothStreamingB
 òIndicates whether you want to distribute media files in Microsoft Smooth Streaming format using the origin that is associated with this cache behavior.
«
targetOriginId" ∞Value of ID for the origin that you want CloudFront to route requests to when a request matches the path pattern either for a cache behavior or for the default cache behavior.
ü
trustedKeyGroupsB*" ÇList of nested attributes for active trusted key groups, if the distribution is set up to serve private content with signed URLs.
ô
trustedSignersB*" List of nested attributes for active trusted signers, if the distribution is set up to serve private content with signed URLs.
â
viewerProtocolPolicy" ÏUse this element to specify the protocol that users can use to access the files in the origin specified by TargetOriginId when a request matches the path pattern in PathPattern. One of `allow-all`, `https-only`, or `redirect-to-https`.
:™
≠

cloudfront/DistributionOrderedCacheBehaviorForwardedValuesnaws:cloudfront/DistributionOrderedCacheBehaviorForwardedValues:DistributionOrderedCacheBehaviorForwardedValues˜
Ù∞
cookies»:≈
¬

cloudfront6DistributionOrderedCacheBehaviorForwardedValuesCookies|aws:cloudfront/DistributionOrderedCacheBehaviorForwardedValuesCookies:DistributionOrderedCacheBehaviorForwardedValuesCookiesZThe forwarded values cookies that specifies how CloudFront handles cookies (maximum one).
á
headersB*" tHeaders, if any, that you want CloudFront to vary upon for this cache behavior. Specify `*` to include all headers.
ç
queryString
 zIndicates whether you want CloudFront to forward query strings to the origin that is associated with this cache behavior.
§
queryStringCacheKeysB*" ÉWhen specified, along with a value of `true` for `query_string`, all query strings are forwarded, however only the query string keys listed in this argument are cached. When omitted with a value of `true` for `query_string`, all query string keys are cached.
:÷
¬

cloudfront6DistributionOrderedCacheBehaviorForwardedValuesCookies|aws:cloudfront/DistributionOrderedCacheBehaviorForwardedValuesCookies:DistributionOrderedCacheBehaviorForwardedValuesCookiesé
ãÌ
forward" ›Whether you want CloudFront to forward cookies to the origin that is associated with this cache behavior. You can specify `all`, `none` or `whitelist`. If `whitelist`, you must include the subsequent `whitelisted_names`.
ò
whitelistedNamesB*" |If you have specified `whitelist` to `forward`, the whitelisted cookies that you want CloudFront to forward to your origin.
:Ë
π

cloudfront3DistributionOrderedCacheBehaviorFunctionAssociationvaws:cloudfront/DistributionOrderedCacheBehaviorFunctionAssociation:DistributionOrderedCacheBehaviorFunctionAssociation©
¶o
	eventType" ^Specific event to trigger this function. Valid values: `viewer-request` or `viewer-response`.
3
functionArn"  ARN of the CloudFront function.
:©
Ã

cloudfront9DistributionOrderedCacheBehaviorLambdaFunctionAssociationÇaws:cloudfront/DistributionOrderedCacheBehaviorLambdaFunctionAssociation:DistributionOrderedCacheBehaviorLambdaFunctionAssociation◊
‘ì
	eventType" ÅSpecific event to trigger this function. Valid values: `viewer-request`, `origin-request`, `viewer-response`, `origin-response`.
å
includeBodyB
 wWhen set to true it exposes the request body to the lambda function. Defaults to false. Valid values: `true`, `false`.
-
	lambdaArn" ARN of the Lambda function.
:ã
V

cloudfrontDistributionOrigin4aws:cloudfront/DistributionOrigin:DistributionOrigin∞
≠É
connectionAttemptsB gNumber of times that CloudFront attempts to connect to the origin. Must be between 1-3. Defaults to 3.
ü
connectionTimeoutB ÉNumber of seconds that CloudFront waits when trying to establish a connection to the origin. Must be between 1-10. Defaults to 10.
£
customHeadersÉBÄ*~:|
z

cloudfrontDistributionOriginCustomHeaderLaws:cloudfront/DistributionOriginCustomHeader:DistributionOriginCustomHeaderãOne or more sub-resources with `name` and `value` parameters that specify header data that will be sent to the origin (multiples allowed).
¬
customOriginConfigïBí:è
å

cloudfront$DistributionOriginCustomOriginConfigXaws:cloudfront/DistributionOriginCustomOriginConfig:DistributionOriginCustomOriginConfigìThe CloudFront custom origin configuration information. If an S3 origin is required, use `origin_access_control_id` or `s3_origin_config` instead.
o

domainName" ]Domain name corresponding to the distribution. For example: `d604721fxaaqy9.cloudfront.net`.
m
originAccessControlIdB" NUnique identifier of a [CloudFront origin access control][8] for this origin.

originId" ñ

originPathB" ÅOptional element that causes CloudFront to request your content from a directory in your Amazon S3 bucket or your custom origin.
∂
originShieldÄB~:|
z

cloudfrontDistributionOriginOriginShieldLaws:cloudfront/DistributionOriginOriginShield:DistributionOriginOriginShield¢CloudFront Origin Shield configuration information. Using Origin Shield can help reduce the load on your origin. For more information, see [Using Origin Shield](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/origin-shield.html) in the Amazon CloudFront Developer Guide.
í
s3OriginConfigâBÜ:É
Ä

cloudfront DistributionOriginS3OriginConfigPaws:cloudfront/DistributionOriginS3OriginConfig:DistributionOriginS3OriginConfigtCloudFront S3 origin configuration information. If a custom origin is required, use `custom_origin_config` instead.
¿
vpcOriginConfigåBâ:Ü
É

cloudfront!DistributionOriginVpcOriginConfigRaws:cloudfront/DistributionOriginVpcOriginConfig:DistributionOriginVpcOriginConfigThe VPC origin configuration.
:ô
z

cloudfrontDistributionOriginCustomHeaderLaws:cloudfront/DistributionOriginCustomHeader:DistributionOriginCustomHeader


name" 
value" :Ì
å

cloudfront$DistributionOriginCustomOriginConfigXaws:cloudfront/DistributionOriginCustomOriginConfig:DistributionOriginCustomOriginConfig€
ÿ8
httpPort (HTTP port the custom origin listens on.
:
	httpsPort )HTTPS port the custom origin listens on.

originKeepaliveTimeoutB Å
originProtocolPolicy" eOrigin protocol policy to apply to your origin. One of `http-only`, `https-only`, or `match-viewer`.

originReadTimeoutB †
originSslProtocols*" ÉList of SSL/TLS protocols that CloudFront can use when connecting to your origin over HTTPS. Valid values: `SSLv3`, `TLSv1`, `TLSv1.1`, `TLSv1.2`. For more information, see [Minimum Origin SSL Protocol](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/distribution-web-values-specify.html#DownloadDistValuesOriginSSLProtocols) in the Amazon CloudFront Developer Guide.
:í
e

cloudfrontDistributionOriginGroup>aws:cloudfront/DistributionOriginGroup:DistributionOriginGroup®
•ˆ
failoverCriteriaõ:ò
ï

cloudfront'DistributionOriginGroupFailoverCriteria^aws:cloudfront/DistributionOriginGroupFailoverCriteria:DistributionOriginGroupFailoverCriteriaDThe failover criteria for when to failover to the secondary origin.
ô
members}*{:y
w

cloudfrontDistributionOriginGroupMemberJaws:cloudfront/DistributionOriginGroupMember:DistributionOriginGroupMemberéOrdered member configuration blocks assigned to the origin group, where the first member is the primary origin. You must specify two members.

originId" :„
ï

cloudfront'DistributionOriginGroupFailoverCriteria^aws:cloudfront/DistributionOriginGroupFailoverCriteria:DistributionOriginGroupFailoverCriteriaI
GE
statusCodes* 0List of HTTP status codes for the origin group.
:ç
w

cloudfrontDistributionOriginGroupMemberJaws:cloudfront/DistributionOriginGroupMember:DistributionOriginGroupMember

originId" :¨
z

cloudfrontDistributionOriginOriginShieldLaws:cloudfront/DistributionOriginOriginShield:DistributionOriginOriginShield≠
™l
enabled
 ]`true` if any of the AWS accounts listed as trusted signers have active CloudFront key pairs
π
originShieldRegionB" úAWS Region for Origin Shield. To specify a region, use the region code, not the region name. For example, specify the US East (Ohio) region as `us-east-2`.
:È
Ä

cloudfront DistributionOriginS3OriginConfigPaws:cloudfront/DistributionOriginS3OriginConfig:DistributionOriginS3OriginConfigd
b`
originAccessIdentity" DThe CloudFront origin access identity to associate with the origin.
:Ì
É

cloudfront!DistributionOriginVpcOriginConfigRaws:cloudfront/DistributionOriginVpcOriginConfig:DistributionOriginVpcOriginConfige
c
originKeepaliveTimeoutB 
originReadTimeoutB &
vpcOriginId" The VPC origin ID.
:û
h

cloudfrontDistributionRestrictions@aws:cloudfront/DistributionRestrictions:DistributionRestrictions±
Æ´
geoRestrictionò:ï
í

cloudfront&DistributionRestrictionsGeoRestriction\aws:cloudfront/DistributionRestrictionsGeoRestriction:DistributionRestrictionsGeoRestriction:ù
í

cloudfront&DistributionRestrictionsGeoRestriction\aws:cloudfront/DistributionRestrictionsGeoRestriction:DistributionRestrictionsGeoRestrictionÖ
ÇÔ
	locationsB*" Ÿ[ISO 3166-1-alpha-2 codes][4] for which you want CloudFront either to distribute your content (`whitelist`) or not distribute your content (`blacklist`). If the type is specified as `none` an empty array can be used.
ç
restrictionType" vMethod that you want to use to restrict distribution of your content by country: `none`, `whitelist`, or `blacklist`.
:±
q

cloudfrontDistributionTrustedKeyGroupFaws:cloudfront/DistributionTrustedKeyGroup:DistributionTrustedKeyGroupª
∏n
enabledB
 ]`true` if any of the AWS accounts listed as trusted signers have active CloudFront key pairs
≈
itemsáBÑ*Å:
}

cloudfrontDistributionTrustedKeyGroupItemNaws:cloudfront/DistributionTrustedKeyGroupItem:DistributionTrustedKeyGroupItem2List of nested attributes for each trusted signer
:¨
}

cloudfrontDistributionTrustedKeyGroupItemNaws:cloudfront/DistributionTrustedKeyGroupItem:DistributionTrustedKeyGroupItem™
ßG

keyGroupIdB" 3ID of the key group that contains the public keys.
\

keyPairIdsB*" FSet of active CloudFront key pairs associated with the signer account
:¢
k

cloudfrontDistributionTrustedSignerBaws:cloudfront/DistributionTrustedSigner:DistributionTrustedSigner≤
Øn
enabledB
 ]`true` if any of the AWS accounts listed as trusted signers have active CloudFront key pairs
º
itemsB}*{:y
w

cloudfrontDistributionTrustedSignerItemJaws:cloudfront/DistributionTrustedSignerItem:DistributionTrustedSignerItem2List of nested attributes for each trusted signer
:í
w

cloudfrontDistributionTrustedSignerItemJaws:cloudfront/DistributionTrustedSignerItem:DistributionTrustedSignerItemñ
ì3
awsAccountNumberB" AWS account ID or `self`
\

keyPairIdsB*" FSet of active CloudFront key pairs associated with the signer account
:«
w

cloudfrontDistributionViewerCertificateJaws:cloudfront/DistributionViewerCertificate:DistributionViewerCertificateÀ
»ú
acmCertificateArnB" ÄARN of the [AWS Certificate Manager](https://aws.amazon.com/certificate-manager/) certificate that you wish to use with this distribution. Specify this, `cloudfront_default_certificate`, or `iam_certificate_id`.  The ACM certificate must be in  US-EAST-1.
Ë
cloudfrontDefaultCertificateB
 ¡`true` if you want viewers to use HTTPS to request your objects and you're using the CloudFront domain name for your distribution. Specify this, `acm_certificate_arn`, or `iam_certificate_id`.
Ÿ
iamCertificateIdB" æIAM certificate identifier of the custom viewer certificate for this distribution if you are using a custom domain. Specify this, `acm_certificate_arn`, or `cloudfront_default_certificate`.
¬
minimumProtocolVersionB" °Minimum version of the SSL protocol that you want CloudFront to use for HTTPS connections. Can only be set if `cloudfront_default_certificate = false`. See all possible values in [this](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/secure-connections-supported-viewer-protocols-ciphers.html) table under "Security policy." Some examples include: `TLSv1.2_2019` and `TLSv1.2_2021`. Default: `TLSv1`. **NOTE**: If you are using a custom certificate (specified with `acm_certificate_arn` or `iam_certificate_id`), and have specified `sni-only` in `ssl_support_method`, `TLSv1` or later must be specified. If you have specified `vip` in `ssl_support_method`, only `SSLv3` or `TLSv1` can be specified. If you have specified `cloudfront_default_certificate`, `TLSv1` must be specified.
ö
sslSupportMethodB" ˇHow you want CloudFront to serve HTTPS requests. One of `vip`, `sni-only`, or `static-ip`. Required if you specify `acm_certificate_arn` or `iam_certificate_id`. **NOTE:** `vip` causes CloudFront to use a dedicated IP address and may incur extra charges.
:î
∂

cloudfront2FieldLevelEncryptionConfigContentTypeProfileConfigtaws:cloudfront/FieldLevelEncryptionConfigContentTypeProfileConfig:FieldLevelEncryptionConfigContentTypeProfileConfigÿ
’±
contentTypeProfilesˆ:Û


cloudfrontEFieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesöaws:cloudfront/FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfiles:FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfiles†Object that contains an attribute `items` that contains the list of configurations for a field-level encryption content type-profile. See Content Type Profile.
û
forwardWhenContentTypeIsUnknown
 ˆspecifies what to do when an unknown content type is provided for the profile. If true, content is forwarded without being encrypted when the content type is unknown. If false (the default), an error is returned when the content type is unknown.
:ã


cloudfrontEFieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesöaws:cloudfront/FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfiles:FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesï
íè
itemsÖ*Ç:ˇ
¸

cloudfrontIFieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesItem¢aws:cloudfront/FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesItem:FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesItem:†
¸

cloudfrontIFieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesItem¢aws:cloudfront/FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesItem:FieldLevelEncryptionConfigContentTypeProfileConfigContentTypeProfilesItemû
õí
contentType" he content type for a field-level encryption content type-profile mapping. Valid value is `application/x-www-form-urlencoded`.
q
format" cThe format for a field-level encryption content type-profile mapping. Valid value is `URLEncoded`.

	profileIdB" :∑
≠

cloudfront/FieldLevelEncryptionConfigQueryArgProfileConfignaws:cloudfront/FieldLevelEncryptionConfigQueryArgProfileConfig:FieldLevelEncryptionConfigQueryArgProfileConfigÑ
Å—
#forwardWhenQueryArgProfileIsUnknown
 •Flag to set if you want a request to be forwarded to the origin even if the profile specified by the field-level encryption query argument, fle-profile, is unknown.
™
queryArgProfilesÁB‰:·
ﬁ

cloudfront?FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfileséaws:cloudfront/FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfiles:FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfiles´Object that contains an attribute `items` that contains the list ofrofiles specified for query argument-profile mapping for field-level encryption. see Query Arg Profile.
:Í
ﬁ

cloudfront?FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfileséaws:cloudfront/FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfiles:FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesÜ
ÉÄ
itemsˆBÛ*:Ì
Í

cloudfrontCFieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesItemñaws:cloudfront/FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesItem:FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesItem:ﬁ
Í

cloudfrontCFieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesItemñaws:cloudfront/FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesItem:FieldLevelEncryptionConfigQueryArgProfileConfigQueryArgProfilesItemo
m
	profileId" Z
queryArg" JQuery argument for field-level encryption query argument-profile mapping.
:¸
ß

cloudfront-FieldLevelEncryptionProfileEncryptionEntitiesjaws:cloudfront/FieldLevelEncryptionProfileEncryptionEntities:FieldLevelEncryptionProfileEncryptionEntitiesœ
Ã…
itemsøBº*π:∂
≥

cloudfront1FieldLevelEncryptionProfileEncryptionEntitiesItemraws:cloudfront/FieldLevelEncryptionProfileEncryptionEntitiesItem:FieldLevelEncryptionProfileEncryptionEntitiesItem:Â
≥

cloudfront1FieldLevelEncryptionProfileEncryptionEntitiesItemraws:cloudfront/FieldLevelEncryptionProfileEncryptionEntitiesItem:FieldLevelEncryptionProfileEncryptionEntitiesItem¨
©¨
fieldPatterns·:ﬁ
€

cloudfront>FieldLevelEncryptionProfileEncryptionEntitiesItemFieldPatternsåaws:cloudfront/FieldLevelEncryptionProfileEncryptionEntitiesItemFieldPatterns:FieldLevelEncryptionProfileEncryptionEntitiesItemFieldPatterns∂Object that contains an attribute `items` that contains the list of field patterns in a field-level encryption content type profile specify the fields that you want to be encrypted.
Y

providerId" GThe provider associated with the public key being used for encryption.
ú
publicKeyId" àThe public key associated with a set of field-level encryption patterns, to be used when encrypting the fields that match the patterns.
:Û
€

cloudfront>FieldLevelEncryptionProfileEncryptionEntitiesItemFieldPatternsåaws:cloudfront/FieldLevelEncryptionProfileEncryptionEntitiesItemFieldPatterns:FieldLevelEncryptionProfileEncryptionEntitiesItemFieldPatterns

itemsB*" :—
_

cloudfrontKeyValueStoreTimeouts:aws:cloudfront/KeyValueStoreTimeouts:KeyValueStoreTimeoutsÌ
ÍÁ
createB" ÷A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:±
§

cloudfront,MonitoringSubscriptionMonitoringSubscriptionhaws:cloudfront/MonitoringSubscriptionMonitoringSubscription:MonitoringSubscriptionMonitoringSubscriptioná
ÑÅ
!realtimeMetricsSubscriptionConfigé:ã
à

cloudfrontMMonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfig™aws:cloudfront/MonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfig:MonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfigKA subscription configuration for additional CloudWatch metrics. See below.
:·
à

cloudfrontMMonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfig™aws:cloudfront/MonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfig:MonitoringSubscriptionMonitoringSubscriptionRealtimeMetricsSubscriptionConfig”
–Õ
!realtimeMetricsSubscriptionStatus" £A flag that indicates whether additional CloudWatch metrics are enabled for a given CloudFront distribution. Valid values are `Enabled` and `Disabled`. See below.
:Ã
Ä

cloudfront OriginRequestPolicyCookiesConfigPaws:cloudfront/OriginRequestPolicyCookiesConfig:OriginRequestPolicyCookiesConfig∆
√
cookieBehavior" ™
cookiesûBõ:ò
ï

cloudfront'OriginRequestPolicyCookiesConfigCookies^aws:cloudfront/OriginRequestPolicyCookiesConfigCookies:OriginRequestPolicyCookiesConfigCookies:≠
ï

cloudfront'OriginRequestPolicyCookiesConfigCookies^aws:cloudfront/OriginRequestPolicyCookiesConfigCookies:OriginRequestPolicyCookiesConfigCookies

itemsB*" :Œ
Ä

cloudfront OriginRequestPolicyHeadersConfigPaws:cloudfront/OriginRequestPolicyHeadersConfig:OriginRequestPolicyHeadersConfig»
≈
headerBehaviorB" ™
headersûBõ:ò
ï

cloudfront'OriginRequestPolicyHeadersConfigHeaders^aws:cloudfront/OriginRequestPolicyHeadersConfigHeaders:OriginRequestPolicyHeadersConfigHeaders:≠
ï

cloudfront'OriginRequestPolicyHeadersConfigHeaders^aws:cloudfront/OriginRequestPolicyHeadersConfigHeaders:OriginRequestPolicyHeadersConfigHeaders

itemsB*" :É
è

cloudfront%OriginRequestPolicyQueryStringsConfigZaws:cloudfront/OriginRequestPolicyQueryStringsConfig:OriginRequestPolicyQueryStringsConfigÓ
Î
queryStringBehavior" Õ
queryStringsºBπ:∂
≥

cloudfront1OriginRequestPolicyQueryStringsConfigQueryStringsraws:cloudfront/OriginRequestPolicyQueryStringsConfigQueryStrings:OriginRequestPolicyQueryStringsConfigQueryStrings:À
≥

cloudfront1OriginRequestPolicyQueryStringsConfigQueryStringsraws:cloudfront/OriginRequestPolicyQueryStringsConfigQueryStrings:OriginRequestPolicyQueryStringsConfigQueryStrings

itemsB*" :Ÿ
k

cloudfrontRealtimeLogConfigEndpointBaws:cloudfront/RealtimeLogConfigEndpoint:RealtimeLogConfigEndpointÈ
ÊÚ
kinesisStreamConfig™:ß
§

cloudfront,RealtimeLogConfigEndpointKinesisStreamConfighaws:cloudfront/RealtimeLogConfigEndpointKinesisStreamConfig:RealtimeLogConfigEndpointKinesisStreamConfig.The Amazon Kinesis data stream configuration.
o

streamType" ]The type of data stream where real-time log data is sent. The only valid value is `Kinesis`.
:ê
§

cloudfront,RealtimeLogConfigEndpointKinesisStreamConfighaws:cloudfront/RealtimeLogConfigEndpointKinesisStreamConfig:RealtimeLogConfigEndpointKinesisStreamConfigÊ
„©
roleArn" ôThe ARN of an IAM role that CloudFront can use to send real-time log data to the Kinesis data stream.
See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-iam-role) for more information.
5
	streamArn" $The ARN of the Kinesis data stream.
:
}

cloudfrontResponseHeadersPolicyCorsConfigNaws:cloudfront/ResponseHeadersPolicyCorsConfig:ResponseHeadersPolicyCorsConfigÓ
Îò
accessControlAllowCredentials
 sA Boolean value that CloudFront uses as the value for the `Access-Control-Allow-Credentials` HTTP response header.
§
accessControlAllowHeadersœ:Ã
…

cloudfront8ResponseHeadersPolicyCorsConfigAccessControlAllowHeadersÄaws:cloudfront/ResponseHeadersPolicyCorsConfigAccessControlAllowHeaders:ResponseHeadersPolicyCorsConfigAccessControlAllowHeaders¥Object that contains an attribute `items` that contains a list of HTTP header names that CloudFront includes as values for the `Access-Control-Allow-Headers` HTTP response header.
Ï
accessControlAllowMethodsœ:Ã
…

cloudfront8ResponseHeadersPolicyCorsConfigAccessControlAllowMethodsÄaws:cloudfront/ResponseHeadersPolicyCorsConfigAccessControlAllowMethods:ResponseHeadersPolicyCorsConfigAccessControlAllowMethods¸Object that contains an attribute `items` that contains a list of HTTP methods that CloudFront includes as values for the `Access-Control-Allow-Methods` HTTP response header. Valid values: `GET` | `POST` | `OPTIONS` | `PUT` | `DELETE` | `HEAD` | `ALL`
õ
accessControlAllowOriginsœ:Ã
…

cloudfront8ResponseHeadersPolicyCorsConfigAccessControlAllowOriginsÄaws:cloudfront/ResponseHeadersPolicyCorsConfigAccessControlAllowOrigins:ResponseHeadersPolicyCorsConfigAccessControlAllowOrigins´Object that contains an attribute `items` that contains a list of origins that CloudFront can use as the value for the `Access-Control-Allow-Origin` HTTP response header.
ß
accessControlExposeHeaders’B“:œ
Ã

cloudfront9ResponseHeadersPolicyCorsConfigAccessControlExposeHeadersÇaws:cloudfront/ResponseHeadersPolicyCorsConfigAccessControlExposeHeaders:ResponseHeadersPolicyCorsConfigAccessControlExposeHeaders∞Object that contains an attribute `items` that contains a list of HTTP headers that CloudFront includes as values for the `Access-Control-Expose-Headers` HTTP response header.
Ç
accessControlMaxAgeSecB bA number that CloudFront uses as the value for the `Access-Control-Max-Age` HTTP response header.
k
originOverride
 UA Boolean value that determines how CloudFront behaves for the HTTP response header.
:·
…

cloudfront8ResponseHeadersPolicyCorsConfigAccessControlAllowHeadersÄaws:cloudfront/ResponseHeadersPolicyCorsConfigAccessControlAllowHeaders:ResponseHeadersPolicyCorsConfigAccessControlAllowHeaders

itemsB*" :·
…

cloudfront8ResponseHeadersPolicyCorsConfigAccessControlAllowMethodsÄaws:cloudfront/ResponseHeadersPolicyCorsConfigAccessControlAllowMethods:ResponseHeadersPolicyCorsConfigAccessControlAllowMethods

itemsB*" :·
…

cloudfront8ResponseHeadersPolicyCorsConfigAccessControlAllowOriginsÄaws:cloudfront/ResponseHeadersPolicyCorsConfigAccessControlAllowOrigins:ResponseHeadersPolicyCorsConfigAccessControlAllowOrigins

itemsB*" :‰
Ã

cloudfront9ResponseHeadersPolicyCorsConfigAccessControlExposeHeadersÇaws:cloudfront/ResponseHeadersPolicyCorsConfigAccessControlExposeHeaders:ResponseHeadersPolicyCorsConfigAccessControlExposeHeaders

itemsB*" :ﬁ
ò

cloudfront(ResponseHeadersPolicyCustomHeadersConfig`aws:cloudfront/ResponseHeadersPolicyCustomHeadersConfig:ResponseHeadersPolicyCustomHeadersConfig¿
Ω∫
items∞B≠*™:ß
§

cloudfront,ResponseHeadersPolicyCustomHeadersConfigItemhaws:cloudfront/ResponseHeadersPolicyCustomHeadersConfigItem:ResponseHeadersPolicyCustomHeadersConfigItem:Ä
§

cloudfront,ResponseHeadersPolicyCustomHeadersConfigItemhaws:cloudfront/ResponseHeadersPolicyCustomHeadersConfigItem:ResponseHeadersPolicyCustomHeadersConfigItemW
U
header" 
override
 5
value" (The value for the HTTP response header.
:ﬁ
ò

cloudfront(ResponseHeadersPolicyRemoveHeadersConfig`aws:cloudfront/ResponseHeadersPolicyRemoveHeadersConfig:ResponseHeadersPolicyRemoveHeadersConfig¿
Ω∫
items∞B≠*™:ß
§

cloudfront,ResponseHeadersPolicyRemoveHeadersConfigItemhaws:cloudfront/ResponseHeadersPolicyRemoveHeadersConfigItem:ResponseHeadersPolicyRemoveHeadersConfigItem:π
§

cloudfront,ResponseHeadersPolicyRemoveHeadersConfigItemhaws:cloudfront/ResponseHeadersPolicyRemoveHeadersConfigItem:ResponseHeadersPolicyRemoveHeadersConfigItem

header" :±
û

cloudfront*ResponseHeadersPolicySecurityHeadersConfigdaws:cloudfront/ResponseHeadersPolicySecurityHeadersConfig:ResponseHeadersPolicySecurityHeadersConfigç
ä∏
contentSecurityPolicyÁB‰:·
ﬁ

cloudfront?ResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicyéaws:cloudfront/ResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicy:ResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicy¥The policy directives and their values that CloudFront includes as values for the `Content-Security-Policy` HTTP response header. See Content Security Policy for more information.
°
contentTypeOptionsﬁB€:ÿ
’

cloudfront<ResponseHeadersPolicySecurityHeadersConfigContentTypeOptionsàaws:cloudfront/ResponseHeadersPolicySecurityHeadersConfigContentTypeOptions:ResponseHeadersPolicySecurityHeadersConfigContentTypeOptions©Determines whether CloudFront includes the `X-Content-Type-Options` HTTP response header with its value set to `nosniff`. See Content Type Options for more information.
Û
frameOptionsÀB»:≈
¬

cloudfront6ResponseHeadersPolicySecurityHeadersConfigFrameOptions|aws:cloudfront/ResponseHeadersPolicySecurityHeadersConfigFrameOptions:ResponseHeadersPolicySecurityHeadersConfigFrameOptionsîDetermines whether CloudFront includes the `X-Frame-Options` HTTP response header and the header‚Äôs value. See Frame Options for more information.
˛
referrerPolicy“Bœ:Ã
…

cloudfront8ResponseHeadersPolicySecurityHeadersConfigReferrerPolicyÄaws:cloudfront/ResponseHeadersPolicySecurityHeadersConfigReferrerPolicy:ResponseHeadersPolicySecurityHeadersConfigReferrerPolicyñDetermines whether CloudFront includes the `Referrer-Policy` HTTP response header and the header‚Äôs value. See Referrer Policy for more information.
∂
strictTransportSecurityÌBÍ:Á
‰

cloudfrontAResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurityíaws:cloudfront/ResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity:ResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity™Determines whether CloudFront includes the `Strict-Transport-Security` HTTP response header and the header‚Äôs value. See Strict Transport Security for more information.
¯
xssProtectionŒBÀ:»
≈

cloudfront7ResponseHeadersPolicySecurityHeadersConfigXssProtection~aws:cloudfront/ResponseHeadersPolicySecurityHeadersConfigXssProtection:ResponseHeadersPolicySecurityHeadersConfigXssProtectionïDetermine whether CloudFront includes the `X-XSS-Protection` HTTP response header and the header‚Äôs value. See XSS Protection for more information.
:ø
ﬁ

cloudfront?ResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicyéaws:cloudfront/ResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicy:ResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicy€
ÿ†
contentSecurityPolicy" ÇThe policy directives and their values that CloudFront includes as values for the `Content-Security-Policy` HTTP response header.
≤
override
 °Whether CloudFront overrides the `Content-Security-Policy` HTTP response header received from the origin with the one specified in this response headers policy.
:í
’

cloudfront<ResponseHeadersPolicySecurityHeadersConfigContentTypeOptionsàaws:cloudfront/ResponseHeadersPolicySecurityHeadersConfigContentTypeOptions:ResponseHeadersPolicySecurityHeadersConfigContentTypeOptions∑
¥±
override
 †Whether CloudFront overrides the `X-Content-Type-Options` HTTP response header received from the origin with the one specified in this response headers policy.
:Í
¬

cloudfront6ResponseHeadersPolicySecurityHeadersConfigFrameOptions|aws:cloudfront/ResponseHeadersPolicySecurityHeadersConfigFrameOptions:ResponseHeadersPolicySecurityHeadersConfigFrameOptions¢
üp
frameOption" ]The value of the `X-Frame-Options` HTTP response header. Valid values: `DENY` | `SAMEORIGIN`
™
override
 ôWhether CloudFront overrides the `X-Frame-Options` HTTP response header received from the origin with the one specified in this response headers policy.
:ä
…

cloudfront8ResponseHeadersPolicySecurityHeadersConfigReferrerPolicyÄaws:cloudfront/ResponseHeadersPolicySecurityHeadersConfigReferrerPolicy:ResponseHeadersPolicySecurityHeadersConfigReferrerPolicyª
∏™
override
 ôWhether CloudFront overrides the `Referrer-Policy` HTTP response header received from the origin with the one specified in this response headers policy.
à
referrerPolicy" ÒThe value of the `Referrer-Policy` HTTP response header. Valid Values: `no-referrer` | `no-referrer-when-downgrade` | `origin` | `origin-when-cross-origin` | `same-origin` | `strict-origin` | `strict-origin-when-cross-origin` | `unsafe-url`
:€
‰

cloudfrontAResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurityíaws:cloudfront/ResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity:ResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurityÒ
Óü
accessControlMaxAgeSec ÄA number that CloudFront uses as the value for the `max-age` directive in the `Strict-Transport-Security` HTTP response header.
í
includeSubdomainsB
 wWhether CloudFront includes the `includeSubDomains` directive in the `Strict-Transport-Security` HTTP response header.
¥
override
 £Whether CloudFront overrides the `Strict-Transport-Security` HTTP response header received from the origin with the one specified in this response headers policy.
~
preloadB
 mWhether CloudFront includes the `preload` directive in the `Strict-Transport-Security` HTTP response header.
:ø
≈

cloudfront7ResponseHeadersPolicySecurityHeadersConfigXssProtection~aws:cloudfront/ResponseHeadersPolicySecurityHeadersConfigXssProtection:ResponseHeadersPolicySecurityHeadersConfigXssProtectionÙ
Òl
	modeBlockB
 YWhether CloudFront includes the `mode=block` directive in the `X-XSS-Protection` header.
´
override
 öWhether CloudFront overrides the `X-XSS-Protection` HTTP response header received from the origin with the one specified in this response headers policy.
é

protection
 ˚A Boolean value that determines the value of the `X-XSS-Protection` HTTP response header. When this setting is `true`, the value of the `X-XSS-Protection` header is `1`. When this setting is `false`, the value of the `X-XSS-Protection` header is `0`.
¡
	reportUriB" ≠A reporting URI, which CloudFront uses as the value of the report directive in the `X-XSS-Protection` header. You cannot specify a `report_uri` when `mode_block` is `true`.
:‹
™

cloudfront.ResponseHeadersPolicyServerTimingHeadersConfiglaws:cloudfront/ResponseHeadersPolicyServerTimingHeadersConfig:ResponseHeadersPolicyServerTimingHeadersConfig¨
©Œ
enabled
 æA Whether CloudFront adds the `Server-Timing` header to HTTP responses that it sends in response to requests that match a cache behavior that's associated with this response headers policy.
’
samplingRate ¿A number 0‚Äì100 (inclusive) that specifies the percentage of responses that you want CloudFront to add the Server-Timing header to. Valid range: Minimum value of 0.0. Maximum value of 100.0.
:ö
S

cloudfrontVpcOriginTimeouts2aws:cloudfront/VpcOriginTimeouts:VpcOriginTimeouts¬
øÁ
createB" ÷A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
Ë
deleteB" ◊A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours). Setting a timeout for a Delete operation is only applicable if changes are saved into state before the destroy operation occurs.
Á
updateB" ÷A string that can be [parsed as a duration](https://pkg.go.dev/time#ParseDuration) consisting of numbers and unit suffixes, such as "30s" or "2h45m". Valid time units are "s" (seconds), "m" (minutes), "h" (hours).
:Ñ
Ä

cloudfront VpcOriginVpcOriginEndpointConfigPaws:cloudfront/VpcOriginVpcOriginEndpointConfig:VpcOriginVpcOriginEndpointConfig˛
˚
arn" The VPC origin ARN.
T
httpPort DThe HTTP port for the CloudFront VPC origin endpoint configuration.
V
	httpsPort EThe HTTPS port for the CloudFront VPC origin endpoint configuration.
J
name" >The name of the CloudFront VPC origin endpoint configuration.
m
originProtocolPolicy" QThe origin protocol policy for the CloudFront VPC origin endpoint configuration.
Ó
originSslProtocolsøBº:π
∂

cloudfront2VpcOriginVpcOriginEndpointConfigOriginSslProtocolstaws:cloudfront/VpcOriginVpcOriginEndpointConfigOriginSslProtocols:VpcOriginVpcOriginEndpointConfigOriginSslProtocolsïA complex type that contains information about the SSL/TLS protocols that CloudFront can use when establishing an HTTPS connection with your origin.
:‹
∂

cloudfront2VpcOriginVpcOriginEndpointConfigOriginSslProtocolstaws:cloudfront/VpcOriginVpcOriginEndpointConfigOriginSslProtocols:VpcOriginVpcOriginEndpointConfigOriginSslProtocols!

items*" 
quantity :Ã
¬

cloudfront6getCachePolicyParametersInCacheKeyAndForwardedToOrigin|aws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOrigin:getCachePolicyParametersInCacheKeyAndForwardedToOriginÑ
Åı
cookiesConfigsÛ*:Ì
Í

cloudfrontCgetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigñaws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig:getCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigÏObject that determines whether any cookies in viewer requests (and if so, which cookies) are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
Ω
enableAcceptEncodingBrotli
 öA flag that can affect whether the Accept-Encoding HTTP header is included in the cache key and included in requests that CloudFront sends to the origin.
ª
enableAcceptEncodingGzip
 öA flag that can affect whether the Accept-Encoding HTTP header is included in the cache key and included in requests that CloudFront sends to the origin.
Á
headersConfigsÛ*:Ì
Í

cloudfrontCgetCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigñaws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig:getCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigﬁObject that determines whether any HTTP headers (and if so, which headers) are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
û
queryStringsConfigsÇ*ˇ:¸
˘

cloudfrontHgetCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig†aws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig:getCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigÅObject that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Query String Config for more information.
:∆
Í

cloudfrontCgetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigñaws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig:getCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig÷
”Ì
cookieBehavior" ÷Determines whether any cookies in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values are `none`, `whitelist`, `allExcept`, `all`.
‡
cookiesÖ*Ç:ˇ
¸

cloudfrontIgetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookie¢aws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookie:getCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookieMObject that contains a list of cookie names. See Items for more information.
:”
¸

cloudfrontIgetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookie¢aws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookie:getCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookieR
PN
items*" ?List of item names (`cookies`, `headers`, or `query_strings`).
:§
Í

cloudfrontCgetCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigñaws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig:getCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig¥
±À
headerBehavior" ¥Determines whether any HTTP headers are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values are `none`, `whitelist`.
‡
headersÖ*Ç:ˇ
¸

cloudfrontIgetCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeader¢aws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeader:getCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeaderMObject that contains a list of header names. See Items for more information.
:”
¸

cloudfrontIgetCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeader¢aws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeader:getCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeaderR
PN
items*" ?List of item names (`cookies`, `headers`, or `query_strings`).
:ç
˘

cloudfrontHgetCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig†aws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig:getCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigé
ã¸
queryStringBehavior" ‡Determines whether any URL query strings in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values are `none`, `whitelist`, `allExcept`, `all`.
â
queryStrings£*†:ù
ö

cloudfrontSgetCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryString∂aws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryString:getCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStringSObject that contains a list of query string names. See Items for more information.
:Ò
ö

cloudfrontSgetCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryString∂aws:cloudfront/getCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryString:getCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStringR
PN
items*" ?List of item names (`cookies`, `headers`, or `query_strings`).
:€
â

cloudfront#getOriginRequestPolicyCookiesConfigVaws:cloudfront/getOriginRequestPolicyCookiesConfig:getOriginRequestPolicyCookiesConfigÃ
…
cookieBehavior" ∞
cookies§*°:û
õ

cloudfront)getOriginRequestPolicyCookiesConfigCookiebaws:cloudfront/getOriginRequestPolicyCookiesConfigCookie:getOriginRequestPolicyCookiesConfigCookie:±
õ

cloudfront)getOriginRequestPolicyCookiesConfigCookiebaws:cloudfront/getOriginRequestPolicyCookiesConfigCookie:getOriginRequestPolicyCookiesConfigCookie

items*" :€
â

cloudfront#getOriginRequestPolicyHeadersConfigVaws:cloudfront/getOriginRequestPolicyHeadersConfig:getOriginRequestPolicyHeadersConfigÃ
…
headerBehavior" ∞
headers§*°:û
õ

cloudfront)getOriginRequestPolicyHeadersConfigHeaderbaws:cloudfront/getOriginRequestPolicyHeadersConfigHeader:getOriginRequestPolicyHeadersConfigHeader:±
õ

cloudfront)getOriginRequestPolicyHeadersConfigHeaderbaws:cloudfront/getOriginRequestPolicyHeadersConfigHeader:getOriginRequestPolicyHeadersConfigHeader

items*" :í
ò

cloudfront(getOriginRequestPolicyQueryStringsConfig`aws:cloudfront/getOriginRequestPolicyQueryStringsConfig:getOriginRequestPolicyQueryStringsConfigÙ
Ò
queryStringBehavior" ”
queryStrings¬*ø:º
π

cloudfront3getOriginRequestPolicyQueryStringsConfigQueryStringvaws:cloudfront/getOriginRequestPolicyQueryStringsConfigQueryString:getOriginRequestPolicyQueryStringsConfigQueryString:œ
π

cloudfront3getOriginRequestPolicyQueryStringsConfigQueryStringvaws:cloudfront/getOriginRequestPolicyQueryStringsConfigQueryString:getOriginRequestPolicyQueryStringsConfigQueryString

items*" :˝
t

cloudfrontgetRealtimeLogConfigEndpointHaws:cloudfront/getRealtimeLogConfigEndpoint:getRealtimeLogConfigEndpointÑ
ÅÜ
kinesisStreamConfigs∂*≥:∞
≠

cloudfront/getRealtimeLogConfigEndpointKinesisStreamConfignaws:cloudfront/getRealtimeLogConfigEndpointKinesisStreamConfig:getRealtimeLogConfigEndpointKinesisStreamConfig5(Required) Amazon Kinesis data stream configuration.
v

streamType" d(Required) Type of data stream where real-time log data is sent. The only valid value is `Kinesis`.
:ß
≠

cloudfront/getRealtimeLogConfigEndpointKinesisStreamConfignaws:cloudfront/getRealtimeLogConfigEndpointKinesisStreamConfig:getRealtimeLogConfigEndpointKinesisStreamConfigÙ
Ò∞
roleArn" †(Required) ARN of an IAM role that CloudFront can use to send real-time log data to the Kinesis data stream.
See the [AWS documentation](https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/real-time-logs.html#understand-real-time-log-config-iam-role) for more information.
<
	streamArn" +(Required) ARN of the Kinesis data stream.
:“
Ü

cloudfront"getResponseHeadersPolicyCorsConfigTaws:cloudfront/getResponseHeadersPolicyCorsConfig:getResponseHeadersPolicyCorsConfig∆
√ñ
accessControlAllowCredentials
 qA Boolean value that CloudFront uses as the value for the Access-Control-Allow-Credentials HTTP response header.
´
accessControlAllowHeadersÿ*’:“
œ

cloudfront:getResponseHeadersPolicyCorsConfigAccessControlAllowHeaderÑaws:cloudfront/getResponseHeadersPolicyCorsConfigAccessControlAllowHeader:getResponseHeadersPolicyCorsConfigAccessControlAllowHeader≤Object that contains an attribute `items` that contains a list of HTTP header names that CloudFront includes as values for the Access-Control-Allow-Headers HTTP response header.
Û
accessControlAllowMethodsÿ*’:“
œ

cloudfront:getResponseHeadersPolicyCorsConfigAccessControlAllowMethodÑaws:cloudfront/getResponseHeadersPolicyCorsConfigAccessControlAllowMethod:getResponseHeadersPolicyCorsConfigAccessControlAllowMethod˙Object that contains an attribute `items` that contains a list of HTTP methods that CloudFront includes as values for the Access-Control-Allow-Methods HTTP response header. Valid values: `GET` | `POST` | `OPTIONS` | `PUT` | `DELETE` | `HEAD` | `ALL`
¢
accessControlAllowOriginsÿ*’:“
œ

cloudfront:getResponseHeadersPolicyCorsConfigAccessControlAllowOriginÑaws:cloudfront/getResponseHeadersPolicyCorsConfigAccessControlAllowOrigin:getResponseHeadersPolicyCorsConfigAccessControlAllowOrigin©Object that contains an attribute `items` that contains a list of origins that CloudFront can use as the value for the Access-Control-Allow-Origin HTTP response header.
´
accessControlExposeHeaders€*ÿ:’
“

cloudfront;getResponseHeadersPolicyCorsConfigAccessControlExposeHeaderÜaws:cloudfront/getResponseHeadersPolicyCorsConfigAccessControlExposeHeader:getResponseHeadersPolicyCorsConfigAccessControlExposeHeaderÆObject that contains an attribute `items` that contains a list of HTTP headers that CloudFront includes as values for the Access-Control-Expose-Headers HTTP response header.
ö
accessControlMaxAgeSec |A number that CloudFront uses as the value for the max-age directive in the Strict-Transport-Security HTTP response header.

originOverride
 :Â
œ

cloudfront:getResponseHeadersPolicyCorsConfigAccessControlAllowHeaderÑaws:cloudfront/getResponseHeadersPolicyCorsConfigAccessControlAllowHeader:getResponseHeadersPolicyCorsConfigAccessControlAllowHeader

items*" :Â
œ

cloudfront:getResponseHeadersPolicyCorsConfigAccessControlAllowMethodÑaws:cloudfront/getResponseHeadersPolicyCorsConfigAccessControlAllowMethod:getResponseHeadersPolicyCorsConfigAccessControlAllowMethod

items*" :Â
œ

cloudfront:getResponseHeadersPolicyCorsConfigAccessControlAllowOriginÑaws:cloudfront/getResponseHeadersPolicyCorsConfigAccessControlAllowOrigin:getResponseHeadersPolicyCorsConfigAccessControlAllowOrigin

items*" :Ë
“

cloudfront;getResponseHeadersPolicyCorsConfigAccessControlExposeHeaderÜaws:cloudfront/getResponseHeadersPolicyCorsConfigAccessControlExposeHeader:getResponseHeadersPolicyCorsConfigAccessControlExposeHeader

items*" :Ì
°

cloudfront+getResponseHeadersPolicyCustomHeadersConfigfaws:cloudfront/getResponseHeadersPolicyCustomHeadersConfig:getResponseHeadersPolicyCustomHeadersConfig∆
√¿
items∂*≥:∞
≠

cloudfront/getResponseHeadersPolicyCustomHeadersConfigItemnaws:cloudfront/getResponseHeadersPolicyCustomHeadersConfigItem:getResponseHeadersPolicyCustomHeadersConfigItem:ª
≠

cloudfront/getResponseHeadersPolicyCustomHeadersConfigItemnaws:cloudfront/getResponseHeadersPolicyCustomHeadersConfigItem:getResponseHeadersPolicyCustomHeadersConfigItemà
Ö$
header" The HTTP header name.
©
override
 òWhether CloudFront overrides the X-XSS-Protection HTTP response header received from the origin with the one specified in this response headers policy.
1
value" $Value for the HTTP response header.
:Ì
°

cloudfront+getResponseHeadersPolicyRemoveHeadersConfigfaws:cloudfront/getResponseHeadersPolicyRemoveHeadersConfig:getResponseHeadersPolicyRemoveHeadersConfig∆
√¿
items∂*≥:∞
≠

cloudfront/getResponseHeadersPolicyRemoveHeadersConfigItemnaws:cloudfront/getResponseHeadersPolicyRemoveHeadersConfigItem:getResponseHeadersPolicyRemoveHeadersConfigItem:⁄
≠

cloudfront/getResponseHeadersPolicyRemoveHeadersConfigItemnaws:cloudfront/getResponseHeadersPolicyRemoveHeadersConfigItem:getResponseHeadersPolicyRemoveHeadersConfigItem(
&$
header" The HTTP header name.
:¡
ß

cloudfront-getResponseHeadersPolicySecurityHeadersConfigjaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfig:getResponseHeadersPolicySecurityHeadersConfigî
ëè
contentSecurityPolicies*Ì:Í
Á

cloudfrontBgetResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicyîaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicy:getResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicyÄThe policy directives and their values that CloudFront includes as values for the Content-Security-Policy HTTP response header.
≤
contentTypeOptions‰*·:ﬁ
€

cloudfront>getResponseHeadersPolicySecurityHeadersConfigContentTypeOptionåaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfigContentTypeOption:getResponseHeadersPolicySecurityHeadersConfigContentTypeOption¥A setting that determines whether CloudFront includes the X-Content-Type-Options HTTP response header with its value set to nosniff. See Content Type Options for more information.
Ö
frameOptions“*œ:Ã
…

cloudfront8getResponseHeadersPolicySecurityHeadersConfigFrameOptionÄaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfigFrameOption:getResponseHeadersPolicySecurityHeadersConfigFrameOptionüSetting that determines whether CloudFront includes the X-Frame-Options HTTP response header and the header‚Äôs value. See Frame Options for more information.
ﬁ
referrerPolicies€*ÿ:’
“

cloudfront;getResponseHeadersPolicySecurityHeadersConfigReferrerPolicyÜaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfigReferrerPolicy:getResponseHeadersPolicySecurityHeadersConfigReferrerPolicyÎValue of the Referrer-Policy HTTP response header. Valid Values: `no-referrer` | `no-referrer-when-downgrade` | `origin` | `origin-when-cross-origin` | `same-origin` | `strict-origin` | `strict-origin-when-cross-origin` | `unsafe-url`
Ã
strictTransportSecuritiesˆ*Û:
Ì

cloudfrontDgetResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurityòaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity:getResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurityµSettings that determine whether CloudFront includes the Strict-Transport-Security HTTP response header and the header‚Äôs value. See Strict Transport Security for more information.
è
xssProtectionsÿ*’:“
œ

cloudfront:getResponseHeadersPolicySecurityHeadersConfigXssProtectionÑaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfigXssProtection:getResponseHeadersPolicySecurityHeadersConfigXssProtection°Settings that determine whether CloudFront includes the X-XSS-Protection HTTP response header and the header‚Äôs value. See XSS Protection for more information.
:Ω
Á

cloudfrontBgetResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicyîaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicy:getResponseHeadersPolicySecurityHeadersConfigContentSecurityPolicy–
Õû
contentSecurityPolicy" ÄThe policy directives and their values that CloudFront includes as values for the Content-Security-Policy HTTP response header.
©
override
 òWhether CloudFront overrides the X-XSS-Protection HTTP response header received from the origin with the one specified in this response headers policy.
:ê
€

cloudfront>getResponseHeadersPolicySecurityHeadersConfigContentTypeOptionåaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfigContentTypeOption:getResponseHeadersPolicySecurityHeadersConfigContentTypeOptionØ
¨©
override
 òWhether CloudFront overrides the X-XSS-Protection HTTP response header received from the origin with the one specified in this response headers policy.
:Í
…

cloudfront8getResponseHeadersPolicySecurityHeadersConfigFrameOptionÄaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfigFrameOption:getResponseHeadersPolicySecurityHeadersConfigFrameOptionõ
òj
frameOption" WValue of the X-Frame-Options HTTP response header. Valid values: `DENY` | `SAMEORIGIN`
©
override
 òWhether CloudFront overrides the X-XSS-Protection HTTP response header received from the origin with the one specified in this response headers policy.
:å
“

cloudfront;getResponseHeadersPolicySecurityHeadersConfigReferrerPolicyÜaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfigReferrerPolicy:getResponseHeadersPolicySecurityHeadersConfigReferrerPolicy¥
±©
override
 òWhether CloudFront overrides the X-XSS-Protection HTTP response header received from the origin with the one specified in this response headers policy.
Ç
referrerPolicy" ÎValue of the Referrer-Policy HTTP response header. Valid Values: `no-referrer` | `no-referrer-when-downgrade` | `origin` | `origin-when-cross-origin` | `same-origin` | `strict-origin` | `strict-origin-when-cross-origin` | `unsafe-url`
:»
Ì

cloudfrontDgetResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurityòaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity:getResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity’
“ö
accessControlMaxAgeSec |A number that CloudFront uses as the value for the max-age directive in the Strict-Transport-Security HTTP response header.
å
includeSubdomains
 sWhether CloudFront includes the includeSubDomains directive in the Strict-Transport-Security HTTP response header.
©
override
 òWhether CloudFront overrides the X-XSS-Protection HTTP response header received from the origin with the one specified in this response headers policy.
x
preload
 iWhether CloudFront includes the preload directive in the Strict-Transport-Security HTTP response header.
:»
œ

cloudfront:getResponseHeadersPolicySecurityHeadersConfigXssProtectionÑaws:cloudfront/getResponseHeadersPolicySecurityHeadersConfigXssProtection:getResponseHeadersPolicySecurityHeadersConfigXssProtectionÛ
f
	modeBlock
 UWhether CloudFront includes the mode=block directive in the X-XSS-Protection header.
©
override
 òWhether CloudFront overrides the X-XSS-Protection HTTP response header received from the origin with the one specified in this response headers policy.
˛

protection
 ÎBoolean value that determines the value of the X-XSS-Protection HTTP response header. When this setting is true, the value of the X-XSS-Protection header is 1. When this setting is false, the value of the X-XSS-Protection header is 0.
Y
	reportUri" HWhether CloudFront sets a reporting URI in the X-XSS-Protection header.
:¶
≥

cloudfront1getResponseHeadersPolicyServerTimingHeadersConfigraws:cloudfront/getResponseHeadersPolicyServerTimingHeadersConfig:getResponseHeadersPolicyServerTimingHeadersConfigÌ
ÍÃ
enabled
 ºWhether CloudFront adds the `Server-Timing` header to HTTP responses that it sends in response to requests that match a cache behavior that's associated with this response headers policy.
ò
samplingRate ÉNumber 0‚Äì100 (inclusive) that specifies the percentage of responses that you want CloudFront to add the Server-Timing header to.
