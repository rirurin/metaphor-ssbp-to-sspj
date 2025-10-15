use std::error::Error;
use std::io::Write;
use std::io::{Cursor, Seek, SeekFrom};
use quick_xml::events::BytesText;
use quick_xml::Writer;
use crate::anime::Anime;
use crate::anime::PartType::effect;
use crate::cell::{tex_pack_settings_to_xml, CellEntry, InterpolateType, TexFilterMode, TexWrapMode};
use crate::effect::Effect;
use crate::util::{create_blank_element, create_name_list, to_xml_anime_settings, Ptr, StringPtr};

#[repr(C)]
#[derive(Debug)]
pub struct ProjectHeader {
    data_id: u32,
    version: u32,
    flags: u32,
    image_base_dir: StringPtr,
    pub(crate) cell: Ptr<CellEntry>,
    pub(crate) anime_pack_data: Ptr<Anime>,
    pub(crate) effect_file: Ptr<Effect>,
    pub(crate) num_cells: u16,
    pub(crate) num_anime_packs: u16,
    pub(crate) num_effect_file_list: u16,
    pub(crate) num_sequence_packs: u16
}

const _: () = {
    ["Size of ProjectHeader"][size_of::<ProjectHeader>() - 0x24];
};

impl ProjectHeader {
    pub fn get_cells(&self, binary: &[u8]) -> &[CellEntry] {
        self.cell.array(binary, self.num_cells as usize)
    }
    pub fn get_num_cells(&self) -> u16 {
        self.num_cells
    }
    pub fn get_anime(&self, binary: &[u8]) -> &[Anime] {
        self.anime_pack_data.array(binary, self.num_anime_packs as usize)
    }
    pub fn get_num_anime(&self) -> u16 {
        self.num_anime_packs
    }
    pub fn get_effects(&self, binary: &[u8]) -> &[Effect] {
        self.effect_file.array(binary, self.num_effect_file_list as usize)
    }
    pub fn get_num_effects(&self) -> u16 {
        self.num_effect_file_list
    }

    pub fn to_xml(&self, name: &str, cell_names: &[String],
        anime_names: &[String], effect_names: &[String]) -> Result<Vec<u8>, Box<dyn Error>> {
        let xml_fmt = "<?xml version=\"1.0\" encoding=\"utf-8\" standalone=\"yes\"?>\n";
        let mut cursor = Cursor::new(xml_fmt.as_bytes().to_vec());
        cursor.seek(SeekFrom::End(0))?;
        let mut writer = Writer::new_with_indent(cursor, '\t' as u8, 1);
        writer.create_element("SpriteStudioProject")
            .with_attributes([("version", "2.00.00")])
            .write_inner_content(|writer| {
                writer.create_element("name")
                    .write_text_content(BytesText::new(name))?;
                create_blank_element(writer, "exportPath")?;
                writer.create_element("settings")
                    .write_inner_content(|writer| self.settings_to_xml(writer))?;
                writer.create_element("animeSettings")
                    .write_inner_content(|writer| to_xml_anime_settings(writer))?;
                tex_pack_settings_to_xml(writer)?;
                create_name_list("cellmapNames", cell_names, writer)?;
                create_name_list("animepackNames", anime_names, writer)?;
                create_name_list("effectFileNames", effect_names, writer)?;
                create_blank_element(writer, "lastAnimeFile")?;
                create_blank_element(writer, "lastAnimeName")?;
                create_blank_element(writer, "lastPart")?;
                create_blank_element(writer, "lastCellMapFile")?;
                create_blank_element(writer, "lastCell")?;
                create_blank_element(writer, "lastEffectMode")?;
            writer.create_element("setupmode")
                    .write_text_content(BytesText::new("0"))?;
                writer.create_element("expandAnimation").write_empty()?;
                writer.create_element("expandSequence").write_empty()?;
                Ok(())
            })?;
        Ok(writer.into_inner().into_inner())
    }

