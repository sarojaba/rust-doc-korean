// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The document model


use doc;

pub type AstId = int;

#[deriving(Eq)]
pub struct Doc {
    pages: ~[Page]
}

#[deriving(Eq)]
pub enum Page {
    CratePage(CrateDoc),
    ItemPage(ItemTag)
}

#[deriving(Eq)]
pub enum Implementation {
    Required,
    Provided,
}

/**
 * Most rustdocs can be parsed into 'sections' according to their markdown
 * headers
 */
#[deriving(Eq)]
pub struct Section {
    header: ~str,
    body: ~str
}

// FIXME (#2596): We currently give topmod the name of the crate.  There
// would probably be fewer special cases if the crate had its own name
// and topmod's name was the empty string.
#[deriving(Eq)]
pub struct CrateDoc {
    topmod: ModDoc
}

#[deriving(Eq)]
pub enum ItemTag {
    ModTag(ModDoc),
    NmodTag(NmodDoc),
    ConstTag(ConstDoc),
    FnTag(FnDoc),
    EnumTag(EnumDoc),
    TraitTag(TraitDoc),
    ImplTag(ImplDoc),
    TyTag(TyDoc),
    StructTag(StructDoc)
}

#[deriving(Eq)]
pub struct ItemDoc {
    id: AstId,
    name: ~str,
    path: ~[~str],
    brief: Option<~str>,
    desc: Option<~str>,
    sections: ~[Section],
    // Indicates that this node is a reexport of a different item
    reexport: bool
}

#[deriving(Eq)]
pub struct SimpleItemDoc {
    item: ItemDoc,
    sig: Option<~str>
}

#[deriving(Eq)]
pub struct ModDoc {
    item: ItemDoc,
    items: ~[ItemTag],
    index: Option<Index>
}

#[deriving(Eq)]
pub struct NmodDoc {
    item: ItemDoc,
    fns: ~[FnDoc],
    index: Option<Index>
}

pub type ConstDoc = SimpleItemDoc;

pub type FnDoc = SimpleItemDoc;

#[deriving(Eq)]
pub struct EnumDoc {
    item: ItemDoc,
    variants: ~[VariantDoc]
}

#[deriving(Eq)]
pub struct VariantDoc {
    name: ~str,
    desc: Option<~str>,
    sig: Option<~str>
}

#[deriving(Eq)]
pub struct TraitDoc {
    item: ItemDoc,
    methods: ~[MethodDoc]
}

#[deriving(Eq)]
pub struct MethodDoc {
    name: ~str,
    brief: Option<~str>,
    desc: Option<~str>,
    sections: ~[Section],
    sig: Option<~str>,
    implementation: Implementation,
}

#[deriving(Eq)]
pub struct ImplDoc {
    item: ItemDoc,
    bounds_str: Option<~str>,
    trait_types: ~[~str],
    self_ty: Option<~str>,
    methods: ~[MethodDoc]
}

pub type TyDoc = SimpleItemDoc;

#[deriving(Eq)]
pub struct StructDoc {
    item: ItemDoc,
    fields: ~[~str],
    sig: Option<~str>
}

#[deriving(Eq)]
pub struct Index {
    entries: ~[IndexEntry]
}

/**
 * A single entry in an index
 *
 * Fields:
 *
 * * kind - The type of thing being indexed, e.g. 'Module'
 * * name - The name of the thing
 * * brief - The brief description
 * * link - A format-specific string representing the link target
 */
#[deriving(Eq)]
pub struct IndexEntry {
    kind: ~str,
    name: ~str,
    brief: Option<~str>,
    link: ~str
}

impl Doc {
    pub fn CrateDoc(&self) -> CrateDoc {
        self.pages.iter().fold(None, |_m, page| {
            match copy *page {
              doc::CratePage(doc) => Some(doc),
              _ => None
            }
        }).get()
    }

    pub fn cratemod(&self) -> ModDoc {
        copy self.CrateDoc().topmod
    }
}

macro_rules! filt_mapper {
    ($vec:expr, $pat:pat) => {
        do ($vec).iter().filter_map |thing| {
            match thing {
                &$pat => Some(copy *x),
                _ => None
            }
        }.collect()
    }
}

macro_rules! md {
    ($id:ident) => {
        filt_mapper!(self.items, $id(ref x))
    }
}
/// Some helper methods on ModDoc, mostly for testing
impl ModDoc {
    pub fn mods(&self) -> ~[ModDoc] {
        md!(ModTag)
    }

