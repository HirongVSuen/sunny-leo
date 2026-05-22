use zed_extension_api as zed;

struct SunnyLeoExtension;

impl zed::Extension for SunnyLeoExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(SunnyLeoExtension);
