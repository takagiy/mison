# Derived types

## Structs

- [x] A struct with named fields is a record whose keys are the field names
- [x] Its fields read back in any order
- [x] A missing, unknown or repeated key is an error
- [x] A field of type `Option` is written `key=` when it is `None`, and cannot be left out
- [x] A generic struct implements `Encode` and `Decode` when its type parameters do
- [x] Deriving on a tuple struct, a unit struct, a struct without fields or a union does not compile

## Enums

- [x] A unit variant is a case without contents, written as the variant's name
- [x] A variant with one unnamed field has that field as its contents
- [x] A variant with several unnamed fields has a tuple of them as its contents
- [x] A variant with named fields has a record of them as its contents
- [x] Fields named like the derived code's own variables (`w`, `r`, `key`, `x`) are written and read back
- [x] A name that is not a variant of the enum is an error
- [x] Deriving on an enum without variants does not compile
