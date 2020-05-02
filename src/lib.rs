use sm_ext::{
    cell_t, native, register_natives, HandleError, HandleId, HandleType, IExtension,
    IExtensionInterface, IHandleSys, IPluginContext, IShareSys, SMExtension, TryIntoPlugin,
};
use sourcenav::{get_quad_tree, NavQuadTree};
use std::cell::RefCell;
use std::error::Error;
use std::ffi::CStr;
use std::fs::read;
use std::rc::Rc;

struct NavTree(NavQuadTree);

impl<'ctx> TryIntoPlugin<'ctx> for NavTree {
    type Error = HandleError;

    fn try_into_plugin(self, ctx: &'ctx IPluginContext) -> Result<cell_t, Self::Error> {
        let object = Rc::new(RefCell::new(self));
        let handle = MyExtension::handle_type().create_handle(object, ctx.get_identity(), None)?;

        Ok(handle.into())
    }
}

#[native]
fn native_obj_new(_ctx: &IPluginContext, path: &CStr) -> NavTree {
    let data = read(path.to_str().unwrap()).unwrap();
    NavTree(get_quad_tree(data).unwrap())
}

#[native]
fn native_obj_query(
    ctx: &IPluginContext,
    this: HandleId,
    x: f32,
    y: f32,
    z_guess: f32,
) -> Result<f32, Box<dyn Error>> {
    let this = MyExtension::handle_type().read_handle(this, ctx.get_identity())?;
    let this = this.try_borrow()?;

    Ok(this.0.find_best_height(x, y, z_guess))
}

#[derive(Default, SMExtension)]
#[extension(name = "sourcenav", description = ".nav parser")]
pub struct MyExtension {
    handle_type: Option<HandleType<RefCell<NavTree>>>,
}

impl MyExtension {
    /// Helper to get the extension singleton from the global provided by sm-ext.
    /// This is implemented here rather than by the SMExtension derive to aid code completion.
    fn get() -> &'static Self {
        EXTENSION_GLOBAL.with(|ext| unsafe { &(*ext.borrow().unwrap()).delegate })
    }

    fn handle_type() -> &'static HandleType<RefCell<NavTree>> {
        Self::get().handle_type.as_ref().unwrap()
    }
}

impl IExtensionInterface for MyExtension {
    fn on_extension_load(
        &mut self,
        myself: IExtension,
        sys: IShareSys,
        _late: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let handlesys: IHandleSys = sys.request_interface(&myself)?;

        self.handle_type = Some(handlesys.create_type("NavTree", None, myself.get_identity())?);

        register_natives!(
            &sys,
            &myself,
            [
                ("SourceNav.SourceNav", native_obj_new),
                ("SourceNav.query", native_obj_query),
            ]
        );

        Ok(())
    }

    fn on_extension_unload(&mut self) {
        self.handle_type = None;
    }
}
