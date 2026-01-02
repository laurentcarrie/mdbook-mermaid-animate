use std::collections::HashMap;

use crate::model as M;

pub(crate) fn tags_of_frame(meta: &M::AnimateData) -> Vec<HashMap<String, String>> {
    let mut ret: Vec<HashMap<String, String>> = Vec::new();
    let mut current = HashMap::new();
    for frame in &meta.frames {
        for toggle in &frame.toggles {
            let parts: Vec<&str> = toggle.splitn(2, '@').collect();
            if parts.len() == 2 {
                let tag_name = parts[0];
                if !meta.tags.contains(&tag_name.to_string()) {
                    log::error!("Tag not found: {}", &tag_name);
                    continue;
                }
                let tag_action = parts[1];
                'inner: {
                    for variant in &meta.variants {
                        if variant.name == tag_action {
                            let value = variant.value.clone();
                            let value = value.replace("{tag}", tag_name);
                            current.insert(tag_name.to_string(), value.clone());
                            break 'inner;
                        }
                    }
                    log::error!("Variant not found: {}@{}", &tag_name, &tag_action);
                }
            } else {
                log::error!("invalid tag format: {}", &toggle);
            }
        }
        ret.push(current.clone());
    }

    ret
}

pub(crate) fn mermaid_of_frame(
    mermaid: String,
    meta: &M::AnimateData,
    frame_index: usize,
) -> anyhow::Result<String> {
    let mut mermaid = mermaid;
    // let frame = &meta.frames.get(frame_index).unwrap();
    let binding = tags_of_frame(meta);
    let tags = binding
        .get(frame_index)
        .ok_or(anyhow::anyhow!("Frame index out of bounds"))?;
    for (tag_name, tag_variant) in tags.iter() {
        let action_str = format!("%% mermaid-animate-tag {tag_name}",);
        mermaid = mermaid.replace(&action_str, tag_variant);
    }
    log::debug!(
        "final mermaid diagram for frame {}:\n{}",
        frame_index + 1,
        &mermaid
    );
    Ok(mermaid.clone())
}
