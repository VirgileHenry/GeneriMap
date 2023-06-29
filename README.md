# GeneriMap

GeneriMap is a rust module that provides a map from a type to an instance of this type. 
This means that any types can be inserted in this map, but inserting any type will remove the previous value of the same type.

At it's core, the GeneriMap is a `HashMap` that binds `TypeId` to `Vec<u8>`. The vector is basically a heap allocated value of the type, and it's length is the size of the type.

This is highly inspirated from the `AnyMap` crate by Chris Morgan:
- [Crate](https://crates.io/crates/anymap)
- [Documentation](https://docs.rs/anymap/0.12.1/anymap/)
- [Repository](https://github.com/chris-morgan/anymap)

However, the main point of this implementation was to allow to clone the anymap. The use of `Vec<u8>` instead of `Box<dyn Any>` allows the GeneriMap to be cloned, unlike it's original inspiration.

## Examples

Here is a simple example of how to use the GeneriMap.

```rust
use generic_map::GeneriMap;

// create a new generi map. The 'with_capacity' method is also available.
let mut map = GeneriMap::new();

// insert values. Note that keys are inferred from the inserted type.
map.insert(black_box(42u8));
map.insert(black_box("Hello World".to_string()));
map.insert(black_box(vec![1u8, 2u8, 3u8]));
map.insert(black_box(42u16));

// get values. We need to specify the value type as a generic parameter.
let u8_value: &u8 = map.get::<u8>();
let string_value: &String = map.get::<String>();
let vec_u8_mut_value: &mut Vec<u8> = map.get_mut::<Vec<u8>>();
let u16_mut_value: &mut u16 = map.get_mut::<u16>();

// remove values 
map.remove::<u8>();
map.remove::<String>();
map.remove::<Vec<u8>>();
map.remove::<u16>();
```

## Status

This module can be used, but have lot of room for improvement. Lots of the `HashMap` methods could easily be implemented for the `GeneriMap`. For now, only insertion, removal and getting values are implemented.

Furthermore, the module is not documented at all. This is a work in progress.

## Performances

As this module was built with similar goals and intent as the `AnyMap` crate, performances are similar. However, as I introduced benchamrking, I did optimize the `GeneriMap` to be faster than the `AnyMap`, but the difference is not significant.

Here are the results of the benchmarking I did, using the `criterion` crate. The benchmarking code is available in the `benches` folder. These benchmarks are run on a 2022 Inspiron 15 7510, with the Processor: 11th Gen Intel(R) Core(TM) i5-11400H @ 2.70GHz.

The given values are the mean of the benchmarking, done by the `criterion` crate over millions of test iterations.

|           | Insertion           | Get                 | Insertion + Removal |
|-----------|---------------------|---------------------|---------------------|
| GeneriMap |      210.31 ns      |      5.2408 ns      |      283.33 ns      |
| AnyMap    |      266.42 ns      |      5.8446 ns      |      304.43 ns      |

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Contributing

Any contribution is welcome. Feel free to open an issue or a pull request.