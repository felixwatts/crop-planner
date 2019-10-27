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

           0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0
Spinach    0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0 32 56 72 72 48 32  8 24 24 24 48 24 24 72 96 88 56 32  8  0  8 16 16 24 16  8  8  0  0  0  0  0  0  0  0  0
Radish     0  0  0  0  0  0  0  0  0  0  0  0120200200200200195195175190200200155195185200200200195200195180195175130170200160150135  0  0  0  0  0  0  0  0  0  0  0
Lettuce    0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  5 15 20 25 25 25 30 30 25 20 25 20 25 25 25 25 25 15 15 15 10  5  5  0  0  0  0  0  0  0  0  0  0  0
Tomato     0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0
Carrot     0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0 30 30 30 30  0  0  0  0  0  0  0  0  0 30 60120150150150210180150120  0
Swede      0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0
BBean      0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0
Brocoli    0  0  0  0 50120180200190200200200200200200200180180160150140100 40  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0
SOnion   200200200200190200190230200200190180150100  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0  0 40 60 70130
$>
```

The above command prints out a table overview of the year. Each column is a week and each row is a variety. Each cell shows the number of harvestable units of that variety in that week

You can also look at an individual bed in more detail

```
$> harvest print -b ~bA11
Bed ~bA11

Week     Variety
8        Radish
17       Radish
22       Radish
29       SOnion

Utilization: 85%

$>
```

Finally, you can get instructions to follow for a given week

```
$> harvest print -w 12
Tasks for week #12
- Harvest 10 units of Radish from bed ~bA11
- Harden off Radish tray ~bA12-13
- Harvest 10 units of SOnion from bed ~bA32
- Harvest 10 units of Brocoli from bed ~bA41
- Label a 144 tray ~bB22-14 and seed it with 6 grams of Radish seed
...
$>
```

For more options and help, run harvest without arguments:

```
$> harvest
```

## Weeks, Beds, Varieties and Baskets

Harvest is built around some core concepts which it will be useful to familiarize yourself with...

Harvest divides time into **weeks**. Each week is a seven day period of time beginning on a Monday. Week #0 starts on the first Monday in January. There are assumed to be 52 weeks in a year.

Space is divided into **beds**. Each bed represents a single growing space in your garden. Each bed has a name,
a size and some other properties which we will discuss later. You will need to tell harvest what beds you have.

Plant **varieties** in harvest represent the different types of crop you might want to grow. Each variety has a
name, some information about when it can be planted, its days to maturity, yield and some other properties we'll discuss later. 

You can either define your own varieties, use some that have been published by other users or start with published varieties and modify them to your own liking.

In order to tell harvest what you want to get out of your garden, you use **basket**s. A basket defines
what you want to harvest from the garden in a given week. Harvest will try to match your garden's output to your baskets as closely as possible.

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
$> harvest continue ../may_farm_2019
```

Last year's beds, varieties and plan will be imported. Now we can plan as normal:

```
$> harvest plan
```

The generated plan will take into account the crops that are still in the ground at the end of last year's plan.

## Crop Rotation

[Crop rotation](https://en.wikipedia.org/wiki/Crop_rotation) is a powerful technique for maintaining soil health and productivity. Harvest has some features to support crop rotation.
