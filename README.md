# custom-nro
A template for replacing a character's fighter NRO

## Using
If you replace a fighter's NRO (in this case the template is setup to replace `prebuilt:/nro/release/lua2cpp_donkey.nro`) you have to edit `cross2_000.nrr` found in the game's romfs (don't worry, this isn't in the data.arc). You need to change the value at offset 0x344 to be incremented by one and then insert the hash of the built nro file into the sorted hash list.

For more info, DM blujay#7630 on discord (or ping me instead if we share servers).

Also see: https://switchbrew.org/wiki/NRR