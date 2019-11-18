# harvest

*A crop planning tool for market gardeners*

When I started studying to become a market gardener I read countless times that planning what to grow, where and when can be a difficult and time consuming chore. Many gardeners spend significant hours in the winter months, poring over 
records, juggling spreadsheets and trying out ideas almost at random in an attempt to come up with a plan that
meets the demands of their market while making efficient use of time and space in the garden.

As someone with a background in computing that all sounded to me like exactly the kind fo boring grunt work that
computers are good at, so I set about writing a software tool to automate the process.

Harvest is a small tool that is designed to make it easy to both create and follow a good growing plan. It is very 
customizable because every farm is different, and it recognizes the fact that real life doesn't always cooperate
with your plan.

## Quick Start

Start a new project by initializing a new harvest repository:

```
$> mkdir my-crop-plan
$> cd my-crop-plan
$> harvest init
```

Now edit the `params.json` file to set up parameters corresponding to your farm, including the beds you have available and the varieties of crop you want to grow:

```
$> nano .harvest/params.json
```

Once you are happy with the parameters, generate a new crop plan:

```
$> harvest plan
```

The program will run for a few minutes while it explores the vast number of different possible assignments of crops to beds over the year. It uses [artificial evolution](https://en.wikipedia.org/wiki/Evolutionary_algorithm) to design a plan that maximizes potential profit. Once it finds a good solution it will stop. You can now explore the crop plan that has been generated.

```
$> harvest print

       Week  Lettuce-I  Spinach-S  Spinach-W     Radish  Lettuce-O     Tomato  Carrot-Su  Carrot-Wi  Swede-Sum  Swede-Win      BBean    Brocoli     SOnion
          0         0%                    0%                                                                           0%         0%                    0%
          1         0%                    0%                                                                           0%                               0%
          2         0%                    0%                                                                           0%                               0%
          3         0%                    0%                                                     0%                    0%                               0%
< SNIP >          
          51        83%                                                                                               100%        50%                      

Utilization: 67%
Saturation: 63%
Profit: 15224.74
$>
```

The above command prints out a table overview of the year. Each column is a variety of crop and each row is a week in the season. Each cell shows the percentage of market saturation that has been achieved for that variety in that week.

After the table, some statistics about the plan are printed. More on these later.

> Note that the plan generated in this example is quite poor, because it starts on January 1st with all beds empty. In order to be profitable, a market gardener should have crops overwintering so that beds are not left empty and crops are ready to harvest early in the season. For more on this see [Continuing a Plan into the Following Year](#continuing-a-plan-into-the-following-year)

You can also look at an individual bed in more detail

```
$> harvest print -b ~bA11
Bed ~bA11

Week     Variety  
0        Lettuce-Indoor
5        Lettuce-Indoor
18       Tomato   
45       Carrot-Winter

Utilization: 94%

$>
```

Finally, you can get instructions to follow for a given week

```
$> harvest print -w 12
Tasks for week #12
- Transplant Lettuce-Outdoor from tray ~bA33-12 into bed ~bA33
- Transplant Lettuce-Indoor from tray ~bA41-12 into bed ~bA41
- Harvest 100 units of Lettuce-Indoor from bed ~bB12
- Harvest 100 units of Lettuce-Indoor from bed ~bB21
- Harvest 100 units of Lettuce-Indoor from bed ~bB22
- Harvest 100 units of Lettuce-Indoor from bed ~bB31
- Label a 144 tray ~bB32-14 and seed it with 6 grams of Lettuce-Indoor seed
<SNIP>
$>
```

For more options and help, run harvest without arguments:

```
$> harvest
```

## Weeks, Beds and Varieties

Harvest is built around some core concepts which it will be useful to familiarize yourself with...

Harvest divides time into **weeks**. Each week is a seven day period of time beginning on a Monday. Week #0 starts on the first Monday in January. There are assumed to be 52 weeks in a year.

Space is divided into **beds**. Each bed represents a single growing space in your garden. Each bed has a name,
a size and some other properties which we will discuss later. You will need to tell harvest what beds you have.

Plant **varieties** in harvest represent the different types of crop you might want to grow. Each variety has a
name, some information about when it can be planted, its days to maturity, yield, market value and some other properties we'll discuss later. 

You can either define your own varieties, use some that have been published by other users or start with published varieties and modify them to your own liking.

## Beds

### Adding a Bed

### Removing a Bed

### Enabling/Disabling a Bed

## Varieties

### Adding a Variety

### Removing a Variety

### Using a Published Variety

### Restricting a Variety to Certain Beds

### Enabling/Disabling a Variety

## Generating a Plan

## Using a Plan

### Bill of Quantities

### Daily Instructions

### Dealing with Crop Failure and Changes of Plan

Sometimes crops fail, sometimes a big customer drops their order, sometimes that exotic variety just isn't selling
as well as you thought it would. There are many reasons why you might want to change your plan part way through the
season. Harvest can help you with that.

#### Re-planning from a Given Week

As well as generating a plan from scratch, harvest can modify an existing plan by rebuilding the part of it that occurs after a given week. Nothing about history will be changed but the future of the plan will be re-designed based on the current parameters.

```
$> harvest plan -w 32
```

> Beware that replanning may significantly change the plan and therefore the bill of quantities. If you have already bought everything you need for this season then some of those resources may not be utilized by the new plan and other resources may be required.

#### Crop Failure

In order to handle crop failure, you tell harvest that a crop has failed and then re-plan from the current week

## Continuing a Plan into the Following Year

Many market gardeners grow year-round, so that one year's growing plan overlaps and affects the next. Harvest is designed to support this type of operation. Whereas harvest generates plans that run for a year, it is able to take into account last year's plan when designing this year's. It also tries to design a plan that sets up next year well, for example, by planting crops at the end of the season that will only be harvested as part of next years' plan.

> Note: Currently, harvest doesn't have an easy way to input last years plan if that plan was not generated by harvest itself, so the first year you use harvest might not be ideal if it's not your first year of production.

For the first year of planning with harvest, there is no *last year's plan*, so we build the plan as previously shown:

```
$> mkdir my_farm_2019
$> cd my_farm_2019
$> harvest init
```

For the subsequent year, we start a new harvest project but use the `continue` command to continue it from last year's:

```
$> mkdir my_farm_2020
$> cd my_farm_2020
$> harvest continue ../my_farm_2019
```

Last year's beds, varieties and plan will be imported. Now we can plan as normal:

```
$> harvest plan
```

The generated plan will take into account the crops that are still in the ground at the end of last year's plan.

## Crop Rotation

[Crop rotation](https://en.wikipedia.org/wiki/Crop_rotation) is a powerful technique for maintaining soil health and productivity. Harvest has some features to support crop rotation.
