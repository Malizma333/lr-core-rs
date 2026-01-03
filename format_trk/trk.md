(*Modified version of [Conqu3red's docs](https://github.com/Conqu3red/TRK-Docs/blob/master/The-TRK-Format.md)*)

# General Structure

# Header

* **0x00:** Magic Number 0x54524BF2 Spelling out TRKò
* **0x04:** Byte indicating version (should have value 0x01)
* **0x05:** Signed 16 bit integer denoting the length of the feature string
* **0x07:** Feature string: ASCII encoded list of features seperated by `;` (see above for length, end feature has a `;` at the end as well.)

# Features

* `REDMULTIPLIER` - Red lines have multipliers
* `SCENERYWIDTH` - Width values for scenery lines
* `6.1` - Version 6.1 physics (default 6.2)
* `SONGINFO` - Track contains song metadata
* `IGNORABLE_TRIGGER`
* `ZEROSTART` - Start rider at (0, 0)
* The following features are only supported by LRA-CE:
* `REMOUNT` - use LRA-CE style remounting implementation (default: no remount)
* `FRICTIONLESS` - disable friction

# C# Encoded String
This is a reference for the C# binary standard of encoding string (reference: [here](https://docs.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-spptc/89cf867b-260d-4fb2-ba04-46d8f5705555))
Please refer to the the General Structure section for file structure, and each sub-structure for when this string is referred to.
* **0x00:** Length of the UTF-8 string: 7BitEncodedInt (T)
    * reference: [here](https://docs.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-spptc/1eeaf7cc-f60b-4144-aa12-4eb9f6e748d1)

    * The value is written out 7 bits at a time starting with the least significant bits. If the value will not fit in 7 bits the high bit of the byte is set to indicate there is another byte of data to be written. The value is then shifted 7 bits to the right and the next byte is written. If the value will fit in the seven bits the high byte is not set and it signals the end of the structure.

* **From the end of T**: UTF-8 string, matching the length above.

# Song Info
Song info is only present if the `SONGINFO` feature is detected.
* **0x00:** Song: C# Encoded String
    * Contains Song name and song offset (as a float converted to a string) seperated by `\r\n`

# Line Data
* **0x00:** Rider start position X, Double precision float (8 Bytes)
* **0x08:** Rider start position Y, Double precision float (8 Bytes)
* **0x016:** 32 bit (4 byte) unsigned integer (N) denoting the number of lines to read

For N Times:
* **0x00:** Line Type + Flags
    * **Bit 8** Line inverted, Boolean
    * **Bit 7 and 6** Line Extension:
        * `0` None
        * `1` Left
        * `2` Right
        * `3` Both
    * **Bits 5 - 1** Line Type:
        * `0` Scenery
        * `1` Blue (Standard)
        * `2` Acceleration
* Type Specific Data (S):
    * Red Line (R)
        * **0x01:** Red Line Multiplier, 1 byte. Only present if this line is type `Acceleration` and the feature `REDMULTIPLIER` is present.

    * Blue or Red Line
        * If feature `IGNORABLE_TRIGGER` is present (T)
            * **R + 1** Zoom trigger, Boolean
            * If Zoom trigger is true:
            * **R + 2:** Target, Single precision float (4 Bytes)
            * **R + 7:** Frames, 16-Bit (2 byte) Signed Integer
            * This trigger fires when the line it is attached to is touched. Taking `Frames` to reach `Target` zoom.
        * **From end of T:** Line ID: Signed 32-Bit Integer
        * If extension is not `None`
            * **From end of T + 4:** 32-Bit Signed Integer (Ignored)
            * **From end of T + 8:** 32-Bit Signed Integer (Ignored)

    * Scenery Line
        * If `SCENERYWIDTH` feature is present (W)
            * **0x01:** Line Width, 1 Byte. (Divide by 10.0 to get width value)
        * Otherwise, assume Line Width is 1.0

* **S + 4:** X position of the start, Double precision float (8 Bytes)
* **S + 12:** Y position of the start, Double precision float (8 Bytes)
* **S + 18** X position of the end, Double precision float (8 Bytes)
* **S + 26:** Y position of the end, Double precision float (8 Bytes)

# Metadata
This section may not be present (older versions of original LRA). If you have reached the end of the file then there is no metadata section.
* **0x00:** Magic Number 0x4D455441 Spelling out META
* **0x04:** Number of metadata entries (N) - 16-Bit Signed Integer

For N Times:
* **0x00:** Signed 16 bit integer denoting the length of the metadata string (L)
* **0x02:** Metadata string, ASCII encoded string of length L, a key-value pair of the structure `KEY=VALUE`, below are some possible keys and value types:
    * NOTE: all values are stored as part of the string, as their string representation not byte representation.
    * `STARTZOOM` - Initial camera zoom (Single precision float)
    * The following metadata values are only supported by `LRA-CE`. `LRA` will ignore them. For compatability reasons you should preserve all metadata even if your program does not directly use or understand some of it.
    * `YGRAVITY` - Y Gravity of rider (default 1.0) (Single precision float)
    * `XGRAVITY` - X Gravity of rider (default 0) (Single precision float)
    * `GRAVITYWELLSIZE` - Size of gravity wells (default 10.0) (Double precision float)
    * `BGCOLORR` - Red Channel of background color (default 244) (Signed 32-Bit Integer)
    * `BGCOLORG` - Green Channel of background color (default 245) (Signed 32-Bit Integer)
    * `BGCOLORB` - Blue Channel of background color (default 249) (Signed 32-Bit Integer)
    * `LINECOLORR` - Red Channel of line color (default 0) (Signed 32-Bit Integer)
    * `LINECOLORG` - Green Channel of line color (default 0) (Signed 32-Bit Integer)
    * `LINECOLORB` - Blue Channel of line color (default 0) (Signed 32-Bit Integer)
    * `TRIGGERS` - LRA-CE implements a more advanced trigger system using metadata, see below for parsing:
        * The list of triggers is stored as the value for the `TRIGGERS` metadata entry, with each trigger being seperated by an `&`. Note again that all values are string representations not bit representations.
        * Each trigger stores its values seperated by `:`
        * **values[0]** The first value is always the trigger version (Signed 32-Bit Integer):
            * `0` - Zoom
                * **values[1]:** Target zoom (Single precision float)
                * **values[2]:** Start frame (Signed 32-Bit Integer)
                * **values[3]:** End frame (Signed 32-Bit Integer)
            * `1` - BGChange
                * **values[1]:** Red Channel of background color (Signed 32-Bit Integer)
                * **values[2]:** Green Channel of background color (Signed 32-Bit Integer)
                * **values[3]:** Blue Channel of background color (Signed 32-Bit Integer)
                * **values[4]:** Start frame (Signed 32-Bit Integer)
                * **values[5]:** End frame (Signed 32-Bit Integer)
            * `2` - LineColor
                * **values[1]:** Red Channel of line color (Signed 32-Bit Integer)
                * **values[2]:** Green Channel of line color (Signed 32-Bit Integer)
                * **values[3]:** Blue Channel of line color (Signed 32-Bit Integer)
                * **values[4]:** Start frame (Signed 32-Bit Integer)
                * **values[5]:** End frame (Signed 32-Bit Integer)
