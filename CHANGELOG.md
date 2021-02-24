# Changelog

## [Unreleased](https://github.com/voxjar/elastiql/compare/v0.3.3...HEAD) (2021-02-24)

### ⚠ BREAKING CHANGE

* **search:** HighlightOptions.tags_schema is now optional

### Features

* **search:** add time_zone field to RangeQuery fd3a871
* **agg:** add sampler agg a20192e
* **agg:** add significant_text agg 9ba505d
* **search:** add prefix query 10e2211
* **agg:** add variable_width_histogram agg d828bc7
* **agg:** add histogram agg 0044292
* **search:** add more fields to HighlightOptions e9a0b35
* **search:** add terminate_after field to Request 600b8c3
* add methods from serde_json to scalar Map 92acd64

### Fixes

* **agg:** support non string agg keys aabef92
* **search:** do not serialize empty pre/post_tags 45d5fc3


### [v0.3.3](https://github.com/voxjar/elastiql/compare/v0.3.2...v0.3.3) (2020-10-25)

#### Fixes

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


