# Storing Keys with Associated Values in Hash Maps

The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a *hashing function*, which determines how it places these keys and values into memory. Many programming languages support this kind of data structure, but they often use a different name, such as *hash, map, object, hash table, dictionary* or *associative array*, just to name a few.

Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type. For example, in a game, you could keep track of each team's score in a hash map in which each key is a team's name and the values are each team's score. Given a team name, you can retrieve its score.

Check the standard library documentation for more information.

# Creating a New Hash Map

Note that we need to first `use` the `HashMap` from the collections portion of the standard library. Of our three common collections, this one is the least often used, so it's not included in the features brought into scope automatically in the prelude. Hash maps also have less support from the standard library; there's no built-in macro to construct them, for example.

Just like vectors, hash maps store their data on the heap. This `HashMap` (code example in hash_maps project) has keys of type `String` and values of type `i32`. Like vectors, hash maps are homogeneous: All of the keys must have the same type, and all of the values must have the same type.

# Managing Ownership in Hash Maps

For types that implement the `Copy` trait, like `i32`, the valus are copied into the hash map. For owned values like `String`, the values will be moved and the hash map will be the owner of those values.

If we insert references to values into the hash map, the values won't be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.

# Updating a Hash Map

Although the number of key and value pairs is growable, each unique key can only have one value associated with it at a time (but not vice versa: the Blue team and Yellow team could have the value 10 stored in the scores hash map).

When you want to change the data in a hash map, you have to decide how to handle the case when a key already has a value assigned. You could replace the old value with the new value, completely disregarding the old value. You could keep the old value and ignore the new value, only adding the new value if the key *doesn't* already have a value. Or you could combine the old value and the new value.

