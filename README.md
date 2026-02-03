# kaworu

A config file manager that generates symlinks to the right config folders. Keep your dotfiles in one place, deploy them anywhere.

![crates banner](https://static.crates.io/og-images/kaworu.png)

## Install

```
cargo install kaworu
```

## Usage

### Initialize a new repo

```
kaworu init
```

This creates a new kaworu repo and initializes a git repository. To skip git initialization:

```
kaworu init --skip-git
```

### Preview changes

```
kaworu show
```

Displays the symlinks that would be created or updated, without making any changes.

### Generate a shell script

```
kaworu gen
```

Outputs a shell script you can inspect before running. Supports multiple formats:

```
kaworu gen bash
kaworu gen powershell
kaworu gen cmd
```

### Apply changes

```
kaworu apply
```

Creates the symlinks directly.

## License

MIT
