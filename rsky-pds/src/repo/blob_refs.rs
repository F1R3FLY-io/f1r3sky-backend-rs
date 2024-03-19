use anyhow::Result;
use libipld::Cid;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TypedJsonBlobRef {
    #[serde(rename = "$type")]
    pub r#type: String, // `blob`
    pub r#ref: Cid,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub size: i128,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UntypedJsonBlobRef {
    pub cid: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum JsonBlobRef {
    Typed(TypedJsonBlobRef),
    Untyped(UntypedJsonBlobRef),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BlobRef {
    pub original: JsonBlobRef,
}

impl BlobRef {
    pub fn new(
        r#ref: Cid,
        mime_type: String,
        size: i128,
        original: Option<JsonBlobRef>,
    ) -> BlobRef {
        if let Some(o) = original {
            BlobRef { original: o }
        } else {
            let o = JsonBlobRef::Typed(TypedJsonBlobRef {
                r#type: "blob".to_owned(),
                r#ref,
                mime_type,
                size,
            });
            BlobRef { original: o }
        }
    }

    pub fn from_json_ref(json: JsonBlobRef) -> Result<BlobRef> {
        match json {
            JsonBlobRef::Typed(j) => Ok(BlobRef::new(j.r#ref, j.mime_type, j.size, None)),
            JsonBlobRef::Untyped(ref j) => Ok(BlobRef::new(
                Cid::from_str(&j.cid)?,
                j.mime_type.clone(),
                -1,
                Some(json),
            )),
        }
    }

    pub fn ipld(&self) -> TypedJsonBlobRef {
        if let JsonBlobRef::Typed(j) = &self.original {
            TypedJsonBlobRef {
                r#type: "blob".to_owned(),
                r#ref: j.r#ref,
                mime_type: j.mime_type.clone(),
                size: j.size,
            }
        } else {
            panic!("Not a TypedJsonBlobRef")
        }
    }
}
