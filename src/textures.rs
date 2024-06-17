use macroquad::prelude::*;

macro_rules! textures {
    ($($name:ident($kind:ident) = $path:literal),* $(,)*) => {
        pub struct Textures {
            $($name: Texture2D)+
        }

        impl Textures {
            pub async fn load() -> Self {
                Self {
                    $(
                        $name: if cfg!(debug_assertions) {
                            load_texture($path).await.unwrap()
                        } else {
                            Texture2D::from_file_with_format(include_bytes!($path), Some(ImageFormat::$kind))
                        }
                    )+
                }
            }
        }
    };
}

textures! {
    ground(Png) = "textures/ground.png",
}