use std::env;
use std::fs;
use std::process;
use ttf_parser::Face;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <font-file>", args[0]);
        eprintln!("Example: {} /path/to/font.ttf", args[0]);
        process::exit(1);
    }

    let font_path = &args[1];

    let font_data = match fs::read(font_path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading font file '{}': {}", font_path, e);
            process::exit(1);
        }
    };

    let face = match Face::parse(&font_data, 0) {
        Ok(face) => face,
        Err(e) => {
            eprintln!("Error parsing font file '{}': {}", font_path, e);
            process::exit(1);
        }
    };

    print_font_info(&face, font_path);
}

fn get_name(face: &Face, name_id: u16) -> Option<String> {
    face.names()
        .into_iter()
        .filter(|n| n.name_id == name_id)
        .find_map(|n| n.to_string())
}

fn print_font_info(face: &Face, path: &str) {
    println!("╔═══════════════════════════════════════════════════════════════");
    println!("║ FONT INFORMATION");
    println!("╠═══════════════════════════════════════════════════════════════");
    println!("║ File: {}", path);
    println!("╚═══════════════════════════════════════════════════════════════");
    println!();

    // Basic font names
    println!("┌─ FONT NAMES ──────────────────────────────────────────────────");

    let mut found_any_name = false;

    if let Some(family) = get_name(&face, ttf_parser::name_id::FAMILY) {
        println!("│ Family Name:      {}", family);
        found_any_name = true;
    }

    if let Some(subfamily) = get_name(&face, ttf_parser::name_id::SUBFAMILY) {
        println!("│ Subfamily:        {}", subfamily);
        found_any_name = true;
    }

    if let Some(full_name) = get_name(&face, ttf_parser::name_id::FULL_NAME) {
        println!("│ Full Name:        {}", full_name);
        found_any_name = true;
    }

    if let Some(postscript) = get_name(&face, ttf_parser::name_id::POST_SCRIPT_NAME) {
        println!("│ PostScript Name:  {}", postscript);
        found_any_name = true;
    }

    if let Some(version) = get_name(&face, 5) {
        println!("│ Version:          {}", version);
        found_any_name = true;
    }

    if !found_any_name {
        println!("│ No standard name entries found");
        println!("│");
        println!("│ Available names:");
        for name in face.names() {
            if let Some(name_str) = name.to_string() {
                println!("│   [ID {}] {}", name.name_id, name_str);
            }
        }
    }

    println!("└───────────────────────────────────────────────────────────────");
    println!();

    // Font metrics
    println!("┌─ FONT METRICS ────────────────────────────────────────────────");
    println!("│ Units per EM:     {}", face.units_per_em());
    println!("│ Ascender:         {}", face.ascender());
    println!("│ Descender:        {}", face.descender());
    println!("│ Line Gap:         {}", face.line_gap());
    println!("│ Glyph Count:      {}", face.number_of_glyphs());
    println!("│ Is Monospaced:    {}", face.is_monospaced());
    println!("│ Is Bold:          {}", face.is_bold());
    println!("│ Is Italic:        {}", face.is_italic());
    println!("│ Is Oblique:       {}", face.is_oblique());
    println!("│ Weight:           {}", face.weight().to_number());
    println!("│ Width:            {:?}", face.width());
    println!("└───────────────────────────────────────────────────────────────");
    println!();

    // OpenType features (GSUB - Glyph Substitution)
    println!("┌─ OPENTYPE FEATURES (GSUB - Glyph Substitution) ───────────────");
    let mut gsub_features = Vec::new();

    if let Some(gsub) = face.tables().gsub {
        for script in gsub.scripts {
            for lang_sys in script.languages {
                for feature_index in lang_sys.feature_indices {
                    if let Some(feature) = gsub.features.get(feature_index) {
                        let tag = feature.tag.to_string();
                        if !gsub_features.contains(&tag) {
                            gsub_features.push(tag);
                        }
                    }
                }
            }

            if let Some(default_lang) = script.default_language {
                for feature_index in default_lang.feature_indices {
                    if let Some(feature) = gsub.features.get(feature_index) {
                        let tag = feature.tag.to_string();
                        if !gsub_features.contains(&tag) {
                            gsub_features.push(tag);
                        }
                    }
                }
            }
        }
    }

    if gsub_features.is_empty() {
        println!("│ No GSUB features found");
    } else {
        gsub_features.sort();
        for (i, feature) in gsub_features.iter().enumerate() {
            let prefix = if i == 0 { "│ Features:" } else { "│          " };
            println!("{} {} - {}", prefix, feature, describe_opentype_feature(feature));
        }
    }
    println!("└───────────────────────────────────────────────────────────────");
    println!();

    // OpenType features (GPOS - Glyph Positioning)
    println!("┌─ OPENTYPE FEATURES (GPOS - Glyph Positioning) ────────────────");
    let mut gpos_features = Vec::new();

    if let Some(gpos) = face.tables().gpos {
        for script in gpos.scripts {
            for lang_sys in script.languages {
                for feature_index in lang_sys.feature_indices {
                    if let Some(feature) = gpos.features.get(feature_index) {
                        let tag = feature.tag.to_string();
                        if !gpos_features.contains(&tag) {
                            gpos_features.push(tag);
                        }
                    }
                }
            }

            if let Some(default_lang) = script.default_language {
                for feature_index in default_lang.feature_indices {
                    if let Some(feature) = gpos.features.get(feature_index) {
                        let tag = feature.tag.to_string();
                        if !gpos_features.contains(&tag) {
                            gpos_features.push(tag);
                        }
                    }
                }
            }
        }
    }

    if gpos_features.is_empty() {
        println!("│ No GPOS features found");
    } else {
        gpos_features.sort();
        for (i, feature) in gpos_features.iter().enumerate() {
            let prefix = if i == 0 { "│ Features:" } else { "│          " };
            println!("{} {} - {}", prefix, feature, describe_opentype_feature(feature));
        }
    }
    println!("└───────────────────────────────────────────────────────────────");
    println!();

    // Scripts supported
    println!("┌─ SUPPORTED SCRIPTS ───────────────────────────────────────────");
    let mut scripts = Vec::new();

    if let Some(gsub) = face.tables().gsub {
        for script in gsub.scripts {
            let tag = script.tag.to_string();
            if !scripts.contains(&tag) {
                scripts.push(tag);
            }
        }
    }

    if let Some(gpos) = face.tables().gpos {
        for script in gpos.scripts {
            let tag = script.tag.to_string();
            if !scripts.contains(&tag) {
                scripts.push(tag);
            }
        }
    }

    if scripts.is_empty() {
        println!("│ No script information found");
    } else {
        scripts.sort();
        for (i, script) in scripts.iter().enumerate() {
            let prefix = if i == 0 { "│ Scripts:" } else { "│         " };
            println!("{} {}", prefix, script);
        }
    }
    println!("└───────────────────────────────────────────────────────────────");
}

