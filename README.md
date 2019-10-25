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
harvest solve
```
