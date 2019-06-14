use std::sync::Arc;

use amethyst::{
    ecs::{ReadExpect, Resources, SystemData},
    renderer::{
        pass::{DrawFlatDesc, DrawFlat2DDesc, DrawFlat2DTransparentDesc},
        rendy::{
            factory::Factory,
            graph::{
                present::PresentNode,
                render::{RenderGroupDesc, SubpassBuilder},
                GraphBuilder,
            },
            hal::{
                command::{ClearDepthStencil, ClearValue},
                format::Format,
                image::Kind,
            },
        },
        types::DefaultBackend,
        GraphCreator,
    },
    ui::DrawUiDesc,
    window::{ScreenDimensions, Window},
};

#[derive(Default)]
pub struct RenderGraph {
    dimensions: Option<ScreenDimensions>,
    surface_format: Option<Format>,
    dirty: bool,
}

impl GraphCreator<DefaultBackend> for RenderGraph {
    fn rebuild(&mut self, res: &Resources) -> bool {
        // Rebuild when dimensions change, but wait until at least two frames have the same.
        let new_dimensions = res.try_fetch::<ScreenDimensions>();
        use std::ops::Deref;
        if self.dimensions.as_ref() != new_dimensions.as_ref().map(|d| d.deref()) {
            self.dirty = true;
            self.dimensions = new_dimensions.map(|d| d.clone());
            return false;
        }
        return self.dirty;
    }

    fn builder(
        &mut self,
        factory: &mut Factory<DefaultBackend>,
        res: &Resources,
    ) -> GraphBuilder<DefaultBackend, Resources> {
        self.dirty = false;

        let window = <ReadExpect<'_, Arc<Window>>>::fetch(res);
        let surface = factory.create_surface(&window);
        // cache surface format to speed things up
        let surface_format = *self
            .surface_format
            .get_or_insert_with(|| factory.get_surface_format(&surface));
        let dimensions = self.dimensions.as_ref().unwrap();
        let window_kind = Kind::D2(dimensions.width() as u32, dimensions.height() as u32, 1, 1);

        let mut graph_builder = GraphBuilder::new();
        let color = graph_builder.create_image(
            window_kind,
            1,
            surface_format,
            Some(ClearValue::Color([0.1, 0.1, 0.1, 1.0].into())),
        );

        let depth = graph_builder.create_image(
            window_kind,
            1,
            Format::D32Sfloat,
            Some(ClearValue::DepthStencil(ClearDepthStencil(1.0, 0))),
        );

        let sprite = graph_builder.add_node(
            SubpassBuilder::new()
                .with_group(DrawFlatDesc::new().builder())
                .with_color(color)
                .with_depth_stencil(depth)
                .into_pass(),
        );
//        let sprite_trans = graph_builder.add_node(
//            SubpassBuilder::new()
//                .with_group(DrawFlat2DTransparentDesc::new().builder())
//                .with_color(color)
//                .with_depth_stencil(depth)
//                .into_pass(),
//        );
//        let ui = graph_builder.add_node(
//            SubpassBuilder::new()
//                .with_group(DrawUiDesc::new().builder())
//                .with_color(color)
//                .with_depth_stencil(depth)
//                .into_pass(),
//        );
//
        let _present = graph_builder.add_node(
            PresentNode::builder(factory, surface, color)
//                .with_dependency(sprite_trans)
                .with_dependency(sprite),
        );

        graph_builder
    }
}
