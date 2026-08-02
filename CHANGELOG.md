# 0.2.3

* Added new missing fields added from the Hobbit set
    * Borderless frame
    * Dwarven language
* Made some fields optional on Bulk Data sets

# 0.2.2 [YANKED]

* Added support for the new Image formats
    * These are the currently supported formats but in Webpack

# 0.2.1

* Added support for the `/cards/manifest` endpoint on Scryfall (types and Request objects)
* Added support for `Tag` objects which can be retrieved in Bulk Data requests

# 0.2.0

* The field `download_uri` on bulk data is now Deprecated and will be removed at a later date
    * It is now returned as an `Option<Url>` instead of a `Url` to allow for it to be silently ignored
* Added a new `call_raw` method to the API clients to allow for objtaining the raw bytes of a request
    * This is support the new `jsonl_download_uri` which is returned as an array of GZipped data which needs to be manually decoded.

# 0.1.1

* Added a fix for Mana Cost serialisations for `Card` objects
    - Mana costs from Scryfall come as a string (i.e "{3}{U}")
    - These are deserialised into a `Vec` of `CostSymbol`
    - When serialising the `Card` back out to a string, the serialisation would serialise it to a `Vec<String>` instead of just a `String`
    - A fix was added to match the serialisation output of Scryfall