    pub fn nmods(&self) -> ~[NmodDoc] {
        md!(NmodTag)
    }

    pub fn fns(&self) -> ~[FnDoc] {
        md!(FnTag)
    }

    pub fn consts(&self) -> ~[ConstDoc] {
        md!(ConstTag)
    }

    pub fn enums(&self) -> ~[EnumDoc] {
        md!(EnumTag)
    }

    pub fn traits(&self) -> ~[TraitDoc] {
        md!(TraitTag)
    }

    pub fn impls(&self) -> ~[ImplDoc] {
        md!(ImplTag)
    }

    pub fn types(&self) -> ~[TyDoc] {
        md!(TyTag)
    }

    pub fn structs(&self) -> ~[StructDoc] {
        md!(StructTag)
    }
}

macro_rules! pu {
    ($id:ident) => {
        filt_mapper!(*self, ItemPage($id(ref x)))
    }
}

pub trait PageUtils {
    fn mods(&self) -> ~[ModDoc];
    fn nmods(&self) -> ~[NmodDoc];
    fn fns(&self) -> ~[FnDoc];
    fn consts(&self) -> ~[ConstDoc];
    fn enums(&self) -> ~[EnumDoc];
    fn traits(&self) -> ~[TraitDoc];
    fn impls(&self) -> ~[ImplDoc];
    fn types(&self) -> ~[TyDoc];
}

impl PageUtils for ~[Page] {

    fn mods(&self) -> ~[ModDoc] {
        pu!(ModTag)
    }

    fn nmods(&self) -> ~[NmodDoc] {
        pu!(NmodTag)
    }

    fn fns(&self) -> ~[FnDoc] {
        pu!(FnTag)
    }

    fn consts(&self) -> ~[ConstDoc] {
        pu!(ConstTag)
    }

    fn enums(&self) -> ~[EnumDoc] {
        pu!(EnumTag)
    }

    fn traits(&self) -> ~[TraitDoc] {
        pu!(TraitTag)
    }

    fn impls(&self) -> ~[ImplDoc] {
        pu!(ImplTag)
    }

    fn types(&self) -> ~[TyDoc] {
        pu!(TyTag)
    }
}

pub trait Item {
    fn item(&self) -> ItemDoc;
}

impl Item for ItemTag {
    fn item(&self) -> ItemDoc {
        match self {
          &doc::ModTag(ref doc) => copy doc.item,
          &doc::NmodTag(ref doc) => copy doc.item,
          &doc::FnTag(ref doc) => copy doc.item,
          &doc::ConstTag(ref doc) => copy doc.item,
          &doc::EnumTag(ref doc) => copy doc.item,
          &doc::TraitTag(ref doc) => copy doc.item,
          &doc::ImplTag(ref doc) => copy doc.item,
          &doc::TyTag(ref doc) => copy doc.item,
          &doc::StructTag(ref doc) => copy doc.item
        }
    }
}

impl Item for SimpleItemDoc {
    fn item(&self) -> ItemDoc { copy self.item }
}

impl Item for ModDoc {
    fn item(&self) -> ItemDoc { copy self.item }
}

impl Item for NmodDoc {
    fn item(&self) -> ItemDoc { copy self.item }
}

impl Item for EnumDoc {
    fn item(&self) -> ItemDoc { copy self.item }
}

impl Item for TraitDoc {
    fn item(&self) -> ItemDoc { copy self.item }
}

impl Item for ImplDoc {
    fn item(&self) -> ItemDoc { copy self.item }
}

impl Item for StructDoc {
    fn item(&self) -> ItemDoc { copy self.item }
}

pub trait ItemUtils {
    fn id(&self) -> AstId;
    fn name(&self) -> ~str;
    fn path(&self) -> ~[~str];
    fn brief(&self) -> Option<~str>;
    fn desc(&self) -> Option<~str>;
    fn sections(&self) -> ~[Section];
}

impl<A:Item> ItemUtils for A {
    fn id(&self) -> AstId {
        self.item().id
    }

    fn name(&self) -> ~str {
        copy self.item().name
    }

    fn path(&self) -> ~[~str] {
        copy self.item().path
    }

    fn brief(&self) -> Option<~str> {
        copy self.item().brief
    }

    fn desc(&self) -> Option<~str> {
        copy self.item().desc
    }

    fn sections(&self) -> ~[Section] {
        copy self.item().sections
    }
}
