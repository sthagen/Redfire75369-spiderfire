/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

use proc_macro2::Ident;
use syn::{ItemFn, ItemStatic};

pub(crate) fn class_finalise(class: &Ident) -> ItemFn {
	let krate = quote!(::ion);
	parse_quote!(
		unsafe extern "C" fn finalise_operation(_: *mut ::mozjs::jsapi::GCContext, this: *mut ::mozjs::jsapi::JSObject) {
			let mut value = ::mozjs::jsval::UndefinedValue();
			::mozjs::glue::JS_GetReservedSlot(this, <#class as #krate::class::ClassInitialiser>::PARENT_PROTOTYPE_CHAIN_LENGTH, &mut value);
			let private = &mut *(value.to_private() as *mut Option<#class>);
			let _ = private.take();
		}
	)
}

pub(crate) fn class_ops() -> ItemStatic {
	let none = quote!(::std::option::Option::None);
	parse_quote!(
		static OPERATIONS: ::mozjs::jsapi::JSClassOps = ::mozjs::jsapi::JSClassOps {
			addProperty: #none,
			delProperty: #none,
			enumerate: #none,
			newEnumerate: #none,
			resolve: #none,
			mayResolve: #none,
			finalize: ::std::option::Option::Some(finalise_operation),
			call: #none,
			construct: #none,
			trace: #none,
		};
	)
}
