use pax_engine::api::*;
use pax_engine::*;
use pax_std::*;
use std::cell::RefCell;
use std::io::BufRead;
use std::rc::Rc;

#[pax]
#[file("controls/tree/treeobj.pax")]
pub struct TreeObj {
    pub ind: Property<Numeric>,
    pub name: Property<String>,
    pub image_path: Property<String>,
    pub is_selected: Property<bool>,
    pub is_collapsed: Property<bool>,
    pub arrow_path: Property<String>,
    pub is_not_leaf: Property<bool>,
}

impl TreeObj {
    pub fn on_mount(&mut self, _ctx: &NodeContext) {
        let collapsed = self.is_collapsed.clone();
        let deps = [collapsed.untyped()];
        self.arrow_path.replace_with(Property::computed(
            move || {
                match collapsed.get() {
                    true => "assets/icons/triangle-down.png",
                    false => "assets/icons/triangle-right.png",
                }
                .into()
            },
            &deps,
        ));
    }

    pub fn arrow_clicked(&mut self, _ctx: &NodeContext, _args: Event<Click>) {
        super::TREE_CLICK_PROP.with_borrow_mut(|cn| {
            cn.push_back(super::TreeMsg::ArrowClicked(self.ind.get().clone().into()));
        })
    }

    pub fn obj_double_clicked(&mut self, _ctx: &NodeContext, _args: Event<DoubleClick>) {
        super::TREE_CLICK_PROP.with_borrow_mut(|cn| {
            cn.push_back(super::TreeMsg::ObjDoubleClicked(
                self.ind.get().clone().into(),
            ));
        });
    }

    pub fn mouse_down(&mut self, _ctx: &NodeContext, event: Event<MouseDown>) {
        event.prevent_default();
        super::TREE_CLICK_PROP.with_borrow_mut(|cn| {
            cn.push_back(super::TreeMsg::ObjMouseDown(self.ind.get().clone().into()));
        });
    }

    pub fn mouse_move(&mut self, _ctx: &NodeContext, event: Event<MouseMove>) {
        event.prevent_default();
        super::TREE_CLICK_PROP.with_borrow_mut(|cn| {
            cn.push_back(super::TreeMsg::ObjMouseMove(self.ind.get().clone().into()));
        });
    }
}