fn describe_opentype_feature(tag: &str) -> &'static str {
    match tag {
        "aalt" => "Access All Alternates",
        "abvf" => "Above-base Forms",
        "abvm" => "Above-base Mark Positioning",
        "abvs" => "Above-base Substitutions",
        "afrc" => "Alternative Fractions",
        "akhn" => "Akhand",
        "blwf" => "Below-base Forms",
        "blwm" => "Below-base Mark Positioning",
        "blws" => "Below-base Substitutions",
        "calt" => "Contextual Alternates",
        "case" => "Case-Sensitive Forms",
        "ccmp" => "Glyph Composition/Decomposition",
        "cfar" => "Conjunct Form After Ro",
        "cjct" => "Conjunct Forms",
        "clig" => "Contextual Ligatures",
        "cpct" => "Centered CJK Punctuation",
        "cpsp" => "Capital Spacing",
        "cswh" => "Contextual Swash",
        "curs" => "Cursive Positioning",
        "cv01" => "Character Variant 1",
        "cv02" => "Character Variant 2",
        "cv03" => "Character Variant 3",
        "cv04" => "Character Variant 4",
        "cv05" => "Character Variant 5",
        "cv99" => "Character Variant 99",
        "c2pc" => "Petite Capitals From Capitals",
        "c2sc" => "Small Capitals From Capitals",
        "dist" => "Distances",
        "dlig" => "Discretionary Ligatures",
        "dnom" => "Denominators",
        "dtls" => "Dotless Forms",
        "expt" => "Expert Forms",
        "falt" => "Final Glyph on Line Alternates",
        "fin2" => "Terminal Forms #2",
        "fin3" => "Terminal Forms #3",
        "fina" => "Terminal Forms",
        "flac" => "Flattened accent forms",
        "frac" => "Fractions",
        "fwid" => "Full Widths",
        "half" => "Half Forms",
        "haln" => "Halant Forms",
        "halt" => "Alternate Half Widths",
        "hist" => "Historical Forms",
        "hkna" => "Horizontal Kana Alternates",
        "hlig" => "Historical Ligatures",
        "hngl" => "Hangul",
        "hojo" => "Hojo Kanji Forms",
        "hwid" => "Half Widths",
        "init" => "Initial Forms",
        "isol" => "Isolated Forms",
        "ital" => "Italics",
        "jalt" => "Justification Alternates",
        "jp78" => "JIS78 Forms",
        "jp83" => "JIS83 Forms",
        "jp90" => "JIS90 Forms",
        "jp04" => "JIS2004 Forms",
        "kern" => "Kerning",
        "lfbd" => "Left Bounds",
        "liga" => "Standard Ligatures",
        "ljmo" => "Leading Jamo Forms",
        "lnum" => "Lining Figures",
        "locl" => "Localized Forms",
        "ltra" => "Left-to-right alternates",
        "ltrm" => "Left-to-right mirrored forms",
        "mark" => "Mark Positioning",
        "med2" => "Medial Forms #2",
        "medi" => "Medial Forms",
        "mgrk" => "Mathematical Greek",
        "mkmk" => "Mark to Mark Positioning",
        "mset" => "Mark Positioning via Substitution",
        "nalt" => "Alternate Annotation Forms",
        "nlck" => "NLC Kanji Forms",
        "nukt" => "Nukta Forms",
        "numr" => "Numerators",
        "onum" => "Oldstyle Figures",
        "opbd" => "Optical Bounds",
        "ordn" => "Ordinals",
        "ornm" => "Ornaments",
        "palt" => "Proportional Alternate Widths",
        "pcap" => "Petite Capitals",
        "pkna" => "Proportional Kana",
        "pnum" => "Proportional Figures",
        "pref" => "Pre-Base Forms",
        "pres" => "Pre-base Substitutions",
        "pstf" => "Post-base Forms",
        "psts" => "Post-base Substitutions",
        "pwid" => "Proportional Widths",
        "qwid" => "Quarter Widths",
        "rand" => "Randomize",
        "rclt" => "Required Contextual Alternates",
        "rkrf" => "Rakar Forms",
        "rlig" => "Required Ligatures",
        "rphf" => "Reph Forms",
        "rtbd" => "Right Bounds",
        "rtla" => "Right-to-left alternates",
        "rtlm" => "Right-to-left mirrored forms",
        "ruby" => "Ruby Notation Forms",
        "rvrn" => "Required Variation Alternates",
        "salt" => "Stylistic Alternates",
        "sinf" => "Scientific Inferiors",
        "size" => "Optical size",
        "smcp" => "Small Capitals",
        "smpl" => "Simplified Forms",
        "ss01" => "Stylistic Set 1",
        "ss02" => "Stylistic Set 2",
        "ss03" => "Stylistic Set 3",
        "ss04" => "Stylistic Set 4",
        "ss05" => "Stylistic Set 5",
        "ss06" => "Stylistic Set 6",
        "ss07" => "Stylistic Set 7",
        "ss08" => "Stylistic Set 8",
        "ss09" => "Stylistic Set 9",
        "ss10" => "Stylistic Set 10",
        "ss11" => "Stylistic Set 11",
        "ss12" => "Stylistic Set 12",
        "ss13" => "Stylistic Set 13",
        "ss14" => "Stylistic Set 14",
        "ss15" => "Stylistic Set 15",
        "ss16" => "Stylistic Set 16",
        "ss17" => "Stylistic Set 17",
        "ss18" => "Stylistic Set 18",
        "ss19" => "Stylistic Set 19",
        "ss20" => "Stylistic Set 20",
        "ssty" => "Math script style alternates",
        "stch" => "Stretching Glyph Decomposition",
        "subs" => "Subscript",
        "sups" => "Superscript",
        "swsh" => "Swash",
        "titl" => "Titling",
        "tjmo" => "Trailing Jamo Forms",
        "tnam" => "Traditional Name Forms",
        "tnum" => "Tabular Figures",
        "trad" => "Traditional Forms",
        "twid" => "Third Widths",
        "unic" => "Unicase",
        "valt" => "Alternate Vertical Metrics",
        "vatu" => "Vattu Variants",
        "vert" => "Vertical Writing",
        "vhal" => "Alternate Vertical Half Metrics",
        "vjmo" => "Vowel Jamo Forms",
        "vkna" => "Vertical Kana Alternates",
        "vkrn" => "Vertical Kerning",
        "vpal" => "Proportional Alternate Vertical Metrics",
        "vrt2" => "Vertical Alternates and Rotation",
        "vrtr" => "Vertical Alternates for Rotation",
        "zero" => "Slashed Zero",
        _ => "Unknown feature",
    }
}
