# Changelog

## [Unreleased](https://github.com/voxjar/elastiql/compare/v0.3.2...HEAD) (2020-10-24)

### Fixes

* fix graphql default values f96b633


### v0.3.2 (2020-10-18)

### ⚠ BREAKING CHANGE

* remove PartialEq and a few others from some types

#### Features

* **search:** add From<Query> for QueryInput types 4ef1e89
* **search:** add builder to CompoundQueryInput f08ab94
* **search:** add missing fields/options e23fecd
* **search:** remove QueryStringQuerynew methods f4e18f5
* **agg:** add TermsAggregation type 509258b

#### Fixes

* fix serializing scalar Map for async-graphql 1326d47
* **agg:** skip serializing null ReverseNested path 10c00e9


