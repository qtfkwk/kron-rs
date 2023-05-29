# About

Date/time CLI utility

*See also the [`kron-lib`] crate.*

[`kron-lib`]: https://crates.io/crates/kron-lib

# Usage

```
$ kron -h
!run:../../target/release/kron -h
```

```
$ kron -V
!run:../../target/release/kron -V
```

# Examples

## Now

```
$ kron
!run:../../target/release/kron
```

## Alternate formats

```
$ kron -f COMPACT
!run:../../target/release/kron -f COMPACT
```

```
$ kron -f ISO8601NS
!run:../../target/release/kron -f ISO8601NS
```

## Custom formats

```
$ kron -f '[hour]:[minute]'
!run:../../target/release/kron -f '[hour]:[minute]'
```

```
$ kron -f '[weekday repr:long], [month repr:long] [day], [year]'
!run:../../target/release/kron -f '[weekday repr:long], [month repr:long] [day], [year]'
```

## Single timestamp

```
$ kron 0
!run:../../target/release/kron 0
```

## Multiple timestamps

```
$ kron 0 3600 7200
!run:../../target/release/kron 0 3600 7200
```

## Subsecond timestamp

```
$ kron -f ISO8601NS 626609862.1234
!run:../../target/release/kron -f ISO8601NS 626609862.1234
```

## Invalid timestamp

```
$ kron not-a-timestamp
!run:../../target/release/kron not-a-timestamp
```

!inc:../../t/FORMAT.md
