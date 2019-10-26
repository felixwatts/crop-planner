# harvest

An experimental tool to automatically design a market garden crop plan in order to efficiently use growing space to satisfy the needs of a Community Supported Agriculture veg box scheme.

## Usage

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
$> harvest solve
```

That program will run for a few minutes while it explores the vast number of different assignments of crops to beds over the year. Once it finds a good solution it will stop. You can now explore the crop plan that was generated.

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
$> harvest ins -w 12
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