    fn settings_to_xml<W>(&self, writer: &mut Writer<W>) -> std::io::Result<()>
    where W: Write + Seek {
        create_blank_element(writer, "animeBaseDirectory")?;
        create_blank_element(writer, "cellMapBaseDirectory")?;
        create_blank_element(writer, "imageBaseDirectory")?;
        create_blank_element(writer, "effectBaseDirectory")?;
        writer.create_element("exportBaseDirectory")
            .write_text_content(BytesText::new("Export"))?;
        writer.create_element("queryExportBaseDirectory")
            .write_text_content(BytesText::new("1"))?;
        writer.create_element("copyWhenImportImageIsOutside")
            .write_text_content(BytesText::new("1"))?;
        writer.create_element("exportAnimeFileFormat")
            .write_text_content(BytesText::new("SSAX"))?;
        writer.create_element("exportCellMapFileFormat")
            .write_text_content(BytesText::new("invalid"))?;
        writer.create_element("exportCellMap")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("copyImageWhenExportCellmap")
            .write_text_content(BytesText::new("1"))?;
        create_blank_element(writer, "ssConverterOptions")?;
        writer.create_element("player")
            .write_text_content(BytesText::new("any"))?;
        create_blank_element(writer, "signal")?;
        writer.create_element("strictVer4")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("dontUseMatrixForTransform")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("rootPartFunctionAsVer4")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("interpolateColorBlendAsVer4")
            .write_text_content(BytesText::new("1"))?;
        writer.create_element("interpolateVertexOffsetAsVer4")
            .write_text_content(BytesText::new("1"))?;
        writer.create_element("restrictXYAsInteger")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("inheritRatesNoKeySave")
            .write_text_content(BytesText::new("1"))?;
        writer.create_element("availableInterpolationTypes")
            .write_inner_content(|writer| {
                for interp in (0..6)
                    .map(|i| TryInto::<InterpolateType>::try_into(i).unwrap()) {
                    writer.create_element("item")
                        .write_text_content(BytesText::new(&format!("{:?}", interp)))?;
                }
                Ok(())
            })?;
        writer.create_element("availableAttributes")
            .write_inner_content(|writer| {
                for attr in ["CELL", "POSX", "POSY", "POSZ", "ROTX", "ROTY", "ROTZ", "SCLX", "SCLY",
                "LSCX", "LSCY", "ALPH", "LALP", "PRIO", "IFLH", "IFLV", "FLPH", "FLPV", "HIDE",
                "PCOL", "VCOL", "VERT", "PVTX", "PVTY", "ANCX", "ANCY", "SIZX", "SIZY", "UVTX",
                "UVTY", "UVRZ", "UVSX", "UVSY", "BNDR", "MASK", "USER", "IPRM", "EFCT"] {
                    writer.create_element("item")
                        .write_text_content(BytesText::new(attr))?;
                }
                Ok(())
            })?;
        writer.create_element("availableFeatures")
            .write_inner_content(|writer| {
                for attr in ["bone", "effect", "mask", "mesh"] {
                    writer.create_element("value")
                        .write_text_content(BytesText::new(attr))?;
                }
                Ok(())
            })?;
        writer.create_element("defaultSetAttributes")
            .write_inner_content(|writer| {
                for attr in ["POSX", "POSY", "ROTZ", "PRIO", "HIDE"] {
                    writer.create_element("item")
                        .write_text_content(BytesText::new(attr))?;
                }
                Ok(())
            })?;
        writer.create_element("wrapMode")
            .write_text_content(BytesText::new(&format!("{:?}", TexWrapMode::clamp)))?;
        writer.create_element("filterMode")
            .write_text_content(BytesText::new(&format!("{:?}", TexFilterMode::linear)))?;
        writer.create_element("interpolateMode")
            .write_text_content(BytesText::new(&format!("{:?}", InterpolateType::linear)))?;
        writer.create_element("coordUnit")
            .write_text_content(BytesText::new("rate"))?;
        writer.create_element("renderingSettings")
            .write_inner_content(|writer| self.render_settings_to_xml(writer))?;
        writer.create_element("effectSettings")
            .write_inner_content(|writer| {
                writer.create_element("gridSize")
                    .write_text_content(BytesText::new("50"))?;
                Ok(())
            })?;
        writer.create_element("cellTags").write_empty()?;
        writer.create_element("useDecimalDigit")
            .write_text_content(BytesText::new("2"))?;
        writer.create_element("opacifyOutsideCanvasFrame")
            .write_text_content(BytesText::new("1"))?;
        writer.create_element("convertImageToPMA")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("blendImageAsPMA")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("unpremultiplyAlpha")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("vertexAnimeFloat")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("allowNPOT")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("maxLoadableImageWidth")
            .write_text_content(BytesText::new("8192"))?;
        writer.create_element("maxLoadableImageHeight")
            .write_text_content(BytesText::new("8192"))?;
        writer.create_element("maxLoadableImageFileSize")
            .write_text_content(BytesText::new("73400320"))?;
        writer.create_element("instanceStackMax")
            .write_text_content(BytesText::new("100"))?;
        writer.create_element("selectedAttrSelPreset")
            .write_text_content(BytesText::new("0"))?;
        for i in 'A'..'K' {
            create_blank_element(writer, &format!("attrSelPresetName{}", i))?;
            writer.create_element(&format!("attrSelPreset{}", i)).write_empty()?;
        }
        Ok(())
    }

    fn render_settings_to_xml<W>(&self, writer: &mut Writer<W>) -> std::io::Result<()>
    where W: Write + Seek {
        create_blank_element(writer, "outputFolder")?;
        writer.create_element("outputType")
            .write_text_content(BytesText::new("AVI"))?;
        writer.create_element("bgColor")
            .write_text_content(BytesText::new("FF606060"))?;
        writer.create_element("addAnimeName")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("addTimeStamp")
            .write_text_content(BytesText::new("0"))?;
        writer.create_element("addAlphaChannel")
            .write_text_content(BytesText::new("1"))?;
        writer.create_element("imageSizeRatioW")
            .write_text_content(BytesText::new("100"))?;
        writer.create_element("imageSizeRatioH")
            .write_text_content(BytesText::new("100"))?;
        writer.create_element("imageSizeRatioFix")
            .write_text_content(BytesText::new("1"))?;
        writer.create_element("imageSizeIsPixcel")
            .write_text_content(BytesText::new("1"))?;
        for i in 0..4 {
            writer.create_element(&format!("imageSizeExpansion{}", i))
                .write_text_content(BytesText::new("0"))?;
        }
        writer.create_element("webpSettings")
            .write_inner_content(|writer| {
            writer.create_element("lossyType")
                .write_text_content(BytesText::new("lossless"))?;
                writer.create_element("qualityFactor")
                    .write_text_content(BytesText::new("75"))?;
                writer.create_element("compMethod")
                    .write_text_content(BytesText::new("4"))?;
                writer.create_element("useLosslessPreset")
                    .write_text_content(BytesText::new("1"))?;
                writer.create_element("losslessPreset")
                    .write_text_content(BytesText::new("0"))?;
                Ok(())
            })?;
        Ok(())
    }
}