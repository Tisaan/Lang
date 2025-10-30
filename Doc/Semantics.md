# NAME

## Philosophy

## Keyword
### Module:
  - import
  - from
  - mod
### Function:
  - lambda
  - func
  - await
  - return
### Qualifier:
  - async
  - export
### Class:
  - class
  - self
### Traits
  - impl
  - for
### Control Flow:
  - if
  - else
  - for
  - while
  - loop
  - break
  - continue
  - in
  - switch
### Type
  - as
  - raw
  - extend
  - type
  - true
  - false
  - null
### Data-type
  - interface
  - enum

### Weak:
  - case ([switch](#control-flow))
  - default ([switch](#control-flow), [traits](#traits))
  - "!" ([Variable declaration]())
  - "?" ([Variable declaration]())
  - pub ([function](#function), [traits](#traits), [data-type](#data-type))
### Funture uses
  - extern [ffi]()
  - where [data-type](#data-type)

## Type
#### Additonal Information
For information about the low-level type, go to [low-level type system]()

### Number
`Number`'s is use to represent a number no matter if it is an **Integer** or a **Real**.

### String
`String`'s is a sequence of value use to represent **Unicode**.

### List
`List`'s values are abritary Object of the same Type, `List` have fixed-size

### Vector
`Vector`'s values are abritary Object of the same Type, `Vector` are growable

### Map
`Map`'s is use to map abritary Object to a key `String`, the underlying index is obtain the key' hashing

### Bool
`Bool`'s is use to represent the truth values **true**, **false**

### Null
`Null`'s is use to represent the absence of a value

## Module

