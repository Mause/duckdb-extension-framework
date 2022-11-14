# Changelog

## [0.7.0](https://github.com/Mause/duckdb-extension-framework/compare/v0.6.0...v0.7.0) (2022-11-14)


### Features

* add config machinery ([9d8b24d](https://github.com/Mause/duckdb-extension-framework/commit/9d8b24d75cef28cb6094881e36b7904beb0ac8f0))

## [0.6.0](https://github.com/Mause/duckdb-extension-framework/compare/v0.5.0...v0.6.0) (2022-11-13)


### Features

* add Vector#get_data_as_slice ([a2499a3](https://github.com/Mause/duckdb-extension-framework/commit/a2499a3802c81a3f9024d4adc338d10af1fefdd4))


### Bug Fixes

* Vector::get_data should return a pointer of type T ([ef6219a](https://github.com/Mause/duckdb-extension-framework/commit/ef6219a28565f6943854d28f18acc5c41cdc28f7))

## [0.5.0](https://github.com/Mause/duckdb-extension-framework/compare/v0.4.1...v0.5.0) (2022-11-10)


### Features

* add ReplacementScanInfo ([74dcc53](https://github.com/Mause/duckdb-extension-framework/commit/74dcc534c7616244a900221e9f7fc4f7cd376f2c))
* add T to Vector&lt;T&gt; ([d764ce4](https://github.com/Mause/duckdb-extension-framework/commit/d764ce44d2bc8622cf7ae8593a953fd2524eaa5c))


### Bug Fixes

* LogicalType::new_union_type ([4ed2a37](https://github.com/Mause/duckdb-extension-framework/commit/4ed2a376a192a57ca2f8cda64e6e32ea84c115c4))

## [0.4.1](https://github.com/Mause/duckdb-extension-framework/compare/v0.4.0...v0.4.1) (2022-11-09)


### Bug Fixes

* correct LogicalTypeId enum name ([9dbebbb](https://github.com/Mause/duckdb-extension-framework/commit/9dbebbbc52213871f3bc0b75e3b606544f758ff6))

## [0.4.0](https://github.com/Mause/duckdb-extension-framework/compare/v0.3.0...v0.4.0) (2022-11-09)


### Features

* add basic map support ([fc734aa](https://github.com/Mause/duckdb-extension-framework/commit/fc734aa81b0d38439b983d3fd3e3deea0ef9b5e4))
* add missing functions to BindInfo ([c8e7afe](https://github.com/Mause/duckdb-extension-framework/commit/c8e7afe0f78ee4fdf05a180bbe0e68f286ab7988))
* add missing functions to FunctionInfo ([468cbad](https://github.com/Mause/duckdb-extension-framework/commit/468cbad542952cd3dc1602da125e5f5c6492581d))
* add missing functions to InitInfo ([d978d6f](https://github.com/Mause/duckdb-extension-framework/commit/d978d6fb0931c491a6f4aa8800bbae975f1a59e6))
* add missing functions to LogicalType ([9136cdd](https://github.com/Mause/duckdb-extension-framework/commit/9136cdde431d9a528889cd35ee2bf32aac7a18fe))
* add missing functions to TableFunction ([cb72f2a](https://github.com/Mause/duckdb-extension-framework/commit/cb72f2a850c7b0caa2d6743cba593c026acc06f4))

## [0.3.0](https://github.com/Mause/duckdb-extension-framework/compare/v0.2.0...v0.3.0) (2022-10-20)


### Features

* add Connection#get_ptr ([3b39d45](https://github.com/Mause/duckdb-extension-framework/commit/3b39d45cefac1bad424fd5a6e59ff3b51235a8eb))
* add malloc_struct ([ee56c93](https://github.com/Mause/duckdb-extension-framework/commit/ee56c93804c9b67b64ed39ff5ac8f1557009c806))
* add ValidityMask struct ([4a7d27d](https://github.com/Mause/duckdb-extension-framework/commit/4a7d27d20dfbb52096156c7c8d119d46b085ead3))

## [0.2.0](https://github.com/Mause/duckdb-extension-framework/compare/v0.1.3...v0.2.0) (2022-10-19)


### Features

* add ability to create new database instances ([495c9dd](https://github.com/Mause/duckdb-extension-framework/commit/495c9dd849fd03b58389d85ee9dea12bd210d8dc))
