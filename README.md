# InoE

TUI viewer for the XML version of the [37C3 Fahrplan].

**Note:** It's very buggy atm. Chances are it'll behave in unexpected ways, especially the grid
view is a bit of questionable quality.

## Installation

[Install Rust], then

```sh
cargo install --git https://github.com/MultisampledNight/inoe
```

## Usage

Launch it with the path to a schedule file downloaded from https://fahrplan.events.ccc.de/congress/2023/fahrplan/schedule.xml, as in

```sh
inoe schedule.xml
```

There's 2 view modes:

1. The **grid** mode, which is also the default. Here, you get a handy overview over all events.
   Select an event using Vim keys and switch into **single** mode with <kbd>Enter</kbd>.
2. The **single** mode. Here, you get to look at one event in detail. You can still navigate the
   events with the Vim keys, but <kbd>j</kbd> and <kbd>k</kbd> can be used for scrolling the text
   instead.

## FAQ

### Name

Spell it however you want. Acronym for "In need of E".
I formally refuse to elaborate any further on what E is, but chances are you
know it already.


[37C3 Fahrplan]: https://fahrplan.events.ccc.de/congress/2023/fahrplan/
[Install Rust]: https://doc.rust-lang.org/stable/book/ch01-01-installation.html
