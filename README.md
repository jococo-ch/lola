# lola

[![](https://github.com/jococo-ch/lola/actions/workflows/verify.yml/badge.svg?branch=main)](https://github.com/jococo-ch/lola/actions/workflows/verify.yml)

CLI to evaluate the monthly Banana accounting Excel export and prepare the monthly closing summary sheet.

Note: The content of this repo was extracted from https://github.com/jococo-ch/lola-sumup.git.

## Summary

The cli application `lola` is used in context of the monthly-closing process of the Quartiertreffpunkt LoLa in Basel.

After the general ledger has been updated, the accounts are exported to an Excel file via Banana accounting software.
The subcommand `close` processes the Excel file and aggregates the year-to-date figures on the level of the LoLa budget.

## CLI

The `lola` command has one subcommand:

```
A cli program to create the monthly closing report from the Banana Excel export

Usage: lola <COMMAND>

Commands:
  close  Run the monthly closing process
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### The close step

The `lola close` command:

```
Run the monthly closing process

Usage: lola close <BUDGET_CONFIG_FILE> <ACCOUNTS_FILE>

Arguments:
  <BUDGET_CONFIG_FILE>  the budget configuration file in TOML format
  <ACCOUNTS_FILE>       The spreadsheet export file from the accounting software

Options:
  -h, --help     Print help
  -V, --version  Print version
```

It dumps the aggregated year-to-date figures to the command line.
