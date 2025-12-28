//! Module for processing Mermaid diagrams.
//!
//!

use crate::handlebar_helpers::get_handlebar;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
struct AnimateData {
    pub nb_frames: u8,
    pub frames: Vec<Vec<String>>,
    pub delay: u16,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
struct DiagramMeta {
    pub title: Option<String>,
    pub animate: Option<AnimateData>,
}

/// in a mermaid diagram, extract the meta information,
/// what is between the --- markers at the start of the diagram
/// e.g.,
/// ---
/// title: My Diagram
/// nb_frames: 10
/// ---
fn extract_meta(diagram: &str) -> anyhow::Result<(DiagramMeta, String)> {
    let re = Regex::new(r#"(?ms)---\n(.*)\n---(.*)"#).unwrap();
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

fn one_diagram(
    i: u8,
    meta: AnimateData,
    data: String,
    prebefore: String,
    preafter: String,
    diagram: &str,
) -> anyhow::Result<String> {
    let mut ret = format!(
        r###"
<pre {prebefore}class="mermaid"{preafter}>
---

{}

---

{}
"###,
        serde_yaml::to_string(&meta)?,
        data.clone()
    );

    let frame_opt = meta.frames.get((i - 1) as usize);
    let mut replacements: Vec<(String, String)> = vec![];
    let mut data = ret.clone();
    if let Some(frame) = frame_opt {
        // dbg!("frame {} content: {:?}", &i, &frame);
        for tag in frame {
            // dbg!("processing tag: {}", &tag);
            let re = Regex::new(format!(r#"%% +mermaid-animate +{} +(?<body>.*)"#, tag).as_str())
                .unwrap();
            // dbg!("regex: {:?}", &re);
            loop {
                let caps = re.captures(&data);
                if caps.is_none() {
                    break;
                }
                let caps = caps.unwrap();
                let body = &caps["body"];
                // dbg!("frame body: {}", &body);
                let id = uuid::Uuid::new_v4().to_string();
                replacements.push((id.clone(), body.to_string()));
                let mut dst = String::new();
                dst.push_str(&data[..caps.get(0).unwrap().start()]);
                dst.push_str(id.to_string().as_str());
                dst.push_str(&data[caps.get(0).unwrap().end()..]);
                data = dst;
            }
        }
    }

    ret = data.clone();

    for (id, body) in replacements {
        ret = ret.replace(&id, &body);
    }

    ret.push_str(
        r###"
</pre>
    "###,
    );
    // dbg!("one_diagram result: {}", &ret);
    Ok(ret.clone())
}

pub fn process_diagram(htmldiv: &str) -> anyhow::Result<String> {
    // dbg!("htmldiv: {}", &htmldiv);
    let re = Regex::new(
        r#"(?ms)<pre (?<prebefore>.*?)class=\"mermaid\"(?<preafter>.*?)>(?<diagram>.*)</pre>"#,
    )
    .unwrap();
    let caps = re
        .captures(htmldiv)
        .ok_or_else(|| anyhow::anyhow!("failed to capture mermaid diagram"))?;
    let prebefore = &caps["prebefore"];
    let preafter = &caps["preafter"];
    let diagram = &caps["diagram"];
    // dbg!("diagram: {}", &diagram);
    let (meta, data) = extract_meta(diagram)?;
    if meta.animate.is_none() {
        // dbg!("no animation requested, returning original diagram");
        return Ok(htmldiv.to_string());
    }
    log::info!("Processing animated mermaid diagram with meta: {:?}", &meta);
    let meta = meta.animate.unwrap();
    // dbg!("meta: {:?}", &meta);
    // dbg!("data: {}", &data);

    let template_script = String::from_utf8(include_bytes!("script.html").to_vec())?;
    let mut h = get_handlebar()?;
    h.register_template_string("t1", &template_script)?;
    let id = uuid::Uuid::new_v4().to_string();
    #[derive(serde::Serialize, Clone)]
    struct X {
        id: String,
        number_of_frames: u8,
        delay: u16,
    };
    let x = X {
        id: id.clone(),
        number_of_frames: meta.nb_frames,
        delay: meta.delay,
    };
    let rendered_script = h.render("t1", &x)?;

    let mut ret = format!(
        r###"
        {rendered_script}
<!-- details close id="details-{id}" -->
<div>
<button id="start-{id}" class="mermaid-animate">1st</button>
<button id="backward-{id}" class="mermaid-animate">Back</button>
<button id="forward-{id}" class="mermaid-animate">Next</button>
<button id="loop-{id}" class="mermaid-animate">Loop</button>
<button id="stop-{id}" class="mermaid-animate">Stop</button>
</div>
"###
    );
    for i in 1..=meta.nb_frames {
        ret.push_str(&format!(
            r###"
<div id="{id}-{i}" style="display:visible;">
<h3>frame {i} / {}</h3>
        {}
</div>
"###,
            meta.nb_frames,
            one_diagram(
                i,
                meta.clone(),
                data.clone(),
                prebefore.to_string(),
                preafter.to_string(),
                &data
            )?
        ));
    }
    // ret.push_str("</details>");
    // dbg!("final processed diagram: {}", &ret);
    Ok(ret.clone())
}
