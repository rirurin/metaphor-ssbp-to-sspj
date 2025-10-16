# metaphor-ssbp-to-sspj

A WIP tool for converting Sprite Studio 6 formatted binary projects (ssbp) from [Metaphor: ReFantazio](https://store.steampowered.com/app/2679460/Metaphor_ReFantazio/) into XML files which are readable in Sprite Studio.

## Usage

This is a command line tool, called by executing the following:
```bash
./metaphor-ssbp-to-sspj.exe [Input] [Output] (Locale)
```
Where for each of the following parameters:
- **Input**: Either a file to a single SSBP or a folder containing one or more SSBPs. This program checks to ensure the path contains COMMON/ui/ss to note to the program it's location within the Metaphor CPK.
- **Output**: A folder where the sprite's output files are exported to.
- **Locale (optional)**: A language ID supported by Metaphor. Default is EN

### Converting a single sprite

```bash
./metaphor-ssbp-to-sspj.exe "E:\Metaphor\base_cpk\COMMON\ui\ss\command_result" "E:\Metaphor\sprite\command_result"
```

### Converting all sprites

```bash
./metaphor-ssbp-to-sspj.exe "E:\Metaphor\base_cpk\COMMON\ui\ss" "E:\Metaphor\sprite"
```

## Quirks

- The textures that Metaphor uses are stored in APK, a custom container format that can store one or more files with LZ4 compression. 
[metaphor-apk-rs](https://github.com/rirurin/metaphor-apk-rs) (GPL 3.0) parses these, based off [**DeathChaos**](https://github.com/DeathChaos25/)' C# implementation in [MetaphorAPKPack](https://github.com/DeathChaos25/MetaphorAPKPack) (GPL 3.0) 
- Sprite Studio doesn't support DDS files, so the program re-encodes them as PNG. This takes a while since the 4K textures are used.
- The sprite project is formatted to use a 4K canvas for fullscreen sprites, and position keyframes match the dimensions of 4K textures.
- Sprite Studio's editor and the viewer [included with the SDK](https://github.com/SpriteStudio/SpriteStudio6-SDK/) render certain elements differently (the viewer doesn't seem to handle masking elements).