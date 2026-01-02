//! Module for processing Mermaid diagrams.
//!
//!

use std::path::PathBuf;

use crate::handlebar_helpers::get_handlebar;
use crate::mermaid_of_frame::mermaid_of_frame;
use crate::model::{AnimateData, DiagramMeta};
use mdbook_preprocessor::PreprocessorContext;
use regex::Regex;

pub fn trim_html(s: String) -> String {
    s.trim()
        .to_string()
        .replace("&gt;", ">")
        .replace("&lt;", "<")
        .replace("&quot;", "\"")
}

pub(crate) fn extract_meta(diagram: &str) -> anyhow::Result<(DiagramMeta, String)> {
    let re = Regex::new(r#"(?ms)<pre.*?>.*---\n(.*?)\n---(.*?)</pre>"#).unwrap();
    if let Some(captures) = re.captures(diagram) {
        if let Some(data) = Some(captures[1].to_string()) {
            // dbg!("extracted meta: {}", &data);
            let meta: DiagramMeta = serde_yaml::from_str(&data)?;
            Ok((meta, captures[2].to_string()))
        } else {
            Err(anyhow::anyhow!("failed to extract diagram meta"))
        }
    } else {
        Err(anyhow::anyhow!("no diagram meta found"))
    }
}

pub fn process_diagram(
    counter: usize,
    ctx: &PreprocessorContext,
    htmldiv: &str,
) -> anyhow::Result<String> {
    let re: Regex = Regex::new(
        r#"(?ms)<pre (?<prebefore>.*?)class=\"mermaid\"(?<preafter>.*?)>(?<diagram>.*?)</pre>"#,
    )?;
    let caps = re
        .captures(htmldiv)
        .ok_or_else(|| anyhow::anyhow!("failed to capture mermaid diagram"))?;
    let prebefore = &caps["prebefore"];
    let preafter = &caps["preafter"];
    // let diagram = &caps["diagram"];
    let extract = extract_meta(htmldiv);
    if extract.is_err() {
        log::error!(
            "could not extract meta: {} {} ",
            htmldiv,
            extract.err().unwrap(),
        );
        return Err(anyhow::anyhow!("failed to extract meta"));
        // return Ok(htmldiv.to_string());
    }
    let mut srcdir = PathBuf::from(&ctx.root);
    srcdir.push(ctx.config.book.src.clone());
    let (diagram_meta, data) = extract.unwrap();
    let animate_data = match (&diagram_meta.animate, &diagram_meta.animate_yml_file) {
        (Some(meta), None) => meta.clone(),
        (None, Some(yml_file)) => {
            let mut whole_path_ymlfile = srcdir.clone();
            whole_path_ymlfile.push(yml_file.clone());
            log::info!("Loading animation metadata from file: {}", &yml_file);
            let yml_content = std::fs::read_to_string(&whole_path_ymlfile)?;
            let meta_from_file: AnimateData = serde_yaml::from_str(&yml_content)?;
            meta_from_file
        }
        (Some(_), Some(_)) => {
            log::error!(
                "Both inline animation metadata and animation YAML file specified; using inline metadata"
            );
            return Err(anyhow::anyhow!(
                "Both inline animation metadata and animation YAML file specified"
            ));
        }
        (None, None) => {
            return Ok(htmldiv.to_string());
        }
    };

    let frames = animate_data.frames.clone();
    // log::info!("frames {:?}", frames);
    // let frames = get_frames(&data);
    let nb_frames = frames.len() as u8;
    log::debug!(
        "Processing animated mermaid diagram with meta: {:?}",
        &animate_data
    );

    let template_script = String::from_utf8(include_bytes!("script.html").to_vec())?;
    let mut h = get_handlebar()?;
    h.register_template_string("t1", &template_script)?;
    let id = uuid::Uuid::new_v4().to_string();
    #[derive(serde::Serialize, Clone)]
    struct X {
        id: String,
        number_of_frames: u8,
        delay: u16,
    }
    let x = X {
        id: id.clone(),
        number_of_frames: nb_frames,
        delay: animate_data.delay,
    };
    let rendered_script = h.render("t1", &x)?;

    let mermaid_class = "mermaid";

    let mut ret = format!(
        r###"
{rendered_script}
<div>
<button id="start-{id}" class="mermaid-animate">1st</button>
<button id="backward-{id}" class="mermaid-animate">Back</button>
<button id="forward-{id}" class="mermaid-animate">Next</button>
<button id="loop-{id}" class="mermaid-animate">Loop</button>
<button id="stop-{id}" class="mermaid-animate">Stop</button>
</div>
"###
    );
    for (i, frame) in frames.iter().enumerate() {
        let i = i + 1;
        let title = trim_html(frame.title.clone());
        ret.push_str(&format!(
            r###"


<div id="{id}-{i}" class="mermaid-frame">

<a id="anchor-animated-mermaid-{counter}-{i}"></a>

<a href="#anchor-animated-mermaid-{counter}-{i}">here</a>


<h3>Frame {i} / {nb_frames}</h3>




<pre {prebefore}class="{mermaid_class}"{preafter}>
---
{}
---

{}

</pre>

{title}


</div>

"###,
            serde_yaml::to_string(&diagram_meta)?,
            trim_html(mermaid_of_frame(data.clone(), &animate_data, i - 1)?)
        ));
    }
    log::debug!("processed diagram: {}", &ret);
    Ok(ret.clone())
}
