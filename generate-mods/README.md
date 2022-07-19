# generate-mods

Binary to generate rust types based on osu!'s [mods.json](https://github.com/ppy/osu-web/blob/master/database/mods.json).

- Requests the file
- Processes its content
- Writes types into `generated_mods.rs`