# datapipelines



## Usage

```bash
USAGE:
    datapipelines [OPTIONS] --aws-profile <aws-profile>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --aws-profile <aws-profile>    
    -f, --format <format>               [default: json]  [possible values: json, bitbar]
    -n, --name <name>                   [default: ]
    -p, --props <props>                 [default: ]
    -s, --show <show>                   [default: all]  [possible values: all, error, healthy]
```





## Argument Details



### aws-profile

Possible options can be found by running the following:

```bash
cat ~/.aws/config | grep "\[profile" | awk '{print $2}' | awk -F']' '{print $1}'
```



### properties

properties files consist of the following:

```json
{
  "filter_operation": "exclude",
  "filters": [
   "An Unimportant Pipeline",
   "Another Unimportant Pipeline",
  ]
}

```

```json
{
  "filter_operation": "include",
  "filters": [
    "Monitor This Pipeline",
    "As Well As This Pipeline"
  ]
}
```

filter_operation can be either _*include*_ or _*exclude*_
filter_names are an array of Pipeline names to be either excluded or included