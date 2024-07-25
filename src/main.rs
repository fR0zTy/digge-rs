mod camera;

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use camera::CameraPlugin;

// Map size represents the number of tiles in each direction
const MAP_XSIZE: u32 = 64;
const MAP_YSIZE: u32 = 64;

// Tile size represents the number of world units per tile
// Since by default bevy maps 1 world unit to 1 pixel. Using them interchangably is fine.
// NOTE: This size must match with the texture size for default ScalingMode.
const TILE_XSIZE: f32 = 32.0;
const TILE_YSIZE: f32 = 32.0;

fn spawn_tilemap(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // Create a map size that represents the bounds of the map
    let tilemap_size = TilemapSize {
        x: MAP_XSIZE,
        y: MAP_YSIZE,
    };
    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(tilemap_size);

    (0..MAP_XSIZE).for_each(|x| {
        (0..MAP_YSIZE).for_each(|y| {
            let position = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&position, tile_entity);
        })
    });

    // The default for Bevy 2D cameras is to have 1 screen pixel correspond to 1 world unit.
    // So this would create an entity which can directly map a 16x16 image.
    let tile_size = TilemapTileSize {
        x: TILE_XSIZE,
        y: TILE_YSIZE,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    // Load a texture to be used for displaying the tiles;
    let texture_handle: Handle<Image> = asset_server.load("tile.png");
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&tilemap_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, spawn_tilemap)
        .run();
}
