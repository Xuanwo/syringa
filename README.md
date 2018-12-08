# Syringa

## Name

Syringa is the scientific name of [lilac](https://github.com/archlinuxcn/lilac) which is the build bot for archlinuxcn.
`syringa` will be a tools used by archlinuxcn maintainers, intends to be `a bridge to lilac`.

## Status

Syringa is in heavy development, compatibility is not guaranteed until `v1.0.0` released.

## Features

- Inquire packages that maintained by specified user.

## Usage

List packages maintained by name or email:

```
syringa list -m farseerfc
syringa list -m xuanwo@archlinuxcn.org
```

Import package from AUR:

```
syringa import --aur tikv
```

> All the file/directory start with `.` in the package will be removed.