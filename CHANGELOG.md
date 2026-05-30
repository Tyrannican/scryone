# 0.1.1

* Added a fix for Mana Cost serialisations for `Card` objects
    - Mana costs from Scryfall come as a string (i.e "{3}{U}")
    - These are deserialised into a `Vec` of `CostSymbol`
    - When serialising the `Card` back out to a string, the serialisation would serialise it to a `Vec<String>` instead of just a `String`
    - A fix was added to match the serialisation output of Scryfall
