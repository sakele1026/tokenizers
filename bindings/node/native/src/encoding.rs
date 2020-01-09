extern crate tokenizers as tk;

use crate::utils::Container;
use neon::prelude::*;

/// Encoding
pub struct Encoding {
    pub encoding: Container<tk::tokenizer::Encoding>,
}

declare_types! {
    pub class JsEncoding for Encoding {
        init(_) {
            // This should never be called from JavaScript
            Ok(Encoding {
                encoding: Container::Empty
            })
        }

        method getIds(mut cx) {
            // getIds(): number[]

            let this = cx.this();
            let guard = cx.lock();
            let ids = this.borrow(&guard).encoding.execute(|encoding| {
                encoding.unwrap().get_ids().to_vec()
            });
            let js_ids = JsArray::new(&mut cx, ids.len() as u32);
            for (i, id) in ids.into_iter().enumerate() {
                let n = JsNumber::new(&mut cx, id as f64);
                js_ids.set(&mut cx, i as u32, n)?;
            }

            Ok(js_ids.upcast())
        }

        method getTypeIds(mut cx) {
            // getTypeIds(): number[]

            let this = cx.this();
            let guard = cx.lock();
            let ids = this.borrow(&guard).encoding.execute(|encoding| {
                encoding.unwrap().get_type_ids().to_vec()
            });
            let js_ids = JsArray::new(&mut cx, ids.len() as u32);
            for (i, id) in ids.into_iter().enumerate() {
                let n = JsNumber::new(&mut cx, id as f64);
                js_ids.set(&mut cx, i as u32, n)?;
            }

            Ok(js_ids.upcast())
        }

        method getAttentionMask(mut cx) {
            // getAttentionMask(): number[]

            let this = cx.this();
            let guard = cx.lock();
            let ids = this.borrow(&guard).encoding.execute(|encoding| {
                encoding.unwrap().get_attention_mask().to_vec()
            });
            let js_ids = JsArray::new(&mut cx, ids.len() as u32);
            for (i, id) in ids.into_iter().enumerate() {
                let n = JsNumber::new(&mut cx, id as f64);
                js_ids.set(&mut cx, i as u32, n)?;
            }

            Ok(js_ids.upcast())
        }

        method getSpecialTokensMask(mut cx) {
            // getSpecialTokensMask(): number[]

            let this = cx.this();
            let guard = cx.lock();
            let ids = this.borrow(&guard).encoding.execute(|encoding| {
                encoding.unwrap().get_special_tokens_mask().to_vec()
            });
            let js_ids = JsArray::new(&mut cx, ids.len() as u32);
            for (i, id) in ids.into_iter().enumerate() {
                let n = JsNumber::new(&mut cx, id as f64);
                js_ids.set(&mut cx, i as u32, n)?;
            }

            Ok(js_ids.upcast())
        }

        method getTokens(mut cx) {
            // getTokens(): string[]

            let this = cx.this();
            let guard = cx.lock();
            let tokens = this.borrow(&guard).encoding.execute(|encoding| {
                encoding.unwrap().get_tokens().to_vec()
            });
            let js_tokens = JsArray::new(&mut cx, tokens.len() as u32);
            for (i, token) in tokens.into_iter().enumerate() {
                let n = JsString::new(&mut cx, token);
                js_tokens.set(&mut cx, i as u32, n)?;
            }

            Ok(js_tokens.upcast())
        }

        method getOffsets(mut cx) {
            // getOffsets(): [number, number][]

            let this = cx.this();
            let guard = cx.lock();
            let offsets = this.borrow(&guard).encoding.execute(|encoding| {
                encoding.unwrap().get_offsets().to_vec()
            });
            let js_offsets = JsArray::new(&mut cx, offsets.len() as u32);
            for (i, offsets) in offsets.into_iter().enumerate() {
                let n = JsArray::new(&mut cx, 2);
                let o_0 = JsNumber::new(&mut cx, offsets.0 as f64);
                let o_1 = JsNumber::new(&mut cx, offsets.1 as f64);
                n.set(&mut cx, 0, o_0)?;
                n.set(&mut cx, 1, o_1)?;
                js_offsets.set(&mut cx, i as u32, n)?;
            }

            Ok(js_offsets.upcast())
        }
    }
}
